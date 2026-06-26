export type OutcomeExplanationTemplate = {
  summary: string;
  expandedHeading: string;
  requiredParams: readonly string[];
  allowedGameIds: readonly string[];
  ruleRefLabel?: string;
};

export const outcomeExplanationTemplates = {
  "race_to_n.exact_target_reached": {
    summary: "{winner} reached {target} exactly.",
    expandedHeading: "Target result",
    requiredParams: ["winner", "target"],
    allowedGameIds: ["race_to_n"],
    ruleRefLabel: "Race target rule",
  },
  "three_marks.line_completed": {
    summary: "{winner} completed {line_label}.",
    expandedHeading: "Winning line",
    requiredParams: ["winner", "line_label"],
    allowedGameIds: ["three_marks"],
    ruleRefLabel: "Line completion rule",
  },
  "three_marks.full_board_draw": {
    summary: "The board filled with no winning line.",
    expandedHeading: "Drawn board",
    requiredParams: [],
    allowedGameIds: ["three_marks"],
    ruleRefLabel: "Full board rule",
  },
  "column_four.line_completed": {
    summary: "{winner} completed {line_label}.",
    expandedHeading: "Winning line",
    requiredParams: ["winner", "line_label"],
    allowedGameIds: ["column_four"],
    ruleRefLabel: "Line completion rule",
  },
  "column_four.full_board_draw": {
    summary: "The board filled with no winning line.",
    expandedHeading: "Drawn board",
    requiredParams: [],
    allowedGameIds: ["column_four"],
    ruleRefLabel: "Full board rule",
  },
  "directional_flip.final_score_win": {
    summary: "{winner} has the higher final score.",
    expandedHeading: "Final score",
    requiredParams: ["winner", "target"],
    allowedGameIds: ["directional_flip"],
    ruleRefLabel: "Final score rule",
  },
  "directional_flip.final_score_draw": {
    summary: "Final scores are tied.",
    expandedHeading: "Final score draw",
    requiredParams: [],
    allowedGameIds: ["directional_flip"],
    ruleRefLabel: "Final score rule",
  },
  "draughts_lite.opponent_no_pieces": {
    summary: "{winner} wins because {loser} has no pieces.",
    expandedHeading: "Piece exhaustion",
    requiredParams: ["winner", "loser"],
    allowedGameIds: ["draughts_lite"],
    ruleRefLabel: "Terminal piece rule",
  },
  "draughts_lite.opponent_no_legal_move": {
    summary: "{winner} wins because {loser} has no legal move.",
    expandedHeading: "No legal move",
    requiredParams: ["winner", "loser"],
    allowedGameIds: ["draughts_lite"],
    ruleRefLabel: "Terminal move rule",
  },
  "high_card_duel.final_score_win": {
    summary: "{winner} has the higher final score.",
    expandedHeading: "Final score",
    requiredParams: ["winner"],
    allowedGameIds: ["high_card_duel"],
    ruleRefLabel: "Final score rule",
  },
  "high_card_duel.final_score_draw": {
    summary: "Final scores are tied.",
    expandedHeading: "Final score draw",
    requiredParams: [],
    allowedGameIds: ["high_card_duel"],
    ruleRefLabel: "Final score rule",
  },
  "token_bazaar.score_win": {
    summary: "{winner} has the higher final score.",
    expandedHeading: "Score result",
    requiredParams: ["winner"],
    allowedGameIds: ["token_bazaar"],
    ruleRefLabel: "Score rule",
  },
  "token_bazaar.fulfilled_tiebreak_win": {
    summary: "{winner} leads on fulfilled contracts.",
    expandedHeading: "Fulfilled contracts",
    requiredParams: ["winner"],
    allowedGameIds: ["token_bazaar"],
    ruleRefLabel: "Fulfilled contracts rule",
  },
  "token_bazaar.inventory_tiebreak_win": {
    summary: "{winner} leads on inventory total.",
    expandedHeading: "Inventory total",
    requiredParams: ["winner"],
    allowedGameIds: ["token_bazaar"],
    ruleRefLabel: "Inventory rule",
  },
  "token_bazaar.all_tied_draw": {
    summary: "All final comparisons are tied.",
    expandedHeading: "Drawn market",
    requiredParams: [],
    allowedGameIds: ["token_bazaar"],
    ruleRefLabel: "Draw rule",
  },
  "secret_draft.score_win": {
    summary: "{winner} has the higher final score.",
    expandedHeading: "Score result",
    requiredParams: ["winner"],
    allowedGameIds: ["secret_draft"],
    ruleRefLabel: "Score rule",
  },
  "secret_draft.complete_sets_tiebreak": {
    summary: "{winner} leads on complete sets.",
    expandedHeading: "Complete sets",
    requiredParams: ["winner"],
    allowedGameIds: ["secret_draft"],
    ruleRefLabel: "Complete sets rule",
  },
  "secret_draft.highest_single_tiebreak": {
    summary: "{winner} leads on highest single value.",
    expandedHeading: "Highest single value",
    requiredParams: ["winner"],
    allowedGameIds: ["secret_draft"],
    ruleRefLabel: "Highest value rule",
  },
  "secret_draft.distinct_threads_tiebreak": {
    summary: "{winner} leads on represented threads.",
    expandedHeading: "Represented threads",
    requiredParams: ["winner"],
    allowedGameIds: ["secret_draft"],
    ruleRefLabel: "Thread rule",
  },
  "secret_draft.fewer_priority_conflict_wins_tiebreak": {
    summary: "{winner} has fewer priority-won conflicts.",
    expandedHeading: "Priority conflicts",
    requiredParams: ["winner"],
    allowedGameIds: ["secret_draft"],
    ruleRefLabel: "Conflict history rule",
  },
  "secret_draft.all_tied_draw": {
    summary: "All public final comparisons are tied.",
    expandedHeading: "Drawn draft",
    requiredParams: [],
    allowedGameIds: ["secret_draft"],
    ruleRefLabel: "Draw rule",
  },
  "poker_lite.yield_win_no_reveal": {
    summary: "{winner} wins after {loser} yields.",
    expandedHeading: "Yield result",
    requiredParams: ["winner", "loser"],
    allowedGameIds: ["poker_lite"],
    ruleRefLabel: "Yield rule",
  },
  "poker_lite.pair_beats_high_card": {
    summary: "{winner} wins with a pair.",
    expandedHeading: "Showdown strength",
    requiredParams: ["winner"],
    allowedGameIds: ["poker_lite"],
    ruleRefLabel: "Showdown rule",
  },
  "poker_lite.private_rank_tiebreak": {
    summary: "{winner} wins on the revealed showdown rank.",
    expandedHeading: "Showdown rank",
    requiredParams: ["winner"],
    allowedGameIds: ["poker_lite"],
    ruleRefLabel: "Showdown rule",
  },
  "poker_lite.equal_strength_split": {
    summary: "The pool is split after equal showdown strength.",
    expandedHeading: "Split pool",
    requiredParams: [],
    allowedGameIds: ["poker_lite"],
    ruleRefLabel: "Split rule",
  },
  "river_ledger.last_live_fold_win": {
    summary: "The last live seat receives the ledger after the other live seats fold.",
    expandedHeading: "Last live hand",
    requiredParams: [],
    allowedGameIds: ["river_ledger"],
    ruleRefLabel: "Terminal rule",
  },
  "river_ledger.showdown_best_hand_win": {
    summary: "The strongest revealed five-card hand receives the ledger.",
    expandedHeading: "Showdown result",
    requiredParams: [],
    allowedGameIds: ["river_ledger"],
    ruleRefLabel: "Showdown rule",
  },
  "river_ledger.showdown_split_pot": {
    summary: "Equal strongest revealed hands share the ledger allocation.",
    expandedHeading: "Split ledger",
    requiredParams: [],
    allowedGameIds: ["river_ledger"],
    ruleRefLabel: "Split rule",
  },
  "plain_tricks.trick_win": {
    summary: "{winner} won more tricks.",
    expandedHeading: "Trick total",
    requiredParams: ["winner"],
    allowedGameIds: ["plain_tricks"],
    ruleRefLabel: "Trick total rule",
  },
  "plain_tricks.split": {
    summary: "Final trick totals are tied.",
    expandedHeading: "Split trick total",
    requiredParams: [],
    allowedGameIds: ["plain_tricks"],
    ruleRefLabel: "Split rule",
  },
  "briar_circuit.low_score_win": {
    summary: "{winner} has the lowest score after the threshold hand.",
    expandedHeading: "Lowest score wins",
    requiredParams: ["winner"],
    allowedGameIds: ["briar_circuit"],
    ruleRefLabel: "Match threshold rule",
  },
  "briar_circuit.moon_adjustment": {
    summary: "A moon shot changed the hand additions.",
    expandedHeading: "Moon adjustment",
    requiredParams: [],
    allowedGameIds: ["briar_circuit"],
    ruleRefLabel: "Moon scoring rule",
  },
  "briar_circuit.tied_low_continuation": {
    summary: "The lowest score is tied, so another complete hand is needed.",
    expandedHeading: "Tied low score",
    requiredParams: [],
    allowedGameIds: ["briar_circuit"],
    ruleRefLabel: "Tie continuation rule",
  },
  "vow_tide.high_score_win": {
    summary: "{winner} has the highest cumulative score.",
    expandedHeading: "Highest score wins",
    requiredParams: ["winner"],
    allowedGameIds: ["vow_tide"],
    ruleRefLabel: "VT-STANDINGS-001",
  },
  "vow_tide.shared_high_score": {
    summary: "The highest cumulative score is shared.",
    expandedHeading: "Shared high score",
    requiredParams: [],
    allowedGameIds: ["vow_tide"],
    ruleRefLabel: "VT-STANDINGS-001",
  },
  "meldfall_ledger.high_score_win": {
    summary: "{winner} reached the target with the unique highest cumulative score.",
    expandedHeading: "Target score result",
    requiredParams: ["winner"],
    allowedGameIds: ["meldfall_ledger"],
    ruleRefLabel: "ML-MATCH-001 through ML-MATCH-005",
  },
  "blackglass_pact.team_score_win": {
    summary: "{winner} reached the terminal score condition with the higher team score.",
    expandedHeading: "Team score result",
    requiredParams: ["winner"],
    allowedGameIds: ["blackglass_pact"],
    ruleRefLabel: "BP-END-001 through BP-END-003",
  },
  "blackglass_pact.tied_threshold_continues": {
    summary: "Both teams are tied at the threshold, so another complete hand is required.",
    expandedHeading: "Tied threshold",
    requiredParams: [],
    allowedGameIds: ["blackglass_pact"],
    ruleRefLabel: "BP-END-004",
  },
  "masked_claims.score_win": {
    summary: "{winner} has the higher final score.",
    expandedHeading: "Final score",
    requiredParams: ["winner"],
    allowedGameIds: ["masked_claims"],
    ruleRefLabel: "MC-END-001",
  },
  "masked_claims.tiebreak_win": {
    summary: "{winner} wins on the public tiebreak ladder.",
    expandedHeading: "Tiebreak result",
    requiredParams: ["winner"],
    allowedGameIds: ["masked_claims"],
    ruleRefLabel: "MC-END-002 through MC-END-004",
  },
  "masked_claims.tiebreak_exposed_lies": {
    summary: "{winner} wins with fewer exposed lies.",
    expandedHeading: "Exposed-lie tiebreak",
    requiredParams: ["winner"],
    allowedGameIds: ["masked_claims"],
    ruleRefLabel: "MC-END-002",
  },
  "masked_claims.tiebreak_successful_challenges": {
    summary: "{winner} wins with more successful challenges.",
    expandedHeading: "Successful-challenge tiebreak",
    requiredParams: ["winner"],
    allowedGameIds: ["masked_claims"],
    ruleRefLabel: "MC-END-003",
  },
  "masked_claims.tiebreak_challenges_declared": {
    summary: "{winner} wins with fewer declared challenges.",
    expandedHeading: "Challenge-discipline tiebreak",
    requiredParams: ["winner"],
    allowedGameIds: ["masked_claims"],
    ruleRefLabel: "MC-END-004",
  },
  "masked_claims.draw": {
    summary: "Scores and all public tiebreakers are tied.",
    expandedHeading: "Drawn claim ledger",
    requiredParams: [],
    allowedGameIds: ["masked_claims"],
    ruleRefLabel: "MC-END-005",
  },
  "flood_watch.shared_loss_inundation": {
    summary: "A public district reached the inundation threshold after {drawn_card_count} drawn storm cards.",
    expandedHeading: "Shared loss",
    requiredParams: ["drawn_card_count"],
    allowedGameIds: ["flood_watch"],
    ruleRefLabel: "FW-END-001",
  },
  "flood_watch.shared_win_deck_exhausted": {
    summary: "The team survived all {drawn_card_count} drawn storm cards.",
    expandedHeading: "Shared win",
    requiredParams: ["drawn_card_count"],
    allowedGameIds: ["flood_watch"],
    ruleRefLabel: "FW-END-002",
  },
  "frontier_control.score_compare": {
    summary: "{winner} wins the frontier {garrison_score}-{prospector_score}.",
    expandedHeading: "Final score",
    requiredParams: ["winner", "garrison_score", "prospector_score"],
    allowedGameIds: ["frontier_control"],
    ruleRefLabel: "FC-TERM-SCORE-COMPARE",
  },
  "frontier_control.garrison_tiebreak": {
    summary: "The Garrison wins the tied frontier {garrison_score}-{prospector_score}.",
    expandedHeading: "Garrison tiebreak",
    requiredParams: ["garrison_score", "prospector_score"],
    allowedGameIds: ["frontier_control"],
    ruleRefLabel: "FC-TERM-GARRISON-TIEBREAK",
  },
  "event_frontier.charter_instant": {
    summary: "{winner} wins by holding enough majority sites at Reckoning.",
    expandedHeading: "Charter instant victory",
    requiredParams: ["winner"],
    allowedGameIds: ["event_frontier"],
    ruleRefLabel: "EF-END-001",
  },
  "event_frontier.freeholder_instant": {
    summary: "{winner} wins by reaching the public cache threshold at Reckoning.",
    expandedHeading: "Freeholder instant victory",
    requiredParams: ["winner"],
    allowedGameIds: ["event_frontier"],
    ruleRefLabel: "EF-END-002",
  },
  "event_frontier.both_met_freeholder": {
    summary: "Both instant conditions were met, so the Freeholders win.",
    expandedHeading: "Both-met rule",
    requiredParams: [],
    allowedGameIds: ["event_frontier"],
    ruleRefLabel: "EF-END-003",
  },
  "event_frontier.final_fallback_score": {
    summary: "{winner} wins the final fallback {charter_score}-{freeholder_score}.",
    expandedHeading: "Final fallback",
    requiredParams: ["winner", "charter_score", "freeholder_score"],
    allowedGameIds: ["event_frontier"],
    ruleRefLabel: "EF-END-004",
  },
  "event_frontier.final_fallback_tiebreak": {
    summary: "The Freeholders win tied final fallback scores.",
    expandedHeading: "Final fallback tiebreak",
    requiredParams: [],
    allowedGameIds: ["event_frontier"],
    ruleRefLabel: "EF-END-004",
  },
} as const satisfies Record<string, OutcomeExplanationTemplate>;

export type OutcomeExplanationTemplateKey = keyof typeof outcomeExplanationTemplates;

export function isOutcomeExplanationTemplateKey(value: string): value is OutcomeExplanationTemplateKey {
  return Object.prototype.hasOwnProperty.call(outcomeExplanationTemplates, value);
}

const outcomeValueCopy: Record<string, string> = {
  all_tied_draw: "All tied draw",
  complete_sets_tiebreak: "Complete sets tiebreak",
  distinct_threads_tiebreak: "Distinct threads tiebreak",
  draw: "Draw",
  equal_strength_split: "Equal strength split",
  exact_target_reached: "Exact target reached",
  fewer_priority_conflict_wins_tiebreak: "Fewer priority-won conflicts tiebreak",
  final_score: "Final score",
  full_board_draw: "Full board draw",
  fulfilled_tiebreak_win: "Fulfilled contracts tiebreak win",
  charter_instant: "Charter instant",
  event_frontier: "Event Frontier",
  faction_charter: "Charter",
  faction_freeholders: "Freeholders",
  final_fallback: "Final fallback",
  final_fallback_score: "Final fallback score",
  final_fallback_tiebreak: "Final fallback tiebreak",
  freeholder_instant: "Freeholder instant",
  flood_watch: "Flood Watch",
  frontier_control: "Frontier Control",
  faction_garrison: "Garrison",
  faction_prospectors: "Prospectors",
  garrison_tiebreak: "Garrison tiebreak",
  high_card: "High card",
  highest_single_tiebreak: "Highest single tiebreak",
  inventory_tiebreak_win: "Inventory tiebreak win",
  line_completed: "Line completed",
  loss: "Loss",
  low: "Low",
  masked_claims_draw: "Masked Claims draw",
  meldfall_ledger: "Meldfall Ledger",
  non_terminal: "Non-terminal",
  opponent_no_legal_move: "Opponent has no legal move",
  opponent_no_pieces: "Opponent has no pieces",
  pair: "Pair",
  pair_beats_high_card: "Pair beats high card",
  private_rank_tiebreak: "Private rank tiebreak",
  rust_terminal_rationale: "Terminal rationale",
  score_win: "Score win",
  shared_loss_inundation: "Shared loss by inundation",
  shared_win_deck_exhausted: "Shared win by deck exhaustion",
  showdown_win: "Showdown win",
  split: "Split",
  terminal_position: "Terminal position",
  trick_win: "Trick win",
  vow_tide: "Vow Tide",
  win: "Win",
  yield_win: "Yield win",
};

export function seatDisplayLabel(seat: string): string {
  return /^seat_\d+$/.test(seat) ? resolveSeatLabel(seat) : seat;
}

export function outcomeDisplayText(value: string): string {
  return value
    .replace(/\bseat_\d+\b/g, (seat) => resolveSeatLabel(seat))
    .replace(/\br(\d+)c(\d+)\b/g, (_match, row: string, column: string) => `row ${row} column ${column}`)
    .replace(/\b[a-z]+(?:_[a-z]+)+\b/g, (token) => outcomeValueCopy[token] ?? token);
}

export function outcomeDisplayValue(value: string): string {
  const copied = outcomeValueCopy[value];
  if (copied) {
    return copied;
  }
  const seatLabel = seatDisplayLabel(value);
  return seatLabel === value ? outcomeDisplayText(value) : seatLabel;
}
import { resolveSeatLabel } from "../seatLabels";
