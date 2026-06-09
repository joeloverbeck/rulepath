use ai_core::RandomLegalBot;
use engine_core::{ActionPath, ActionTree, Actor, Diagnostic, EffectEnvelope, Seed, Viewer};

use crate::{
    bot_chose_action_private_effect, bot_chose_action_public_effect, legal_action_tree,
    parse_action_segment, project_view, CrestRank, Phase, PokerLiteAction, PokerLiteEffect,
    PokerLiteSeat, PokerLiteState, PrivateView,
};

pub const RANDOM_POLICY_ID: &str = "poker-lite-random-legal-v0";
pub const LEVEL2_POLICY_ID: &str = "poker-lite-crest-ledger-level2-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PokerLiteBotInput {
    pub bot_seat: PokerLiteSeat,
    pub legal_action_tree: ActionTree,
    pub phase: Phase,
    pub active_seat: Option<PokerLiteSeat>,
    pub shared_pool: u8,
    pub own_contribution: u8,
    pub other_contribution: u8,
    pub round_index: u8,
    pub round_unit: u8,
    pub outstanding_amount: u8,
    pub lift_cap_remaining: u8,
    pub center_visible: bool,
    pub public_center_rank: Option<CrestRank>,
    pub own_private_rank: CrestRank,
    pub own_strength_bucket: String,
}

impl PokerLiteBotInput {
    pub fn stable_summary(&self) -> String {
        format!(
            "seat={};choices={};phase={};active={};pool={};own_contribution={};other_contribution={};round={}:unit{}:amount{}:lift{};center_visible={};center_rank={};own_rank={};own_bucket={}",
            self.bot_seat.as_str(),
            self.legal_action_tree.root.choices.len(),
            self.phase.as_str(),
            self.active_seat.map(PokerLiteSeat::as_str).unwrap_or("none"),
            self.shared_pool,
            self.own_contribution,
            self.other_contribution,
            self.round_index,
            self.round_unit,
            self.outstanding_amount,
            self.lift_cap_remaining,
            self.center_visible,
            self.public_center_rank
                .map(CrestRank::as_str)
                .unwrap_or("hidden"),
            self.own_private_rank.as_str(),
            self.own_strength_bucket
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
    pub effects: Vec<EffectEnvelope<PokerLiteEffect>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PokerLiteRandomBot {
    pub seed: Seed,
}

impl PokerLiteRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &PokerLiteState, bot_seat: PokerLiteSeat) -> PokerLiteBotInput {
        bot_input_for(state, bot_seat)
    }

    pub fn select_action(
        &self,
        state: &PokerLiteState,
        bot_seat: PokerLiteSeat,
    ) -> Result<ActionPath, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        RandomLegalBot::new(self.seed).select_action(&input.legal_action_tree)
    }

    pub fn select_decision(
        &self,
        state: &PokerLiteState,
        bot_seat: PokerLiteSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        Ok(decision(
            0,
            RANDOM_POLICY_ID,
            state,
            bot_seat,
            action_path,
            "Selected a seeded random legal Crest Ledger action.".to_owned(),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PokerLiteLevel2Bot {
    pub seed: Seed,
}

impl PokerLiteLevel2Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &PokerLiteState, bot_seat: PokerLiteSeat) -> PokerLiteBotInput {
        bot_input_for(state, bot_seat)
    }

    pub fn select_action(
        &self,
        state: &PokerLiteState,
        bot_seat: PokerLiteSeat,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }

    pub fn select_decision(
        &self,
        state: &PokerLiteState,
        bot_seat: PokerLiteSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        let legal = legal_actions_from_input(&input);
        if legal.is_empty() {
            return Err(Diagnostic {
                code: "no_legal_actions".to_owned(),
                message: "no legal action is available".to_owned(),
            });
        }

        let action = choose_level2_action(&input, &legal, self.seed);
        Ok(decision(
            2,
            LEVEL2_POLICY_ID,
            state,
            bot_seat,
            ActionPath {
                segments: vec![action.segment().to_owned()],
            },
            level2_rationale(&input, action),
        ))
    }
}

pub fn actor_for_seat(state: &PokerLiteState, seat: PokerLiteSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

pub fn action_from_decision(decision: &BotDecision) -> Option<PokerLiteAction> {
    let [segment] = decision.action_path.segments.as_slice() else {
        return None;
    };
    parse_action_segment(segment)
}

fn bot_input_for(state: &PokerLiteState, bot_seat: PokerLiteSeat) -> PokerLiteBotInput {
    let viewer = Viewer {
        seat_id: Some(state.seats[bot_seat.index()].clone()),
    };
    let view = project_view(state, &viewer);
    let own_private = state.private_card_for_internal(bot_seat);
    let own_strength_bucket = match &view.private_view {
        PrivateView::Seat(private) if private.seat == bot_seat => private
            .own_strength_bucket
            .clone()
            .unwrap_or_else(|| "unknown_private".to_owned()),
        _ => "unknown_private".to_owned(),
    };
    let public_center_rank = if state.center_visible {
        Some(state.center_card_internal().rank())
    } else {
        None
    };

    PokerLiteBotInput {
        bot_seat,
        legal_action_tree: legal_action_tree(state, &actor_for_seat(state, bot_seat)),
        phase: view.phase,
        active_seat: view.active_seat,
        shared_pool: view.shared_pool,
        own_contribution: view.contributions[bot_seat.index()],
        other_contribution: view.contributions[bot_seat.other().index()],
        round_index: view.round.round_index,
        round_unit: view.round.round_unit,
        outstanding_amount: view.round.outstanding_amount,
        lift_cap_remaining: view.round.lift_cap_remaining,
        center_visible: state.center_visible,
        public_center_rank,
        own_private_rank: own_private.rank(),
        own_strength_bucket,
    }
}

fn legal_actions_from_input(input: &PokerLiteBotInput) -> Vec<PokerLiteAction> {
    input
        .legal_action_tree
        .root
        .choices
        .iter()
        .filter_map(|choice| parse_action_segment(&choice.segment))
        .collect()
}

fn choose_level2_action(
    input: &PokerLiteBotInput,
    legal: &[PokerLiteAction],
    seed: Seed,
) -> PokerLiteAction {
    legal
        .iter()
        .copied()
        .max_by_key(|action| level2_rank(input, *action, seed))
        .expect("legal action list is non-empty")
}

fn level2_rank(
    input: &PokerLiteBotInput,
    action: PokerLiteAction,
    seed: Seed,
) -> (u8, u8, u8, u8, u8, u8, std::cmp::Reverse<&'static str>, u64) {
    let made_pair = made_public_pair(input);
    let high_private = input.own_private_rank == CrestRank::High;
    let facing_price = input.outstanding_amount > 0;
    let affordable = input.outstanding_amount <= input.round_unit;
    let protects_pair =
        u8::from(made_pair && matches!(action, PokerLiteAction::Lift | PokerLiteAction::Match));
    let avoids_reckless_lift = u8::from(action != PokerLiteAction::Lift || made_pair);
    let respects_price = u8::from(!facing_price || affordable || action == PokerLiteAction::Yield);
    let closes_uncertain = u8::from(matches!(
        action,
        PokerLiteAction::Hold | PokerLiteAction::Match
    ));
    let high_rank_pressure =
        u8::from(!input.center_visible && high_private && action == PokerLiteAction::Press);
    let survival = u8::from(action != PokerLiteAction::Yield || (!made_pair && !affordable));

    (
        survival,
        protects_pair,
        respects_price,
        high_rank_pressure,
        avoids_reckless_lift,
        closes_uncertain,
        std::cmp::Reverse(action.segment()),
        seeded_tie(seed, action),
    )
}

fn made_public_pair(input: &PokerLiteBotInput) -> bool {
    input
        .public_center_rank
        .is_some_and(|center| center == input.own_private_rank)
}

fn seeded_tie(seed: Seed, action: PokerLiteAction) -> u64 {
    let mut value = seed.0;
    for byte in action.segment().bytes() {
        value ^= u64::from(byte);
        value = value.wrapping_mul(0x100_0000_01b3);
    }
    value
}

fn level2_rationale(input: &PokerLiteBotInput, action: PokerLiteAction) -> String {
    let posture = if made_public_pair(input) {
        "made public pair"
    } else if input.center_visible {
        "revealed center without pair"
    } else if input.own_private_rank == CrestRank::High {
        "high private rank before center reveal"
    } else {
        "bounded public price"
    };
    format!(
        "{posture}; chose {} from legal public pledge options.",
        action.segment()
    )
}

fn decision(
    level: u8,
    policy_id: &str,
    state: &PokerLiteState,
    bot_seat: PokerLiteSeat,
    action_path: ActionPath,
    rationale: String,
) -> BotDecision {
    let action_family = action_path
        .segments
        .first()
        .cloned()
        .unwrap_or_else(|| "none".to_owned());
    let private_bucket = bot_input_for(state, bot_seat).own_strength_bucket;
    BotDecision {
        policy_id: policy_id.to_owned(),
        policy_version: 1,
        level,
        action_path,
        rationale,
        effects: vec![
            bot_chose_action_public_effect(policy_id, action_family.clone()),
            bot_chose_action_private_effect(
                bot_seat,
                state.seats[bot_seat.index()].clone(),
                policy_id,
                action_family,
                private_bucket,
            ),
        ],
    }
}
