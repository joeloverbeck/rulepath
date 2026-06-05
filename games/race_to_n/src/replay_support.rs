use engine_core::{
    ActionPath, ActionTree, Actor, CommandEnvelope, EffectEnvelope, HashValue, RulesVersion,
    SeatId, Seed, StableSerialize,
};

use crate::{
    apply_action, legal_action_tree, project_view, setup_match, validate_command, RaceEffect,
    RaceRandomBot, RaceSeat, RaceSnapshot, RaceState, SetupOptions,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayHashes {
    pub state_hash: HashValue,
    pub effect_hash: HashValue,
    pub action_tree_hash: HashValue,
    pub view_hash: HashValue,
    pub diagnostic_hash: Option<HashValue>,
    pub outcome: Option<RaceSeat>,
}

pub fn default_seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

pub fn actor_for_state(state: &RaceState) -> Actor {
    Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    }
}

pub fn command_for_state(state: &RaceState, segment: String) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_state(state),
        action_path: ActionPath {
            segments: vec![segment],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

pub fn replay_commands(seed: u64, commands: &[String]) -> ReplayHashes {
    let mut state = setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).unwrap();
    let mut effects = Vec::new();

    for segment in commands {
        let command = command_for_state(&state, segment.clone());
        let action = validate_command(&state, &command).expect("trace command validates");
        effects.extend(apply_action(&mut state, action));
    }

    hashes_for_state(&state, &effects, None)
}

pub fn replay_bot_action(seed: u64, bot_seed: u64) -> ReplayHashes {
    let mut state = setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).unwrap();
    let bot = RaceRandomBot::new(Seed(bot_seed));
    let action_path = bot
        .select_action(&state, state.active_seat)
        .expect("bot action selected");
    let command = CommandEnvelope {
        actor: actor_for_state(&state),
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    let action = validate_command(&state, &command).expect("bot action validates");
    let effects = apply_action(&mut state, action);

    hashes_for_state(&state, &effects, None)
}

pub fn replay_invalid(seed: u64, invalid: &str, stale: &str) -> ReplayHashes {
    let state = setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).unwrap();
    let invalid_diagnostic =
        validate_command(&state, &command_for_state(&state, invalid.to_owned()))
            .expect_err("invalid command rejected");
    let stale_command = CommandEnvelope {
        actor: actor_for_state(&state),
        action_path: ActionPath {
            segments: vec![stale.to_owned()],
        },
        freshness_token: state.freshness_token.next(),
        rules_version: RulesVersion(1),
    };
    let stale_diagnostic =
        validate_command(&state, &stale_command).expect_err("stale command rejected");
    let diagnostic_hash = HashValue::from_stable_bytes(
        format!(
            "{}:{}|{}:{}",
            invalid_diagnostic.code,
            invalid_diagnostic.message,
            stale_diagnostic.code,
            stale_diagnostic.message
        )
        .as_bytes(),
    );

    hashes_for_state(&state, &[], Some(diagnostic_hash))
}

pub fn hashes_for_state(
    state: &RaceState,
    effects: &[EffectEnvelope<RaceEffect>],
    diagnostic_hash: Option<HashValue>,
) -> ReplayHashes {
    let actor = actor_for_state(state);
    ReplayHashes {
        state_hash: RaceSnapshot::from_state(state).stable_hash(),
        effect_hash: effect_hash(effects),
        action_tree_hash: action_tree_hash(&legal_action_tree(state, &actor)),
        view_hash: project_view(state).stable_hash(),
        diagnostic_hash,
        outcome: state.winner,
    }
}

pub fn effect_hash(effects: &[EffectEnvelope<RaceEffect>]) -> HashValue {
    let bytes = effects
        .iter()
        .map(effect_stable_string)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

pub fn effect_stable_string(effect: &EffectEnvelope<RaceEffect>) -> String {
    match &effect.payload {
        RaceEffect::ActionStarted { actor, amount } => {
            format!("ActionStarted:{}:{amount}", actor.as_str())
        }
        RaceEffect::CounterAdvanced {
            actor,
            from,
            to,
            amount,
        } => format!(
            "CounterAdvanced:{}:{}:{}:{amount}",
            actor.as_str(),
            from.0,
            to.0
        ),
        RaceEffect::TurnChanged { next_actor } => {
            format!("TurnChanged:{}", next_actor.as_str())
        }
        RaceEffect::GameEnded { winner } => format!("GameEnded:{}", winner.as_str()),
        RaceEffect::ActionCompleted { actor } => format!("ActionCompleted:{}", actor.as_str()),
    }
}

pub fn action_tree_hash(tree: &ActionTree) -> HashValue {
    let bytes = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(bytes.as_bytes())
}
