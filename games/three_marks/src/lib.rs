//! `three_marks` board-smoke crate.

pub mod actions;
pub mod bots;
pub mod effects;
pub mod ids;
pub mod replay_support;
pub mod rules;
pub mod setup;
pub mod state;
pub mod ui;
pub mod variants;
pub mod visibility;

use engine_core::{
    ActionTree, Actor, CommandEnvelope, DeterministicRng, Diagnostic, EffectEnvelope, Game, SeatId,
    Seed, Viewer,
};

pub use actions::legal_action_tree;
pub use bots::{BotDecision, ThreeMarksLevel1Bot, ThreeMarksRandomBot};
pub use effects::{RejectionReason, ThreeMarksEffect};
pub use ids::{CellId, ThreeMarksSeat};
pub use rules::{apply_action, validate_command, validate_command_with_effects, ValidatedAction};
pub use setup::{setup_match, SetupOptions};
pub use state::{CellOccupancy, TerminalOutcome, ThreeMarksSnapshot, ThreeMarksState, WinningLine};
pub use variants::{Manifest, Variant, VariantCatalog};
pub use visibility::{project_view, CellView, LegalTargetView, PublicView, TerminalView};

#[derive(Clone, Debug, Default)]
pub struct ThreeMarks;

impl Game for ThreeMarks {
    type Setup = SetupOptions;
    type State = ThreeMarksState;
    type ValidatedAction = ValidatedAction;
    type Effect = ThreeMarksEffect;
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

    fn project_view(&self, state: &Self::State, viewer: &Viewer) -> Self::View {
        project_view(state, viewer)
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
    use engine_core::{FreshnessToken, SeatId};

    #[test]
    fn static_data_parses_and_rejects_unknown_fields() {
        let manifest = load_manifest().expect("manifest parses");
        let variants = load_variants().expect("variants parse");

        assert_eq!(manifest.game_id, "three_marks");
        assert_eq!(manifest.rules_version_label, "three_marks-rules-v1");
        assert_eq!(variants.selected.id, "three_marks_standard");
        assert_eq!(variants.selected.seat_count, 2);
        assert!(Manifest::parse("game_id = \"three_marks\"\nextra = \"nope\"\n").is_err());
        assert!(
            VariantCatalog::parse("variant_id = \"three_marks_standard\"\nwhen = \"bad\"\n")
                .is_err()
        );
    }

    #[test]
    fn setup_wires_initial_state() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let state = setup_match(engine_core::Seed(1), &seats, &SetupOptions::default())
            .expect("setup succeeds");

        assert_eq!(state.active_seat, ThreeMarksSeat::Seat0);
        assert_eq!(state.seats[0], seats[0]);
        assert_eq!(state.seats[1], seats[1]);
        assert_eq!(state.ply_count, 0);
        assert_eq!(state.terminal_outcome, None);
        assert_eq!(state.freshness_token, FreshnessToken(0));
        assert!(state.cells.iter().all(|cell| cell.is_empty()));
    }
}
