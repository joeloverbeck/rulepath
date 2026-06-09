use ai_core::RandomLegalBot;
use engine_core::{ActionPath, ActionTree, Actor, Diagnostic, EffectEnvelope, Seed, Viewer};

use crate::{
    bot_chose_action_public_effect, legal_action_tree, parse_action_path, project_view, CardView,
    CompletedTrickView, CurrentTrickView, Phase, PlainTricksAction, PlainTricksEffect,
    PlainTricksSeat, PlainTricksState, PrivateView, TrickCardId, TrickCounts, TrickSuit,
    ACTION_PLAY,
};

pub const RANDOM_POLICY_ID: &str = "plain-tricks-random-legal-v0";
pub const LEVEL2_POLICY_ID: &str = "plain-tricks-level2-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlainTricksBotInput {
    pub bot_seat: PlainTricksSeat,
    pub legal_action_tree: ActionTree,
    pub phase: Phase,
    pub active_seat: Option<PlainTricksSeat>,
    pub round_index: u8,
    pub trick_index: u8,
    pub current_leader: PlainTricksSeat,
    pub current_trick: CurrentTrickView,
    pub trick_history: Vec<CompletedTrickView>,
    pub round_trick_counts: TrickCounts,
    pub total_trick_counts: TrickCounts,
    pub own_hand: Vec<TrickCardId>,
}

impl PlainTricksBotInput {
    pub fn stable_summary(&self) -> String {
        format!(
            "seat={};choices={};phase={};active={};round={};trick={};leader={};own_hand={};current={};history={};round_counts={},{};total_counts={},{}",
            self.bot_seat.as_str(),
            legal_actions_from_input(self).len(),
            self.phase.as_str(),
            self.active_seat.map(PlainTricksSeat::as_str).unwrap_or("none"),
            self.round_index,
            self.trick_index,
            self.current_leader.as_str(),
            self.own_hand
                .iter()
                .map(|card| card.as_str())
                .collect::<Vec<_>>()
                .join(","),
            stable_current_trick(&self.current_trick),
            self.trick_history
                .iter()
                .map(stable_completed_trick)
                .collect::<Vec<_>>()
                .join("|"),
            self.round_trick_counts.seat_0,
            self.round_trick_counts.seat_1,
            self.total_trick_counts.seat_0,
            self.total_trick_counts.seat_1,
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
    pub effects: Vec<EffectEnvelope<PlainTricksEffect>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlainTricksRandomBot {
    pub seed: Seed,
}

impl PlainTricksRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &PlainTricksState, bot_seat: PlainTricksSeat) -> PlainTricksBotInput {
        bot_input_for(state, bot_seat)
    }

    pub fn select_action(
        &self,
        state: &PlainTricksState,
        bot_seat: PlainTricksSeat,
    ) -> Result<ActionPath, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        RandomLegalBot::new(self.seed).select_action(&input.legal_action_tree)
    }

    pub fn select_decision(
        &self,
        state: &PlainTricksState,
        bot_seat: PlainTricksSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        Ok(decision(
            0,
            RANDOM_POLICY_ID,
            action_path,
            "Selected a seeded random legal Plain Tricks action.".to_owned(),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlainTricksLevel2Bot {
    pub seed: Seed,
}

impl PlainTricksLevel2Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &PlainTricksState, bot_seat: PlainTricksSeat) -> PlainTricksBotInput {
        bot_input_for(state, bot_seat)
    }

    pub fn select_action(
        &self,
        state: &PlainTricksState,
        bot_seat: PlainTricksSeat,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }

    pub fn select_decision(
        &self,
        state: &PlainTricksState,
        bot_seat: PlainTricksSeat,
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
            ActionPath {
                segments: vec![ACTION_PLAY.to_owned(), action.card.as_str().to_owned()],
            },
            level2_rationale(&input, action.card),
        ))
    }
}

pub fn actor_for_seat(state: &PlainTricksState, seat: PlainTricksSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

pub fn action_from_decision(decision: &BotDecision) -> Option<PlainTricksAction> {
    parse_action_path(&decision.action_path.segments)
}

fn bot_input_for(state: &PlainTricksState, bot_seat: PlainTricksSeat) -> PlainTricksBotInput {
    let viewer = Viewer {
        seat_id: Some(state.seats[bot_seat.index()].clone()),
    };
    let view = project_view(state, &viewer);
    let own_hand = match &view.private_view {
        PrivateView::Seat(private) if private.seat == bot_seat => {
            private.own_hand.iter().map(card_from_view).collect()
        }
        _ => Vec::new(),
    };

    PlainTricksBotInput {
        bot_seat,
        legal_action_tree: legal_action_tree(state, &actor_for_seat(state, bot_seat)),
        phase: view.phase,
        active_seat: view.active_seat,
        round_index: view.round_index,
        trick_index: view.trick_index,
        current_leader: view.current_leader,
        current_trick: view.current_trick,
        trick_history: view.trick_history,
        round_trick_counts: view.round_trick_counts,
        total_trick_counts: view.total_trick_counts,
        own_hand,
    }
}

fn card_from_view(card: &CardView) -> TrickCardId {
    TrickCardId::parse(&card.card_id).expect("seat-private view card id is known")
}

fn legal_actions_from_input(input: &PlainTricksBotInput) -> Vec<PlainTricksAction> {
    input
        .legal_action_tree
        .root
        .choices
        .iter()
        .filter(|choice| choice.segment == ACTION_PLAY)
        .filter_map(|choice| choice.next.as_ref())
        .flat_map(|node| node.choices.iter())
        .filter_map(|choice| {
            TrickCardId::parse(&choice.segment).map(|card| PlainTricksAction { card })
        })
        .collect()
}

fn choose_level2_action(
    input: &PlainTricksBotInput,
    legal: &[PlainTricksAction],
    seed: Seed,
) -> PlainTricksAction {
    legal
        .iter()
        .copied()
        .max_by_key(|action| level2_rank(input, action.card, seed))
        .expect("legal action list is non-empty")
}

fn level2_rank(
    input: &PlainTricksBotInput,
    card: TrickCardId,
    seed: Seed,
) -> (
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    std::cmp::Reverse<u8>,
    std::cmp::Reverse<u8>,
    u64,
) {
    let Some(lead) = input.current_trick.plays.first() else {
        return lead_rank(input, card, seed);
    };

    let lead_card = card_from_view(&lead.card);
    let can_win = card.suit() == lead_card.suit() && card.rank() > lead_card.rank();
    let cannot_win = !can_win;
    let late_or_behind = input.trick_index >= 4
        || input.round_trick_counts.get(input.bot_seat)
            <= input.round_trick_counts.get(input.bot_seat.other());

    (
        u8::from(can_win),
        u8::from(can_win && late_or_behind),
        u8::from(cannot_win),
        0,
        0,
        0,
        std::cmp::Reverse(card.rank().value()),
        std::cmp::Reverse(suit_order(card.suit())),
        seeded_tie(seed, card),
    )
}

fn lead_rank(
    input: &PlainTricksBotInput,
    card: TrickCardId,
    seed: Seed,
) -> (
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    std::cmp::Reverse<u8>,
    std::cmp::Reverse<u8>,
    u64,
) {
    let behind_or_tied = input.round_trick_counts.get(input.bot_seat)
        <= input.round_trick_counts.get(input.bot_seat.other());
    let likely_winner = card.rank().value() >= 5 || public_lower_same_suit_count(input, card) >= 2;
    let suit_length = input
        .own_hand
        .iter()
        .filter(|owned| owned.suit() == card.suit())
        .count() as u8;

    (
        u8::from(behind_or_tied && likely_winner),
        u8::from(likely_winner),
        suit_length,
        u8::from(card.rank().value() <= 2),
        0,
        0,
        std::cmp::Reverse(if likely_winner {
            7 - card.rank().value()
        } else {
            card.rank().value()
        }),
        std::cmp::Reverse(suit_order(card.suit())),
        seeded_tie(seed, card),
    )
}

fn public_lower_same_suit_count(input: &PlainTricksBotInput, card: TrickCardId) -> u8 {
    public_played_cards(input)
        .into_iter()
        .filter(|played| played.suit() == card.suit() && played.rank() < card.rank())
        .count() as u8
}

fn public_played_cards(input: &PlainTricksBotInput) -> Vec<TrickCardId> {
    let mut cards = Vec::new();
    for play in &input.current_trick.plays {
        cards.push(card_from_view(&play.card));
    }
    for trick in &input.trick_history {
        cards.push(card_from_view(&trick.plays[0].card));
        cards.push(card_from_view(&trick.plays[1].card));
    }
    cards
}

fn seeded_tie(seed: Seed, card: TrickCardId) -> u64 {
    let mut value = seed.0;
    for byte in card.as_str().bytes() {
        value ^= u64::from(byte);
        value = value.wrapping_mul(0x100_0000_01b3);
    }
    value
}

fn suit_order(suit: TrickSuit) -> u8 {
    match suit {
        TrickSuit::Gale => 0,
        TrickSuit::River => 1,
        TrickSuit::Ember => 2,
    }
}

fn level2_rationale(input: &PlainTricksBotInput, card: TrickCardId) -> String {
    let posture = if let Some(lead) = input.current_trick.plays.first() {
        let lead_card = card_from_view(&lead.card);
        if card.suit() == lead_card.suit() && card.rank() > lead_card.rank() {
            "can win the led suit cheaply"
        } else {
            "cannot win this trick"
        }
    } else if card.rank().value() >= 5 {
        "likely winning lead"
    } else {
        "low lead from length"
    };
    format!(
        "{posture}; chose {} from legal Plain Tricks play options.",
        card.as_str()
    )
}

fn decision(level: u8, policy_id: &str, action_path: ActionPath, rationale: String) -> BotDecision {
    BotDecision {
        policy_id: policy_id.to_owned(),
        policy_version: 1,
        level,
        action_path,
        rationale,
        effects: vec![bot_chose_action_public_effect(policy_id)],
    }
}

fn stable_current_trick(trick: &CurrentTrickView) -> String {
    format!(
        "led={}:plays={}",
        trick.led_suit.as_deref().unwrap_or("none"),
        trick
            .plays
            .iter()
            .map(|play| format!("{}:{}", play.seat.as_str(), play.card.card_id))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn stable_completed_trick(trick: &CompletedTrickView) -> String {
    format!(
        "r{}t{}:{}:{}-{}:{}",
        trick.round_index,
        trick.trick_index,
        trick.winner.as_str(),
        trick.plays[0].card.card_id,
        trick.plays[1].card.card_id,
        trick.trick_counts_after.get(trick.winner)
    )
}
