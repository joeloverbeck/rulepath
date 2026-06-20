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
    pub own_remaining_stack: u16,
    pub own_is_all_in: bool,
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
            "seat={};choices={};phase={};active={};street={};pot={};own_stack={};own_all_in={};own_contribution={};call_price={};raises_remaining={};live_opponents={};board_count={};own_bucket={}",
            self.bot_seat.as_str(),
            self.legal_action_tree.root.choices.len(),
            self.phase,
            self.active_seat
                .map(RiverLedgerSeat::as_str)
                .unwrap_or_else(|| "none".to_owned()),
            self.street.as_str(),
            self.pot_total,
            self.own_remaining_stack,
            self.own_is_all_in,
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
    pub public_explanation: Option<BotDecisionPublicExplanation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecisionPublicExplanation {
    pub seat: RiverLedgerSeat,
    pub seat_label: String,
    pub action_label: String,
    pub short_reason: String,
    pub public_facts: Vec<BotDecisionPublicFact>,
    pub hidden_information_notice: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecisionPublicFact {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BotActionCandidate {
    action: RiverLedgerAction,
    path: ActionPath,
    amount_owed: u16,
    adds_to_pot: u16,
    stack_before: u16,
    stack_after: u16,
    is_all_in: bool,
    is_full_raise: bool,
    raise_right_open: bool,
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
            None,
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
        let legal = legal_action_candidates_from_input(&input);
        let action = choose_level1_action(&input, &legal)?;
        Ok(decision(
            1,
            LEVEL1_POLICY_ID,
            action.path.clone(),
            format!(
                "Conservative public price posture; chose {} from legal River Ledger actions.",
                action_choice_label(action)
            ),
            Some(public_explanation(&input, action)),
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
        let legal = legal_action_candidates_from_input(&input);
        if legal.is_empty() {
            return Err(no_legal_actions());
        }
        let action = legal
            .iter()
            .max_by_key(|action| level2_rank(&input, action, self.seed))
            .expect("non-empty legal action list");
        Ok(decision(
            2,
            LEVEL2_POLICY_ID,
            action.path.clone(),
            level2_rationale(&input, action),
            Some(public_explanation(&input, action)),
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
        own_remaining_stack: own.remaining_stack,
        own_is_all_in: own.is_all_in,
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

fn legal_action_candidates_from_input(input: &RiverLedgerBotInput) -> Vec<BotActionCandidate> {
    input
        .legal_action_tree
        .root
        .choices
        .iter()
        .filter_map(|choice| {
            let action = parse_action_segment(&choice.segment)?;
            Some(BotActionCandidate {
                action,
                path: choice.path(),
                amount_owed: metadata_u16(choice, "amount_owed").unwrap_or(input.call_price),
                adds_to_pot: metadata_u16(choice, "adds_to_pot").unwrap_or(0),
                stack_before: metadata_u16(choice, "stack_before")
                    .unwrap_or(input.own_remaining_stack),
                stack_after: metadata_u16(choice, "stack_after")
                    .unwrap_or(input.own_remaining_stack),
                is_all_in: metadata_bool(choice, "is_all_in").unwrap_or(false),
                is_full_raise: metadata_bool(choice, "is_full_raise").unwrap_or(false),
                raise_right_open: metadata_bool(choice, "raise_right_open").unwrap_or(false),
            })
        })
        .collect()
}

fn metadata_u16(choice: &engine_core::ActionChoice, key: &str) -> Option<u16> {
    choice
        .metadata
        .iter()
        .find(|entry| entry.key == key)
        .and_then(|entry| entry.value.parse().ok())
}

fn metadata_bool(choice: &engine_core::ActionChoice, key: &str) -> Option<bool> {
    choice
        .metadata
        .iter()
        .find(|entry| entry.key == key)
        .and_then(|entry| entry.value.parse().ok())
}

fn choose_level1_action<'a>(
    input: &RiverLedgerBotInput,
    legal: &'a [BotActionCandidate],
) -> Result<&'a BotActionCandidate, Diagnostic> {
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
        if let Some(candidate) = legal.iter().find(|candidate| candidate.action == preferred) {
            if preferred != RiverLedgerAction::Call || input.call_price <= 2 || candidate.is_all_in
            {
                return Ok(candidate);
            }
        }
    }
    Ok(&legal[0])
}

fn level2_rank(
    input: &RiverLedgerBotInput,
    candidate: &BotActionCandidate,
    seed: Seed,
) -> (
    u8,
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
    let action = candidate.action;
    let strong = matches!(
        input.own_hole_bucket.as_str(),
        "pair" | "high_connected" | "high"
    );
    let weak = input.own_hole_bucket == "weak";
    let free = input.call_price == 0;
    let expensive = input.call_price >= input.street.unit() as u16 && input.street.unit() >= 4;
    let few_opponents = input.live_opponent_count <= 2;
    let can_pressure = strong && few_opponents && input.raises_remaining > 0;
    let short_all_in_raise =
        action == RiverLedgerAction::Raise && candidate.is_all_in && !candidate.is_full_raise;

    (
        u8::from(action != RiverLedgerAction::Fold || (!free && (expensive || weak))),
        u8::from(
            (free && action == RiverLedgerAction::Check)
                || (!free && action == RiverLedgerAction::Call)
                || (!free && can_pressure && action == RiverLedgerAction::Raise)
                || (!free && expensive && weak && action == RiverLedgerAction::Fold),
        ),
        u8::from(
            strong
                && matches!(
                    action,
                    RiverLedgerAction::Bet | RiverLedgerAction::Raise | RiverLedgerAction::Call
                ),
        ),
        u8::from(candidate.is_all_in && action != RiverLedgerAction::Fold && !weak),
        u8::from(input.board_count > 0 && strong && action != RiverLedgerAction::Fold),
        u8::from(
            (few_opponents && action != RiverLedgerAction::Fold)
                || (!few_opponents && action != RiverLedgerAction::Raise),
        ),
        u8::from(!expensive || strong || action == RiverLedgerAction::Fold),
        u8::from(
            can_pressure
                && matches!(action, RiverLedgerAction::Bet | RiverLedgerAction::Raise)
                && (candidate.is_full_raise || short_all_in_raise || candidate.raise_right_open),
        ),
        std::cmp::Reverse(action.segment()),
        seeded_tie(seed, candidate),
    )
}

fn level2_rationale(input: &RiverLedgerBotInput, action: &BotActionCandidate) -> String {
    format!(
        "own authorized {} bucket; public price {}; own public stack {}; live opponent count {}; street/cap pressure {}; chose {}.",
        input.own_hole_bucket,
        input.call_price,
        input.own_remaining_stack,
        input.live_opponent_count,
        input.raises_remaining,
        action_choice_label(action)
    )
}

fn seeded_tie(seed: Seed, action: &BotActionCandidate) -> u64 {
    let mut value = seed.0;
    for byte in action.action.segment().bytes() {
        value ^= u64::from(byte);
        value = value.wrapping_mul(0x100_0000_01b3);
    }
    value ^= u64::from(action.adds_to_pot);
    value = value.wrapping_mul(0x100_0000_01b3);
    value ^= u64::from(action.stack_after);
    value
}

fn decision(
    level: u8,
    policy_id: &str,
    action_path: ActionPath,
    rationale: String,
    public_explanation: Option<BotDecisionPublicExplanation>,
) -> BotDecision {
    BotDecision {
        policy_id: policy_id.to_owned(),
        policy_version: 1,
        level,
        action_path,
        rationale,
        public_explanation,
    }
}

fn public_explanation(
    input: &RiverLedgerBotInput,
    action: &BotActionCandidate,
) -> BotDecisionPublicExplanation {
    BotDecisionPublicExplanation {
        seat: input.bot_seat,
        seat_label: seat_public_label(input.bot_seat),
        action_label: action_label(action),
        short_reason: public_reason(input, action),
        public_facts: vec![
            fact("Street", street_label(input.street)),
            fact("Call price", input.call_price.to_string()),
            fact("Own public stack", input.own_remaining_stack.to_string()),
            fact("Amount owed", action.amount_owed.to_string()),
            fact("Adds to ledger", action.adds_to_pot.to_string()),
            fact("Raises left", input.raises_remaining.to_string()),
            fact("Live opponents", input.live_opponent_count.to_string()),
            fact("Ledger total", input.pot_total.to_string()),
        ],
        hidden_information_notice:
            "This public explanation omits private hole cards and any cards not already public."
                .to_owned(),
    }
}

fn public_reason(input: &RiverLedgerBotInput, action: &BotActionCandidate) -> String {
    match action.action {
        RiverLedgerAction::Fold => {
            "The public call price and street pressure made folding the public-safe choice."
                .to_owned()
        }
        RiverLedgerAction::Check => {
            "No contribution is required, so the bot keeps the ledger unchanged.".to_owned()
        }
        RiverLedgerAction::Call => {
            if action.is_all_in {
                return format!(
                    "The bot makes a call all-in by adding its remaining public stack of {} against the public price of {}.",
                    action.adds_to_pot, input.call_price
                );
            }
            format!(
                "The bot matches the public price of {} to stay live.",
                input.call_price
            )
        }
        RiverLedgerAction::Bet => {
            if action.is_all_in {
                return format!(
                    "No call is owed, so the bot makes a bet all-in with its remaining public stack of {}.",
                    action.adds_to_pot
                );
            }
            "No call is owed and raises remain, so the bot opens public pressure.".to_owned()
        }
        RiverLedgerAction::Raise => {
            if action.is_all_in && action.is_full_raise {
                return format!(
                    "The bot makes a full raise all-in after covering the public price of {}.",
                    action.amount_owed
                );
            }
            if action.is_all_in {
                return format!(
                    "The bot makes a short raise all-in by adding its remaining public stack of {}.",
                    action.adds_to_pot
                );
            }
            "Raises remain, so the bot adds public pressure after matching the price.".to_owned()
        }
    }
}

fn fact(label: &str, value: String) -> BotDecisionPublicFact {
    BotDecisionPublicFact {
        label: label.to_owned(),
        value,
    }
}

fn action_choice_label(action: &BotActionCandidate) -> String {
    if action.is_all_in {
        match action.action {
            RiverLedgerAction::Call => return "call all-in".to_owned(),
            RiverLedgerAction::Bet => return "bet all-in".to_owned(),
            RiverLedgerAction::Raise if action.is_full_raise => {
                return "full raise all-in".to_owned()
            }
            RiverLedgerAction::Raise => return "short raise all-in".to_owned(),
            RiverLedgerAction::Fold | RiverLedgerAction::Check => {}
        }
    }
    action.action.segment().to_owned()
}

fn action_label(action: &BotActionCandidate) -> String {
    match action.action {
        RiverLedgerAction::Fold => "Fold",
        RiverLedgerAction::Check => "Check",
        RiverLedgerAction::Call if action.is_all_in => "Call all-in",
        RiverLedgerAction::Call => "Call",
        RiverLedgerAction::Bet if action.is_all_in => "Bet all-in",
        RiverLedgerAction::Bet => "Bet",
        RiverLedgerAction::Raise if action.is_all_in && action.is_full_raise => "Full raise all-in",
        RiverLedgerAction::Raise if action.is_all_in => "Short raise all-in",
        RiverLedgerAction::Raise => "Raise",
    }
    .to_owned()
}

fn street_label(street: Street) -> String {
    match street {
        Street::Preflop => "Preflop",
        Street::Flop => "Flop",
        Street::Turn => "Turn",
        Street::River => "River",
    }
    .to_owned()
}

fn seat_public_label(seat: RiverLedgerSeat) -> String {
    format!("Seat {}", seat.index())
}

fn no_legal_actions() -> Diagnostic {
    Diagnostic {
        code: "no_legal_actions".to_owned(),
        message: "no legal River Ledger action is available".to_owned(),
    }
}
