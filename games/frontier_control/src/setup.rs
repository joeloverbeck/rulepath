//! Deterministic setup for Frontier Control.

use std::collections::{BTreeSet, VecDeque};

use engine_core::{Diagnostic, SeatId};

use crate::{
    ids::{FactionId, SiteId, STANDARD_SEAT_COUNT},
    state::{AdjacencyEntry, FrontierControlState},
    variants::VariantMap,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: VariantMap,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: VariantMap::standard(),
        }
    }
}

pub fn setup_match(
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<FrontierControlState, Diagnostic> {
    if seats.len() != STANDARD_SEAT_COUNT as usize {
        return Err(diagnostic(
            "invalid_seat_count",
            "frontier_control requires exactly two seats",
        ));
    }
    let adjacency = validate_variant(&options.variant)?;

    Ok(FrontierControlState::new_after_setup(
        options.variant.clone(),
        [seats[0].clone(), seats[1].clone()],
        adjacency,
    ))
}

pub fn validate_variant(variant: &VariantMap) -> Result<Vec<AdjacencyEntry>, Diagnostic> {
    if variant.seat_count != STANDARD_SEAT_COUNT {
        return Err(diagnostic(
            "invalid_variant_seat_count",
            "frontier_control variants require exactly two seats",
        ));
    }
    if variant.action_budget == 0 || variant.round_count == 0 || variant.unit_cap_per_site == 0 {
        return Err(diagnostic(
            "invalid_variant_zero",
            "frontier_control action budget, round count, and unit cap must be nonzero",
        ));
    }
    if variant.faction_order != [FactionId::Garrison, FactionId::Prospectors] {
        return Err(diagnostic(
            "invalid_variant_factions",
            "frontier_control variants require Garrison then Prospectors",
        ));
    }
    validate_site_counts(
        &variant.start_units.guards,
        variant.unit_cap_per_site,
        "guards",
    )?;
    validate_site_counts(
        &variant.start_units.crews,
        variant.unit_cap_per_site,
        "crews",
    )?;
    validate_site_counts(&variant.stake_values, u8::MAX, "stake values")?;
    ensure_unique_sites(&variant.fort_sites, "fort sites")?;
    ensure_unique_sites_with_counts(&variant.stake_values, "stake values")?;

    let adjacency = build_adjacency(&variant.edges)?;
    ensure_connected(&adjacency)?;
    Ok(adjacency)
}

fn build_adjacency(edges: &[(SiteId, SiteId)]) -> Result<Vec<AdjacencyEntry>, Diagnostic> {
    if edges.is_empty() {
        return Err(diagnostic(
            "invalid_variant_edges",
            "frontier_control variants require at least one trail",
        ));
    }

    let mut seen_edges = BTreeSet::new();
    let mut adjacency = SiteId::ALL
        .into_iter()
        .map(|site| AdjacencyEntry {
            site,
            neighbors: Vec::new(),
        })
        .collect::<Vec<_>>();

    for (left, right) in edges {
        if left == right {
            return Err(diagnostic(
                "invalid_variant_loop_edge",
                "frontier_control trails must connect two different sites",
            ));
        }
        let normalized = if left <= right {
            (*left, *right)
        } else {
            (*right, *left)
        };
        if !seen_edges.insert(normalized) {
            return Err(diagnostic(
                "invalid_variant_duplicate_edge",
                "frontier_control variants must not repeat trails",
            ));
        }
        push_neighbor(&mut adjacency, *left, *right);
        push_neighbor(&mut adjacency, *right, *left);
    }

    for entry in &mut adjacency {
        entry.neighbors.sort();
    }

    Ok(adjacency)
}

fn push_neighbor(adjacency: &mut [AdjacencyEntry], site: SiteId, neighbor: SiteId) {
    if let Some(entry) = adjacency.iter_mut().find(|entry| entry.site == site) {
        entry.neighbors.push(neighbor);
    }
}

fn ensure_connected(adjacency: &[AdjacencyEntry]) -> Result<(), Diagnostic> {
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::from([SiteId::ALL[0]]);

    while let Some(site) = queue.pop_front() {
        if !seen.insert(site) {
            continue;
        }
        if let Some(entry) = adjacency.iter().find(|entry| entry.site == site) {
            for neighbor in &entry.neighbors {
                queue.push_back(*neighbor);
            }
        }
    }

    if seen.len() != SiteId::ALL.len() {
        return Err(diagnostic(
            "invalid_variant_disconnected_graph",
            "frontier_control site graph must be connected",
        ));
    }
    Ok(())
}

fn validate_site_counts(
    counts: &[(SiteId, u8)],
    max_value: u8,
    label: &str,
) -> Result<(), Diagnostic> {
    ensure_unique_sites_with_counts(counts, label)?;
    for (_, count) in counts {
        if *count > max_value {
            return Err(diagnostic(
                "invalid_variant_site_count",
                "frontier_control site counts exceed the variant cap",
            ));
        }
    }
    Ok(())
}

fn ensure_unique_sites(sites: &[SiteId], label: &str) -> Result<(), Diagnostic> {
    let mut seen = BTreeSet::new();
    for site in sites {
        if !seen.insert(*site) {
            return Err(diagnostic(
                "invalid_variant_duplicate_site",
                &format!("frontier_control {label} must not repeat sites"),
            ));
        }
    }
    Ok(())
}

fn ensure_unique_sites_with_counts(counts: &[(SiteId, u8)], label: &str) -> Result<(), Diagnostic> {
    let sites = counts.iter().map(|(site, _)| *site).collect::<Vec<_>>();
    ensure_unique_sites(&sites, label)
}

fn diagnostic(code: &str, message: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::StableSerialize;

    fn seats() -> [SeatId; 2] {
        [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
    }

    #[test]
    fn setup_initializes_standard_state_without_rng() {
        let state = setup_match(&seats(), &SetupOptions::default()).expect("setup succeeds");

        assert_eq!(state.round_number, 1);
        assert_eq!(state.active_faction, FactionId::Prospectors);
        assert_eq!(
            state.phase,
            crate::state::Phase::Action {
                budget_remaining: 2
            }
        );
        assert_eq!(
            state.factions,
            [FactionId::Garrison, FactionId::Prospectors]
        );
        assert_eq!(state.scores.garrison, 0);
        assert_eq!(state.scores.prospectors, 0);
        assert!(state.terminal_outcome.is_none());
        assert_eq!(state.site(SiteId::Gatehouse).expect("site").guards, 2);
        assert_eq!(state.site(SiteId::BaseCamp).expect("site").crews, 3);
        assert_eq!(state.neighbors(SiteId::Quarry).expect("neighbors").len(), 4);
    }

    #[test]
    fn setup_is_deterministic_for_same_variant() {
        let options = SetupOptions::default();
        let first = setup_match(&seats(), &options).expect("first setup succeeds");
        let second = setup_match(&seats(), &options).expect("second setup succeeds");

        assert_eq!(first.stable_summary(), second.stable_summary());
        assert_eq!(first.stable_hash(), second.stable_hash());
    }

    #[test]
    fn highlands_setup_uses_second_map() {
        let options = SetupOptions {
            variant: VariantMap::highlands(),
        };
        let state = setup_match(&seats(), &options).expect("setup succeeds");

        assert_eq!(state.variant.id, crate::ids::VARIANT_HIGHLANDS_ID);
        assert_eq!(state.variant.round_count, 7);
        assert!(state.site(SiteId::Quarry).expect("site").fort);
        assert_eq!(state.site(SiteId::Timberline).expect("site").crews, 1);
    }

    #[test]
    fn setup_rejects_wrong_seat_count() {
        assert!(setup_match(&[SeatId("seat_0".to_owned())], &SetupOptions::default()).is_err());
    }

    #[test]
    fn invalid_map_data_is_rejected() {
        let mut duplicate_edge = VariantMap::standard();
        duplicate_edge.edges.push((SiteId::Ford, SiteId::Gatehouse));
        assert_eq!(
            validate_variant(&duplicate_edge)
                .expect_err("duplicate edge rejected")
                .code,
            "invalid_variant_duplicate_edge"
        );

        let mut disconnected = VariantMap::standard();
        disconnected
            .edges
            .retain(|(left, right)| *left != SiteId::BaseCamp && *right != SiteId::BaseCamp);
        assert_eq!(
            validate_variant(&disconnected)
                .expect_err("disconnected graph rejected")
                .code,
            "invalid_variant_disconnected_graph"
        );

        let mut duplicate_start = VariantMap::standard();
        duplicate_start
            .start_units
            .crews
            .push((SiteId::BaseCamp, 1));
        assert_eq!(
            validate_variant(&duplicate_start)
                .expect_err("duplicate start rejected")
                .code,
            "invalid_variant_duplicate_site"
        );

        let mut over_cap = VariantMap::standard();
        over_cap.start_units.guards = vec![(SiteId::Gatehouse, 4)];
        assert_eq!(
            validate_variant(&over_cap)
                .expect_err("over-cap start rejected")
                .code,
            "invalid_variant_site_count"
        );
    }
}
