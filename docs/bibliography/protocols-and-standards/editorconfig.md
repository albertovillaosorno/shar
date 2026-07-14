# EditorConfig

This non-governing record documents a configuration convention and reference
implementation without applying implementation licenses to configured source
files.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository configuration, official
  EditorConfig project identity, specification material, and implementation
  source verified; exact editor and plug-in interpretation remains
  environment-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-13.
- Subject class: Editor-independent configuration format and implementation
  ecosystem.

## Covered Material

The EditorConfig file format, project documentation, editor integrations, and
reference core implementation relevant to SHAR configuration validation.

## Repository Use And Scope

SHAR tracks EditorConfig files to express whitespace, encoding, newline, and
language-specific editor behavior. The canonical validator independently checks
those policies. EditorConfig does not make a particular editor mandatory and
does not relicense files whose formatting it describes.

## Provenance And Version History

The effective behavior depends on the tracked configuration, the consuming
editor or core implementation, and implementation version. Publication,
distribution, or validation evidence should identify the parser used where
behavior is disputed.

## Authorship, Ownership, And Attribution

EditorConfig contributors and individual plugin or core-implementation authors
retain rights in their code and documentation. SHAR contributors retain rights
in repository-authored configuration and source.

## License Or Terms Basis

The official C core repository states that most files use the Simplified BSD
License and identifies separately licensed bundled source files. Editor plugins
and other implementations may use different licenses. The exact implementation
and license inventory control.

## Distribution, Modification, And Compatibility

Tracking `.editorconfig` files does not distribute an EditorConfig parser.
Redistributing a core library or plugin requires its license, attribution, and
third-party notices.

## Compliance Posture

Treat EditorConfig as an editor-independent policy format. Verify the specific
implementation and bundled components before distributing any parser or plugin.

## Source References

- EditorConfig contributors (n.d.) *EditorConfig*. Available at:
  <https://editorconfig.org/> (Accessed: 12 July 2026).
- EditorConfig contributors (n.d.) *Official C core GitHub repository*.
  Available at: <https://github.com/editorconfig/editorconfig-core-c> (Accessed:
  12 July 2026).
- SHAR repository (2026) `.editorconfig` files and `validate.sh`.
