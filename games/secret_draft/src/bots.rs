use ai_core::RandomLegalBot;
use engine_core::{ActionPath, Actor, Diagnostic, Seed};

use crate::{
    actions::{commit_segment, legal_action_tree, parse_commit_segment, SecretDraftAction},
    ids::{DraftItemId, DraftThread, SecretDraftSeat},
    state::SecretDraftState,
};

pub const RANDOM_POLICY_ID: &str = "secret_draft_random_legal_v0";
pub const LEVEL1_POLICY_ID: &str = "secret_draft_level1_v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecision {
    pub policy_id: String,
    pub policy_version: u32,
    pub level: u8,
    pub action_path: ActionPath,
    pub rationale: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecretDraftRandomBot {
    pub seed: Seed,
}

impl SecretDraftRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(
        &self,
        state: &SecretDraftState,
        bot_seat: SecretDraftSeat,
    ) -> Result<ActionPath, Diagnostic> {
        let tree = legal_action_tree(state, &actor_for_seat(state, bot_seat));
        RandomLegalBot::new(self.seed).select_action(&tree)
    }

    pub fn select_decision(
        &self,
        state: &SecretDraftState,
        bot_seat: SecretDraftSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        Ok(BotDecision {
            policy_id: RANDOM_POLICY_ID.to_owned(),
            policy_version: 1,
            level: 0,
            action_path,
            rationale: "Selected a seeded random legal Veiled Draft commitment.".to_owned(),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecretDraftLevel1Bot {
    pub seed: Seed,
}

impl SecretDraftLevel1Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(
        &self,
        state: &SecretDraftState,
        bot_seat: SecretDraftSeat,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }

    pub fn select_decision(
        &self,
        state: &SecretDraftState,
        bot_seat: SecretDraftSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let legal = legal_items_for_seat(state, bot_seat);
        if legal.is_empty() {
            return Err(Diagnostic {
                code: "no_legal_actions".to_owned(),
                message: "no legal action is available".to_owned(),
            });
        }

        let item = choose_level1_item(state, bot_seat, &legal, self.seed);
        Ok(BotDecision {
            policy_id: LEVEL1_POLICY_ID.to_owned(),
            policy_version: 1,
            level: 1,
            action_path: ActionPath {
                segments: vec![commit_segment(item)],
            },
            rationale: level1_rationale(state, bot_seat, item),
        })
    }
}

pub fn actor_for_seat(state: &SecretDraftState, seat: SecretDraftSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

pub fn action_from_decision(
    decision: &BotDecision,
    bot_seat: SecretDraftSeat,
) -> Option<SecretDraftAction> {
    let [segment] = decision.action_path.segments.as_slice() else {
        return None;
    };
    Some(SecretDraftAction {
        actor: bot_seat,
        item: parse_commit_segment(segment)?,
    })
}

fn legal_items_for_seat(state: &SecretDraftState, bot_seat: SecretDraftSeat) -> Vec<DraftItemId> {
    let tree = legal_action_tree(state, &actor_for_seat(state, bot_seat));
    tree.root
        .choices
        .iter()
        .filter_map(|choice| parse_commit_segment(&choice.segment))
        .collect()
}

fn choose_level1_item(
    state: &SecretDraftState,
    bot_seat: SecretDraftSeat,
    legal: &[DraftItemId],
    seed: Seed,
) -> DraftItemId {
    legal
        .iter()
        .copied()
        .max_by_key(|item| level1_rank(state, bot_seat, *item, seed))
        .expect("legal item list is non-empty")
}

fn level1_rank(
    state: &SecretDraftState,
    bot_seat: SecretDraftSeat,
    item: DraftItemId,
    seed: Seed,
) -> (u8, u8, u8, u8, std::cmp::Reverse<&'static str>, u64) {
    (
        completes_thread_set(state.drafted_for(bot_seat), item),
        item.value(),
        adds_high_thread_bonus(state.drafted_for(bot_seat), item),
        public_fallback_safety(state, bot_seat, item),
        std::cmp::Reverse(item.as_str()),
        seeded_tie(seed, item),
    )
}

fn completes_thread_set(drafted: &[DraftItemId], item: DraftItemId) -> u8 {
    let before = complete_set_count(drafted);
    let mut after = drafted.to_vec();
    after.push(item);
    u8::from(complete_set_count(&after) > before)
}

fn complete_set_count(items: &[DraftItemId]) -> u8 {
    thread_count(items, DraftThread::Ember)
        .min(thread_count(items, DraftThread::Tide))
        .min(thread_count(items, DraftThread::Grove))
}

fn adds_high_thread_bonus(drafted: &[DraftItemId], item: DraftItemId) -> u8 {
    let before = thread_count(drafted, item.thread());
    u8::from(before < 3 && before.saturating_add(1) >= 3)
}

fn thread_count(items: &[DraftItemId], thread: DraftThread) -> u8 {
    items.iter().filter(|item| item.thread() == thread).count() as u8
}

fn public_fallback_safety(
    state: &SecretDraftState,
    bot_seat: SecretDraftSeat,
    item: DraftItemId,
) -> u8 {
    if state.priority_seat == bot_seat {
        return 4;
    }

    state
        .visible_pool
        .iter()
        .copied()
        .find(|visible| *visible != item)
        .map(DraftItemId::value)
        .unwrap_or(0)
}

fn seeded_tie(seed: Seed, item: DraftItemId) -> u64 {
    let mut value = seed.0 ^ item.value() as u64;
    for byte in item.as_str().bytes() {
        value ^= u64::from(byte);
        value = value.wrapping_mul(0x100_0000_01b3);
    }
    value
}

fn level1_rationale(
    state: &SecretDraftState,
    bot_seat: SecretDraftSeat,
    item: DraftItemId,
) -> String {
    let set_note = if completes_thread_set(state.drafted_for(bot_seat), item) == 1 {
        " completes a public thread set"
    } else {
        " does not complete a public thread set"
    };
    let bonus_note = if adds_high_thread_bonus(state.drafted_for(bot_seat), item) == 1 {
        " and adds a public high-thread bonus"
    } else {
        ""
    };
    let priority_note = if state.priority_seat == bot_seat {
        " with public priority for conflicts"
    } else {
        " with public fallback exposure considered"
    };

    format!(
        "Ranked visible legal commitments by public set completion, public value {}, public thread bonuses, public fallback safety, and deterministic tie-breaks; selected a commitment that{set_note}{bonus_note}{priority_note}.",
        item.value()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{setup::SetupOptions, setup_match};
    use engine_core::SeatId;

    fn standard_state() -> SecretDraftState {
        setup_match(
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    #[test]
    fn level1_prefers_completing_a_thread_set() {
        let mut state = standard_state();
        state.drafted[SecretDraftSeat::Seat0.index()] =
            vec![DraftItemId::Ember1, DraftItemId::Tide1];

        let decision = SecretDraftLevel1Bot::new(Seed(9))
            .select_decision(&state, SecretDraftSeat::Seat0)
            .expect("decision selected");

        assert_eq!(
            action_from_decision(&decision, SecretDraftSeat::Seat0),
            Some(SecretDraftAction {
                actor: SecretDraftSeat::Seat0,
                item: DraftItemId::Grove4,
            })
        );
    }

    #[test]
    fn committed_seat_has_no_decision() {
        let mut state = standard_state();
        state.set_commitment_for_internal(SecretDraftSeat::Seat0, DraftItemId::Ember1);

        let diagnostic = SecretDraftLevel1Bot::new(Seed(1))
            .select_decision(&state, SecretDraftSeat::Seat0)
            .expect_err("committed seat rejected");

        assert_eq!(diagnostic.code, "no_legal_actions");
    }
}
