//! `race_to_n` foundation crate.

pub mod ids;
pub mod setup;
pub mod state;
pub mod variants;

use engine_core::{
    ActionTree, Actor, CommandEnvelope, DeterministicRng, Diagnostic, EffectEnvelope,
    FreshnessToken, Game, SeatId, Seed, Viewer,
};

pub use ids::RaceSeat;
pub use setup::{setup_match, SetupOptions};
pub use state::{CounterValue, FoundationView, RaceState};
pub use variants::{Manifest, Variant, VariantCatalog};

#[derive(Clone, Debug, Default)]
pub struct RaceToN;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedAction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaceEffect;

impl Game for RaceToN {
    type Setup = SetupOptions;
    type State = RaceState;
    type ValidatedAction = ValidatedAction;
    type Effect = RaceEffect;
    type View = FoundationView;

    fn setup(
        &self,
        seed: Seed,
        seats: &[SeatId],
        setup: &Self::Setup,
    ) -> Result<Self::State, Diagnostic> {
        setup_match(seed, seats, setup)
    }

    fn legal_action_tree(&self, state: &Self::State, _actor: &Actor) -> ActionTree {
        ActionTree::flat(state.freshness_token, Vec::new())
    }

    fn validate(
        &self,
        _state: &Self::State,
        _command: &CommandEnvelope,
    ) -> Result<Self::ValidatedAction, Diagnostic> {
        Err(Diagnostic {
            code: "rules_not_implemented".to_owned(),
            message: "legal actions are added by GAT1RACTON-005".to_owned(),
        })
    }

    fn apply(
        &self,
        _state: &mut Self::State,
        _action: Self::ValidatedAction,
        _rng: &mut dyn DeterministicRng,
    ) -> Vec<EffectEnvelope<Self::Effect>> {
        Vec::new()
    }

    fn project_view(&self, state: &Self::State, _viewer: &Viewer) -> Self::View {
        FoundationView {
            counter: state.counter,
            active_seat: state.active_seat,
            winner: state.winner,
            freshness_token: FreshnessToken(state.freshness_token.0),
        }
    }
}

pub fn load_manifest() -> Result<Manifest, String> {
    Manifest::parse(include_str!("../data/manifest.toml"))
}

pub fn load_variants() -> Result<VariantCatalog, String> {
    VariantCatalog::parse(include_str!("../data/variants.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_data_parses_and_rejects_unknown_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");

        assert_eq!(manifest.game_id, "race_to_n");
        assert_eq!(variants.selected.target, 21);
        assert!(Manifest::parse("game_id = \"race_to_n\"\nextra = \"nope\"\n").is_err());
        assert!(VariantCatalog::parse("variant_id = \"race_to_21\"\nwhen = \"never\"\n").is_err());
    }

    #[test]
    fn game_impl_wires_setup() {
        let game = RaceToN;
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let state = game
            .setup(Seed(1), &seats, &SetupOptions::default())
            .expect("setup succeeds");

        assert_eq!(state.counter, CounterValue(0));
        assert_eq!(state.active_seat, RaceSeat::Seat0);
        assert_eq!(state.seats[0], seats[0]);
        assert_eq!(state.seats[1], seats[1]);
    }
}
