use engine_core::{
    ActionPath, ActionTreeEncodingVersion, Actor, CommandEnvelope, EffectEnvelope, FreshnessToken,
    HashValue, RulesVersion, SeatId, Seed, StableSerialize, Viewer,
};

use crate::{
    actions::{legal_action_tree, parse_action_path, StarbridgeAction},
    effects::StarbridgeEffect,
    rules::{apply_jump_command, apply_pass_blocked_command, apply_step_command},
    setup::{setup_match, SetupOptions},
    state::{StarbridgeSnapshot, StarbridgeState},
    visibility::project_view,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayHashes {
    pub state_hash: HashValue,
    pub effect_hash: HashValue,
    pub action_tree_hash: HashValue,
    pub view_hash: HashValue,
    pub replay_hash: HashValue,
}

pub fn replay_commands(
    seed: u64,
    seat_count: usize,
    commands: &[Vec<String>],
) -> Result<ReplayHashes, String> {
    let seats = (0..seat_count)
        .map(|index| SeatId::from_zero_based_index(index as u32))
        .collect::<Vec<_>>();
    let mut state = setup_match(Seed(seed), &seats, &SetupOptions::default())
        .map_err(|diagnostic| diagnostic.message)?;
    let mut all_effects = Vec::new();

    for path in commands {
        let active_seat = state
            .seats
            .get(usize::from(state.active_seat_index))
            .ok_or_else(|| "active seat missing".to_owned())?
            .seat_id
            .clone();
        let command = CommandEnvelope {
            actor: Actor {
                seat_id: active_seat,
            },
            action_path: ActionPath {
                segments: path.clone(),
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        };
        let effects = apply_command(&mut state, &command)?;
        all_effects.extend(effects);
    }

    Ok(hashes_for_state(&state, &all_effects))
}

pub fn hashes_for_state(
    state: &StarbridgeState,
    effects: &[EffectEnvelope<StarbridgeEffect>],
) -> ReplayHashes {
    let actor = state
        .seats
        .get(usize::from(state.active_seat_index))
        .map(|seat| Actor {
            seat_id: seat.seat_id.clone(),
        })
        .unwrap_or_else(|| Actor {
            seat_id: SeatId::from_zero_based_index(0),
        });
    let action_tree = legal_action_tree(state, &actor);
    let view = project_view(state, &Viewer { seat_id: None });
    let state_hash = StarbridgeSnapshot::from_state(state).stable_hash();
    let effect_hash = effect_hash(effects);
    let action_tree_hash = action_tree.stable_hash(ActionTreeEncodingVersion::V1);
    let view_hash = view.stable_hash();
    let replay_hash = replay_hash(&[
        state_hash,
        effect_hash,
        action_tree_hash,
        view_hash,
        HashValue(state.freshness_token.0),
    ]);
    ReplayHashes {
        state_hash,
        effect_hash,
        action_tree_hash,
        view_hash,
        replay_hash,
    }
}

pub fn effect_hash(effects: &[EffectEnvelope<StarbridgeEffect>]) -> HashValue {
    HashValue::from_stable_bytes(
        effects
            .iter()
            .map(|effect| format!("{:?}", effect))
            .collect::<Vec<_>>()
            .join("|")
            .as_bytes(),
    )
}

fn apply_command(
    state: &mut StarbridgeState,
    command: &CommandEnvelope,
) -> Result<Vec<EffectEnvelope<StarbridgeEffect>>, String> {
    match parse_action_path(&command.action_path.segments)
        .map_err(|diagnostic| diagnostic.message)?
    {
        StarbridgeAction::Step { .. } => {
            apply_step_command(state, command).map_err(|diagnostic| diagnostic.message)
        }
        StarbridgeAction::Jump { .. } => {
            apply_jump_command(state, command).map_err(|diagnostic| diagnostic.message)
        }
        StarbridgeAction::PassBlocked => {
            apply_pass_blocked_command(state, command).map_err(|diagnostic| diagnostic.message)
        }
    }
}

fn replay_hash(hashes: &[HashValue]) -> HashValue {
    HashValue::from_stable_bytes(
        hashes
            .iter()
            .map(|hash| hash.0.to_string())
            .collect::<Vec<_>>()
            .join(":")
            .as_bytes(),
    )
}

#[allow(dead_code)]
fn _freshness_token_is_versioned(_: FreshnessToken) {}
