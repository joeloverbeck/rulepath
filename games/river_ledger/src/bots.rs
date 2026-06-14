use ai_core::RandomLegalBot;
use engine_core::{ActionPath, ActionTree, Actor, Diagnostic, Seed, Viewer};

use crate::{
    call_price, legal_action_tree, parse_action_segment, project_view, RiverLedgerAction,
    RiverLedgerSeat, RiverLedgerState, SeatStatus, Street,
};

pub const RANDOM_POLICY_ID: &str = "river-ledger-random-legal-v0";
pub const LEVEL1_POLICY_ID: &str = "river-ledger-conservative-level1-v1";
pub const LEVEL2_POLICY_ID: &str = "river-ledger-level2-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RiverLedgerBotInput {
    pub bot_seat: RiverLedgerSeat,
    pub legal_action_tree: ActionTree,
    pub phase: String,
    pub active_seat: Option<RiverLedgerSeat>,
    pub street: Street,
    pub pot_total: u16,
    pub own_contribution: u16,
    pub call_price: u16,
    pub raises_remaining: u8,
    pub live_opponent_count: u8,
    pub board_count: u8,
    pub own_hole_bucket: String,
}

impl RiverLedgerBotInput {
    pub fn stable_summary(&self) -> String {
        format!(
            "seat={};choices={};phase={};active={};street={};pot={};own_contribution={};call_price={};raises_remaining={};live_opponents={};board_count={};own_bucket={}",
            self.bot_seat.as_str(),
            self.legal_action_tree.root.choices.len(),
            self.phase,
            self.active_seat
                .map(RiverLedgerSeat::as_str)
                .unwrap_or_else(|| "none".to_owned()),
            self.street.as_str(),
            self.pot_total,
            self.own_contribution,
            self.call_price,
            self.raises_remaining,
            self.live_opponent_count,
            self.board_count,
            self.own_hole_bucket
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecision {
    pub policy_id: String,
    pub policy_version: u32,
    pub level: u8,
    pub action_path: ActionPath,
    pub rationale: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RiverLedgerRandomBot {
    pub seed: Seed,
}

impl RiverLedgerRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &RiverLedgerState, bot_seat: RiverLedgerSeat) -> RiverLedgerBotInput {
        bot_input_for(state, bot_seat)
    }

    pub fn select_action(
        &self,
        state: &RiverLedgerState,
        bot_seat: RiverLedgerSeat,
    ) -> Result<ActionPath, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        RandomLegalBot::new(self.seed).select_action(&input.legal_action_tree)
    }

    pub fn select_decision(
        &self,
        state: &RiverLedgerState,
        bot_seat: RiverLedgerSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        Ok(decision(
            0,
            RANDOM_POLICY_ID,
            action_path,
            "Selected a seeded random legal River Ledger action.".to_owned(),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RiverLedgerLevel1Bot {
    pub seed: Seed,
}

impl RiverLedgerLevel1Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &RiverLedgerState, bot_seat: RiverLedgerSeat) -> RiverLedgerBotInput {
        bot_input_for(state, bot_seat)
    }

    pub fn select_decision(
        &self,
        state: &RiverLedgerState,
        bot_seat: RiverLedgerSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        let legal = legal_actions_from_input(&input);
        let action = choose_level1_action(&input, &legal)?;
        Ok(decision(
            1,
            LEVEL1_POLICY_ID,
            ActionPath {
                segments: vec![action.segment().to_owned()],
            },
            format!(
                "Conservative public price posture; chose {} from legal River Ledger actions.",
                action.segment()
            ),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RiverLedgerLevel2Bot {
    pub seed: Seed,
}

impl RiverLedgerLevel2Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &RiverLedgerState, bot_seat: RiverLedgerSeat) -> RiverLedgerBotInput {
        bot_input_for(state, bot_seat)
    }

    pub fn select_action(
        &self,
        state: &RiverLedgerState,
        bot_seat: RiverLedgerSeat,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }

    pub fn select_decision(
        &self,
        state: &RiverLedgerState,
        bot_seat: RiverLedgerSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        let legal = legal_actions_from_input(&input);
        if legal.is_empty() {
            return Err(no_legal_actions());
        }
        let action = legal
            .iter()
            .copied()
            .max_by_key(|action| level2_rank(&input, *action, self.seed))
            .expect("non-empty legal action list");
        Ok(decision(
            2,
            LEVEL2_POLICY_ID,
            ActionPath {
                segments: vec![action.segment().to_owned()],
            },
            level2_rationale(&input, action),
        ))
    }
}

pub fn actor_for_bot_seat(state: &RiverLedgerState, seat: RiverLedgerSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

pub fn action_from_decision(decision: &BotDecision) -> Option<RiverLedgerAction> {
    let [segment] = decision.action_path.segments.as_slice() else {
        return None;
    };
    parse_action_segment(segment)
}

fn bot_input_for(state: &RiverLedgerState, bot_seat: RiverLedgerSeat) -> RiverLedgerBotInput {
    let view = project_view(
        state,
        &Viewer {
            seat_id: Some(state.seats[bot_seat.index()].clone()),
        },
    );
    let own = view
        .seats
        .iter()
        .find(|seat| seat.seat == bot_seat)
        .expect("bot seat appears in projection");
    let legal_action_tree = legal_action_tree(state, &actor_for_bot_seat(state, bot_seat));
    let own_hole_bucket = match &view.private_view {
        crate::PrivateView::Seat(private) if private.seat == bot_seat => {
            own_hole_bucket(&private.hole_cards)
        }
        _ => "unknown".to_owned(),
    };
    let live_opponent_count = view
        .seats
        .iter()
        .filter(|seat| seat.seat != bot_seat && seat.status == SeatStatus::Live)
        .count() as u8;

    RiverLedgerBotInput {
        bot_seat,
        legal_action_tree,
        phase: format!("{:?}", view.phase),
        active_seat: view.active_seat,
        street: state.betting.street,
        pot_total: view.pot_total,
        own_contribution: own.total_contribution,
        call_price: call_price(state, bot_seat).unwrap_or(0),
        raises_remaining: crate::MAX_RAISES_PER_STREET
            .saturating_sub(state.betting.raises_this_street),
        live_opponent_count,
        board_count: view.board.len() as u8,
        own_hole_bucket,
    }
}

fn own_hole_bucket(cards: &[crate::CardView; 2]) -> String {
    let high = cards.iter().map(|card| card.rank_value).max().unwrap_or(0);
    let low = cards.iter().map(|card| card.rank_value).min().unwrap_or(0);
    if cards[0].rank_value == cards[1].rank_value {
        "pair".to_owned()
    } else if high >= 12 && high.saturating_sub(low) <= 4 {
        "high_connected".to_owned()
    } else if high >= 12 {
        "high".to_owned()
    } else if high.saturating_sub(low) <= 2 {
        "connected".to_owned()
    } else {
        "weak".to_owned()
    }
}

fn legal_actions_from_input(input: &RiverLedgerBotInput) -> Vec<RiverLedgerAction> {
    input
        .legal_action_tree
        .root
        .choices
        .iter()
        .filter_map(|choice| parse_action_segment(&choice.segment))
        .collect()
}

fn choose_level1_action(
    input: &RiverLedgerBotInput,
    legal: &[RiverLedgerAction],
) -> Result<RiverLedgerAction, Diagnostic> {
    if legal.is_empty() {
        return Err(no_legal_actions());
    }
    for preferred in [
        RiverLedgerAction::Check,
        RiverLedgerAction::Call,
        RiverLedgerAction::Fold,
        RiverLedgerAction::Bet,
        RiverLedgerAction::Raise,
    ] {
        if legal.contains(&preferred)
            && (preferred != RiverLedgerAction::Call || input.call_price <= 2)
        {
            return Ok(preferred);
        }
    }
    Ok(legal[0])
}

fn level2_rank(
    input: &RiverLedgerBotInput,
    action: RiverLedgerAction,
    seed: Seed,
) -> (
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    std::cmp::Reverse<&'static str>,
    u64,
) {
    let strong = matches!(
        input.own_hole_bucket.as_str(),
        "pair" | "high_connected" | "high"
    );
    let weak = input.own_hole_bucket == "weak";
    let free = input.call_price == 0;
    let expensive = input.call_price >= input.street.unit() as u16 && input.street.unit() >= 4;
    let few_opponents = input.live_opponent_count <= 2;
    let can_pressure = strong && few_opponents && input.raises_remaining > 0;

    (
        u8::from(action != RiverLedgerAction::Fold || (!free && (expensive || weak))),
        u8::from(
            (free && action == RiverLedgerAction::Check)
                || (!free && action == RiverLedgerAction::Call)
                || (!free && expensive && weak && action == RiverLedgerAction::Fold),
        ),
        u8::from(
            strong
                && matches!(
                    action,
                    RiverLedgerAction::Bet | RiverLedgerAction::Raise | RiverLedgerAction::Call
                ),
        ),
        u8::from(input.board_count > 0 && strong && action != RiverLedgerAction::Fold),
        u8::from(
            (few_opponents && action != RiverLedgerAction::Fold)
                || (!few_opponents && action != RiverLedgerAction::Raise),
        ),
        u8::from(!expensive || strong || action == RiverLedgerAction::Fold),
        u8::from(
            can_pressure && matches!(action, RiverLedgerAction::Bet | RiverLedgerAction::Raise),
        ),
        std::cmp::Reverse(action.segment()),
        seeded_tie(seed, action),
    )
}

fn level2_rationale(input: &RiverLedgerBotInput, action: RiverLedgerAction) -> String {
    format!(
        "own authorized {} bucket; public price {}; live opponent count {}; street/cap pressure {}; chose {}.",
        input.own_hole_bucket,
        input.call_price,
        input.live_opponent_count,
        input.raises_remaining,
        action.segment()
    )
}

fn seeded_tie(seed: Seed, action: RiverLedgerAction) -> u64 {
    let mut value = seed.0;
    for byte in action.segment().bytes() {
        value ^= u64::from(byte);
        value = value.wrapping_mul(0x100_0000_01b3);
    }
    value
}

fn decision(level: u8, policy_id: &str, action_path: ActionPath, rationale: String) -> BotDecision {
    BotDecision {
        policy_id: policy_id.to_owned(),
        policy_version: 1,
        level,
        action_path,
        rationale,
    }
}

fn no_legal_actions() -> Diagnostic {
    Diagnostic {
        code: "no_legal_actions".to_owned(),
        message: "no legal River Ledger action is available".to_owned(),
    }
}
