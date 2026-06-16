type WasmExports = {
  memory: WebAssembly.Memory;
  rulepath_placeholder_version_ptr: () => number;
  rulepath_placeholder_version_len: () => number;
  rulepath_alloc: (len: number) => number;
  rulepath_dealloc: (ptr: number, len: number) => void;
  rulepath_last_output_ptr: () => number;
  rulepath_last_output_len: () => number;
  rulepath_feature_report: () => number;
  rulepath_list_games: () => number;
  rulepath_new_match: (gamePtr: number, gameLen: number, seed: bigint) => number;
  rulepath_new_match_with_seat_count: (gamePtr: number, gameLen: number, seed: bigint, seatCount: number) => number;
  rulepath_new_match_with_variant: (
    gamePtr: number,
    gameLen: number,
    variantPtr: number,
    variantLen: number,
    seed: bigint,
  ) => number;
  rulepath_get_view: (matchPtr: number, matchLen: number) => number;
  rulepath_get_view_for_viewer: (
    matchPtr: number,
    matchLen: number,
    viewerPtr: number,
    viewerLen: number,
  ) => number;
  rulepath_get_action_tree: (
    matchPtr: number,
    matchLen: number,
    seatPtr: number,
    seatLen: number,
  ) => number;
  rulepath_get_action_tree_for_viewer: (
    matchPtr: number,
    matchLen: number,
    seatPtr: number,
    seatLen: number,
    viewerPtr: number,
    viewerLen: number,
  ) => number;
  rulepath_apply_action: (
    matchPtr: number,
    matchLen: number,
    seatPtr: number,
    seatLen: number,
    pathPtr: number,
    pathLen: number,
    freshnessToken: bigint,
  ) => number;
  rulepath_run_bot_turn: (
    matchPtr: number,
    matchLen: number,
    seatPtr: number,
    seatLen: number,
    botSeed: bigint,
  ) => number;
  rulepath_get_effects: (
    matchPtr: number,
    matchLen: number,
    sinceCursor: bigint,
    viewerPtr: number,
    viewerLen: number,
  ) => number;
  rulepath_export_replay: (matchPtr: number, matchLen: number) => number;
  rulepath_import_replay: (docPtr: number, docLen: number) => number;
  rulepath_replay_step: (replayPtr: number, replayLen: number, cursor: number) => number;
  rulepath_replay_reset: (replayPtr: number, replayLen: number) => number;
};

export type MatchCreated = {
  match_id: string;
  game_id: string;
  variant_id?: string;
};

export type GameVariantCatalogEntry = {
  id: string;
  label: string;
  description?: string;
};

export function selectVariantDescription(variant: GameVariantCatalogEntry | null | undefined): string | undefined {
  const description = variant?.description?.trim();
  return description ? description : undefined;
}

export type GameCatalogEntry = {
  game_id: string;
  display_name: string;
  rules_version: number;
  schema_version: number;
  min_seats?: number;
  max_seats?: number;
  default_seats?: number;
  supported_seats?: number[];
  seat_labels?: SeatDisplayLabel[];
  variants?: GameVariantCatalogEntry[];
  viewer_modes?: ViewerModeId[];
  hidden_information?: boolean;
  cooperative?: boolean;
  tags?: string[];
  ui?: GameCatalogUiMetadata;
};

export type SeatDisplayLabel = { seat: ViewerSeatId | string; label: string };
export type FactionDisplayLabel = { faction: string; label: string };

export type GameCatalogUiMetadata = {
  seat_labels?: SeatDisplayLabel[];
  faction_labels?: FactionDisplayLabel[];
};

export type FeatureReport = {
  api_version: string;
  operations: string[];
  features: string[];
};

export type SeatId = "seat_0" | "seat_1";
export type RiverLedgerSeatId = SeatId | "seat_2" | "seat_3" | "seat_4" | "seat_5";
export type ViewerSeatId = RiverLedgerSeatId;
export type ViewerModeId = "observer" | ViewerSeatId;
export type ViewerMode = { kind: "observer" } | { kind: "seat"; seat: ViewerSeatId };

export type OutcomeRationaleField = {
  label: string;
  value: string | number | boolean | null;
  emphasized?: boolean;
  rule_id?: string;
};

export type OutcomeRationaleStanding = {
  seat: SeatId | string;
  label?: string;
  result?: string;
  emphasized?: boolean;
  values?: OutcomeRationaleField[];
};

export type OutcomeRationaleBreakdownSection = {
  id: string;
  heading: string;
  summary?: string;
  rows?: OutcomeRationaleField[];
};

export type OutcomeRationalePayload = {
  result_kind: string;
  decisive_cause: string;
  template_key: string;
  template_params?: Record<string, string | number | boolean | null>;
  decisive_rule_ids?: string[];
  final_standing?: OutcomeRationaleStanding[];
  breakdown_sections?: OutcomeRationaleBreakdownSection[];
};

export type RaceToNOutcomeRationale = OutcomeRationalePayload;
export type ThreeMarksOutcomeRationale = OutcomeRationalePayload;
export type ColumnFourOutcomeRationale = OutcomeRationalePayload;
export type DirectionalFlipOutcomeRationale = OutcomeRationalePayload;
export type DraughtsLiteOutcomeRationale = OutcomeRationalePayload;
export type HighCardDuelOutcomeRationale = OutcomeRationalePayload;
export type TokenBazaarOutcomeRationale = OutcomeRationalePayload;
export type SecretDraftOutcomeRationale = OutcomeRationalePayload;
export type PokerLiteOutcomeRationale = OutcomeRationalePayload;
export type PlainTricksOutcomeRationale = {
  result_kind: string;
  decisive_cause: string;
  template_key: string;
  decisive_rule_ids: string[];
  per_seat: Array<{ seat: SeatId; total_tricks: number; result: string }>;
};

export type RaceToNPublicView = {
  counter: number;
  target: number;
  max_add: number;
  active_seat: SeatId;
  winner: SeatId | null;
  terminal_rationale?: RaceToNOutcomeRationale | null;
  freshness_token: number;
};

export type RacePublicView = RaceToNPublicView;

export type ThreeMarksPublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "three_marks";
  display_name: string;
  variant_id: string;
  rules_version_label: string;
  board_rows: number;
  board_columns: number;
  cells: string[];
  active_seat: SeatId;
  ply_count: number;
  status_label: string;
  freshness_token: number;
  legal_targets: string[];
  terminal_kind: "non_terminal" | "win" | "draw";
  winning_seat: SeatId | null;
  winning_line: string[];
  terminal_rationale?: ThreeMarksOutcomeRationale | null;
  private_view_status: string;
  hidden_fields: string[];
  replay_step_index: number | null;
};

export type ColumnFourCellView = {
  cell: string;
  row: number;
  column: number;
  occupancy: "empty" | "occupied";
  owner: SeatId | null;
  piece_token_key: string | null;
  piece_shape_label: string | null;
};

export type ColumnFourColumnView = {
  column: string;
  column_id: string;
  label: string;
  is_full: boolean;
  legal_action_segment: string | null;
  landing_preview: string | null;
};

export type ColumnFourLegalTargetView = {
  column: string;
  action_segment: string;
  label: string;
  accessibility_label: string;
  freshness_token: number;
  landing_preview: string;
};

export type ColumnFourPublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "column_four";
  display_name: string;
  variant_id: string;
  rules_version_label: string;
  board_rows: number;
  board_columns: number;
  cells: ColumnFourCellView[];
  columns: ColumnFourColumnView[];
  active_seat: SeatId | null;
  ply_count: number;
  status_label: string;
  freshness_token: number;
  legal_targets: ColumnFourLegalTargetView[];
  terminal_kind: "non_terminal" | "win" | "draw";
  winning_seat: SeatId | null;
  winning_line: string[];
  terminal_rationale?: ColumnFourOutcomeRationale | null;
  private_view_status: string;
  hidden_fields: string[];
  replay_step_index: number | null;
};

export type DirectionalFlipCellView = {
  cell: string;
  cell_id: string;
  row: number;
  column: number;
  occupancy: "empty" | "occupied";
  owner: SeatId | null;
  disc_token_key: string | null;
  disc_shape_label: string | null;
  disc_pattern_label: string | null;
};

export type DirectionalFlipScoreView = {
  seat_0: number;
  seat_1: number;
};

export type DirectionalFlipDirectionGroupView = {
  direction: string;
  cells: string[];
  cell_ids: string[];
};

export type DirectionalFlipPreviewView = {
  preview_id: string;
  target_cell: string;
  target_cell_id: string;
  row: number;
  column: number;
  ordered_flip_cells: string[];
  ordered_flip_cell_ids: string[];
  direction_groups: DirectionalFlipDirectionGroupView[];
  explanation: string;
};

export type DirectionalFlipLegalTargetView = {
  action_kind: "placement" | "forced_pass" | string;
  action_segment: string;
  label: string;
  accessibility_label: string;
  freshness_token: number;
  cell: string | null;
  preview: DirectionalFlipPreviewView | null;
  reason_code: string | null;
  explanation: string;
};

export type DirectionalFlipUiMetadata = {
  board_label: string;
  row_count: number;
  column_count: number;
  first_disc_token_key: string;
  first_disc_shape_label: string;
  first_disc_pattern_label: string;
  second_disc_token_key: string;
  second_disc_shape_label: string;
  second_disc_pattern_label: string;
  legal_target_shape_label: string;
  forced_pass_label: string;
};

export type DirectionalFlipPublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "directional_flip";
  display_name: string;
  variant_id: string;
  rules_version_label: string;
  board_rows: number;
  board_columns: number;
  cells: DirectionalFlipCellView[];
  active_seat: SeatId | null;
  ply_count: number;
  status_label: string;
  freshness_token: number;
  score: DirectionalFlipScoreView;
  legal_targets: DirectionalFlipLegalTargetView[];
  terminal_kind: "non_terminal" | "win" | "draw";
  winning_seat: SeatId | null;
  final_score: DirectionalFlipScoreView | null;
  terminal_rationale?: DirectionalFlipOutcomeRationale | null;
  private_view_status: string;
  hidden_fields: string[];
  ui: DirectionalFlipUiMetadata;
  last_action_summary: string | null;
  bot_rationale: string | null;
  replay_step_index: number | null;
};

export type DraughtsLiteCellView = {
  cell: string;
  cell_id: string;
  row: number;
  column: number;
  playable: boolean;
  presentation_token: string;
  accessibility_label: string;
  occupancy: "empty" | "occupied";
  owner: SeatId | null;
  piece_id: string | null;
  piece_kind: "man" | "crown" | null;
  piece_token_key: string | null;
  piece_shape_label: string | null;
  piece_pattern_label: string | null;
  piece_label: string | null;
  piece_accessibility_label: string | null;
};

export type DraughtsLiteUiMetadata = {
  board_label: string;
  row_count: number;
  column_count: number;
  playable_cell_token: string;
  non_playable_cell_token: string;
  first_man_token_key: string;
  first_man_shape_label: string;
  first_crown_token_key: string;
  first_crown_shape_label: string;
  second_man_token_key: string;
  second_man_shape_label: string;
  second_crown_token_key: string;
  second_crown_shape_label: string;
};

export type DraughtsLitePublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "draughts_lite";
  display_name: string;
  variant_id: "draughts_lite_standard";
  rules_version_label: string;
  board_rows: number;
  board_columns: number;
  cells: DraughtsLiteCellView[];
  active_seat: SeatId | null;
  ply_count: number;
  command_count: number;
  status_label: string;
  freshness_token: number;
  terminal_kind: "non_terminal" | "win";
  winning_seat: SeatId | null;
  terminal_rationale?: DraughtsLiteOutcomeRationale | null;
  private_view_status: string;
  hidden_fields: string[];
  ui: DraughtsLiteUiMetadata;
  replay_step_index: number | null;
};

export type HighCardDuelCardView = {
  card_id: `hcd:r${string}`;
  rank: number;
  sigil: string;
  accessibility_label: string;
};

export type HighCardDuelCommitmentView =
  | {
      seat: SeatId;
      status: "empty" | "face_down";
      card: null;
      accessibility_label: string;
    }
  | {
      seat: SeatId;
      status: "own_card";
      card: HighCardDuelCardView;
      accessibility_label: string;
    };

export type HighCardDuelPrivateView =
  | {
      status: "observer";
      hand: [];
      own_commitment: null;
    }
  | {
      status: "seat";
      seat: SeatId;
      hand: HighCardDuelCardView[];
      own_commitment: HighCardDuelCardView | null;
    };

export type HighCardDuelUiMetadata = {
  table_label: string;
  card_back_token: string;
  own_card_token: string;
  revealed_card_token: string;
  empty_commitment_token: string;
  face_down_commitment_token: string;
  commit_action_label: string;
  observer_disabled_reason: string;
};

export type HighCardDuelPublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "high_card_duel";
  display_name: string;
  variant_id: "high_card_duel_standard";
  rules_version_label: string;
  round_number: number;
  round_limit: number;
  phase: "lead_commit" | "reply_commit" | "revealed" | "terminal";
  active_seat: SeatId | null;
  lead_seat: SeatId | null;
  reply_seat: SeatId | null;
  score: { seat_0: number; seat_1: number };
  hand_counts: { seat_0: number; seat_1: number };
  deck_count: number;
  commitments: {
    seat_0: HighCardDuelCommitmentView;
    seat_1: HighCardDuelCommitmentView;
  };
  revealed_cards: Array<{
    round_number: number;
    seat_0_card: HighCardDuelCardView;
    seat_1_card: HighCardDuelCardView;
    winner: SeatId | null;
  }>;
  terminal_kind: "non_terminal" | "win" | "draw";
  winning_seat: SeatId | null;
  terminal_rationale?: HighCardDuelOutcomeRationale | null;
  freshness_token: number;
  private_view: HighCardDuelPrivateView;
  ui: HighCardDuelUiMetadata;
};

export type TokenBazaarResourceCounts = {
  amber: number;
  jade: number;
  iron: number;
};

export type TokenBazaarInventoryView = {
  seat: SeatId;
  resources: TokenBazaarResourceCounts;
};

export type TokenBazaarContractView = {
  contract_id: string;
  label: string;
  cost: TokenBazaarResourceCounts;
  points: number;
  accessibility_label: string;
};

export type TokenBazaarMarketSlotView = {
  slot: string;
  slot_id: string;
  contract: TokenBazaarContractView | null;
  is_empty: boolean;
  accessibility_label: string;
};

export type TokenBazaarLegalActionView = {
  action_segment: string;
  label: string;
  accessibility_label: string;
  metadata: Array<{ key: string; value: string }>;
  freshness_token: number;
};

export type TokenBazaarTerminalView =
  | { terminal: false; winner: null; draw: false }
  | { terminal: true; winner: SeatId; draw: false }
  | { terminal: true; winner: null; draw: true };

export type TokenBazaarUiMetadata = {
  table_label: string;
  supply_label: string;
  inventory_label: string;
  market_label: string;
  score_label: string;
  turn_counter_label: string;
  reduced_motion_token: string;
};

export type TokenBazaarPublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "token_bazaar";
  display_name: string;
  variant_id: "token_bazaar_standard";
  rules_version_label: string;
  supply: TokenBazaarResourceCounts;
  inventories: [TokenBazaarInventoryView, TokenBazaarInventoryView];
  scores: { seat_0: number; seat_1: number };
  turns_taken: { seat_0: number; seat_1: number; turns_per_seat: number };
  active_seat: SeatId | null;
  market_slots: TokenBazaarMarketSlotView[];
  queue_remaining: number;
  fulfilled: { seat_0: string[]; seat_1: string[] };
  legal_actions: TokenBazaarLegalActionView[];
  terminal: TokenBazaarTerminalView;
  terminal_rationale?: TokenBazaarOutcomeRationale | null;
  freshness_token: number;
  recent_effects: Array<{ kind: string; summary: string }>;
  private_view_status: string;
  hidden_fields: string[];
  ui: TokenBazaarUiMetadata;
};

export type SecretDraftItemView = {
  item_id: string;
  label: string;
  thread: string;
  value: number;
  accessibility_label: string;
};

export type SecretDraftCommitmentView = {
  seat: SeatId;
  committed: boolean;
  status: "waiting" | "committed" | string;
  accessibility_label: string;
};

export type SecretDraftTerminalView =
  | { terminal: false; winner: null; draw: false }
  | { terminal: true; winner: SeatId; draw: false }
  | { terminal: true; winner: null; draw: true };

export type SecretDraftPrivateView =
  | { status: "observer"; own_committed: false; waiting_copy: "" }
  | { status: "seat"; seat: SeatId; own_committed: boolean; waiting_copy: string };

export type SecretDraftUiMetadata = {
  game_id: "secret_draft";
  display_name: "Veiled Draft";
  table_label: string;
  visible_pool_label: string;
  drafted_label: string;
  pending_label: string;
  score_label: string;
  reveal_group_token: string;
  reduced_motion_token: string;
};

export type SecretDraftPublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "secret_draft";
  display_name: string;
  variant_id: "secret_draft_standard";
  rules_version_label: string;
  round_number: number;
  round_limit: number;
  phase: "commit" | "terminal" | string;
  active_seat: SeatId | null;
  priority_seat: SeatId;
  visible_pool: SecretDraftItemView[];
  drafted: { seat_0: SecretDraftItemView[]; seat_1: SecretDraftItemView[] };
  commitments: { seat_0: SecretDraftCommitmentView; seat_1: SecretDraftCommitmentView; copy: string };
  scores: { seat_0: number; seat_1: number };
  revealed_history: Array<{
    round_number: number;
    seat_0_choice: SecretDraftItemView;
    seat_1_choice: SecretDraftItemView;
    seat_0_award: SecretDraftItemView;
    seat_1_award: SecretDraftItemView;
    priority_seat: SeatId;
    contested: boolean;
  }>;
  terminal: SecretDraftTerminalView;
  terminal_rationale?: SecretDraftOutcomeRationale | null;
  freshness_token: number;
  private_view: SecretDraftPrivateView;
  ui: SecretDraftUiMetadata;
};

export type PokerLiteCardView = {
  card_id: string;
  rank: "low" | "middle" | "high" | string;
  rank_value: number;
  copy: string;
  label: string;
  accessibility_label: string;
};

export type PokerLiteRoundView = {
  round_index: number;
  round_unit: number;
  outstanding_actor: SeatId | null;
  outstanding_amount: number;
  lift_cap_remaining: number;
};

export type PokerLiteCenterView =
  | { status: "hidden" | string; card: null }
  | { status: "revealed"; card: PokerLiteCardView };

export type PokerLiteShowdownView = {
  seat_0_private: PokerLiteCardView;
  seat_1_private: PokerLiteCardView;
  center: PokerLiteCardView;
  winner: SeatId | null;
};

export type PokerLiteTerminalView =
  | { terminal: false; kind: "non_terminal"; winner: null; draw: false }
  | { terminal: true; kind: "yield_win"; winner: SeatId; loser: SeatId; draw: false; shared_pool: number }
  | { terminal: true; kind: "showdown_win"; winner: SeatId; draw: false; shared_pool: number }
  | { terminal: true; kind: "split"; winner: null; draw: true; shared_pool: number; each: number };

export type PokerLitePrivateView =
  | { status: "observer"; own_private: null; own_strength_bucket: null }
  | { status: "seat"; seat: SeatId; own_private: PokerLiteCardView | null; own_strength_bucket: string | null };

export type PokerLiteUiMetadata = {
  game_id: "poker_lite";
  display_name: "Crest Ledger";
  surface_label: string;
  shared_pool_label: string;
  hidden_center_label: string;
  hidden_private_label: string;
  hold_label: string;
  press_label: string;
  lift_label: string;
  match_label: string;
  yield_label: string;
  reduced_motion_note: string;
};

export type PokerLitePublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "poker_lite";
  display_name: string;
  variant_id: "poker_lite_standard";
  rules_version_label: string;
  phase: "pledge_round_1" | "pledge_round_2" | "terminal" | string;
  active_seat: SeatId | null;
  shared_pool: number;
  contributions: { seat_0: number; seat_1: number };
  round: PokerLiteRoundView;
  private_counts: { seat_0: number; seat_1: number };
  center: PokerLiteCenterView;
  showdown: PokerLiteShowdownView | null;
  terminal: PokerLiteTerminalView;
  terminal_rationale?: PokerLiteOutcomeRationale | null;
  freshness_token: number;
  private_view: PokerLitePrivateView;
  ui: PokerLiteUiMetadata;
};

export type RiverLedgerCardView = {
  card_id: string;
  rank: string;
  rank_value: number;
  suit: string;
  label: string;
  accessibility_label: string;
};

export type RiverLedgerSeatView = {
  seat: RiverLedgerSeatId;
  status: "live" | "folded" | "showdown_eligible" | string;
  street_contribution: number;
  total_contribution: number;
  hidden_hole_count: number;
};

export type RiverLedgerShowdownStrength = {
  category: string;
  tie_break_vector: number[];
  best_five: RiverLedgerCardView[];
  category_ladder_position: RiverLedgerCategoryLadderPosition;
  result_label: string;
  hand_name: string;
  rank_explanation: string;
  comparison_note: string;
  best_five_accessibility_label: string;
};

export type RiverLedgerCategoryLadderPosition = {
  position: number;
  total: number;
  description: string;
};

export type RiverLedgerOutcomeStanding = OutcomeRationaleStanding & {
  seat: RiverLedgerSeatId;
  strength?: RiverLedgerShowdownStrength | null;
};

export type RiverLedgerOutcomeRationale = OutcomeRationalePayload & {
  headline?: string | null;
  decisive_comparison?: string | null;
  comparison_basis?: string | null;
  final_standing?: RiverLedgerOutcomeStanding[];
};

export type RiverLedgerTerminalView =
  | {
      kind: "non_terminal";
      terminal: false;
      winners: [];
      pot_total: 0;
      allocations: [];
      explanations: [];
    }
  | {
      kind: "last_live_hand" | "showdown" | string;
      terminal: true;
      winners: RiverLedgerSeatId[];
      pot_total: number;
      allocations: Array<{ seat: RiverLedgerSeatId; amount: number }>;
      explanations: string[];
    };

export type RiverLedgerPrivateView =
  | { status: "observer"; seat: null; hole_cards: [] }
  | { status: "seat"; seat: RiverLedgerSeatId; hole_cards: RiverLedgerCardView[] };

export type RiverLedgerUiMetadata = {
  game_id: "river_ledger";
  display_name: "River Ledger";
  surface_label: string;
  viewer_modes: string[];
  min_seats: number;
  default_seats: number;
  max_seats: number;
  seat_metadata_label: string;
  action_hint_label: string;
  outcome_explanation_label: string;
  contribution_label: string;
  board_label: string;
  hidden_hole_label: string;
  reduced_motion_note: string;
  hand_rankings: RiverLedgerHandRankingMetadata[];
};

export type RiverLedgerHandRankingMetadata = {
  category: string;
  label: string;
  definition: string;
};

export type RiverLedgerPublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "river_ledger";
  display_name: string;
  variant_id: "river_ledger_standard";
  rules_version_label: string;
  phase: "setup" | "preflop" | "flop" | "turn" | "river" | "showdown" | "terminal" | string;
  active_seat: RiverLedgerSeatId | null;
  button: RiverLedgerSeatId;
  small_blind: RiverLedgerSeatId;
  big_blind: RiverLedgerSeatId;
  pot_total: number;
  seats: RiverLedgerSeatView[];
  board: RiverLedgerCardView[];
  terminal: RiverLedgerTerminalView;
  terminal_rationale?: RiverLedgerOutcomeRationale | null;
  freshness_token: number;
  private_view: RiverLedgerPrivateView;
  ui: RiverLedgerUiMetadata;
};

export type PlainTricksCardView = {
  card_id: string;
  suit: string;
  rank: string;
  rank_value: number;
  label: string;
  accessibility_label: string;
};

export type PlainTricksPlayedCardView = {
  seat: SeatId;
  card: PlainTricksCardView;
};

export type PlainTricksCompletedTrickView = {
  round_index: number;
  trick_index: number;
  leader: SeatId;
  plays: PlainTricksPlayedCardView[];
  winner: SeatId;
  trick_counts_after: { seat_0: number; seat_1: number };
};

export type PlainTricksTerminalView =
  | { kind: "non_terminal"; winner: null; draw: false }
  | { kind: "trick_win"; winner: SeatId; draw: false; totals: { seat_0: number; seat_1: number } }
  | { kind: "split"; winner: null; draw: true; each: number; totals: { seat_0: number; seat_1: number } };

export type PlainTricksPrivateView =
  | { status: "observer"; own_hand: [] }
  | { status: "seat"; seat: SeatId; own_hand: PlainTricksCardView[] };

export type PlainTricksPublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "plain_tricks";
  display_name: string;
  variant_id: "plain_tricks_standard";
  rules_version_label: string;
  phase: string;
  active_seat: SeatId | null;
  round_index: number;
  trick_index: number;
  round_leader: SeatId;
  current_leader: SeatId;
  hand_counts: { seat_0: number; seat_1: number };
  current_trick: { led_suit: string | null; plays: PlainTricksPlayedCardView[] };
  trick_history: PlainTricksCompletedTrickView[];
  round_trick_counts: { seat_0: number; seat_1: number };
  total_trick_counts: { seat_0: number; seat_1: number };
  terminal: PlainTricksTerminalView;
  terminal_rationale?: PlainTricksOutcomeRationale | null;
  freshness_token: number;
  private_view: PlainTricksPrivateView;
  ui: {
    game_id: "plain_tricks";
    display_name: "Plain Tricks";
    table_label: string;
    own_hand_label: string;
    opponent_hand_label: string;
    current_trick_label: string;
    trick_history_label: string;
    score_label: string;
    play_action_label: string;
    observer_disabled_reason: string;
    reduced_motion_note: string;
    rules_summary: string[];
  };
};

export type MaskedClaimsOutcomeRationale = OutcomeRationalePayload;

export type MaskedClaimsMaskView = {
  tile_id: string;
  grade: string;
  label: string;
  accessibility_label: string;
};

export type MaskedClaimsVeiledClaimView = {
  declared_grade: string;
  declared_label: string;
};

export type MaskedClaimsExposedMaskView = {
  tile_id: string;
  actual_grade: string;
  declared_grade: string;
  claimant: SeatId;
  challenger: SeatId;
};

export type MaskedClaimsTerminalView =
  | { kind: "non_terminal"; winner: null; draw: false }
  | { kind: "score_win"; winner: SeatId; draw: false; scores: { seat_0: number; seat_1: number } }
  | { kind: "tiebreak_win"; winner: SeatId; draw: false; tiebreak: string; scores: { seat_0: number; seat_1: number } }
  | { kind: "draw"; winner: null; draw: true; scores: { seat_0: number; seat_1: number } };

export type MaskedClaimsPrivateView =
  | { status: "observer"; own_hand: [] }
  | { status: "seat"; seat: SeatId; own_hand: MaskedClaimsMaskView[] };

export type MaskedClaimsPublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "masked_claims";
  display_name: "Masked Claims";
  variant_id: "masked_claims_standard";
  rules_version_label: "masked-claims-rules-v1";
  phase: string;
  active_seat: SeatId | null;
  turn_index: number;
  claimant: SeatId;
  hand_counts: { seat_0: number; seat_1: number };
  pedestal: { claimant: SeatId; declared_grade: string; declared_label: string } | null;
  veiled_gallery: [MaskedClaimsVeiledClaimView[], MaskedClaimsVeiledClaimView[]];
  exposed_rows: [MaskedClaimsExposedMaskView[], MaskedClaimsExposedMaskView[]];
  scores: { seat_0: number; seat_1: number };
  counters: Array<{ exposed_lies: number; successful_challenges: number; challenges_declared: number }>;
  terminal: MaskedClaimsTerminalView;
  terminal_rationale?: MaskedClaimsOutcomeRationale | null;
  freshness_token: number;
  private_view: MaskedClaimsPrivateView;
  ui: {
    game_id: "masked_claims";
    variant_id: "masked_claims_standard";
    display_name: "Masked Claims";
    grade_labels: string[];
    claim_preview_template: string;
    reaction_prompt_template: string;
  };
};

export type FloodWatchOutcomeRationale = OutcomeRationalePayload;

export type FloodWatchRoleView = {
  seat: SeatId;
  role: string;
  label: string;
};

export type FloodWatchPhaseView =
  | { kind: "action"; budget_remaining: number }
  | { kind: "terminal"; budget_remaining: 0 };

export type FloodWatchDistrictView = {
  district: string;
  label: string;
  flood_level: number;
  levees: number;
};

export type FloodWatchRemainingComposition = {
  downpours_per_district: Array<{ district: string; count: number }>;
  surges_per_district: Array<{ district: string; count: number }>;
  reprieves: number;
};

export type FloodWatchTerminalSummary = {
  rule_id: string;
  public_summary: string;
  drawn_card_count: number;
  surviving_levels: Array<{ district: string; count: number }>;
};

export type FloodWatchTerminalView =
  | { kind: "non_terminal"; outcome: null; summary: null }
  | { kind: "complete"; outcome: "won" | "lost" | string; summary: FloodWatchTerminalSummary };

export type FloodWatchUiMetadata = {
  display_name: "Flood Watch" | string;
  event_deck_label: string;
  forecast_label: string;
  drawn_label: string;
  face_down_label: string;
  face_down_summary: string;
  reduced_motion_token: string;
};

export type FloodWatchPublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "flood_watch";
  display_name: "Flood Watch";
  variant_id: "flood_watch_standard" | "flood_watch_deluge" | string;
  rules_version_label: "flood-watch-rules-v1" | string;
  seats: SeatId[];
  roles: FloodWatchRoleView[];
  turn_number: number;
  active_seat: SeatId;
  phase: FloodWatchPhaseView;
  districts: FloodWatchDistrictView[];
  drawn_cards: CardFaceView[];
  forecast: CardFaceView | null;
  remaining_composition: FloodWatchRemainingComposition;
  undrawn_count: number;
  terminal: FloodWatchTerminalView;
  terminal_rationale?: FloodWatchOutcomeRationale | null;
  freshness_token: number;
  ui: FloodWatchUiMetadata;
};

export type FrontierControlOutcomeRationale = OutcomeRationalePayload;

export type FrontierControlFactionId = "faction_garrison" | "faction_prospectors" | string;

export type FrontierControlFactionView = {
  seat: SeatId;
  faction: FrontierControlFactionId;
  label: string;
};

export type FrontierControlPhaseView =
  | { kind: "action"; budget_remaining: number }
  | { kind: "terminal" };

export type FrontierControlSiteView = {
  site: string;
  label: string;
  guards: number;
  crews: number;
  stake: boolean;
  fort: boolean;
  stake_value: number;
  supplied: boolean | null;
};

export type FrontierControlScoreView = {
  garrison: number;
  prospectors: number;
};

export type FrontierControlTerminalView =
  | { kind: "non_terminal"; winner: null }
  | {
      kind: "winner";
      winner: FrontierControlFactionId;
      scores: FrontierControlScoreView;
      garrison_tiebreak: boolean;
      summary: string;
    };

export type FrontierControlPublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "frontier_control";
  display_name: string;
  variant_id: "frontier_control_standard" | "frontier_control_highlands" | string;
  rules_version_label: "frontier-control-rules-v1" | string;
  seats: SeatId[];
  factions: FrontierControlFactionView[];
  round_number: number;
  active_faction: FrontierControlFactionId;
  active_seat: SeatId | null;
  phase: FrontierControlPhaseView;
  sites: FrontierControlSiteView[];
  scores: FrontierControlScoreView;
  terminal: FrontierControlTerminalView;
  terminal_rationale?: FrontierControlOutcomeRationale | null;
  freshness_token: number;
};

export type EventFrontierOutcomeRationale = OutcomeRationalePayload;

export type EventFrontierFactionId = "faction_charter" | "faction_freeholders" | string;

export type EventFrontierSiteView = {
  site: string;
  label: string;
  agents: number;
  settlers: number;
  depot: boolean;
  cache_count: number;
};

export type EventFrontierTerminalView =
  | { kind: "non_terminal"; winner: null }
  | {
      kind: "winner";
      winner: EventFrontierFactionId;
      victory_type: "charter_instant" | "freeholder_instant" | "final_fallback" | string;
      scores: { charter: number; freeholders: number };
      decisive_rule: string;
    };

export type CardFaceView = {
  id: string;
  label: string;
  summary: string;
  details?: string | null;
  family: string;
  accessibility_label: string;
};

export type EventFrontierUiMetadata = {
  table_label: string;
  event_deck_label: string;
  current_card_label: string;
  next_card_label: string;
  discard_label: string;
  face_down_label: string;
  face_down_summary: string;
  reduced_motion_token: string;
  seat_labels: SeatDisplayLabel[];
  faction_labels: FactionDisplayLabel[];
  action_affordance_templates: Array<{ id: string; text: string }>;
};

export type EventFrontierPublicView = {
  schema_version: number;
  rules_version: number;
  game_id: "event_frontier";
  display_name: string;
  variant_id: "event_frontier_standard" | "event_frontier_hard_winter" | "event_frontier_land_rush" | string;
  rules_version_label: "event-frontier-rules-v1" | string;
  seats: SeatId[];
  factions: EventFrontierFactionId[];
  active_seat: SeatId | null;
  sites: EventFrontierSiteView[];
  adjacency: Array<{ site: string; neighbors: string[] }>;
  resources: { funds: number; provisions: number };
  scores: { charter: number; freeholders: number };
  eligibility: Array<{ faction: EventFrontierFactionId; eligible: "eligible" | "ineligible" | string }>;
  current_card: CardFaceView | null;
  next_public_card: CardFaceView | null;
  discard: CardFaceView[];
  active_edicts: string[];
  epoch: number;
  reckoning_count: number;
  victory_distance: { charter_sites_needed: number; freeholder_caches_needed: number };
  terminal: EventFrontierTerminalView;
  terminal_rationale?: EventFrontierOutcomeRationale | null;
  freshness_token: number;
  ui: EventFrontierUiMetadata;
};

export type PublicView =
  | RacePublicView
  | ThreeMarksPublicView
  | ColumnFourPublicView
  | DirectionalFlipPublicView
  | DraughtsLitePublicView
  | HighCardDuelPublicView
  | TokenBazaarPublicView
  | SecretDraftPublicView
  | PokerLitePublicView
  | RiverLedgerPublicView
  | PlainTricksPublicView
  | MaskedClaimsPublicView
  | FloodWatchPublicView
  | FrontierControlPublicView
  | EventFrontierPublicView;

export type ActionChoice = {
  segment: string;
  label: string;
  accessibility_label: string;
  metadata?: Array<{ key: string; value: string }>;
  tags?: string[];
  next?: { choices: ActionChoice[] } | null;
};

export type ActionTree = {
  freshness_token: number;
  choices: ActionChoice[];
};

export type EffectEntry = {
  cursor: number;
  effect: {
    payload: {
      type: string;
      [key: string]: unknown;
      actor?: string;
      next_actor?: string;
      winner?: string;
      from?: number;
      to?: number;
      amount?: number;
    };
  };
};

export type BotTurnResult = {
  view: PublicView;
  policy_id?: string;
  policy_version?: number;
  rationale?: string;
};

export type ApiError = {
  code: string;
  message: string;
};

export type ReplayCommand = {
  index: number;
  actor_seat: string;
  action_path: string[];
  freshness_token: string;
  expect: "applied";
};

export type ReplayDocument = {
  schema_version: number;
  trace_id: string;
  fixture_kind: string;
  purpose: string;
  note: string;
  migration_update_note: string;
  game_id: string;
  rules_version: string;
  engine_version: string;
  data_version: string;
  seed: number;
  variant: string;
  options: Record<string, never>;
  seats: { seat_id: string; player_id: string }[];
  commands: ReplayCommand[];
  checkpoints: { id: string; after_command_index: number }[];
  expected_state_hashes: { final: number };
  expected_effect_hashes: { final: number };
  expected_action_tree_hashes: { final: number };
  expected_public_view_hashes: { all: number };
  expected_private_view_hashes?: { not_applicable: string };
  expected_replay_hashes?: { final: number };
  expected_diagnostic_hashes?: { final: number } | null;
  expected_public_export_hashes?: { final: number };
  expected_outcome: { terminal: boolean; winner: string | null; kind?: string; draw?: boolean };
  expected_terminal_state: { terminal: boolean; winner: string | null; kind?: string; draw?: boolean };
  not_applicable: Record<string, string>;
};

export type PublicObserverReplayExport = {
  schema_version: number;
  export_class: "public_observer_projection_v1";
  viewer: "observer";
  game_id: "high_card_duel";
  rules_version: string;
  variant: "high_card_duel_standard";
  steps: Array<{
    step_index: number;
    public_view_summary: string;
    public_effects: string[];
    redacted_command_summary: string;
    terminal: boolean;
  }>;
};

export type SecretDraftPublicReplayExport = {
  schema_version: number;
  export_class: "viewer_scoped_observation_v1";
  viewer: "observer";
  game_id: "secret_draft";
  rules_version: string;
  variant: "secret_draft_standard";
  steps: Array<{
    step_index: number;
    public_view_summary: string;
    public_effects: string[];
    redacted_command_summary: string;
    terminal: boolean;
  }>;
};

export type MaskedClaimsPublicReplayExport = {
  schema_version: number;
  export_class: "viewer_scoped_observation";
  viewer: "observer";
  game_id: "masked_claims";
  rules_version: string;
  variant: "masked_claims_standard";
  steps: Array<{
    step_index: number;
    public_view_summary: string;
    public_effects: string[];
    redacted_command_summary: string;
    terminal: boolean;
  }>;
};

export type ReplayExportDocument =
  | ReplayDocument
  | PublicObserverReplayExport
  | SecretDraftPublicReplayExport
  | MaskedClaimsPublicReplayExport;

export type ReplayImportSummary = {
  replay_id: string;
  game_id: string;
  command_count: number;
  final_view: PublicView | null;
  effect_count: number;
  public_export?: boolean;
  viewer?: "observer";
  step_count?: number;
};

export type ReplayStep = {
  replay_id: string;
  cursor: number;
  command_count?: number;
  total_steps?: number;
  done?: boolean;
  public_export?: boolean;
  viewer?: "observer";
  view: PublicView | null;
  effects: EffectEntry["effect"][];
  public_effects?: string[];
  redacted_command_summary?: string;
};

type EncodedArg = {
  ptr: number;
  len: number;
};

export class RulepathApi {
  private readonly encoder = new TextEncoder();
  private readonly decoder = new TextDecoder();

  constructor(private readonly exports: WasmExports) {}

  version(): string {
    const ptr = this.exports.rulepath_placeholder_version_ptr();
    const len = this.exports.rulepath_placeholder_version_len();
    return this.read(ptr, len);
  }

  listGames(): GameCatalogEntry[] {
    return this.invokeJson<GameCatalogEntry[]>(() => this.exports.rulepath_list_games(), []);
  }

  featureReport(): FeatureReport {
    return this.invokeJson<FeatureReport>(() => this.exports.rulepath_feature_report(), []);
  }

  newMatch(gameId: string, seed: number, variantId?: string, seatCount?: number): MatchCreated {
    if (seatCount !== undefined && !variantId) {
      return this.invokeJson<MatchCreated>(
        (args) => this.exports.rulepath_new_match_with_seat_count(args[0].ptr, args[0].len, BigInt(seed), seatCount),
        [gameId],
      );
    }
    if (variantId) {
      return this.invokeJson<MatchCreated>(
        (args) =>
          this.exports.rulepath_new_match_with_variant(
            args[0].ptr,
            args[0].len,
            args[1].ptr,
            args[1].len,
            BigInt(seed),
          ),
        [gameId, variantId],
      );
    }
    return this.invokeJson<MatchCreated>(
      (args) => this.exports.rulepath_new_match(args[0].ptr, args[0].len, BigInt(seed)),
      [gameId],
    );
  }

  getView(matchId: string, viewerMode: ViewerMode = { kind: "observer" }): PublicView {
    const viewer = viewerModeArg(viewerMode);
    if (viewer === null) {
      return this.invokeJson<PublicView>((args) =>
        this.exports.rulepath_get_view_for_viewer(args[0].ptr, args[0].len, 0, 0),
      [matchId]);
    }
    return this.invokeJson<PublicView>((args) =>
      this.exports.rulepath_get_view_for_viewer(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
    [matchId, viewer]);
  }

  getActionTree(matchId: string, seat: ViewerSeatId, viewerMode: ViewerMode = { kind: "seat", seat }): ActionTree {
    const viewer = viewerModeArg(viewerMode);
    if (viewer === null) {
      return this.invokeJson<ActionTree>((args) =>
        this.exports.rulepath_get_action_tree_for_viewer(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 0, 0),
      [matchId, seat]);
    }
    return this.invokeJson<ActionTree>((args) =>
      this.exports.rulepath_get_action_tree_for_viewer(
        args[0].ptr,
        args[0].len,
        args[1].ptr,
        args[1].len,
        args[2].ptr,
        args[2].len,
      ),
    [matchId, seat, viewer]);
  }

  applyAction(matchId: string, seat: string, path: string, freshnessToken: number): PublicView {
    return this.applyActionPath(matchId, seat, [path], freshnessToken);
  }

  applyActionPath(matchId: string, seat: string, path: string[], freshnessToken: number): PublicView {
    const response = this.invokeJson<{ view: PublicView }>((args) =>
      this.exports.rulepath_apply_action(
        args[0].ptr,
        args[0].len,
        args[1].ptr,
        args[1].len,
        args[2].ptr,
        args[2].len,
        BigInt(freshnessToken),
      ),
    [matchId, seat, encodeActionPath(path)]);
    return response.view;
  }

  runBotTurn(matchId: string, seat: string, seed: number): BotTurnResult {
    return this.invokeJson<BotTurnResult>((args) =>
      this.exports.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, BigInt(seed)),
    [matchId, seat]);
  }

  getEffects(matchId: string, sinceCursor: number, viewerMode: ViewerMode = { kind: "observer" }): EffectEntry[] {
    const viewer = viewerModeArg(viewerMode);
    if (viewer === null) {
      return this.invokeJson<EffectEntry[]>((args) =>
        this.exports.rulepath_get_effects(args[0].ptr, args[0].len, BigInt(sinceCursor), 0, 0),
      [matchId]);
    }
    return this.invokeJson<EffectEntry[]>((args) =>
      this.exports.rulepath_get_effects(args[0].ptr, args[0].len, BigInt(sinceCursor), args[1].ptr, args[1].len),
    [matchId, viewer]);
  }

  exportReplay(matchId: string): ReplayExportDocument {
    return this.invokeJson<ReplayExportDocument>((args) =>
      this.exports.rulepath_export_replay(args[0].ptr, args[0].len),
    [matchId]);
  }

  importReplay(doc: string): ReplayImportSummary {
    return this.invokeJson<ReplayImportSummary>((args) =>
      this.exports.rulepath_import_replay(args[0].ptr, args[0].len),
    [doc]);
  }

  replayStep(replayId: string, cursor: number): ReplayStep {
    return this.invokeJson<ReplayStep>((args) =>
      this.exports.rulepath_replay_step(args[0].ptr, args[0].len, cursor),
    [replayId]);
  }

  replayReset(replayId: string): ReplayStep {
    return this.invokeJson<ReplayStep>((args) =>
      this.exports.rulepath_replay_reset(args[0].ptr, args[0].len),
    [replayId]);
  }

  private invokeJson<T>(call: (args: EncodedArg[]) => number, values: string[]): T {
    const args = values.map((value) => this.write(value));
    try {
      const status = call(args);
      const output = this.lastOutput();
      const parsed = JSON.parse(output) as T | ApiError;
      if (status !== 0) {
        throw parsed;
      }
      return parsed as T;
    } finally {
      for (const arg of args) {
        this.exports.rulepath_dealloc(arg.ptr, arg.len);
      }
    }
  }

  private write(value: string): EncodedArg {
    const bytes = this.encoder.encode(value);
    const ptr = this.exports.rulepath_alloc(bytes.length);
    new Uint8Array(this.exports.memory.buffer, ptr, bytes.length).set(bytes);
    return { ptr, len: bytes.length };
  }

  private lastOutput(): string {
    const ptr = this.exports.rulepath_last_output_ptr();
    const len = this.exports.rulepath_last_output_len();
    return this.read(ptr, len);
  }

  private read(ptr: number, len: number): string {
    return this.decoder.decode(new Uint8Array(this.exports.memory.buffer, ptr, len));
  }
}

function encodeActionPath(path: string[]): string {
  return path.map((segment) => encodeURIComponent(segment)).join(">");
}

function viewerModeArg(viewerMode: ViewerMode): ViewerSeatId | null {
  return viewerMode.kind === "observer" ? null : viewerMode.seat;
}

export async function loadApi(): Promise<RulepathApi> {
  const response = await fetch(wasmArtifactUrl());
  if (!response.ok) {
    throw new Error(`Unable to load wasm-api artifact: ${response.status}`);
  }

  const bytes = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes, {});
  return new RulepathApi(instance.exports as WasmExports);
}

function wasmArtifactUrl(): URL {
  const base = new URL(import.meta.env.BASE_URL, window.location.href);
  return new URL("wasm_api.wasm", base);
}
