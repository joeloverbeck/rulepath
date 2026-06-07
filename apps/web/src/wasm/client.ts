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
  rulepath_get_action_tree: (
    matchPtr: number,
    matchLen: number,
    seatPtr: number,
    seatLen: number,
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
};

export type FeatureReport = {
  api_version: string;
  operations: string[];
  features: string[];
};

export type SeatId = "seat_0" | "seat_1";

export type RacePublicView = {
  counter: number;
  target: number;
  max_add: number;
  active_seat: SeatId;
  winner: SeatId | null;
  freshness_token: number;
};

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
  private_view_status: string;
  hidden_fields: string[];
  ui: DirectionalFlipUiMetadata;
  last_action_summary: string | null;
  bot_rationale: string | null;
  replay_step_index: number | null;
};

export type PublicView =
  | RacePublicView
  | ThreeMarksPublicView
  | ColumnFourPublicView
  | DirectionalFlipPublicView;

export type ActionChoice = {
  segment: string;
  label: string;
  accessibility_label: string;
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
  expected_private_view_hashes: { not_applicable: string };
  expected_replay_hashes?: { final: number };
  expected_outcome: { terminal: boolean; winner: string | null; kind?: string };
  expected_terminal_state: { terminal: boolean; winner: string | null; kind?: string };
  not_applicable: Record<string, string>;
};

export type ReplayImportSummary = {
  replay_id: string;
  game_id: string;
  command_count: number;
  final_view: PublicView;
  effect_count: number;
};

export type ReplayStep = {
  replay_id: string;
  cursor: number;
  command_count: number;
  done: boolean;
  view: PublicView;
  effects: EffectEntry["effect"][];
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

  getView(matchId: string): PublicView {
    return this.invokeJson<PublicView>((args) =>
      this.exports.rulepath_get_view(args[0].ptr, args[0].len),
    [matchId]);
  }

  getActionTree(matchId: string, seat: string): ActionTree {
    return this.invokeJson<ActionTree>((args) =>
      this.exports.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
    [matchId, seat]);
  }

  applyAction(matchId: string, seat: string, path: string, freshnessToken: number): PublicView {
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
    [matchId, seat, path]);
    return response.view;
  }

  runBotTurn(matchId: string, seat: string, seed: number): PublicView {
    const response = this.invokeJson<{ view: PublicView }>((args) =>
      this.exports.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, BigInt(seed)),
    [matchId, seat]);
    return response.view;
  }

  getEffects(matchId: string, sinceCursor: number): EffectEntry[] {
    return this.invokeJson<EffectEntry[]>((args) =>
      this.exports.rulepath_get_effects(args[0].ptr, args[0].len, BigInt(sinceCursor), 0, 0),
    [matchId]);
  }

  exportReplay(matchId: string): ReplayDocument {
    return this.invokeJson<ReplayDocument>((args) =>
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
