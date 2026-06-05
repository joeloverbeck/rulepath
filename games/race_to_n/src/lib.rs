//! `race_to_n` foundation crate.

pub mod actions;
pub mod bots;
pub mod effects;
pub mod ids;
pub mod replay_support;
pub mod rules;
pub mod setup;
pub mod state;
pub mod variants;
pub mod visibility;

use engine_core::{
    ActionTree, Actor, CommandEnvelope, DeterministicRng, Diagnostic, EffectEnvelope, Game, SeatId,
    Seed, Viewer,
};

pub use actions::legal_action_tree;
pub use bots::RaceRandomBot;
pub use effects::RaceEffect;
pub use ids::RaceSeat;
pub use rules::{apply_action, validate_command, ValidatedAction};
pub use setup::{setup_match, SetupOptions};
pub use state::{CounterValue, RaceReplayJson, RaceSnapshot, RaceState};
pub use variants::{Manifest, Variant, VariantCatalog};
pub use visibility::{project_view, PublicView};

#[derive(Clone, Debug, Default)]
pub struct RaceToN;

impl Game for RaceToN {
    type Setup = SetupOptions;
    type State = RaceState;
    type ValidatedAction = ValidatedAction;
    type Effect = RaceEffect;
    type View = PublicView;

    fn setup(
        &self,
        seed: Seed,
        seats: &[SeatId],
        setup: &Self::Setup,
    ) -> Result<Self::State, Diagnostic> {
        setup_match(seed, seats, setup)
    }

    fn legal_action_tree(&self, state: &Self::State, actor: &Actor) -> ActionTree {
        legal_action_tree(state, actor)
    }

    fn validate(
        &self,
        state: &Self::State,
        command: &CommandEnvelope,
    ) -> Result<Self::ValidatedAction, Diagnostic> {
        validate_command(state, command)
    }

    fn apply(
        &self,
        state: &mut Self::State,
        action: Self::ValidatedAction,
        _rng: &mut dyn DeterministicRng,
    ) -> Vec<EffectEnvelope<Self::Effect>> {
        apply_action(state, action)
    }

    fn project_view(&self, state: &Self::State, _viewer: &Viewer) -> Self::View {
        project_view(state)
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
