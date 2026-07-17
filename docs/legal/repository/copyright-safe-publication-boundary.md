# Copyright-Safe Publication Boundary

> [Legal Research Disclaimer](../disclaimer.md) applies to this record.

- Status: Repository publication boundary.
- As-of date: 2026-07-16.
- Counsel review: Not performed.

## Purpose

A later deletion does not remove a file from earlier Git history. Public history
must therefore exclude material that the repository is not authorized to publish
at the moment each commit is created.

## Publication Boundary

The public history must not contain original game source, executables,
proprietary assets, extracted payloads, internal production documents,
unlicensed tools, credentials, personal machine data, absolute user paths, or
row-level inventories of protected local evidence.

Tracked tests use synthetic, independently authored, or otherwise lawfully
redistributable fixtures. Local research evidence remains outside Git.
Unpublished unsafe history must be repaired before publication rather than
relying on a later deletion.

The following matrix applies the accepted [Lawful local input and publication
boundary][lawful-input-boundary] to the current repository surface. It records
the publication baseline, not a license grant for any particular artifact.

<!-- markdownlint-disable MD013 -->

| Artifact class                                  | Public baseline                                           | Required evidence before inclusion                                                             |
| :---------------------------------------------- | :-------------------------------------------------------- | :--------------------------------------------------------------------------------------------- |
| Repository source, schemas, and documentation   | Allowed when independently authored or otherwise licensed | Provenance, license scope, and excluded-material scan                                          |
| Synthetic tests and fixtures                    | Allowed                                                   | Reproducible synthetic origin or a redistribution license                                      |
| Binaries, installers, and plugins               | Conditional                                               | Source mapping, dependency notices, license compatibility, and secret scan                     |
| Manifests and reports                           | Conditional                                               | No protected payload, unnecessary original names, personal data, credentials, or private paths |
| Screenshots, video, audio, models, and textures | Excluded by default                                       | Artifact-specific ownership or redistribution permission                                       |
| Original game files and extracted payloads      | Excluded                                                  | No public-repository exception is established                                                  |
| Converted game-derived output                   | Local only by default                                     | Source-content rights and output-specific distribution authority                               |
| Third-party replacement media                   | Excluded by default                                       | Exact license, attribution, modification, and redistribution authority                         |

<!-- markdownlint-enable MD013 -->

The baseline does not change according to whether the repository is described as
personal, educational, noncommercial, preservation-oriented, or open source.
Those facts may be relevant to a legal analysis but do not supply publication
authority by themselves.

*Universal City Studios, Inc. v. Corley* also requires a separate publication
review for functional access-circumvention material. A potential lawful use of
content reached after circumvention does not itself authorize distribution of a
decryption tool. Purposeful links to such a tool can also present trafficking
risk when the publisher knows the destination material and maintains the link to
disseminate it. Ordinary source links, compatibility references, and research
citations require their own facts and are not categorically prohibited.

*Capitol Records, LLC v. ReDigi Inc.* also rejects treating destination storage
as the same material object merely because source data is deleted during a
digital transfer. Fixing protected content on a repository host, artifact
server, or recipient device can create a new copy even when only one complete
usable instance exists at a time. First sale therefore does not by itself
authorize uploading game-derived output or a protected payload.

*Authors Guild v. Google, Inc.* distinguishes complete internal processing from
a constrained public search output on its record. That distinction does not make
a repository index automatically publishable. Search results, manifests,
reports, and previews must omit reconstructable expression and must not provide
a substitute for the original files. Complete local input remains outside public
history unless separate distribution authority exists.

*Metro-Goldwyn-Mayer Studios Inc. v. Grokster, Ltd.* requires review of the
repository's public purpose evidence. Feature descriptions, examples, tutorials,
demonstrations, support messages, and release wording must not promote
infringement or target demand for unauthorized substitutes. Mere knowledge,
ordinary technical support, or failure to filter is not independently enough to
establish inducement.

## Validation

- Inspect every proposed tracked path and blob.
- Scan proposed history for excluded material and secrets.
- Verify license authority for repository-authored and third-party material.
- Preserve required notices and attribution.
- Stop when publication authority is unclear.

## Related Authorities

- [Interoperability and user responsibility][interoperability-boundary]
- [Authors Guild v. Google, Inc](../cases/authors-guild-v-google.md)
- [Capitol Records, LLC v. ReDigi Inc.](../cases/capitol-records-v-redigi.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Metro-Goldwyn-Mayer Studios Inc. v. Grokster, Ltd.](../cases/mgm-studios-v-grokster.md)
- [Universal City Studios, Inc. v. Corley](../cases/universal-v-corley.md)
- [GitHub hosting and terms](../platforms/github-hosting-and-terms.md)

[interoperability-boundary]: interoperability-and-user-responsibility.md
[lawful-input-boundary]:
  ../../adr/legal/lawful-local-input-and-publication-boundary.md
