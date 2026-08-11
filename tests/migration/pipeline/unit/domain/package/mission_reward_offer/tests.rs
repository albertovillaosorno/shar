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
//   - Unit evidence for source reward merchandise offers.
// - Must-Not:
//   - Assert ownership, unlock, transaction, or persistence behavior.
// - Allows:
//   - Verify reviewed offer type/vendor/price source bindings.
// - Split-When:
//   - Runtime purchase tests gain separate authority.
// - Merge-When:
//   - Reward catalog tests own these exact source offer invariants.
// - Summary:
//   - Source reward offer unit tests.
// - Description:
//   - Proves `forsale` rows become typed package-backed offer evidence.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - Unsupported pairings and nonpositive prices fail closed.
//

//! Unit evidence for typed source merchandise and prices.

use super::*;
use crate::domain::MissionRewardReferenceReport;

fn references() -> MissionRewardReferenceReport {
    MissionRewardReferenceReport::from_entries_for_tests(&[
        (
            1,
            "default_v",
            "car",
            "defaultcar",
            "1",
            None,
            None,
            "package-default",
        ),
        (
            2,
            "sale_car",
            "car",
            "forsale",
            "3",
            Some("300"),
            Some("simpson"),
            "package-car",
        ),
        (
            3,
            "sale_skin",
            "skin",
            "forsale",
            "3",
            Some("250"),
            Some("interior"),
            "package-skin",
        ),
    ])
}

#[test]
fn types_package_backed_sale_offers() -> Result<(), String> {
    let report = preflight_mission_reward_offers(&references())?;
    let [car, skin] = report.bindings() else {
        return Err("reward offer count changed".to_owned());
    };
    assert_eq!(car.source_ordinal(), 2);
    assert_eq!(car.reward_id(), "sale_car");
    assert_eq!(car.kind(), MissionRewardOfferKind::Car);
    assert_eq!(car.source_level(), "3");
    assert_eq!(car.level(), 3);
    assert_eq!(car.source_price(), "300");
    assert_eq!(car.price(), 300);
    assert_eq!(car.vendor(), MissionRewardOfferVendor::Simpson);
    assert_eq!(car.package_id(), "package-car");
    assert_eq!(car.package_root(), "package-car-root");
    assert_eq!(skin.kind(), MissionRewardOfferKind::Skin);
    assert_eq!(skin.vendor(), MissionRewardOfferVendor::Interior);
    Ok(())
}

#[test]
fn rejects_unreviewed_offer_pairing() {
    let references = MissionRewardReferenceReport::from_entries_for_tests(&[(
        4,
        "bad",
        "skin",
        "forsale",
        "1",
        Some("100"),
        Some("gil"),
        "package-bad",
    )]);
    assert!(preflight_mission_reward_offers(&references).is_err());
}

#[test]
fn rejects_zero_price() {
    let references = MissionRewardReferenceReport::from_entries_for_tests(&[(
        5,
        "free",
        "car",
        "forsale",
        "1",
        Some("0"),
        Some("gil"),
        "package-free",
    )]);
    assert!(preflight_mission_reward_offers(&references).is_err());
}

#[test]
fn rejects_priced_non_sale_reward() {
    let references = MissionRewardReferenceReport::from_entries_for_tests(&[(
        6,
        "priced-default",
        "car",
        "defaultcar",
        "1",
        Some("100"),
        Some("gil"),
        "package-default",
    )]);
    assert!(preflight_mission_reward_offers(&references).is_err());
}
