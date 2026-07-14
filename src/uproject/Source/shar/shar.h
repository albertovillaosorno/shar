#ifndef SHAR_UPROJECT_SOURCE_SHAR_SHAR_H
#define SHAR_UPROJECT_SOURCE_SHAR_SHAR_H
// File:
//   - shar.h
// Path:
//   - src/uproject/Source/shar/shar.h
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - The public include boundary for the authored SHAR runtime module.
// - Must-Not:
//   - Contain implementation, gameplay state, or Unreal Engine private
//   - headers.
// - Allows:
//   - Shared declarations required by authored SHAR runtime code.
// - Split-When:
//   - A declaration group forms an independently testable public contract.
// - Merge-When:
//   - Another public header owns the identical include boundary with no
//   - distinct invariant.
// - Summary:
//   - Defines the SHAR runtime public header boundary.
// - Description:
//   - Provides a stable guarded include surface without embedding engine or
//   - gameplay implementation.
// - Usage:
//   - Included by authored SHAR translation units that require shared public
//   - declarations.
// - Defaults:
//   - Exports no declarations until a concrete shared contract is required.
//
// ADRs:
// - docs/adr/unreal/project/cpp-primary-blueprint-compatible-project.md
//
// Large file:
//   - false
//

// Provides the guarded public include surface for the SHAR runtime module and
// intentionally exports no declarations until a shared contract is required.

#endif // SHAR_UPROJECT_SOURCE_SHAR_SHAR_H
