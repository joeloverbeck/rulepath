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

export type GameCatalogEntry = {
  game_id: string;
  display_name: string;
  rules_version: number;
  schema_version: number;
  variants?: string[];
  viewer_modes?: ViewerModeId[];
  hidden_information?: boolean;
  tags?: string[];
};

export type FeatureReport = {
  api_version: string;
  operations: string[];
  features: string[];
};

export type SeatId = "seat_0" | "seat_1";
export type ViewerModeId = "observer" | SeatId;
export type ViewerMode = { kind: "observer" } | { kind: "seat"; seat: SeatId };

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

export type PublicView =
  | RacePublicView
  | ThreeMarksPublicView
  | ColumnFourPublicView
  | DirectionalFlipPublicView
  | DraughtsLitePublicView
  | HighCardDuelPublicView
  | TokenBazaarPublicView
  | SecretDraftPublicView
  | PokerLitePublicView;

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

export type ReplayExportDocument = ReplayDocument | PublicObserverReplayExport | SecretDraftPublicReplayExport;

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

  newMatch(gameId: string, seed: number): MatchCreated {
    return this.invokeJson<MatchCreated>((args) =>
      this.exports.rulepath_new_match(args[0].ptr, args[0].len, BigInt(seed)),
    [gameId]);
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

  getActionTree(matchId: string, seat: SeatId, viewerMode: ViewerMode = { kind: "seat", seat }): ActionTree {
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
    [matchId, seat, path.join(">")]);
    return response.view;
  }

  runBotTurn(matchId: string, seat: string, seed: number): PublicView {
    const response = this.invokeJson<{ view: PublicView }>((args) =>
      this.exports.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, BigInt(seed)),
    [matchId, seat]);
    return response.view;
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

function viewerModeArg(viewerMode: ViewerMode): SeatId | null {
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
