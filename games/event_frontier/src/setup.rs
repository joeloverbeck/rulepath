//! Deterministic setup for Event Frontier.

use std::collections::{BTreeSet, VecDeque};

use engine_core::{DeterministicRng, Diagnostic, SeatId, Seed, SeededRng};

use crate::{
    cards::{CardCatalog, CardId},
    ids::{FactionId, SiteId, STANDARD_CARD_COUNT, STANDARD_EPOCH_COUNT, STANDARD_SEAT_COUNT},
    rules::initialize_card_phase,
    state::{is_reckoning, AdjacencyEntry, EventFrontierState},
    variants::{ScenarioVariant, VariantCatalog},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: ScenarioVariant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self::standard()
    }
}

impl SetupOptions {
    pub fn standard() -> Self {
        let variants =
            VariantCatalog::parse(include_str!("../data/variants.toml")).expect("variants parse");
        Self {
            variant: variants.standard,
        }
    }

    pub fn hard_winter() -> Self {
        let variants =
            VariantCatalog::parse(include_str!("../data/variants.toml")).expect("variants parse");
        Self {
            variant: variants.hard_winter,
        }
    }

    pub fn land_rush() -> Self {
        let variants =
            VariantCatalog::parse(include_str!("../data/variants.toml")).expect("variants parse");
        Self {
            variant: variants.land_rush,
        }
    }
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<EventFrontierState, Diagnostic> {
    if seats.len() != STANDARD_SEAT_COUNT as usize {
        return Err(diagnostic(
            "invalid_seat_count",
            "event_frontier requires exactly two seats",
        ));
    }

    let adjacency = validate_variant(&options.variant)?;
    let catalog = CardCatalog::parse(include_str!("../data/cards.toml"))
        .map_err(|message| diagnostic("invalid_card_catalog", &message))?;
    let deck_order = build_seeded_deck(seed, &catalog)?;

    let mut state = EventFrontierState::new_after_setup(
        options.variant.clone(),
        [seats[0].clone(), seats[1].clone()],
        adjacency,
        deck_order,
    );
    initialize_card_phase(&mut state)?;
    Ok(state)
}

pub fn validate_variant(variant: &ScenarioVariant) -> Result<Vec<AdjacencyEntry>, Diagnostic> {
    if variant.seat_count != STANDARD_SEAT_COUNT {
        return Err(diagnostic(
            "invalid_variant_seat_count",
            "event_frontier variants require exactly two seats",
        ));
    }
    if variant.resource_cap == 0
        || variant.starting_resources.0 > variant.resource_cap
        || variant.starting_resources.1 > variant.resource_cap
    {
        return Err(diagnostic(
            "invalid_variant_resources",
            "event_frontier starting resources must fit the resource cap",
        ));
    }
    if variant.faction_order != [FactionId::Charter, FactionId::Freeholders] {
        return Err(diagnostic(
            "invalid_variant_factions",
            "event_frontier variants require Charter then Freeholders",
        ));
    }
    if variant.epoch_composition.len() != STANDARD_EPOCH_COUNT as usize {
        return Err(diagnostic(
            "invalid_variant_epochs",
            "event_frontier variants require exactly three epochs",
        ));
    }
    validate_site_counts(&variant.start_agents, 3, "agents")?;
    validate_site_counts(&variant.start_settlers, 3, "settlers")?;
    validate_site_counts(&variant.start_caches, 2, "caches")?;
    ensure_unique_sites(&variant.start_depots, "depots")?;

    build_adjacency(&variant.edges)
}

pub fn build_seeded_deck(seed: Seed, catalog: &CardCatalog) -> Result<Vec<CardId>, Diagnostic> {
    let mut rng = SeededRng::from_seed(seed);
    let mut deck = Vec::with_capacity(STANDARD_CARD_COUNT as usize);

    for epoch in 1..=STANDARD_EPOCH_COUNT {
        let mut cards = catalog
            .cards
            .iter()
            .filter(|card| card.epoch_pool == epoch)
            .map(|card| card.id)
            .collect::<Vec<_>>();

        if cards.len() != 7 || cards.iter().filter(|card| is_reckoning(**card)).count() != 1 {
            return Err(diagnostic(
                "invalid_epoch_composition",
                "event_frontier epochs require six events and one Reckoning",
            ));
        }

        shuffle_epoch(&mut cards, &mut rng);
        if cards.first().is_some_and(|card| is_reckoning(*card)) {
            let swap_index = next_bounded_index_unbiased(&mut rng, cards.len() - 1)
                .expect("epoch has non-Reckoning slots")
                + 1;
            cards.swap(0, swap_index);
        }
        deck.extend(cards);
    }

    Ok(deck)
}

pub fn shuffle_epoch<R: DeterministicRng>(cards: &mut [CardId], rng: &mut R) {
    for index in (1..cards.len()).rev() {
        let swap_index =
            next_bounded_index_unbiased(rng, index + 1).expect("shuffle upper bound is nonzero");
        cards.swap(index, swap_index);
    }
}

fn build_adjacency(edges: &[(SiteId, SiteId)]) -> Result<Vec<AdjacencyEntry>, Diagnostic> {
    if edges.len() != 8 {
        return Err(diagnostic(
            "invalid_variant_edges",
            "event_frontier variants require exactly eight trails",
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
                "event_frontier trails must connect two different sites",
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
                "event_frontier variants must not repeat trails",
            ));
        }
        push_neighbor(&mut adjacency, *left, *right);
        push_neighbor(&mut adjacency, *right, *left);
    }

    for entry in &mut adjacency {
        entry.neighbors.sort();
    }
    ensure_connected(&adjacency)?;
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
            "event_frontier site graph must be connected",
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
                "event_frontier starting site counts exceed caps",
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
                &format!("event_frontier {label} must not repeat sites"),
            ));
        }
    }
    Ok(())
}

fn ensure_unique_sites_with_counts(counts: &[(SiteId, u8)], label: &str) -> Result<(), Diagnostic> {
    let sites = counts.iter().map(|(site, _)| *site).collect::<Vec<_>>();
    ensure_unique_sites(&sites, label)
}

fn next_bounded_index_unbiased<R: DeterministicRng>(
    rng: &mut R,
    upper_bound: usize,
) -> Option<usize> {
    if upper_bound == 0 {
        return None;
    }

    let upper = upper_bound as u128;
    let range = u128::from(u64::MAX) + 1;
    let accepted_zone = range - (range % upper);

    loop {
        let value = u128::from(rng.next_u64());
        if value < accepted_zone {
            return Some((value % upper) as usize);
        }
    }
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

    fn scenario_options() -> [SetupOptions; 3] {
        [
            SetupOptions::standard(),
            SetupOptions::hard_winter(),
            SetupOptions::land_rush(),
        ]
    }

    #[test]
    fn setup_rejects_wrong_seat_count() {
        assert!(setup_match(
            Seed(0),
            &[SeatId("seat_0".to_owned())],
            &SetupOptions::default()
        )
        .is_err());
    }

    #[test]
    fn setup_initializes_standard_state() {
        let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup");

        assert_eq!(state.factions, [FactionId::Charter, FactionId::Freeholders]);
        assert_eq!(state.resources.funds, 2);
        assert_eq!(state.resources.provisions, 4);
        assert_eq!(state.scores.charter, 0);
        assert_eq!(state.scores.freeholders, 0);
        assert_eq!(state.reckoning_count, 0);
        assert_eq!(state.terminal_outcome, None);
        assert_eq!(
            state.card_phase,
            crate::state::CardPhase::AwaitingFirstChoice {
                faction: FactionId::Freeholders
            }
        );
        assert_eq!(state.site(SiteId::Charterhouse).expect("site").agents, 2);
        assert!(state.site(SiteId::Charterhouse).expect("site").depot);
        assert_eq!(state.site(SiteId::Landing).expect("site").settlers, 3);
        assert_eq!(state.site(SiteId::Landing).expect("site").cache_count, 1);
        assert_eq!(state.adjacency.len(), SiteId::ALL.len());
        assert!(state.neighbors(SiteId::Crossing).expect("neighbors").len() >= 3);
        assert!(state.deck.current.is_some());
        assert!(state.deck.next_public.is_some());
        assert_eq!(state.deck.undrawn.len(), STANDARD_CARD_COUNT as usize - 2);
        assert!(state.deck.discard.is_empty());
    }

    #[test]
    fn setup_is_deterministic_for_same_seed_and_scenario() {
        let seats = seats();
        for options in scenario_options() {
            let first = setup_match(Seed(42), &seats, &options).expect("first setup");
            let second = setup_match(Seed(42), &seats, &options).expect("second setup");

            assert_eq!(first.deck, second.deck);
            assert_eq!(first.stable_summary(), second.stable_summary());
            assert_eq!(first.stable_hash(), second.stable_hash());
        }
    }

    #[test]
    fn reckoning_is_never_first_in_any_epoch_for_many_seeds() {
        let catalog = CardCatalog::parse(include_str!("../data/cards.toml")).expect("cards");

        for seed in 0..200 {
            let deck = build_seeded_deck(Seed(seed), &catalog).expect("deck");
            for epoch_start in [0, 7, 14] {
                assert!(
                    !is_reckoning(deck[epoch_start]),
                    "seed {seed} placed a Reckoning first at {epoch_start}"
                );
                assert_eq!(
                    deck[epoch_start..epoch_start + 7]
                        .iter()
                        .filter(|card| is_reckoning(**card))
                        .count(),
                    1
                );
            }
        }
    }

    #[test]
    fn fixed_seed_has_known_deck_order() {
        let catalog = CardCatalog::parse(include_str!("../data/cards.toml")).expect("cards");
        let deck = build_seeded_deck(Seed(1), &catalog).expect("deck");

        assert_eq!(
            deck.iter().map(|card| card.as_str()).collect::<Vec<_>>(),
            vec![
                "ef_high_meadow_fair",
                "ef_reckoning_one",
                "ef_survey_ban",
                "ef_storehouse_fire",
                "ef_border_survey",
                "ef_toll_roads",
                "ef_river_mists",
                "ef_long_season",
                "ef_requisition",
                "ef_freeholder_moot",
                "ef_trail_washout",
                "ef_reckoning_two",
                "ef_charter_audit",
                "ef_depot_grants",
                "ef_granite_pass_snows",
                "ef_last_light",
                "ef_reckoning_three",
                "ef_cache_boom",
                "ef_crossing_market",
                "ef_agents_recall",
                "ef_old_mill_strike",
            ]
        );
    }

    #[test]
    fn scenario_variants_initialize_distinct_starts() {
        let seats = seats();
        let winter = setup_match(Seed(5), &seats, &SetupOptions::hard_winter()).expect("winter");
        let land_rush = setup_match(Seed(5), &seats, &SetupOptions::land_rush()).expect("land");

        assert_eq!(winter.resources.funds, 2);
        assert_eq!(winter.site(SiteId::Landing).expect("site").settlers, 2);
        assert_eq!(land_rush.resources.funds, 4);
        assert_eq!(land_rush.site(SiteId::Crossing).expect("site").agents, 1);
        assert_eq!(
            land_rush
                .site(SiteId::HighMeadow)
                .expect("site")
                .cache_count,
            1
        );
    }
}
