# Open-Source Distribution Compliance

> [Legal Research Disclaimer](../disclaimer.md) applies to this record.

- Status: License framework verified; distribution-specific application requires
  exact evidence.
- Jurisdiction: Every territory of distribution and the governing law of each
  license.
- Authority level: Controlling license texts, notices, source offers, and
  distribution facts.
- As-of date: 2026-07-12.
- Counsel review: Not performed.

## Question Presented

What attribution, notice, source-code, relinking, patent, modification,
installation-information, and reciprocal-license obligations apply to every
third-party component actually distributed with SHAR?

## Verified Baseline

Invoking an external tool is different from redistributing it. A project name or
SPDX identifier is not a substitute for the exact license, notices, dependency
graph, build configuration, and delivered artifact.

## Not Established

This record does not establish that every dependency is permissively licensed,
that dynamic or static linking has one universal result, that a binary download
includes required source, or that an upstream notice inventory is complete.

## Required Facts

- Exact source and binary artifacts delivered to users.
- Direct and transitive dependencies, features, build flags, and linked
  libraries.
- Modifications, patches, vendoring, static linking, and generated code.
- License files, NOTICE files, copyright lines, exceptions, and source offers.
- Distribution channel, recipients, territory, and installation model.

## Required Authorities

- The exact license and notice files shipped by every upstream project.
- Source archives and build metadata corresponding to each distributed binary.
- Official GNU GPL and LGPL texts for applicable components.
- Apache-2.0, MIT, BSD, curl, Unicode, and other controlling license texts.

## Verification Checklist

- Generate a distribution-specific software bill of materials.
- Map every conveyed file to its license, notices, and source origin.
- Verify corresponding-source and relinking requirements where applicable.
- Preserve build scripts, configuration, patches, and written source offers.
- Obtain qualified counsel before distributing unresolved mixed-license bundles.
