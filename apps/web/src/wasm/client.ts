type WasmExports = {
  memory: WebAssembly.Memory;
  rulepath_placeholder_version_ptr: () => number;
  rulepath_placeholder_version_len: () => number;
  rulepath_alloc: (len: number) => number;
  rulepath_dealloc: (ptr: number, len: number) => void;
  rulepath_last_output_ptr: () => number;
  rulepath_last_output_len: () => number;
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
};

export type MatchCreated = {
  match_id: string;
  game_id: string;
};

export type PublicView = {
  counter: number;
  target: number;
  max_add: number;
  active_seat: "seat_0" | "seat_1";
  winner: "seat_0" | "seat_1" | null;
  freshness_token: number;
};

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
  const response = await fetch("/wasm_api.wasm");
  if (!response.ok) {
    throw new Error(`Unable to load wasm-api artifact: ${response.status}`);
  }

  const bytes = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes, {});
  return new RulepathApi(instance.exports as WasmExports);
}
