import type {
  ActionTree,
  ApiError,
  EffectEntry,
  GameCatalogEntry,
  PublicView,
  ReplayDocument,
  ReplayStep,
  RulepathApi,
} from "../wasm/client";

export type ShellMode = "loading" | "ready" | "setup" | "play" | "replay" | "error";

export type PendingOperation =
  | "loadWasm"
  | "startMatch"
  | "applyAction"
  | "submitStale"
  | "botTurn"
  | "importReplay"
  | "stepReplay"
  | null;

export type SetupPlayMode = "human_vs_bot" | "hotseat" | "bot_vs_bot";

export type ReplaySessionState = {
  replayId: string;
  document: ReplayDocument | null;
  cursor: number;
  step: ReplayStep | null;
};

export type ShellState = {
  mode: ShellMode;
  api: RulepathApi | null;
  version: string;
  catalog: GameCatalogEntry[];
  selectedGameId: string;
  setup: {
    seed: number;
    playMode: SetupPlayMode;
  };
  matchId: string | null;
  actorSeat: "seat_0" | "seat_1";
  viewerSeat: "seat_0" | "seat_1" | null;
  view: PublicView | null;
  actionTree: ActionTree | null;
  effects: EffectEntry[];
  effectCursor: number;
  diagnostic: ApiError | null;
  staleToken: number | null;
  replay: ReplaySessionState | null;
  autoplay: {
    running: boolean;
  };
  devPanelOpen: boolean;
  reducedMotion: boolean;
  pendingOperation: PendingOperation;
};

export type RefreshPayload = {
  view: PublicView;
  actionTree: ActionTree;
  effects: EffectEntry[];
  effectCursor: number;
};

export type ShellAction =
  | { type: "wasmLoaded"; api: RulepathApi; version: string; catalog?: GameCatalogEntry[] }
  | { type: "wasmLoadFailed"; message: string }
  | { type: "gameSelected"; gameId: string }
  | { type: "setupSeedChanged"; seed: number }
  | { type: "setupPlayModeChanged"; playMode: SetupPlayMode }
  | { type: "matchStarting" }
  | { type: "matchStarted"; matchId: string }
  | { type: "refreshed"; payload: RefreshPayload }
  | { type: "actionApplied"; staleToken: number }
  | { type: "staleDiagnostic"; diagnostic: ApiError }
  | { type: "diagnosticCleared" }
  | { type: "replayImported"; replayId: string; document: ReplayDocument | null; step: ReplayStep | null }
  | { type: "replayStepped"; step: ReplayStep }
  | { type: "replayReset"; step: ReplayStep }
  | { type: "botTurnStarted" }
  | { type: "autoplayStarted" }
  | { type: "autoplayPaused" }
  | { type: "devPanelToggled" }
  | { type: "reducedMotionChanged"; reducedMotion: boolean }
  | { type: "pendingOperationChanged"; pendingOperation: PendingOperation };

export const initialShellState: ShellState = {
  mode: "loading",
  api: null,
  version: "Loading wasm-api...",
  catalog: [],
  selectedGameId: "",
  setup: {
    seed: 1,
    playMode: "human_vs_bot",
  },
  matchId: null,
  actorSeat: "seat_0",
  viewerSeat: null,
  view: null,
  actionTree: null,
  effects: [],
  effectCursor: 0,
  diagnostic: null,
  staleToken: null,
  replay: null,
  autoplay: {
    running: false,
  },
  devPanelOpen: false,
  reducedMotion: false,
  pendingOperation: "loadWasm",
};

export function shellReducer(state: ShellState, action: ShellAction): ShellState {
  switch (action.type) {
    case "wasmLoaded":
      const catalog = action.catalog ?? state.catalog;
      return {
        ...state,
        mode: catalog.length > 0 ? "setup" : "ready",
        api: action.api,
        version: action.version,
        catalog,
        selectedGameId: state.selectedGameId || catalog[0]?.game_id || "",
        pendingOperation: null,
      };
    case "wasmLoadFailed":
      return {
        ...state,
        mode: "error",
        version: action.message,
        diagnostic: { code: "wasm_load_failed", message: action.message },
        pendingOperation: null,
      };
    case "gameSelected":
      return {
        ...state,
        mode: "setup",
        selectedGameId: action.gameId,
        matchId: null,
        view: null,
        actionTree: null,
        effects: [],
        effectCursor: 0,
        diagnostic: null,
        staleToken: null,
        replay: null,
        autoplay: { running: false },
      };
    case "setupSeedChanged":
      return {
        ...state,
        setup: {
          ...state.setup,
          seed: action.seed,
        },
      };
    case "setupPlayModeChanged":
      return {
        ...state,
        setup: {
          ...state.setup,
          playMode: action.playMode,
        },
      };
    case "matchStarting":
      return {
        ...state,
        diagnostic: null,
        pendingOperation: "startMatch",
      };
    case "matchStarted":
      return {
        ...state,
        mode: "play",
        matchId: action.matchId,
        view: null,
        actionTree: null,
        effects: [],
        effectCursor: 0,
        diagnostic: null,
        staleToken: null,
        replay: null,
        autoplay: { running: false },
        pendingOperation: null,
      };
    case "refreshed":
      return {
        ...state,
        mode: "play",
        view: action.payload.view,
        actionTree: action.payload.actionTree,
        effects: [...state.effects, ...action.payload.effects].slice(-12),
        effectCursor: action.payload.effectCursor,
        pendingOperation: null,
      };
    case "actionApplied":
      return {
        ...state,
        staleToken: action.staleToken,
        diagnostic: null,
        pendingOperation: "applyAction",
      };
    case "staleDiagnostic":
      return {
        ...state,
        diagnostic: action.diagnostic,
        pendingOperation: null,
      };
    case "diagnosticCleared":
      return {
        ...state,
        diagnostic: null,
      };
    case "replayImported":
      return {
        ...state,
        mode: "replay",
        replay: {
          replayId: action.replayId,
          document: action.document,
          cursor: action.step?.cursor ?? 0,
          step: action.step,
        },
        autoplay: { running: false },
        pendingOperation: null,
      };
    case "replayStepped":
      return {
        ...state,
        mode: "replay",
        replay: state.replay
          ? {
              ...state.replay,
              cursor: action.step.cursor,
              step: action.step,
            }
          : state.replay,
        pendingOperation: null,
      };
    case "replayReset":
      return {
        ...state,
        mode: "replay",
        replay: state.replay
          ? {
              ...state.replay,
              cursor: 0,
              step: action.step,
            }
          : state.replay,
        pendingOperation: null,
      };
    case "botTurnStarted":
      return {
        ...state,
        pendingOperation: "botTurn",
      };
    case "autoplayStarted":
      return {
        ...state,
        autoplay: { running: true },
      };
    case "autoplayPaused":
      return {
        ...state,
        autoplay: { running: false },
        pendingOperation: state.pendingOperation === "botTurn" ? null : state.pendingOperation,
      };
    case "devPanelToggled":
      return {
        ...state,
        devPanelOpen: !state.devPanelOpen,
      };
    case "reducedMotionChanged":
      return {
        ...state,
        reducedMotion: action.reducedMotion,
      };
    case "pendingOperationChanged":
      return {
        ...state,
        pendingOperation: action.pendingOperation,
      };
    default:
      return state;
  }
}
