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
    requiredParams: ["winner"],
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
} as const satisfies Record<string, OutcomeExplanationTemplate>;

export type OutcomeExplanationTemplateKey = keyof typeof outcomeExplanationTemplates;

export function isOutcomeExplanationTemplateKey(value: string): value is OutcomeExplanationTemplateKey {
  return Object.prototype.hasOwnProperty.call(outcomeExplanationTemplates, value);
}
