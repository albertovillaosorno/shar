// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - World catalog unit tests.
// - Must-Not:
//   - Own production behavior or artifact publication.
// - Allows:
//   - Focused serialization assertions for deferred source relationships.
// - Split-When:
//   - Another world-catalog contract gains independent test ownership.
// - Merge-When:
//   - The world catalog no longer owns deferred relationship rendering.
// - Summary:
//   - World catalog unit tests.
// - Description:
//   - Locks source-backed deferred composite relationship serialization.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Relationship fields must remain explicit and non-inferred.
//

//! World catalog unit tests.

use super::deferred_binding_value;
use crate::adapters::driven::local::prop_catalog::model::DeferredRenderBinding;

#[test]
fn deferred_binding_serialization_keeps_exact_source_relationship() {
    let value = deferred_binding_value(&DeferredRenderBinding {
        composite_prop_index: 2,
        source_identity: "light-beam".to_owned(),
        skeleton_joint_id: 7,
        is_translucent: true,
        component_kind: Some("quad_group".to_owned()),
        component_member_id: Some("light-beam__ordinal_42".to_owned()),
        source_ordinal: Some(42),
    });
    assert_eq!(value.get("composite_prop_index"), Some(&2.into()));
    assert_eq!(value.get("source_identity"), Some(&"light-beam".into()));
    assert_eq!(value.get("skeleton_joint_id"), Some(&7.into()));
    assert_eq!(value.get("is_translucent"), Some(&true.into()));
    assert_eq!(value.get("component_kind"), Some(&"quad_group".into()));
    assert_eq!(
        value.get("component_member_id"),
        Some(&"light-beam__ordinal_42".into())
    );
    assert_eq!(value.get("source_ordinal"), Some(&42.into()));
}
