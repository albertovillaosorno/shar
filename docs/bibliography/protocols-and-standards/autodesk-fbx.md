# Autodesk FBX

This non-governing record documents a proprietary three-dimensional interchange
technology without granting rights in Autodesk code, SDKs, documentation, names,
marks, or embedded content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The emitted file-version value `7700` is
  confirmed in the repository FBX writer source, and Autodesk product
  documentation was reviewed; a complete normative public binary-format
  specification does not exist and was not relied upon.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Independently authored interoperability output.
- Subject class: Proprietary three-dimensional interchange technology.

## Covered Material

Binary FBX 7.7 artifacts authored by SHAR, including document structure,
objects, connections, geometry, materials, embedded PNG media, skeletons,
skinning, animation, and metadata.

## Repository Use And Scope

The `src/fbx/` crate independently serializes canonical binary FBX 7.7 without
using the Autodesk FBX SDK. Blender and Maya are optional review adapters. The
repository rejects ASCII FBX as canonical output.

## Provenance And Version History

FBX originated with Kaydara and is now Autodesk technology. Autodesk publishes
SDK and product documentation, but this record has not verified a complete
normative public byte-level specification for binary FBX 7.7. Repository
conformance is therefore defined by independently authored contracts and
cross-tool validation, not claimed Autodesk certification.

## Authorship, Ownership, And Attribution

Autodesk and relevant contributors retain rights in FBX technology, SDKs,
documentation, names, and marks. SHAR contributors retain rights in the
independently authored serializer. Rights in embedded models, textures,
animation, and metadata remain separate.

## License Or Terms Basis

The Autodesk FBX SDK and documentation have their own terms. SHAR does not use
or redistribute the SDK merely by writing interoperable files. No claim is made
that the FBX name or format is open source.

## Distribution, Modification, And Compatibility

Writing a compatible container does not create rights in embedded content.
Validation in Autodesk or third-party applications demonstrates observed
interoperability only and must not be represented as endorsement, certification,
or authorization.

## Compliance Posture

- Keep serializer implementation independent from proprietary SDK source.
- Record the generated FBX version, hashes, embedded media, and review tools.
- Do not redistribute Autodesk SDK binaries or documentation without terms
  review.
- Review rights in every embedded texture, mesh, skeleton, and animation.
- Do not describe application acceptance as formal FBX certification.

## Technical Baseline And SHAR Profile

### Public baseline

Autodesk publishes FBX SDK and product interoperability documentation, but the
reviewed first-party material is implementation-oriented and is not treated here
as a complete open normative binary wire-format specification comparable to an
RFC. Compatibility claims therefore require exact emitted-version and importer
test evidence.

### SHAR profile

The repository-owned writer currently emits deterministic binary FBX 7.7:

- file version value `7700`;
- the `Kaydara FBX Binary` signature;
- little-endian encoding;
- 64-bit node metadata;
- uncompressed property arrays;
- checked absolute node offsets, null records, and a deterministic footer; and
- a fixed creation timestamp and file identity for byte-stable output.

Blender and Maya are not generation, conversion, repair, validation, or
acceptance authorities for the canonical artifact. They may not substitute for
repository serializer tests. This record does not claim Autodesk certification
or universal FBX compatibility.

### Use-specific evidence limits

Before declaring support for a specific consumer and version, record the exact
node and property subset, embedded-media rule, animation-profile matrix, and
importer acceptance evidence. This bibliography record establishes the public
and repository evidence reviewed; it does not establish universal FBX
compatibility.

### Verified sources

- Autodesk, *FBX SDK documentation*. <https://help.autodesk.com/>
- SHAR repository evidence: `src/fbx/src/adapters/driven/binary_fbx.rs` and the
  binary writer tests.

## Source References

- Autodesk (n.d.) *FBX overview*. Available at:
  <https://www.autodesk.com/products/fbx/overview> (Accessed: 12 July 2026).
- Autodesk (n.d.) *FBX SDK documentation*. Available at:
  <https://help.autodesk.com/view/FBX/2020/ENU/> (Accessed: 12 July 2026).
- SHAR repository (2026) `src/fbx/` and FBX architecture records.
