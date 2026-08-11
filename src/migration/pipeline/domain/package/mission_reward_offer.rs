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
//   - Typed source merchandise and price evidence from `BindReward`.
// - Must-Not:
//   - Assign ownership, unlock, purchase transaction, or save-game behavior.
// - Allows:
//   - Type reviewed `forsale` reward, level, price, and vendor tokens.
// - Split-When:
//   - Runtime purchase or persistence policy gains an independent authority.
// - Merge-When:
//   - Final reward catalog compilation owns these exact source offers.
// - Summary:
//   - Source reward merchandise-offer preflight.
// - Description:
//   - Promotes reviewed `forsale` reward tokens into deterministic offer rows.
// - Usage:
//   - Runs after canonical `BindReward` package-reference preflight.
// - Defaults:
//   - Missing price/vendor, unsupported pairings, and duplicate ids fail
//     closed.
//

//! Typed source merchandise and price evidence from `BindReward`.

use std::collections::BTreeSet;

use super::MissionRewardReferenceReport;

/// Reviewed source reward type for a `forsale` offer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionRewardOfferKind {
    /// Exact source `car` reward type.
    Car,
    /// Exact source `skin` reward type.
    Skin,
}

/// Reviewed exact vendor token for a `forsale` offer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionRewardOfferVendor {
    /// Exact source `gil` vendor token.
    Gil,
    /// Exact source `simpson` vendor token.
    Simpson,
    /// Exact source `interior` vendor token.
    Interior,
}

/// One canonical package-backed source merchandise offer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionRewardOfferBinding {
    source_ordinal: usize,
    reward_id: String,
    kind: MissionRewardOfferKind,
    source_level: String,
    level: u8,
    source_price: String,
    price: u32,
    vendor: MissionRewardOfferVendor,
    package_id: String,
    package_root: String,
}

impl MissionRewardOfferBinding {
    /// Return source `BindReward` statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return exact source reward identity.
    #[must_use]
    pub fn reward_id(&self) -> &str {
        &self.reward_id
    }

    /// Return typed exact source reward kind.
    #[must_use]
    pub const fn kind(&self) -> MissionRewardOfferKind {
        self.kind
    }

    /// Return exact source level lexeme.
    #[must_use]
    pub fn source_level(&self) -> &str {
        &self.source_level
    }

    /// Return validated base-game level number.
    #[must_use]
    pub const fn level(&self) -> u8 {
        self.level
    }

    /// Return exact source price lexeme.
    #[must_use]
    pub fn source_price(&self) -> &str {
        &self.source_price
    }

    /// Return validated positive source price.
    #[must_use]
    pub const fn price(&self) -> u32 {
        self.price
    }

    /// Return exact reviewed vendor token as a typed source value.
    #[must_use]
    pub const fn vendor(&self) -> MissionRewardOfferVendor {
        self.vendor
    }

    /// Return canonical package id for the offered reward.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return canonical package root for the offered reward.
    #[must_use]
    pub fn package_root(&self) -> &str {
        &self.package_root
    }
}

/// Typed source merchandise offers in authored order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionRewardOfferReport {
    bindings: Vec<MissionRewardOfferBinding>,
}

impl MissionRewardOfferReport {
    /// Return typed `forsale` offers in source order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionRewardOfferBinding] {
        &self.bindings
    }
}

/// Compile package-backed `forsale` source rewards into typed offer evidence.
///
/// # Errors
///
/// Returns an error for price/vendor shape drift, unsupported source token
/// pairings, nonpositive prices, or duplicate offer identities.
pub fn preflight_mission_reward_offers(
    references: &MissionRewardReferenceReport,
) -> Result<MissionRewardOfferReport, String> {
    let mut bindings = Vec::new();
    let mut ids = BTreeSet::new();
    for reference in references.bindings() {
        if reference.source_mode_token() != "forsale" {
            if reference.source_cost().is_some()
                || reference.source_vendor().is_some()
            {
                return Err(
                    "non-forsale BindReward unexpectedly carries offer fields"
                        .to_owned(),
                );
            }
            continue;
        }
        let price_source = reference
            .source_cost()
            .ok_or_else(|| {
                "forsale BindReward is missing source price".to_owned()
            })?;
        let vendor_source = reference
            .source_vendor()
            .ok_or_else(|| {
                "forsale BindReward is missing source vendor".to_owned()
            })?;
        let price = price_source
            .parse::<u32>()
            .map_err(|_error| {
                "forsale BindReward price is not numeric".to_owned()
            })?;
        if price == 0 {
            return Err("forsale BindReward price must be positive".to_owned());
        }
        let level = reference
            .source_level()
            .parse::<u8>()
            .map_err(|_error| {
                "forsale BindReward level is not numeric".to_owned()
            })?;
        let (kind, vendor) = offer_tokens(
            reference.reward_type_token(),
            vendor_source,
        )?;
        if !ids.insert(reference.reward_id().to_owned()) {
            return Err("forsale BindReward reward id is duplicated".to_owned());
        }
        bindings.push(MissionRewardOfferBinding {
            source_ordinal: reference.source_ordinal(),
            reward_id: reference.reward_id().to_owned(),
            kind,
            source_level: reference.source_level().to_owned(),
            level,
            source_price: price_source.to_owned(),
            price,
            vendor,
            package_id: reference.package_id().to_owned(),
            package_root: reference.package_root().to_owned(),
        });
    }
    Ok(MissionRewardOfferReport { bindings })
}

fn offer_tokens(
    reward_type: &str,
    vendor: &str,
) -> Result<(MissionRewardOfferKind, MissionRewardOfferVendor), String> {
    match (reward_type, vendor) {
        ("car", "gil") => Ok((
            MissionRewardOfferKind::Car,
            MissionRewardOfferVendor::Gil,
        )),
        ("car", "simpson") => Ok((
            MissionRewardOfferKind::Car,
            MissionRewardOfferVendor::Simpson,
        )),
        ("skin", "interior") => Ok((
            MissionRewardOfferKind::Skin,
            MissionRewardOfferVendor::Interior,
        )),
        _ => Err(
            "forsale BindReward type/vendor pairing is not reviewed".to_owned(),
        ),
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_reward_offer/tests.rs"]
mod tests;
