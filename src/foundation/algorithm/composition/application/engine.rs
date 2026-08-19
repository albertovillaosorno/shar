// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Source-bound authoring and replay application behavior.
// - Must-Not:
//   - Own product-specific reconstruction policy.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Algorithm application engine.
// - Description:
//   - Source-bound authoring and replay application behavior.
// - Usage:
//   - Used through the owning algorithm function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Source-bound algorithm authoring and replay service.

use std::collections::{BTreeSet, HashSet};
use std::path::{Component, Path, PathBuf};

use chacha20poly1305::{
    ChaCha20Poly1305, KeyInit, Nonce,
    aead::{Aead, Payload},
};
use schoenwald_filesystem::adapters::driving::local;
use same_file::Handle;
use schoenwald_filesystem::{PathKind, resolve_under, validate_portable_path};
use shar_sha256::{Sha256, digest, digest_hex};

use crate::document::{
    ALGORITHM_SCHEMA, AlgorithmDocument, AuthenticatedMetadata, ProtectedTarget, SourceRecord,
    TargetDescriptor, TargetKind, settings_json_bytes,
};
use crate::domain::{AlgorithmError, Settings};

const SOURCE_KEY_DOMAIN: &[u8] = b"shar.algorithm.source-key.v1\0";
const NONCE_DOMAIN: &[u8] = b"shar.algorithm.nonce.v1\0";
const AAD_DOMAIN: &[u8] = b"shar.algorithm.aad.v1\0";
const SHA256_HEX_LEN: usize = 64;
const PROTECTED_NONCE_HEX_LEN: usize = 24;
const PROTECTED_TAG_BYTES: u64 = 16;
const HEX_CHARS_PER_BYTE: u64 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
struct InputFile {
    input: u64,
    logical_path: String,
    path: PathBuf,
    bytes: u64,
    sha256: String,
    data: Vec<u8>,
}

impl InputFile {
    fn source_record(&self) -> SourceRecord {
        SourceRecord {
            input: self.input,
            path: self.logical_path.clone(),
            bytes: self.bytes,
            sha256: self.sha256.clone(),
        }
    }

    fn target_descriptor(&self) -> TargetDescriptor {
        TargetDescriptor {
            path: self.logical_path.clone(),
            bytes: self.bytes,
            sha256: self.sha256.clone(),
        }
    }
}

#[derive(Debug)]
struct CollectedSource {
    files: Vec<InputFile>,
    roots: Vec<PathBuf>,
}

fn io_failure(context: &str, error: &std::io::Error) -> AlgorithmError {
    AlgorithmError::new(format!("{context}: {:?}", error.kind()))
}

fn usize_to_u64(value: usize, context: &str) -> Result<u64, AlgorithmError> {
    u64::try_from(value).map_err(|_conversion_error| {
        AlgorithmError::new(format!("{context} exceeds 64-bit limits"))
    })
}

fn portable_relative(path: &Path) -> Result<String, AlgorithmError> {
    validate_portable_path(path)
        .map_err(|error| AlgorithmError::new(format!("non-portable relative path: {error}")))?;
    let mut text = String::new();
    for component in path.components() {
        let Component::Normal(part) = component else {
            return Err(AlgorithmError::new("relative path is not canonical"));
        };
        let part = part
            .to_str()
            .ok_or_else(|| AlgorithmError::new("relative path must be valid Unicode"))?;
        if !text.is_empty() {
            text.push('/');
        }
        text.push_str(part);
    }
    if text.is_empty() {
        return Err(AlgorithmError::new("relative path must not be empty"));
    }
    Ok(text)
}

fn inspect_file(
    input: u64,
    logical_path: String,
    path: PathBuf,
    settings: &Settings,
) -> Result<InputFile, AlgorithmError> {
    let bytes = local::file_len(&path)
        .map_err(|error| io_failure("cannot inspect input file", &error))?;
    if bytes > settings.maximum_file_bytes() {
        return Err(AlgorithmError::new("input file exceeds maximum_file_bytes"));
    }
    let data = local::read_bytes(&path)
        .map_err(|error| io_failure("cannot read input file", &error))?;
    let observed = usize_to_u64(data.len(), "input file length")?;
    if observed != bytes {
        return Err(AlgorithmError::new(
            "input file changed while it was being read",
        ));
    }
    Ok(InputFile {
        input,
        logical_path,
        path,
        bytes,
        sha256: digest_hex(&data),
        data,
    })
}

fn collect_one_root(
    input: u64,
    root: &Path,
    settings: &Settings,
) -> Result<(PathBuf, Vec<InputFile>), AlgorithmError> {
    let kind = local::path_kind(root)
        .map_err(|error| io_failure("cannot inspect input path", &error))?;
    let canonical = local::canonicalize(root)
        .map_err(|error| io_failure("cannot canonicalize input path", &error))?;
    match kind {
        PathKind::File => {
            let file = inspect_file(input, String::new(), canonical.clone(), settings)?;
            Ok((canonical, vec![file]))
        }
        PathKind::Directory => {
            let paths = local::strict_regular_files(&canonical)
                .map_err(|error| io_failure("cannot traverse input directory", &error))?;
            let mut files = Vec::with_capacity(paths.len());
            for path in paths {
                let relative = path
                    .strip_prefix(&canonical)
                    .map_err(|_prefix_error| AlgorithmError::new("input file escaped its root"))?;
                let logical = portable_relative(relative)?;
                files.push(inspect_file(input, logical, path, settings)?);
            }
            Ok((canonical, files))
        }
        PathKind::Missing => Err(AlgorithmError::new("input path does not exist")),
        PathKind::Other => Err(AlgorithmError::new(
            "input path must be a regular file or directory",
        )),
    }
}

fn reject_root_overlap(roots: &[PathBuf]) -> Result<(), AlgorithmError> {
    for (index, root) in roots.iter().enumerate() {
        for other in roots.iter().skip(index.saturating_add(1)) {
            if root.starts_with(other) || other.starts_with(root) {
                return Err(AlgorithmError::new("input roots must not overlap"));
            }
        }
    }
    Ok(())
}

fn reject_duplicate_physical_sources(files: &[InputFile]) -> Result<(), AlgorithmError> {
    let mut identities = HashSet::new();
    for file in files {
        let identity = Handle::from_path(&file.path)
            .map_err(|_error| AlgorithmError::new("cannot identify source input file"))?;
        if !identities.insert(identity) {
            return Err(AlgorithmError::new(
                "source inputs repeat one physical file",
            ));
        }
    }
    Ok(())
}

fn reject_duplicate_physical_targets(files: &[InputFile]) -> Result<(), AlgorithmError> {
    let mut identities = HashSet::new();
    for file in files {
        let identity = Handle::from_path(&file.path)
            .map_err(|_error| AlgorithmError::new("cannot identify target input file"))?;
        if !identities.insert(identity) {
            return Err(AlgorithmError::new(
                "target inputs repeat one physical file",
            ));
        }
    }
    Ok(())
}

fn collect_source(
    paths: &[PathBuf],
    settings: &Settings,
) -> Result<CollectedSource, AlgorithmError> {
    if paths.is_empty() {
        return Err(AlgorithmError::new("at least one source path is required"));
    }
    let mut roots = Vec::with_capacity(paths.len());
    let mut files = Vec::new();
    let mut total_bytes = 0_u64;
    for (index, path) in paths.iter().enumerate() {
        let input = usize_to_u64(index, "source input index")?;
        let (root, mut root_files) = collect_one_root(input, path, settings)?;
        roots.push(root);
        for file in &root_files {
            total_bytes = total_bytes.saturating_add(file.bytes);
        }
        files.append(&mut root_files);
    }
    reject_root_overlap(&roots)?;
    reject_duplicate_physical_sources(&files)?;
    let file_count = usize_to_u64(files.len(), "source file count")?;
    if file_count < settings.minimum_source_files() {
        return Err(AlgorithmError::new(
            "source has fewer than minimum_source_files",
        ));
    }
    if file_count > settings.maximum_source_files() {
        return Err(AlgorithmError::new("source exceeds maximum_source_files"));
    }
    if total_bytes < settings.minimum_source_bytes() {
        return Err(AlgorithmError::new(
            "source has fewer than minimum_source_bytes",
        ));
    }
    if total_bytes > settings.maximum_source_bytes() {
        return Err(AlgorithmError::new("source exceeds maximum_source_bytes"));
    }
    Ok(CollectedSource { files, roots })
}

fn collect_target(
    path: &Path,
    settings: &Settings,
) -> Result<(TargetKind, Vec<InputFile>, PathBuf), AlgorithmError> {
    let kind = local::path_kind(path)
        .map_err(|error| io_failure("cannot inspect target path", &error))?;
    let canonical = local::canonicalize(path).map_err(|error| {
        io_failure("cannot canonicalize target path", &error)
    })?;
    let (target_kind, files) = match kind {
        PathKind::File => (
            TargetKind::File,
            vec![inspect_file(0, String::new(), canonical.clone(), settings)?],
        ),
        PathKind::Directory => {
            let paths = local::strict_regular_files(&canonical)
                .map_err(|error| io_failure("cannot traverse target directory", &error))?;
            let mut files = Vec::with_capacity(paths.len());
            for target_file in paths {
                let relative = target_file
                    .strip_prefix(&canonical)
                    .map_err(|_prefix_error| AlgorithmError::new("target file escaped its root"))?;
                let logical = portable_relative(relative)?;
                files.push(inspect_file(0, logical, target_file, settings)?);
            }
            (TargetKind::Directory, files)
        }
        PathKind::Missing => return Err(AlgorithmError::new("target path does not exist")),
        PathKind::Other => {
            return Err(AlgorithmError::new(
                "target path must be a regular file or directory",
            ));
        }
    };
    if files.is_empty() {
        return Err(AlgorithmError::new(
            "target must contain at least one regular file",
        ));
    }
    reject_duplicate_physical_targets(&files)?;
    let file_count = usize_to_u64(files.len(), "target file count")?;
    if file_count > settings.maximum_target_files() {
        return Err(AlgorithmError::new("target exceeds maximum_target_files"));
    }
    let total_bytes = files
        .iter()
        .fold(0_u64, |total, file| total.saturating_add(file.bytes));
    if total_bytes > settings.maximum_target_bytes() {
        return Err(AlgorithmError::new("target exceeds maximum_target_bytes"));
    }
    Ok((target_kind, files, canonical))
}

fn settings_sha256(settings: &Settings) -> Result<String, AlgorithmError> {
    Ok(digest_hex(&settings_json_bytes(settings)?))
}

fn update_frame(state: &mut Sha256, bytes: &[u8]) -> Result<(), AlgorithmError> {
    let length = usize_to_u64(bytes.len(), "hash frame length")?;
    state.update(&length.to_be_bytes());
    state.update(bytes);
    Ok(())
}

fn source_key(files: &[InputFile]) -> Result<[u8; 32], AlgorithmError> {
    let mut state = Sha256::new();
    state.update(SOURCE_KEY_DOMAIN);
    for file in files {
        state.update(&file.input.to_be_bytes());
        update_frame(&mut state, file.logical_path.as_bytes())?;
        update_frame(&mut state, &file.data)?;
    }
    Ok(state.finalize())
}

fn metadata_bytes(
    settings_hash: &str,
    source: &[SourceRecord],
    target_kind: TargetKind,
    target: &[TargetDescriptor],
) -> Result<Vec<u8>, AlgorithmError> {
    let metadata = AuthenticatedMetadata {
        schema: ALGORITHM_SCHEMA,
        settings_sha256: settings_hash,
        source,
        target_kind,
        target,
    };
    serde_json::to_vec(&metadata).map_err(|error| {
        AlgorithmError::new(format!("cannot serialize algorithm metadata: {error}"))
    })
}

fn aad(metadata: &[u8], path: &str) -> Result<Vec<u8>, AlgorithmError> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(AAD_DOMAIN);
    let metadata_hash = digest(metadata);
    bytes.extend_from_slice(&metadata_hash);
    let path_bytes = path.as_bytes();
    let path_length = usize_to_u64(path_bytes.len(), "target path length")?;
    bytes.extend_from_slice(&path_length.to_be_bytes());
    bytes.extend_from_slice(path_bytes);
    Ok(bytes)
}

fn nonce_for(key: &[u8; 32], metadata: &[u8], path: &str) -> Result<[u8; 12], AlgorithmError> {
    let mut state = Sha256::new();
    state.update(NONCE_DOMAIN);
    state.update(key);
    state.update(&digest(metadata));
    update_frame(&mut state, path.as_bytes())?;
    let digest = state.finalize();
    let mut nonce = [0_u8; 12];
    let source = digest
        .get(..nonce.len())
        .ok_or_else(|| AlgorithmError::new("nonce derivation failed"))?;
    nonce.copy_from_slice(source);
    Ok(nonce)
}

fn hex_bytes(bytes: &[u8]) -> String {
    use core::fmt::Write as _;
    let mut output = String::with_capacity(bytes.len().saturating_mul(2));
    for byte in bytes {
        if write!(output, "{byte:02x}").is_err() {
            return output;
        }
    }
    output
}

fn hex_nibble(byte: u8) -> Result<u8, AlgorithmError> {
    match byte {
        b'0'..=b'9' => Ok(byte.saturating_sub(b'0')),
        b'a'..=b'f' => Ok(byte.saturating_sub(b'a').saturating_add(10)),
        _ => Err(AlgorithmError::new(
            "protected payload contains invalid hexadecimal",
        )),
    }
}

fn decode_hex(text: &str) -> Result<Vec<u8>, AlgorithmError> {
    let bytes = text.as_bytes();
    let (pairs, remainder) = bytes.as_chunks::<2>();
    if !remainder.is_empty() {
        return Err(AlgorithmError::new(
            "protected payload hexadecimal has odd length",
        ));
    }
    let mut output = Vec::with_capacity(pairs.len());
    for pair in pairs {
        let [high_byte, low_byte] = *pair;
        let high = hex_nibble(high_byte)?;
        let low = hex_nibble(low_byte)?;
        output.push((high << 4) | low);
    }
    Ok(output)
}

fn validate_txt_path(path: &Path) -> Result<(), AlgorithmError> {
    if path.extension().and_then(|value| value.to_str()) != Some("txt") {
        return Err(AlgorithmError::new("algorithm output must end in .txt"));
    }
    Ok(())
}

fn absolute_lexical(path: &Path) -> Result<PathBuf, AlgorithmError> {
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(AlgorithmError::new(
            "output path must not contain parent traversal",
        ));
    }
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        let current = std::env::current_dir().map_err(|error| {
            AlgorithmError::new(format!("cannot resolve current directory: {error}"))
        })?;
        Ok(current.join(path))
    }
}

fn projected_identity(path: &Path) -> Result<PathBuf, AlgorithmError> {
    let absolute = absolute_lexical(path)?;
    for ancestor in absolute.ancestors() {
        let kind = local::path_kind(ancestor)
            .map_err(|error| io_failure("cannot inspect output path", &error))?;
        if kind != PathKind::Missing {
            let canonical = local::canonicalize(ancestor).map_err(|error| {
                io_failure("cannot canonicalize output ancestor", &error)
            })?;
            let suffix = absolute.strip_prefix(ancestor).map_err(|_prefix_error| {
                AlgorithmError::new("cannot resolve output path identity")
            })?;
            return Ok(canonical.join(suffix));
        }
    }
    Err(AlgorithmError::new(
        "output path has no existing filesystem ancestor",
    ))
}

fn reject_output_overlap(path: &Path, protected_roots: &[PathBuf]) -> Result<(), AlgorithmError> {
    let output = projected_identity(path)?;
    if protected_roots
        .iter()
        .any(|root| output.starts_with(root) || root.starts_with(&output))
    {
        return Err(AlgorithmError::new(
            "output path overlaps a protected input root",
        ));
    }
    Ok(())
}

fn reject_target_source_overlap(
    target_root: &Path,
    source_roots: &[PathBuf],
) -> Result<(), AlgorithmError> {
    if source_roots
        .iter()
        .any(|root| target_root.starts_with(root) || root.starts_with(target_root))
    {
        return Err(AlgorithmError::new(
            "target path overlaps a source input root",
        ));
    }
    Ok(())
}

fn reject_physical_source_target_overlap(
    source_files: &[InputFile],
    target_files: &[InputFile],
) -> Result<(), AlgorithmError> {
    let source_identities = source_files
        .iter()
        .map(|file| {
            Handle::from_path(&file.path).map_err(|_error| {
                AlgorithmError::new("cannot identify source input file")
            })
        })
        .collect::<Result<HashSet<_>, _>>()?;
    for target in target_files {
        let identity = Handle::from_path(&target.path)
            .map_err(|_error| AlgorithmError::new("cannot identify target input file"))?;
        if source_identities.contains(&identity) {
            return Err(AlgorithmError::new(
                "target file aliases a source input file",
            ));
        }
    }
    Ok(())
}

/// Authors one deterministic source-bound `.txt` algorithm.
///
/// # Errors
/// Returns an error when inputs, settings, target evidence, encryption, or the
/// explicit algorithm output path violate the generic contract.
pub fn create_algorithm(
    settings: &Settings,
    source_paths: &[PathBuf],
    target_path: &Path,
    algorithm_path: &Path,
) -> Result<(), AlgorithmError> {
    validate_txt_path(algorithm_path)?;
    let source = collect_source(source_paths, settings)?;
    let (target_kind, target_files, target_root) = collect_target(target_path, settings)?;
    reject_target_source_overlap(&target_root, &source.roots)?;
    reject_physical_source_target_overlap(&source.files, &target_files)?;
    reject_output_overlap(algorithm_path, &source.roots)?;
    reject_output_overlap(algorithm_path, &[target_root])?;

    let settings_hash = settings_sha256(settings)?;
    let source_records = source
        .files
        .iter()
        .map(InputFile::source_record)
        .collect::<Vec<_>>();
    let target_descriptors = target_files
        .iter()
        .map(InputFile::target_descriptor)
        .collect::<Vec<_>>();
    let metadata = metadata_bytes(
        &settings_hash,
        &source_records,
        target_kind,
        &target_descriptors,
    )?;
    let key = source_key(&source.files)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key)
        .map_err(|_key_error| AlgorithmError::new("cannot initialize protected payload cipher"))?;
    let mut protected = Vec::with_capacity(target_files.len());
    for (file, descriptor) in target_files.iter().zip(target_descriptors) {
        let plaintext = &file.data;
        let nonce = nonce_for(&key, &metadata, &descriptor.path)?;
        let associated = aad(&metadata, &descriptor.path)?;
        let nonce_value = Nonce::try_from(nonce.as_slice())
            .map_err(|_nonce_error| AlgorithmError::new("invalid derived nonce"))?;
        let ciphertext = cipher
            .encrypt(
                &nonce_value,
                Payload {
                    msg: plaintext,
                    aad: &associated,
                },
            )
            .map_err(|_cipher_error| AlgorithmError::new("protected target encryption failed"))?;
        protected.push(ProtectedTarget {
            descriptor,
            nonce: hex_bytes(&nonce),
            ciphertext: hex_bytes(&ciphertext),
        });
    }
    let document = AlgorithmDocument {
        schema: ALGORITHM_SCHEMA.to_owned(),
        settings_sha256: settings_hash,
        source: source_records,
        target_kind,
        target: protected,
    };
    let mut text = serde_json::to_string_pretty(&document)
        .map_err(|error| AlgorithmError::new(format!("cannot serialize algorithm: {error}")))?;
    text.push('\n');
    local::write_new_text(algorithm_path, &text, true)
        .map_err(|error| io_failure("cannot write algorithm output", &error))
}

fn portable_target_identity(path: &str) -> String {
    path.chars().flat_map(char::to_uppercase).collect()
}

fn validate_lower_hex(
    value: &str,
    expected_len: Option<usize>,
    context: &str,
) -> Result<(), AlgorithmError> {
    if expected_len.is_some_and(|length| value.len() != length)
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(AlgorithmError::new(format!(
            "{context} must be canonical lowercase hexadecimal"
        )));
    }
    Ok(())
}

fn validate_record_path(
    path: &str,
    allow_empty: bool,
    context: &str,
) -> Result<(), AlgorithmError> {
    if path.is_empty() {
        if allow_empty {
            return Ok(());
        }
        return Err(AlgorithmError::new(format!(
            "{context} path must not be empty"
        )));
    }
    if path.contains("//") || path.ends_with('/') {
        return Err(AlgorithmError::new(format!(
            "{context} path must use canonical separators"
        )));
    }
    let candidate = Path::new(path);
    if candidate
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(AlgorithmError::new(format!(
            "{context} path must be a canonical relative path"
        )));
    }
    validate_portable_path(candidate).map_err(|error| {
        AlgorithmError::new(format!("invalid {context} path: {error}"))
    })
}

fn validate_source_records(
    source: &[SourceRecord],
    settings: &Settings,
) -> Result<(), AlgorithmError> {
    let mut identities = BTreeSet::new();
    let mut file_inputs = HashSet::new();
    let mut directory_inputs = HashSet::new();
    let mut previous_input: Option<u64> = None;
    let mut source_bytes = 0_u64;
    for record in source {
        validate_record_path(&record.path, true, "algorithm source")?;
        validate_lower_hex(
            &record.sha256,
            Some(SHA256_HEX_LEN),
            "algorithm source sha256",
        )?;
        if record.bytes > settings.maximum_file_bytes() {
            return Err(AlgorithmError::new(
                "algorithm source file exceeds settings",
            ));
        }
        source_bytes = source_bytes
            .checked_add(record.bytes)
            .ok_or_else(|| {
                AlgorithmError::new("algorithm source length overflow")
            })?;
        match previous_input {
            None if record.input != 0 => {
                return Err(AlgorithmError::new(
                    "algorithm source inputs must begin at zero",
                ));
            },
            Some(previous)
                if record.input != previous
                    && previous.checked_add(1) != Some(record.input) =>
            {
                return Err(AlgorithmError::new(
                    "algorithm source inputs must be contiguous",
                ));
            },
            _ => {},
        }
        previous_input = Some(record.input);
        if !identities.insert((record.input, record.path.as_str())) {
            return Err(AlgorithmError::new(
                "algorithm contains duplicate source records",
            ));
        }
        if record.path.is_empty() {
            if directory_inputs.contains(&record.input) {
                return Err(AlgorithmError::new(
                    "algorithm source input mixes file and directory records",
                ));
            }
            let _inserted = file_inputs.insert(record.input);
        } else {
            if file_inputs.contains(&record.input) {
                return Err(AlgorithmError::new(
                    "algorithm source input mixes file and directory records",
                ));
            }
            let _inserted = directory_inputs.insert(record.input);
        }
    }
    if source_bytes < settings.minimum_source_bytes()
        || source_bytes > settings.maximum_source_bytes()
    {
        return Err(AlgorithmError::new(
            "algorithm source bytes violate settings",
        ));
    }
    Ok(())
}

fn validate_document(
    document: &AlgorithmDocument,
    settings: &Settings,
) -> Result<Vec<TargetDescriptor>, AlgorithmError> {
    if document.schema != ALGORITHM_SCHEMA {
        return Err(AlgorithmError::new("unsupported algorithm schema"));
    }
    if document.settings_sha256 != settings_sha256(settings)? {
        return Err(AlgorithmError::new(
            "algorithm settings do not match active settings",
        ));
    }
    if document.source.is_empty() || document.target.is_empty() {
        return Err(AlgorithmError::new(
            "algorithm source and target records must not be empty",
        ));
    }
    let source_count = usize_to_u64(document.source.len(), "algorithm source count")?;
    if source_count < settings.minimum_source_files()
        || source_count > settings.maximum_source_files()
    {
        return Err(AlgorithmError::new(
            "algorithm source count violates settings",
        ));
    }
    validate_source_records(&document.source, settings)?;
    let target_count = usize_to_u64(document.target.len(), "algorithm target count")?;
    if target_count > settings.maximum_target_files() {
        return Err(AlgorithmError::new(
            "algorithm target count violates settings",
        ));
    }
    if document.target_kind == TargetKind::File {
        let Some(target) = document.target.first() else {
            return Err(AlgorithmError::new(
                "file target algorithm has invalid target records",
            ));
        };
        if document.target.len() != 1 || !target.descriptor.path.is_empty() {
            return Err(AlgorithmError::new(
                "file target algorithm has invalid target records",
            ));
        }
    }
    let mut paths = BTreeSet::new();
    let mut target_bytes = 0_u64;
    let mut descriptors = Vec::with_capacity(document.target.len());
    for target in &document.target {
        if document.target_kind == TargetKind::Directory {
            validate_record_path(
                &target.descriptor.path,
                false,
                "algorithm target",
            )?;
        }
        let identity = portable_target_identity(&target.descriptor.path);
        let candidate = Path::new(&identity);
        if paths.iter().any(|existing: &String| {
            let existing = Path::new(existing);
            candidate.starts_with(existing) || existing.starts_with(candidate)
        }) {
            return Err(AlgorithmError::new(
                "algorithm contains overlapping target paths",
            ));
        }
        let _inserted = paths.insert(identity);
        if target.descriptor.bytes > settings.maximum_file_bytes() {
            return Err(AlgorithmError::new(
                "algorithm target file exceeds settings",
            ));
        }
        target_bytes = target_bytes.saturating_add(target.descriptor.bytes);
        validate_lower_hex(
            &target.descriptor.sha256,
            Some(SHA256_HEX_LEN),
            "algorithm target sha256",
        )?;
        validate_lower_hex(
            &target.nonce,
            Some(PROTECTED_NONCE_HEX_LEN),
            "algorithm target nonce",
        )?;
        let expected_ciphertext_hex = target
            .descriptor
            .bytes
            .checked_add(PROTECTED_TAG_BYTES)
            .and_then(|bytes| bytes.checked_mul(HEX_CHARS_PER_BYTE))
            .ok_or_else(|| {
                AlgorithmError::new("algorithm target length overflow")
            })?;
        let observed_ciphertext_hex = usize_to_u64(
            target.ciphertext.len(),
            "algorithm target ciphertext length",
        )?;
        if observed_ciphertext_hex != expected_ciphertext_hex {
            return Err(AlgorithmError::new(
                "algorithm target ciphertext length does not match target",
            ));
        }
        validate_lower_hex(
            &target.ciphertext,
            None,
            "algorithm target ciphertext",
        )?;
        descriptors.push(target.descriptor.clone());
    }
    if target_bytes > settings.maximum_target_bytes() {
        return Err(AlgorithmError::new(
            "algorithm target bytes violate settings",
        ));
    }
    Ok(descriptors)
}

fn parse_document(path: &Path) -> Result<AlgorithmDocument, AlgorithmError> {
    validate_txt_path(path)?;
    let text = local::read_utf8(path)
        .map_err(|error| io_failure("cannot read algorithm", &error))?;
    serde_json::from_str(&text)
        .map_err(|error| AlgorithmError::new(format!("invalid algorithm JSON: {error}")))
}

fn output_path_for(
    output: &Path,
    target_kind: TargetKind,
    descriptor: &TargetDescriptor,
) -> Result<PathBuf, AlgorithmError> {
    match target_kind {
        TargetKind::File => Ok(output.to_path_buf()),
        TargetKind::Directory => resolve_under(output, Path::new(&descriptor.path))
            .map_err(|error| AlgorithmError::new(format!("invalid target output path: {error}"))),
    }
}

/// Replays one source-bound `.txt` algorithm into a new explicit output path.
///
/// # Errors
/// Returns an error before writing when settings, source evidence, authenticated
/// metadata, protected target bytes, or output containment fail validation.
pub fn replay_algorithm(
    settings: &Settings,
    source_paths: &[PathBuf],
    algorithm_path: &Path,
    output_path: &Path,
) -> Result<(), AlgorithmError> {
    let document = parse_document(algorithm_path)?;
    let descriptors = validate_document(&document, settings)?;
    let source = collect_source(source_paths, settings)?;
    let observed_source = source
        .files
        .iter()
        .map(InputFile::source_record)
        .collect::<Vec<_>>();
    if observed_source != document.source {
        return Err(AlgorithmError::new(
            "source evidence does not match algorithm",
        ));
    }
    reject_output_overlap(output_path, &source.roots)?;
    let output_kind = local::path_kind(output_path)
        .map_err(|error| io_failure("cannot inspect replay output", &error))?;
    if output_kind != PathKind::Missing {
        return Err(AlgorithmError::new(
            "replay output path must not already exist",
        ));
    }

    let metadata = metadata_bytes(
        &document.settings_sha256,
        &document.source,
        document.target_kind,
        &descriptors,
    )?;
    let key = source_key(&source.files)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key)
        .map_err(|_key_error| AlgorithmError::new("cannot initialize protected payload cipher"))?;
    let mut recovered = Vec::with_capacity(document.target.len());
    let mut output_identities = BTreeSet::new();
    for target in &document.target {
        let nonce = decode_hex(&target.nonce)?;
        let ciphertext = decode_hex(&target.ciphertext)?;
        let associated = aad(&metadata, &target.descriptor.path)?;
        let nonce_value = Nonce::try_from(nonce.as_slice()).map_err(|_nonce_error| {
            AlgorithmError::new("algorithm target nonce must be 12 bytes")
        })?;
        let plaintext = cipher
            .decrypt(
                &nonce_value,
                Payload {
                    msg: &ciphertext,
                    aad: &associated,
                },
            )
            .map_err(|_cipher_error| {
                AlgorithmError::new("protected target authentication failed")
            })?;
        let observed_bytes = usize_to_u64(plaintext.len(), "recovered target length")?;
        if observed_bytes != target.descriptor.bytes
            || digest_hex(&plaintext) != target.descriptor.sha256
        {
            return Err(AlgorithmError::new(
                "recovered target identity does not match algorithm",
            ));
        }
        let destination = output_path_for(output_path, document.target_kind, &target.descriptor)?;
        if !output_identities.insert(destination.clone()) {
            return Err(AlgorithmError::new("replay target output collision"));
        }
        recovered.push((destination, plaintext));
    }

    for (destination, bytes) in recovered {
        local::write_new_bytes(&destination, &bytes, true)
            .map_err(|error| io_failure("cannot write replay output", &error))?;
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/foundation/algorithm/unit/application/engine/tests.rs"]
mod tests;
