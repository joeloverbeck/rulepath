//! Bot policies for Meldfall Ledger.
//!
//! L0 is a deterministic random-legal selector over viewer-authorized legal
//! action surfaces. L1 is intentionally not admitted until strategy evidence
//! lands.

use ai_core::RandomLegalBot;
use engine_core::{
    ActionChoice, ActionNode, ActionPath, ActionTree, Diagnostic, FreshnessToken, Seed, Viewer,
};

use crate::{
    actions::{
        action_choice, draw_source_action_tree, LayoffPosition, MeldfallAction,
        DISCARD_SEGMENT_PREFIX, FINISH_TURN_SEGMENT, GO_OUT_WITHOUT_DISCARD_SEGMENT,
        LAY_OFF_SEGMENT_PREFIX, MELD_NEW_SEGMENT_PREFIX,
    },
    cards::CardId,
    rules::{lay_off_card, validate_new_meld},
    state::{MatchState, SeatIndex, TurnOrdinal, TurnPhase},
    visibility::{project_action_tree_for_viewer, project_view, MeldfallView},
};

pub const L0_POLICY_ID: &str = "meldfall-ledger-l0-random-legal-v1";
pub const L1_POLICY_STATUS: &str = "not_admitted_pending_strategy_evidence";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldfallBotInput {
    pub bot_seat: SeatIndex,
    pub view: MeldfallView,
    pub legal_action_tree: ActionTree,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldfallBotDecision {
    pub policy_id: &'static str,
    pub bot_seat: SeatIndex,
    pub action_path: ActionPath,
    pub explanation: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MeldfallL0Bot {
    seed: Seed,
}

impl MeldfallL0Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(state: &MatchState, bot_seat: SeatIndex) -> MeldfallBotInput {
        bot_input_for(state, bot_seat)
    }

    pub fn select_action(
        &self,
        state: &MatchState,
        bot_seat: SeatIndex,
    ) -> Result<ActionPath, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        RandomLegalBot::new(self.seed).select_action(&input.legal_action_tree)
    }

    pub fn select_decision(
        &self,
        state: &MatchState,
        bot_seat: SeatIndex,
    ) -> Result<MeldfallBotDecision, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        let action_path = RandomLegalBot::new(self.seed).select_action(&input.legal_action_tree)?;
        Ok(MeldfallBotDecision {
            policy_id: L0_POLICY_ID,
            bot_seat,
            action_path,
            explanation: format!(
                "Selected one seeded legal action from {} viewer-authorized choices.",
                legal_action_paths(&input.legal_action_tree).len()
            ),
        })
    }
}

pub fn legal_action_paths(tree: &ActionTree) -> Vec<ActionPath> {
    let mut paths = Vec::new();
    collect_paths(&tree.root, Vec::new(), &mut paths);
    paths
}

pub fn bot_input_for(state: &MatchState, bot_seat: SeatIndex) -> MeldfallBotInput {
    let viewer = Viewer {
        seat_id: Some(state.seats[bot_seat].clone()),
    };
    let tree = legal_action_tree_for_seat(state, bot_seat, FreshnessToken(0));
    MeldfallBotInput {
        bot_seat,
        view: project_view(state, &viewer),
        legal_action_tree: project_action_tree_for_viewer(&tree, state, &viewer),
    }
}

pub fn legal_action_tree_for_seat(
    state: &MatchState,
    bot_seat: SeatIndex,
    freshness_token: FreshnessToken,
) -> ActionTree {
    if bot_seat != state.round.active_seat_index {
        return ActionTree::flat(freshness_token, Vec::new());
    }
    match state.round.phase {
        TurnPhase::Draw => draw_source_action_tree(freshness_token, &state.round),
        TurnPhase::Table => table_phase_action_tree(state, bot_seat, freshness_token),
        TurnPhase::Discard => discard_action_tree(state, bot_seat, freshness_token),
        TurnPhase::RoundSettled | TurnPhase::MatchComplete => {
            ActionTree::flat(freshness_token, Vec::new())
        }
    }
}

pub fn parse_bot_action(path: &ActionPath) -> Result<MeldfallAction, Diagnostic> {
    let Some(segment) = path.segments.first() else {
        return Err(no_legal_actions());
    };
    if segment == crate::actions::DRAW_STOCK_SEGMENT {
        return Ok(MeldfallAction::DrawFromStock);
    }
    if let Some(index) =
        segment.strip_prefix(&format!("{}-", crate::actions::DRAW_DISCARD_SEGMENT_PREFIX))
    {
        let discard_index = index.parse::<usize>().map_err(|_| Diagnostic {
            code: "ML_BOT_UNKNOWN_ACTION".to_owned(),
            message: "meldfall_ledger bot selected an unknown discard index".to_owned(),
        })?;
        return Ok(MeldfallAction::DrawFromDiscard { discard_index });
    }
    if segment == FINISH_TURN_SEGMENT {
        return Ok(MeldfallAction::FinishTurn);
    }
    if segment == GO_OUT_WITHOUT_DISCARD_SEGMENT {
        return Ok(MeldfallAction::GoOutWithoutDiscard);
    }
    if let Some(card_list) = segment.strip_prefix(&format!("{MELD_NEW_SEGMENT_PREFIX}-")) {
        return Ok(MeldfallAction::MeldNew {
            cards: parse_card_list(card_list)?,
        });
    }
    if let Some(rest) = segment.strip_prefix(&format!("{LAY_OFF_SEGMENT_PREFIX}-")) {
        let parts = rest.split('-').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(Diagnostic {
                code: "ML_BOT_UNKNOWN_ACTION".to_owned(),
                message: "meldfall_ledger bot selected a malformed lay-off action".to_owned(),
            });
        }
        let card = parse_card(parts[0])?;
        let meld_id = parts[1]
            .strip_prefix("meld_")
            .and_then(|value| value.parse::<u32>().ok())
            .ok_or_else(|| Diagnostic {
                code: "ML_BOT_UNKNOWN_ACTION".to_owned(),
                message: "meldfall_ledger bot selected an unknown lay-off meld".to_owned(),
            })?;
        let position = match parts[2] {
            "prepend" => LayoffPosition::Prepend,
            "append" => LayoffPosition::Append,
            _ => {
                return Err(Diagnostic {
                    code: "ML_BOT_UNKNOWN_ACTION".to_owned(),
                    message: "meldfall_ledger bot selected an unknown lay-off position".to_owned(),
                })
            }
        };
        return Ok(MeldfallAction::LayOff {
            card,
            target_meld: crate::state::MeldId(meld_id),
            position,
        });
    }
    if let Some(card_id) = segment.strip_prefix(&format!("{DISCARD_SEGMENT_PREFIX}-")) {
        let card = parse_card(card_id)?;
        return Ok(MeldfallAction::Discard { card });
    }
    Err(Diagnostic {
        code: "ML_BOT_UNKNOWN_ACTION".to_owned(),
        message: "meldfall_ledger bot selected an unknown action".to_owned(),
    })
}

fn discard_action_tree(
    state: &MatchState,
    bot_seat: SeatIndex,
    freshness_token: FreshnessToken,
) -> ActionTree {
    let choices = state.round.seats[bot_seat]
        .hand
        .iter()
        .copied()
        .map(|card| action_choice(MeldfallAction::Discard { card }))
        .collect::<Vec<ActionChoice>>();
    ActionTree::flat(freshness_token, choices)
}

fn table_phase_action_tree(
    state: &MatchState,
    seat: SeatIndex,
    freshness_token: FreshnessToken,
) -> ActionTree {
    let hand = &state.round.seats[seat].hand;
    let mut melds = legal_new_melds(hand);
    let mut lay_offs = legal_lay_offs(state, seat);
    let pending_card = state
        .round
        .pending_pickup
        .as_ref()
        .filter(|pending| pending.required_by_seat == seat)
        .map(|pending| pending.selected_card);
    if let Some(card) = pending_card {
        melds.retain(|action| match action {
            MeldfallAction::MeldNew { cards } => cards.contains(&card),
            _ => false,
        });
        lay_offs.retain(|action| match action {
            MeldfallAction::LayOff {
                card: layoff_card, ..
            } => *layoff_card == card,
            _ => false,
        });
    }
    let can_go_out_without_discard = hand.is_empty();
    let mut choices = Vec::new();
    choices.extend(melds.into_iter().map(action_choice));
    choices.extend(lay_offs.into_iter().map(action_choice));
    if pending_card.is_none() {
        if can_go_out_without_discard {
            choices.push(action_choice(MeldfallAction::GoOutWithoutDiscard));
        }
        choices.push(action_choice(MeldfallAction::FinishTurn));
    }
    ActionTree::flat(freshness_token, choices)
}

fn legal_new_melds(hand: &[CardId]) -> Vec<MeldfallAction> {
    let mut actions = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    let mut by_rank = std::collections::BTreeMap::<_, Vec<CardId>>::new();
    let mut by_suit = std::collections::BTreeMap::<_, Vec<CardId>>::new();

    for card in hand {
        by_rank.entry(card.card().rank).or_default().push(*card);
        by_suit.entry(card.card().suit).or_default().push(*card);
    }

    for cards in by_rank.values() {
        for size in 3..=cards.len().min(4) {
            collect_combinations(cards, size, &mut |meld| {
                push_valid_meld(&mut actions, &mut seen, meld)
            });
        }
    }

    for cards in by_suit.values() {
        let mut low = cards.clone();
        low.sort_by_key(|card| card.card().rank.low_run_value());
        collect_consecutive_runs(&low, |card| card.card().rank.low_run_value(), &mut |meld| {
            push_valid_meld(&mut actions, &mut seen, meld)
        });

        let mut high = cards.clone();
        high.sort_by_key(|card| card.card().rank.high_run_value());
        collect_consecutive_runs(
            &high,
            |card| card.card().rank.high_run_value(),
            &mut |meld| push_valid_meld(&mut actions, &mut seen, meld),
        );
    }

    actions
}

fn legal_lay_offs(state: &MatchState, seat: SeatIndex) -> Vec<MeldfallAction> {
    let mut actions = Vec::new();
    for card in &state.round.seats[seat].hand {
        for group in &state.round.tableau.groups {
            for position in [LayoffPosition::Prepend, LayoffPosition::Append] {
                let mut round = state.round.clone();
                if lay_off_card(&mut round, seat, *card, group.id, position, TurnOrdinal(0)).is_ok()
                {
                    actions.push(MeldfallAction::LayOff {
                        card: *card,
                        target_meld: group.id,
                        position,
                    });
                }
            }
        }
    }
    actions
}

fn collect_combinations<F>(cards: &[CardId], size: usize, push: &mut F)
where
    F: FnMut(Vec<CardId>),
{
    fn visit<F>(
        cards: &[CardId],
        size: usize,
        start: usize,
        current: &mut Vec<CardId>,
        push: &mut F,
    ) where
        F: FnMut(Vec<CardId>),
    {
        if current.len() == size {
            push(current.clone());
            return;
        }
        let remaining = size - current.len();
        for index in start..=cards.len() - remaining {
            current.push(cards[index]);
            visit(cards, size, index + 1, current, push);
            current.pop();
        }
    }

    if size <= cards.len() {
        visit(cards, size, 0, &mut Vec::new(), push);
    }
}

fn collect_consecutive_runs<F, K>(cards: &[CardId], key: K, push: &mut F)
where
    F: FnMut(Vec<CardId>),
    K: Fn(CardId) -> u8,
{
    for start in 0..cards.len() {
        let mut run = vec![cards[start]];
        for next in cards.iter().copied().skip(start + 1) {
            let previous = *run.last().expect("run has a start card");
            if key(next) == key(previous) + 1 {
                run.push(next);
                if run.len() >= 3 {
                    push(run.clone());
                }
            } else {
                break;
            }
        }
    }
}

fn push_valid_meld(
    actions: &mut Vec<MeldfallAction>,
    seen: &mut std::collections::BTreeSet<String>,
    cards: Vec<CardId>,
) {
    if validate_new_meld(&cards).is_ok() {
        let action = MeldfallAction::MeldNew { cards };
        if seen.insert(action.segment()) {
            actions.push(action);
        }
    }
}

fn parse_card(value: &str) -> Result<CardId, Diagnostic> {
    CardId::parse(value).ok_or_else(|| Diagnostic {
        code: "ML_BOT_UNKNOWN_CARD".to_owned(),
        message: "meldfall_ledger bot selected an unknown card".to_owned(),
    })
}

fn parse_card_list(value: &str) -> Result<Vec<CardId>, Diagnostic> {
    let parts = value.split('_').collect::<Vec<_>>();
    if parts.len() % 2 != 0 {
        return Err(Diagnostic {
            code: "ML_BOT_UNKNOWN_CARD".to_owned(),
            message: "meldfall_ledger bot selected a malformed meld card list".to_owned(),
        });
    }
    parts
        .chunks(2)
        .map(|chunk| parse_card(&format!("{}_{}", chunk[0], chunk[1])))
        .collect()
}

fn collect_paths(node: &ActionNode, prefix: Vec<String>, paths: &mut Vec<ActionPath>) {
    for choice in &node.choices {
        let mut next_prefix = prefix.clone();
        next_prefix.push(choice.segment.clone());
        if let Some(next) = &choice.next {
            collect_paths(next, next_prefix, paths);
        } else {
            paths.push(ActionPath {
                segments: next_prefix,
            });
        }
    }
}

fn no_legal_actions() -> Diagnostic {
    Diagnostic {
        code: "no_legal_actions".to_owned(),
        message: "no Meldfall Ledger legal action is available to the bot".to_owned(),
    }
}
