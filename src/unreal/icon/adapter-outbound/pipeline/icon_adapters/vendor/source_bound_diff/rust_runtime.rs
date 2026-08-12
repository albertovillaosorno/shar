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
//   - The repository behavior implemented by this source file.
// - Must-Not:
//   - Bypass the contracts or authority boundaries of its owning package.
// - Allows:
//   - Inputs: values admitted by the file's public or internal interface.
//   - Outputs: deterministic values or effects declared by that interface.
//   - Side effects: only those explicitly owned by the implementation.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another file owns the exact same responsibility.
// - Summary:
//   - Standalone std-only runtime template for emitted exact source-bound.
// - Description:
//   - Implements the responsibility summarized by this module.
// - Usage:
//   - Used through the owning package, executable, or document boundary.
// - Defaults:
//   - Invalid inputs or broken invariants fail closed.
//

//! Standalone std-only runtime template for emitted exact source-bound
//! transforms.

use std::collections::BTreeSet;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Component, Path, PathBuf};

// BEGIN GENERATED CONSTANTS
const PROFILE: &str = "unconfigured-source-bound-transform";
const AAD_HEX: &str = "";
const BINDING_HEX: &str = "";
const NONCE_HEX: &str = "";
const CIPHERTEXT_HEX: &str = "";
const TAG_HEX: &str = "";
// END GENERATED CONSTANTS

const ANCHOR_BASE: u64 = 257;
const GF_REDUCTION: u8 = 0x1b;
const AAD_MAGIC: &[u8] = b"source-bound-exact-plan-aad-v2\0";
const BINDING_CONTEXT_MAGIC: &[u8] = b"source-binding-context-v1\0";
const BINDING_COMMITMENT_MAGIC: &[u8] =
    b"source-binding-secret-commitment-v1\0";
const SHARE_MASK_MAGIC: &[u8] = b"source-binding-anchor-share-mask-v1\0";

#[derive(Clone, Debug, Eq, PartialEq)]
struct FileRecord {
    path: String,
    sha256: [u8; 32],
    size: u64,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Snapshot {
    files: Vec<FileRecord>,
}
#[derive(Clone, Debug)]
enum Segment {
    Source { offset: u64, length: u64 },
    Payload { offset: u64, length: u64 },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InstructionKind {
    CopySource,
    PatchSource,
    Payload,
}
#[derive(Clone, Debug)]
struct Instruction {
    output_path: String,
    kind: InstructionKind,
    expected_sha256: [u8; 32],
    source_path: Option<String>,
    payload_slice: Option<(u64, u64)>,
    segments: Vec<Segment>,
}
#[derive(Clone, Debug)]
struct Metadata {
    source: Snapshot,
    target: Snapshot,
    passthrough_roots: Vec<String>,
    instructions: Vec<Instruction>,
}
#[derive(Clone, Debug)]
struct BoundShare {
    source_path: String,
    anchor_digest: [u8; 32],
    x: u8,
    masked_share: Vec<u8>,
}
#[derive(Clone, Debug)]
struct Binding {
    context: Vec<u8>,
    threshold: usize,
    minimum_anchor_files: usize,
    secret_length: usize,
    secret_commitment: [u8; 32],
    window_bytes: usize,
    selection_modulus: u64,
    shares: Vec<BoundShare>,
}
#[derive(Clone, Debug)]
struct RecoveredShare {
    source_path: String,
    x: u8,
    value: Vec<u8>,
}

struct Cursor<'a> {
    data: &'a [u8],
    offset: usize,
}
impl<'a> Cursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let end = self
            .offset
            .checked_add(n)
            .ok_or_else(|| String::from("binary cursor overflow"))?;
        if end > self.data.len() {
            return Err(String::from("truncated generated transform data"));
        }
        let out = &self.data[self.offset..end];
        self.offset = end;
        Ok(out)
    }
    fn byte(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    fn u64_be(&mut self) -> Result<u64, String> {
        Ok(u64::from_be_bytes(
            self.take(8)?
                .try_into()
                .map_err(|_| String::from("invalid u64 frame"))?,
        ))
    }
    fn usize_be(&mut self) -> Result<usize, String> {
        usize::try_from(self.u64_be()?)
            .map_err(|_| String::from("generated length exceeds host usize"))
    }
    fn frame(&mut self) -> Result<&'a [u8], String> {
        let n = self.usize_be()?;
        self.take(n)
    }
    fn text(&mut self) -> Result<String, String> {
        String::from_utf8(self.frame()?.to_vec())
            .map_err(|_| String::from("generated text is not UTF-8"))
    }
    fn finish(self) -> Result<(), String> {
        if self.offset == self.data.len() {
            Ok(())
        } else {
            Err(String::from("generated frame has trailing bytes"))
        }
    }
}

fn hex_nibble(b: u8) -> Result<u8, String> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err(String::from("non-hex generated byte")),
    }
}
fn decode_hex(text: &str) -> Result<Vec<u8>, String> {
    let bytes = text.as_bytes();
    if bytes.len() % 2 != 0 {
        return Err(String::from("odd generated hex length"));
    }
    bytes
        .chunks_exact(2)
        .map(|p| Ok((hex_nibble(p[0])? << 4) | hex_nibble(p[1])?))
        .collect()
}
fn digest_bytes(value: &[u8]) -> Result<[u8; 32], String> {
    value
        .try_into()
        .map_err(|_| String::from("SHA-256 digest has wrong width"))
}
fn digest_hex(value: &[u8]) -> Result<[u8; 32], String> {
    let text = std::str::from_utf8(value)
        .map_err(|_| String::from("SHA-256 text is not UTF-8"))?;
    digest_bytes(&decode_hex(text)?)
}
fn parse_snapshot(data: &[u8]) -> Result<Snapshot, String> {
    let mut c = Cursor::new(data);
    let count = c.usize_be()?;
    let mut files = Vec::with_capacity(count);
    for _ in 0..count {
        files.push(FileRecord {
            path: c.text()?,
            sha256: digest_hex(c.frame()?)?,
            size: c.u64_be()?,
        });
    }
    c.finish()?;
    Ok(Snapshot { files })
}
fn optional_text(c: &mut Cursor<'_>) -> Result<Option<String>, String> {
    match c.byte()? {
        0 => Ok(None),
        1 => Ok(Some(c.text()?)),
        _ => Err(String::from("invalid optional text tag")),
    }
}
fn optional_slice(c: &mut Cursor<'_>) -> Result<Option<(u64, u64)>, String> {
    match c.byte()? {
        0 => Ok(None),
        1 => Ok(Some((c.u64_be()?, c.u64_be()?))),
        _ => Err(String::from("invalid optional slice tag")),
    }
}
fn parse_instruction(c: &mut Cursor<'_>) -> Result<Instruction, String> {
    let kind = match c.byte()? {
        b'C' => InstructionKind::CopySource,
        b'P' => InstructionKind::PatchSource,
        b'L' => InstructionKind::Payload,
        _ => return Err(String::from("invalid instruction kind")),
    };
    let output_path = c.text()?;
    let expected_sha256 = digest_hex(c.frame()?)?;
    let source_path = optional_text(c)?;
    let payload_slice = optional_slice(c)?;
    let count = c.usize_be()?;
    let mut segments = Vec::with_capacity(count);
    for _ in 0..count {
        let tag = c.byte()?;
        let offset = c.u64_be()?;
        let length = c.u64_be()?;
        segments.push(match tag {
            b'S' => Segment::Source { offset, length },
            b'P' => Segment::Payload { offset, length },
            _ => return Err(String::from("invalid segment kind")),
        });
    }
    let valid = match kind {
        InstructionKind::CopySource => {
            source_path.is_some()
                && payload_slice.is_none()
                && segments.is_empty()
        },
        InstructionKind::PatchSource => {
            source_path.is_some()
                && payload_slice.is_none()
                && !segments.is_empty()
        },
        InstructionKind::Payload => {
            source_path.is_none()
                && payload_slice.is_some()
                && segments.is_empty()
        },
    };
    if !valid {
        return Err(String::from("inconsistent generated instruction"));
    }
    Ok(Instruction {
        output_path,
        kind,
        expected_sha256,
        source_path,
        payload_slice,
        segments,
    })
}
fn parse_metadata(aad: &[u8]) -> Result<Metadata, String> {
    if !aad.starts_with(AAD_MAGIC) {
        return Err(String::from("AAD magic mismatch"));
    }
    let mut c = Cursor::new(&aad[AAD_MAGIC.len()..]);
    let context = c.frame()?;
    if context.is_empty() {
        return Err(String::from("empty transform context"));
    }
    let source = parse_snapshot(c.frame()?)?;
    let target = parse_snapshot(c.frame()?)?;
    let passthrough_count = c.usize_be()?;
    let mut passthrough_roots = Vec::with_capacity(passthrough_count);
    for _ in 0..passthrough_count {
        passthrough_roots.push(c.text()?);
    }
    validate_passthrough_roots(&passthrough_roots)?;
    let count = c.usize_be()?;
    let mut instructions = Vec::with_capacity(count);
    for _ in 0..count {
        instructions.push(parse_instruction(&mut c)?);
    }
    c.finish()?;
    Ok(Metadata {
        source,
        target,
        passthrough_roots,
        instructions,
    })
}
fn parse_binding(data: &[u8]) -> Result<Binding, String> {
    let mut c = Cursor::new(data);
    let context = c.frame()?.to_vec();
    let threshold = c.usize_be()?;
    let minimum_anchor_files = c.usize_be()?;
    let secret_length = c.usize_be()?;
    let secret_commitment = digest_bytes(c.frame()?)?;
    let window_bytes = c.usize_be()?;
    let selection_modulus = c.u64_be()?;
    let count = c.usize_be()?;
    let mut shares = Vec::with_capacity(count);
    for _ in 0..count {
        let source_path = c.text()?;
        let anchor_digest = digest_bytes(c.frame()?)?;
        let raw_x = c.u64_be()?;
        let x = u8::try_from(raw_x)
            .map_err(|_| String::from("share coordinate exceeds u8"))?;
        let masked_share = c.frame()?.to_vec();
        shares.push(BoundShare {
            source_path,
            anchor_digest,
            x,
            masked_share,
        });
    }
    c.finish()?;
    if context.is_empty()
        || threshold == 0
        || threshold > shares.len()
        || minimum_anchor_files == 0
        || minimum_anchor_files > shares.len()
        || secret_length == 0
        || window_bytes == 0
        || selection_modulus == 0
        || shares
            .iter()
            .any(|s| s.x == 0 || s.masked_share.len() != secret_length)
    {
        return Err(String::from("invalid generated binding"));
    }
    Ok(Binding {
        context,
        threshold,
        minimum_anchor_files,
        secret_length,
        secret_commitment,
        window_bytes,
        selection_modulus,
        shares,
    })
}

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
    0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];
fn sha256(data: &[u8]) -> [u8; 32] {
    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut padded = Vec::with_capacity(data.len() + 72);
    padded.extend_from_slice(data);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    let mut s = [
        0x6a09e667u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    for chunk in padded.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, x) in w[..16].iter_mut().enumerate() {
            let j = i * 4;
            *x = u32::from_be_bytes(
                chunk[j..j + 4].try_into().expect("sha word"),
            );
        }
        for i in 16..64 {
            let a = w[i - 15].rotate_right(7)
                ^ w[i - 15].rotate_right(18)
                ^ (w[i - 15] >> 3);
            let b = w[i - 2].rotate_right(17)
                ^ w[i - 2].rotate_right(19)
                ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(a)
                .wrapping_add(w[i - 7])
                .wrapping_add(b);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) =
            (s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]);
        for i in 0..64 {
            let q = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = h
                .wrapping_add(q)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K[i])
                .wrapping_add(w[i]);
            let p = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = p.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (i, x) in [a, b, c, d, e, f, g, h].iter().enumerate() {
            s[i] = s[i].wrapping_add(*x);
        }
    }
    let mut out = [0u8; 32];
    for (i, x) in s.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&x.to_be_bytes());
    }
    out
}
fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let normalized = if key.len() > 64 {
        sha256(key).to_vec()
    } else {
        key.to_vec()
    };
    let mut kb = [0u8; 64];
    kb[..normalized.len()].copy_from_slice(&normalized);
    let mut inner = Vec::with_capacity(64 + data.len());
    let mut outer = Vec::with_capacity(96);
    for b in kb {
        inner.push(b ^ 0x36);
        outer.push(b ^ 0x5c);
    }
    inner.extend_from_slice(data);
    outer.extend_from_slice(&sha256(&inner));
    sha256(&outer)
}
fn hkdf_expand(
    prk: &[u8],
    info: &[u8],
    length: usize,
) -> Result<Vec<u8>, String> {
    if length > 255 * 32 {
        return Err(String::from("HKDF output too long"));
    }
    let mut out = Vec::with_capacity(length);
    let mut previous = Vec::new();
    let blocks = length.div_ceil(32);
    for index in 1..=blocks {
        let mut input = Vec::new();
        input.extend_from_slice(&previous);
        input.extend_from_slice(info);
        input.push(
            u8::try_from(index)
                .map_err(|_| String::from("HKDF block overflow"))?,
        );
        previous = hmac_sha256(prk, &input).to_vec();
        out.extend_from_slice(&previous);
    }
    out.truncate(length);
    Ok(out)
}
fn frame_bytes(value: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + value.len());
    out.extend_from_slice(&(value.len() as u64).to_be_bytes());
    out.extend_from_slice(value);
    out
}
fn context_digest(context: &[u8]) -> [u8; 32] {
    let mut v = Vec::new();
    v.extend_from_slice(BINDING_CONTEXT_MAGIC);
    v.extend_from_slice(&frame_bytes(context));
    sha256(&v)
}
fn secret_commitment(context: &[u8], secret: &[u8]) -> [u8; 32] {
    let mut v = Vec::new();
    v.extend_from_slice(BINDING_COMMITMENT_MAGIC);
    v.extend_from_slice(&frame_bytes(context));
    v.extend_from_slice(&frame_bytes(secret));
    sha256(&v)
}
fn safe_join(root: &Path, relative: &str) -> Result<PathBuf, String> {
    if relative.is_empty()
        || relative.starts_with('/')
        || relative.contains('\\')
    {
        return Err(String::from("unsafe generated relative path"));
    }
    let mut out = root.to_path_buf();
    for part in relative.split('/') {
        if part.is_empty() || part == "." || part == ".." {
            return Err(String::from("unsafe generated relative path"));
        }
        out.push(part);
    }
    Ok(out)
}
fn relative_path(root: &Path, path: &Path) -> Result<String, String> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| String::from("path escaped root"))?;
    let mut parts = Vec::new();
    for c in relative.components() {
        match c {
            Component::Normal(v) => parts
                .push(v.to_str().ok_or_else(|| String::from("non-UTF8 path"))?),
            _ => return Err(String::from("unsafe filesystem path")),
        }
    }
    Ok(parts.join("/"))
}
fn valid_relative_root(relative: &str) -> bool {
    !relative.is_empty()
        && !relative.starts_with('/')
        && !relative.contains('\\')
        && relative
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}
fn path_in_roots(relative: &str, roots: &[String]) -> bool {
    roots.iter().any(|root| {
        relative == root
            || relative
                .strip_prefix(root)
                .is_some_and(|tail| tail.starts_with('/'))
    })
}
fn validate_passthrough_roots(roots: &[String]) -> Result<(), String> {
    let mut previous: Option<&str> = None;
    for root in roots {
        if !valid_relative_root(root) {
            return Err(String::from("invalid generated passthrough root"));
        }
        if let Some(prior) = previous {
            if root.as_str() <= prior
                || root
                    .strip_prefix(prior)
                    .is_some_and(|tail| tail.starts_with('/'))
            {
                return Err(String::from(
                    "generated passthrough roots are unsorted or overlapping",
                ));
            }
        }
        previous = Some(root);
    }
    Ok(())
}
fn collect_files(
    root: &Path,
    current: &Path,
    out: &mut Vec<String>,
) -> Result<(), String> {
    for item in fs::read_dir(current)
        .map_err(|e| format!("cannot read {}: {e}", current.display()))?
    {
        let entry = item.map_err(|e| format!("directory entry failed: {e}"))?;
        let path = entry.path();
        let meta = fs::symlink_metadata(&path)
            .map_err(|e| format!("cannot inspect {}: {e}", path.display()))?;
        let kind = meta.file_type();
        if kind.is_symlink() {
            return Err(format!("symlink unsupported: {}", path.display()));
        }
        if kind.is_dir() {
            collect_files(root, &path, out)?;
        } else if kind.is_file() {
            out.push(relative_path(root, &path)?);
        } else {
            return Err(format!(
                "non-regular entry unsupported: {}",
                path.display()
            ));
        }
    }
    Ok(())
}
fn snapshot_tree(root: &Path) -> Result<Snapshot, String> {
    if !root.is_dir() {
        return Err(format!(
            "tree root is not a directory: {}",
            root.display()
        ));
    }
    let mut paths = Vec::new();
    collect_files(root, root, &mut paths)?;
    paths.sort();
    let mut files = Vec::with_capacity(paths.len());
    for relative in paths {
        let path = safe_join(root, &relative)?;
        let data = fs::read(&path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        files.push(FileRecord {
            path: relative,
            sha256: sha256(&data),
            size: data.len() as u64,
        });
    }
    Ok(Snapshot { files })
}
fn snapshot_tree_excluding(
    root: &Path,
    excluded_roots: &[String],
) -> Result<Snapshot, String> {
    let mut snapshot = snapshot_tree(root)?;
    snapshot
        .files
        .retain(|record| !path_in_roots(&record.path, excluded_roots));
    Ok(snapshot)
}
fn snapshot_declared_source(
    root: &Path,
    expected: &Snapshot,
) -> Result<Snapshot, String> {
    if !root.is_dir() {
        return Err(format!(
            "source root is not a directory: {}",
            root.display()
        ));
    }
    let mut files = Vec::with_capacity(expected.files.len());
    for record in &expected.files {
        let path = safe_join(root, &record.path)?;
        let metadata = fs::symlink_metadata(&path)
            .map_err(|e| format!("cannot inspect {}: {e}", path.display()))?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(format!(
                "declared source is not a regular file: {}",
                path.display()
            ));
        }
        let data = fs::read(&path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        files.push(FileRecord {
            path: record.path.clone(),
            sha256: sha256(&data),
            size: data.len() as u64,
        });
    }
    Ok(Snapshot { files })
}
fn rolling_power(window: usize) -> u64 {
    let mut v = 1u64;
    for _ in 1..window {
        v = v.wrapping_mul(ANCHOR_BASE);
    }
    v
}
fn initial_rolling(data: &[u8], window: usize) -> u64 {
    let mut v = 0u64;
    for b in &data[..window] {
        v = v.wrapping_mul(ANCHOR_BASE).wrapping_add(u64::from(*b));
    }
    v
}
fn find_anchor_window(
    data: &[u8],
    window: usize,
    modulus: u64,
    wanted: &[u8; 32],
) -> Option<Vec<u8>> {
    if data.is_empty() {
        return None;
    }
    if data.len() < window {
        return (sha256(data) == *wanted).then(|| data.to_vec());
    }
    let last = data.len() - window;
    let power = rolling_power(window);
    let mut rolling = initial_rolling(data, window);
    let mut fallback = rolling;
    let mut fallback_offset = 0usize;
    let mut selected = false;
    for offset in 0..=last {
        if rolling < fallback {
            fallback = rolling;
            fallback_offset = offset;
        }
        if rolling % modulus == 0 {
            selected = true;
            let slice = &data[offset..offset + window];
            if sha256(slice) == *wanted {
                return Some(slice.to_vec());
            }
        }
        if offset < last {
            let removed = u64::from(data[offset]).wrapping_mul(power);
            rolling = rolling
                .wrapping_sub(removed)
                .wrapping_mul(ANCHOR_BASE)
                .wrapping_add(u64::from(data[offset + window]));
        }
    }
    if !selected {
        let slice = &data[fallback_offset..fallback_offset + window];
        if sha256(slice) == *wanted {
            return Some(slice.to_vec());
        }
    }
    None
}
fn share_mask(
    binding: &Binding,
    share: &BoundShare,
    window: &[u8],
) -> Result<Vec<u8>, String> {
    let salt = context_digest(&binding.context);
    let prk = hmac_sha256(&salt, window);
    let mut info = Vec::new();
    info.extend_from_slice(SHARE_MASK_MAGIC);
    info.extend_from_slice(&frame_bytes(share.source_path.as_bytes()));
    info.extend_from_slice(&frame_bytes(&share.anchor_digest));
    info.extend_from_slice(&u16::from(share.x).to_be_bytes());
    hkdf_expand(&prk, &info, binding.secret_length)
}
fn xor_equal(a: &[u8], b: &[u8]) -> Result<Vec<u8>, String> {
    if a.len() != b.len() {
        return Err(String::from("source-binding widths differ"));
    }
    Ok(a.iter().zip(b).map(|(x, y)| x ^ y).collect())
}
fn gf_multiply(left: u8, right: u8) -> u8 {
    let mut result = 0u8;
    let mut a = left;
    let mut b = right;
    for _ in 0..8 {
        if b & 1 != 0 {
            result ^= a;
        }
        let high = a & 0x80;
        a <<= 1;
        if high != 0 {
            a ^= GF_REDUCTION;
        }
        b >>= 1;
    }
    result
}
fn gf_power(value: u8, exponent: u16) -> u8 {
    let mut result = 1u8;
    let mut base = value;
    let mut remaining = exponent;
    while remaining != 0 {
        if remaining & 1 != 0 {
            result = gf_multiply(result, base);
        }
        base = gf_multiply(base, base);
        remaining >>= 1;
    }
    result
}
fn gf_inverse(value: u8) -> Result<u8, String> {
    if value == 0 {
        Err(String::from("cannot invert zero GF coordinate"))
    } else {
        Ok(gf_power(value, 254))
    }
}
fn lagrange_weight(x: u8, values: &[u8]) -> Result<u8, String> {
    let mut numerator = 1u8;
    let mut denominator = 1u8;
    for other in values {
        if *other == x {
            continue;
        }
        numerator = gf_multiply(numerator, *other);
        denominator = gf_multiply(denominator, x ^ *other);
    }
    Ok(gf_multiply(numerator, gf_inverse(denominator)?))
}
fn recover_secret(
    shares: &[RecoveredShare],
    length: usize,
) -> Result<Vec<u8>, String> {
    if shares.is_empty() {
        return Err(String::from("cannot recover zero shares"));
    }
    let xs: Vec<u8> = shares.iter().map(|s| s.x).collect();
    let unique: BTreeSet<u8> = xs.iter().copied().collect();
    if unique.len() != xs.len()
        || xs.iter().any(|x| *x == 0)
        || shares.iter().any(|s| s.value.len() != length)
    {
        return Err(String::from("malformed recovered shares"));
    }
    let mut weights = Vec::with_capacity(xs.len());
    for x in &xs {
        weights.push(lagrange_weight(*x, &xs)?);
    }
    let mut secret = vec![0u8; length];
    for index in 0..length {
        let mut value = 0u8;
        for (share, weight) in shares.iter().zip(&weights) {
            value ^= gf_multiply(share.value[index], *weight);
        }
        secret[index] = value;
    }
    Ok(secret)
}
fn constant_time_equal(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b) {
        diff |= x ^ y;
    }
    diff == 0
}
fn unlock_key(root: &Path, binding: &Binding) -> Result<Vec<u8>, String> {
    let mut available = Vec::new();
    for share in &binding.shares {
        let path = safe_join(root, &share.source_path)?;
        let data = fs::read(&path).map_err(|e| {
            format!("cannot read binding source {}: {e}", path.display())
        })?;
        if let Some(window) = find_anchor_window(
            &data,
            binding.window_bytes,
            binding.selection_modulus,
            &share.anchor_digest,
        ) {
            let mask = share_mask(binding, share, &window)?;
            available.push(RecoveredShare {
                source_path: share.source_path.clone(),
                x: share.x,
                value: xor_equal(&share.masked_share, &mask)?,
            });
        }
    }
    if available.len() < binding.threshold {
        return Err(format!(
            "insufficient source-bound anchors: need {}, found {}",
            binding.threshold,
            available.len()
        ));
    }
    let files: BTreeSet<&str> =
        available.iter().map(|s| s.source_path.as_str()).collect();
    if files.len() < binding.minimum_anchor_files {
        return Err(format!(
            "insufficient distributed source-bound files: need {}, found {}",
            binding.minimum_anchor_files,
            files.len()
        ));
    }
    let secret =
        recover_secret(&available[..binding.threshold], binding.secret_length)?;
    if !constant_time_equal(
        &secret_commitment(&binding.context, &secret),
        &binding.secret_commitment,
    ) {
        return Err(String::from("source-bound secret commitment failed"));
    }
    Ok(secret)
}
fn load_u32_le(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        data[offset..offset + 4]
            .try_into()
            .expect("little-endian u32"),
    )
}
fn quarter_round(s: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    s[a] = s[a].wrapping_add(s[b]);
    s[d] ^= s[a];
    s[d] = s[d].rotate_left(16);
    s[c] = s[c].wrapping_add(s[d]);
    s[b] ^= s[c];
    s[b] = s[b].rotate_left(12);
    s[a] = s[a].wrapping_add(s[b]);
    s[d] ^= s[a];
    s[d] = s[d].rotate_left(8);
    s[c] = s[c].wrapping_add(s[d]);
    s[b] ^= s[c];
    s[b] = s[b].rotate_left(7);
}
fn chacha20_block(key: &[u8; 32], nonce: &[u8; 12], counter: u32) -> [u8; 64] {
    let constants = *b"expand 32-byte k";
    let mut initial = [0u32; 16];
    for i in 0..4 {
        initial[i] = load_u32_le(&constants, i * 4);
    }
    for i in 0..8 {
        initial[4 + i] = load_u32_le(key, i * 4);
    }
    initial[12] = counter;
    for i in 0..3 {
        initial[13 + i] = load_u32_le(nonce, i * 4);
    }
    let mut s = initial;
    for _ in 0..10 {
        quarter_round(&mut s, 0, 4, 8, 12);
        quarter_round(&mut s, 1, 5, 9, 13);
        quarter_round(&mut s, 2, 6, 10, 14);
        quarter_round(&mut s, 3, 7, 11, 15);
        quarter_round(&mut s, 0, 5, 10, 15);
        quarter_round(&mut s, 1, 6, 11, 12);
        quarter_round(&mut s, 2, 7, 8, 13);
        quarter_round(&mut s, 3, 4, 9, 14);
    }
    let mut out = [0u8; 64];
    for i in 0..16 {
        out[i * 4..i * 4 + 4]
            .copy_from_slice(&s[i].wrapping_add(initial[i]).to_le_bytes());
    }
    out
}
fn chacha20_xor(
    key: &[u8; 32],
    nonce: &[u8; 12],
    input: &[u8],
) -> Result<Vec<u8>, String> {
    let blocks = input.len().div_ceil(64);
    if blocks > u32::MAX as usize {
        return Err(String::from("ChaCha20 counter space exceeded"));
    }
    let mut out = Vec::with_capacity(input.len());
    for (index, block) in input.chunks(64).enumerate() {
        let counter = u32::try_from(index + 1)
            .map_err(|_| String::from("ChaCha20 counter overflow"))?;
        let stream = chacha20_block(key, nonce, counter);
        out.extend(block.iter().zip(stream.iter()).map(|(a, b)| a ^ b));
    }
    Ok(out)
}
fn poly1305_mac(message: &[u8], key: &[u8; 32]) -> [u8; 16] {
    let t0 = u64::from(load_u32_le(key, 0));
    let t1 = u64::from(load_u32_le(key, 4));
    let t2 = u64::from(load_u32_le(key, 8));
    let t3 = u64::from(load_u32_le(key, 12));
    let r0 = t0 & 0x3ffffff;
    let r1 = ((t0 >> 26) | (t1 << 6)) & 0x3ffff03;
    let r2 = ((t1 >> 20) | (t2 << 12)) & 0x3ffc0ff;
    let r3 = ((t2 >> 14) | (t3 << 18)) & 0x3f03fff;
    let r4 = (t3 >> 8) & 0x00fffff;
    let s1 = r1 * 5;
    let s2 = r2 * 5;
    let s3 = r3 * 5;
    let s4 = r4 * 5;
    let mask = 0x3ffffffu64;
    let (mut h0, mut h1, mut h2, mut h3, mut h4) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    for chunk in message.chunks(16) {
        let mut block = [0u8; 17];
        block[..chunk.len()].copy_from_slice(chunk);
        block[chunk.len()] = 1;
        let b0 = u64::from(load_u32_le(&block, 0));
        let b1 = u64::from(load_u32_le(&block, 4));
        let b2 = u64::from(load_u32_le(&block, 8));
        let b3 = u64::from(load_u32_le(&block, 12));
        h0 += b0 & mask;
        h1 += ((b0 >> 26) | (b1 << 6)) & mask;
        h2 += ((b1 >> 20) | (b2 << 12)) & mask;
        h3 += ((b2 >> 14) | (b3 << 18)) & mask;
        h4 += (b3 >> 8) | (u64::from(block[16]) << 24);
        let d0 = h0 * r0 + h1 * s4 + h2 * s3 + h3 * s2 + h4 * s1;
        let d1 = h0 * r1 + h1 * r0 + h2 * s4 + h3 * s3 + h4 * s2;
        let d2 = h0 * r2 + h1 * r1 + h2 * r0 + h3 * s4 + h4 * s3;
        let d3 = h0 * r3 + h1 * r2 + h2 * r1 + h3 * r0 + h4 * s4;
        let d4 = h0 * r4 + h1 * r3 + h2 * r2 + h3 * r1 + h4 * r0;
        let mut carry = d0 >> 26;
        h0 = d0 & mask;
        let d1 = d1 + carry;
        carry = d1 >> 26;
        h1 = d1 & mask;
        let d2 = d2 + carry;
        carry = d2 >> 26;
        h2 = d2 & mask;
        let d3 = d3 + carry;
        carry = d3 >> 26;
        h3 = d3 & mask;
        let d4 = d4 + carry;
        carry = d4 >> 26;
        h4 = d4 & mask;
        h0 += carry * 5;
        carry = h0 >> 26;
        h0 &= mask;
        h1 += carry;
    }
    let mut carry = h1 >> 26;
    h1 &= mask;
    h2 += carry;
    carry = h2 >> 26;
    h2 &= mask;
    h3 += carry;
    carry = h3 >> 26;
    h3 &= mask;
    h4 += carry;
    carry = h4 >> 26;
    h4 &= mask;
    h0 += carry * 5;
    carry = h0 >> 26;
    h0 &= mask;
    h1 += carry;
    let mut g0 = h0 + 5;
    carry = g0 >> 26;
    g0 &= mask;
    let mut g1 = h1 + carry;
    carry = g1 >> 26;
    g1 &= mask;
    let mut g2 = h2 + carry;
    carry = g2 >> 26;
    g2 &= mask;
    let mut g3 = h3 + carry;
    carry = g3 >> 26;
    g3 &= mask;
    let g4 = h4.wrapping_add(carry).wrapping_sub(1 << 26);
    let select_g = (g4 >> 63).wrapping_sub(1);
    let select_h = !select_g;
    h0 = (h0 & select_h) | (g0 & select_g);
    h1 = (h1 & select_h) | (g1 & select_g);
    h2 = (h2 & select_h) | (g2 & select_g);
    h3 = (h3 & select_h) | (g3 & select_g);
    h4 = (h4 & select_h) | (g4 & select_g);
    let mut f0 = (h0 | (h1 << 26)) & 0xffffffff;
    let mut f1 = ((h1 >> 6) | (h2 << 20)) & 0xffffffff;
    let mut f2 = ((h2 >> 12) | (h3 << 14)) & 0xffffffff;
    let mut f3 = ((h3 >> 18) | (h4 << 8)) & 0xffffffff;
    f0 += u64::from(load_u32_le(key, 16));
    f1 += u64::from(load_u32_le(key, 20)) + (f0 >> 32);
    f0 &= 0xffffffff;
    f2 += u64::from(load_u32_le(key, 24)) + (f1 >> 32);
    f1 &= 0xffffffff;
    f3 += u64::from(load_u32_le(key, 28)) + (f2 >> 32);
    f2 &= 0xffffffff;
    f3 &= 0xffffffff;
    let mut out = [0u8; 16];
    out[0..4].copy_from_slice(&(f0 as u32).to_le_bytes());
    out[4..8].copy_from_slice(&(f1 as u32).to_le_bytes());
    out[8..12].copy_from_slice(&(f2 as u32).to_le_bytes());
    out[12..16].copy_from_slice(&(f3 as u32).to_le_bytes());
    out
}
fn aead_mac_data(aad: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(aad);
    if aad.len() % 16 != 0 {
        out.resize(out.len() + (16 - aad.len() % 16), 0);
    }
    out.extend_from_slice(ciphertext);
    if ciphertext.len() % 16 != 0 {
        out.resize(out.len() + (16 - ciphertext.len() % 16), 0);
    }
    out.extend_from_slice(&(aad.len() as u64).to_le_bytes());
    out.extend_from_slice(&(ciphertext.len() as u64).to_le_bytes());
    out
}
fn decrypt_payload(
    key: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
    tag: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, String> {
    let key: [u8; 32] = key
        .try_into()
        .map_err(|_| String::from("payload key width mismatch"))?;
    let nonce: [u8; 12] = nonce
        .try_into()
        .map_err(|_| String::from("AEAD nonce width mismatch"))?;
    if tag.len() != 16 {
        return Err(String::from("AEAD tag width mismatch"));
    }
    let block = chacha20_block(&key, &nonce, 0);
    let one_time: [u8; 32] = block[..32].try_into().expect("Poly1305 key");
    let expected = poly1305_mac(&aead_mac_data(aad, ciphertext), &one_time);
    if !constant_time_equal(&expected, tag) {
        return Err(String::from("ChaCha20-Poly1305 authentication failed"));
    }
    chacha20_xor(&key, &nonce, ciphertext)
}
fn payload_range(
    data: &[u8],
    offset: u64,
    length: u64,
) -> Result<&[u8], String> {
    let start = usize::try_from(offset)
        .map_err(|_| String::from("payload offset exceeds usize"))?;
    let length = usize::try_from(length)
        .map_err(|_| String::from("payload length exceeds usize"))?;
    let end = start
        .checked_add(length)
        .ok_or_else(|| String::from("payload slice overflow"))?;
    data.get(start..end)
        .ok_or_else(|| String::from("payload slice exceeds plaintext"))
}
fn instruction_bytes(
    source_root: &Path,
    instruction: &Instruction,
    plaintext: &[u8],
) -> Result<Vec<u8>, String> {
    match instruction.kind {
        InstructionKind::CopySource => {
            let rel = instruction
                .source_path
                .as_ref()
                .ok_or_else(|| String::from("copy lost source path"))?;
            let path = safe_join(source_root, rel)?;
            fs::read(&path)
                .map_err(|e| format!("cannot read {}: {e}", path.display()))
        },
        InstructionKind::Payload => {
            let (offset, length) = instruction
                .payload_slice
                .ok_or_else(|| String::from("payload lost range"))?;
            Ok(payload_range(plaintext, offset, length)?.to_vec())
        },
        InstructionKind::PatchSource => {
            let rel = instruction
                .source_path
                .as_ref()
                .ok_or_else(|| String::from("patch lost source path"))?;
            let path = safe_join(source_root, rel)?;
            let source = fs::read(&path)
                .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
            let mut out = Vec::new();
            for segment in &instruction.segments {
                match *segment {
                    Segment::Source { offset, length } => {
                        let start = usize::try_from(offset).map_err(|_| {
                            String::from("source offset exceeds usize")
                        })?;
                        let length = usize::try_from(length).map_err(|_| {
                            String::from("source length exceeds usize")
                        })?;
                        let end =
                            start.checked_add(length).ok_or_else(|| {
                                String::from("source slice overflow")
                            })?;
                        out.extend_from_slice(
                            source.get(start..end).ok_or_else(|| {
                                String::from("source slice exceeds file")
                            })?,
                        );
                    },
                    Segment::Payload { offset, length } => out
                        .extend_from_slice(payload_range(
                            plaintext, offset, length,
                        )?),
                }
            }
            Ok(out)
        },
    }
}
fn prepare_staging(output: &Path) -> Result<PathBuf, String> {
    if output.exists() {
        return Err(format!("output already exists: {}", output.display()));
    }
    let parent = output.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)
        .map_err(|e| format!("cannot create output parent: {e}"))?;
    let name = output
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or_else(|| String::from("output filename is not UTF-8"))?;
    let staging = parent.join(format!(".{name}.diff-staging"));
    if staging.exists() {
        return Err(format!("staging already exists: {}", staging.display()));
    }
    fs::create_dir(&staging)
        .map_err(|e| format!("cannot create staging: {e}"))?;
    Ok(staging)
}
fn copy_passthrough_entry(
    source_root: &Path,
    staging_root: &Path,
    source: &Path,
) -> Result<(), String> {
    let meta = fs::symlink_metadata(source)
        .map_err(|e| format!("cannot inspect {}: {e}", source.display()))?;
    let kind = meta.file_type();
    if kind.is_symlink() {
        return Err(format!("symlink unsupported: {}", source.display()));
    }
    let relative = relative_path(source_root, source)?;
    let output = safe_join(staging_root, &relative)?;
    if kind.is_file() {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                format!("cannot create {}: {e}", parent.display())
            })?;
        }
        fs::copy(source, &output).map_err(|e| {
            format!(
                "cannot copy passthrough {} to {}: {e}",
                source.display(),
                output.display()
            )
        })?;
        return Ok(());
    }
    if !kind.is_dir() {
        return Err(format!(
            "non-regular passthrough entry unsupported: {}",
            source.display()
        ));
    }
    fs::create_dir_all(&output)
        .map_err(|e| format!("cannot create {}: {e}", output.display()))?;
    for item in fs::read_dir(source)
        .map_err(|e| format!("cannot read {}: {e}", source.display()))?
    {
        let entry = item.map_err(|e| format!("directory entry failed: {e}"))?;
        copy_passthrough_entry(source_root, staging_root, &entry.path())?;
    }
    Ok(())
}
fn copy_passthrough_roots(
    source_root: &Path,
    staging_root: &Path,
    roots: &[String],
) -> Result<(), String> {
    for root in roots {
        let source = safe_join(source_root, root)?;
        copy_passthrough_entry(source_root, staging_root, &source)?;
    }
    Ok(())
}
fn populate_staging(
    source: &Path,
    staging: &Path,
    metadata: &Metadata,
    plaintext: &[u8],
) -> Result<(), String> {
    for instruction in &metadata.instructions {
        let data = instruction_bytes(source, instruction, plaintext)?;
        if sha256(&data) != instruction.expected_sha256 {
            return Err(format!(
                "instruction hash mismatch: {}",
                instruction.output_path
            ));
        }
        let output = safe_join(staging, &instruction.output_path)?;
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                format!("cannot create {}: {e}", parent.display())
            })?;
        }
        fs::write(&output, data)
            .map_err(|e| format!("cannot write {}: {e}", output.display()))?;
    }
    copy_passthrough_roots(source, staging, &metadata.passthrough_roots)?;
    if snapshot_tree_excluding(staging, &metadata.passthrough_roots)?
        != metadata.target
    {
        return Err(String::from(
            "materialized tree does not match target snapshot",
        ));
    }
    Ok(())
}
fn materialize(
    source: &Path,
    output: &Path,
    metadata: &Metadata,
    plaintext: &[u8],
) -> Result<(), String> {
    let staging = prepare_staging(output)?;
    let result = populate_staging(source, &staging, metadata, plaintext)
        .and_then(|()| {
            fs::rename(&staging, output).map_err(|e| {
                format!("cannot publish staging {}: {e}", staging.display())
            })
        });
    if result.is_err() && staging.exists() {
        let _ = fs::remove_dir_all(&staging);
    }
    result
}
fn arguments() -> Result<(PathBuf, PathBuf), String> {
    let values: Vec<OsString> = env::args_os().collect();
    if values.len() != 3 {
        return Err(format!("usage: {PROFILE} <source-root> <output-root>"));
    }
    Ok((PathBuf::from(&values[1]), PathBuf::from(&values[2])))
}
fn run() -> Result<(), String> {
    let (source, output) = arguments()?;
    let aad = decode_hex(AAD_HEX)?;
    let binding = parse_binding(&decode_hex(BINDING_HEX)?)?;
    let nonce = decode_hex(NONCE_HEX)?;
    let ciphertext = decode_hex(CIPHERTEXT_HEX)?;
    let tag = decode_hex(TAG_HEX)?;
    let metadata = parse_metadata(&aad)?;
    if snapshot_declared_source(&source, &metadata.source)? != metadata.source {
        return Err(String::from(
            "declared source files do not match exact transform snapshot",
        ));
    }
    let key = unlock_key(&source, &binding)?;
    let plaintext = decrypt_payload(&key, &nonce, &ciphertext, &tag, &aad)?;
    materialize(&source, &output, &metadata, &plaintext)
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{PROFILE}: {error}");
        std::process::exit(1);
    }
}
