use crate::{
    ActionTree, Actor, CommandEnvelope, DeterministicRng, Diagnostic, EffectEnvelope, SeatId, Seed,
    Viewer,
};

pub trait Game {
    type Setup;
    type State;
    type ValidatedAction;
    type Effect;
    type View;

    fn setup(
        &self,
        seed: Seed,
        seats: &[SeatId],
        setup: &Self::Setup,
    ) -> Result<Self::State, Diagnostic>;

    fn legal_action_tree(&self, state: &Self::State, actor: &Actor) -> ActionTree;

    fn validate(
        &self,
        state: &Self::State,
        command: &CommandEnvelope,
    ) -> Result<Self::ValidatedAction, Diagnostic>;

    fn apply(
        &self,
        state: &mut Self::State,
        action: Self::ValidatedAction,
        rng: &mut dyn DeterministicRng,
    ) -> Vec<EffectEnvelope<Self::Effect>>;

    fn project_view(&self, state: &Self::State, viewer: &Viewer) -> Self::View;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ActionChoice, ActionPath, FreshnessToken, RulesVersion, SeededRng, VisibilityScope,
    };

    struct DemoGame;

    impl Game for DemoGame {
        type Setup = u8;
        type State = u8;
        type ValidatedAction = u8;
        type Effect = u8;
        type View = u8;

        fn setup(
            &self,
            _seed: Seed,
            _seats: &[SeatId],
            setup: &Self::Setup,
        ) -> Result<Self::State, Diagnostic> {
            Ok(*setup)
        }

        fn legal_action_tree(&self, _state: &Self::State, _actor: &Actor) -> ActionTree {
            ActionTree::flat(
                FreshnessToken(0),
                vec![ActionChoice::leaf("step", "Step", "Step")],
            )
        }

        fn validate(
            &self,
            _state: &Self::State,
            command: &CommandEnvelope,
        ) -> Result<Self::ValidatedAction, Diagnostic> {
            if command.action_path.segments == ["step"] {
                Ok(1)
            } else {
                Err(Diagnostic {
                    code: "invalid_action".to_owned(),
                    message: "action path is unavailable".to_owned(),
                })
            }
        }

        fn apply(
            &self,
            state: &mut Self::State,
            action: Self::ValidatedAction,
            _rng: &mut dyn DeterministicRng,
        ) -> Vec<EffectEnvelope<Self::Effect>> {
            *state += action;
            vec![EffectEnvelope {
                visibility: VisibilityScope::Public,
                payload: *state,
            }]
        }

        fn project_view(&self, state: &Self::State, _viewer: &Viewer) -> Self::View {
            *state
        }
    }

    #[test]
    fn game_trait_drives_opaque_payloads() {
        let game: &dyn Game<Setup = u8, State = u8, ValidatedAction = u8, Effect = u8, View = u8> =
            &DemoGame;
        let actor = Actor {
            seat_id: SeatId("seat-a".to_owned()),
        };
        let mut state = game
            .setup(Seed(0), std::slice::from_ref(&actor.seat_id), &4)
            .expect("setup succeeds");

        let tree = game.legal_action_tree(&state, &actor);
        assert_eq!(tree.root.choices[0].segment, "step");

        let command = CommandEnvelope {
            actor,
            action_path: ActionPath {
                segments: vec!["step".to_owned()],
            },
            freshness_token: FreshnessToken(0),
            rules_version: RulesVersion(1),
        };
        let action = game.validate(&state, &command).expect("action validates");
        let mut rng = SeededRng::from_seed(Seed(10));
        let effects = game.apply(&mut state, action, &mut rng);

        assert_eq!(effects[0].payload, 5);
        assert_eq!(game.project_view(&state, &Viewer { seat_id: None }), 5);
    }
}
