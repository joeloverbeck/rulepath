import type {
  ActionTree,
  ApiError,
  EffectEntry,
  FeatureReport,
  GameCatalogEntry,
  PublicView,
  ReplayExportDocument,
  ReplayStep,
  RulepathApi,
  ViewerMode,
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
  document: ReplayExportDocument | null;
  cursor: number;
  step: ReplayStep | null;
};

export type ShellState = {
  mode: ShellMode;
  api: RulepathApi | null;
  version: string;
  catalog: GameCatalogEntry[];
  featureReport: FeatureReport | null;
  selectedGameId: string;
  setup: {
    seed: number;
    playMode: SetupPlayMode;
  };
  matchId: string | null;
  actorSeat: "seat_0" | "seat_1";
  viewerSeat: "seat_0" | "seat_1" | null;
  viewerMode: ViewerMode;
  view: PublicView | null;
  actionTree: ActionTree | null;
  pendingActionPath: string[];
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
  viewerMode: ViewerMode;
};

export type ShellAction =
  | { type: "wasmLoaded"; api: RulepathApi; version: string; catalog?: GameCatalogEntry[]; featureReport?: FeatureReport }
  | { type: "wasmLoadFailed"; message: string }
  | { type: "gameSelected"; gameId: string }
  | { type: "setupSeedChanged"; seed: number }
  | { type: "setupPlayModeChanged"; playMode: SetupPlayMode }
  | { type: "matchStarting" }
  | { type: "matchStarted"; matchId: string }
  | { type: "refreshed"; payload: RefreshPayload }
  | { type: "actionApplied"; staleToken: number }
  | { type: "pendingActionPathChanged"; path: string[] }
  | { type: "pendingActionPathCleared" }
  | { type: "staleDiagnostic"; diagnostic: ApiError }
  | { type: "diagnosticCleared" }
  | { type: "replayImported"; replayId: string; document: ReplayExportDocument | null; step: ReplayStep | null }
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
  featureReport: null,
  selectedGameId: "",
  setup: {
    seed: 1,
    playMode: "human_vs_bot",
  },
  matchId: null,
  actorSeat: "seat_0",
  viewerSeat: null,
  viewerMode: { kind: "seat", seat: "seat_0" },
  view: null,
  actionTree: null,
  pendingActionPath: [],
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
        featureReport: action.featureReport ?? state.featureReport,
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
        pendingActionPath: [],
        effects: [],
        effectCursor: 0,
        viewerMode: viewerModeForPlayMode(state.setup.playMode),
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
        viewerMode: viewerModeForPlayMode(action.playMode),
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
        pendingActionPath: [],
        effects: [],
        effectCursor: 0,
        viewerMode: viewerModeForPlayMode(state.setup.playMode),
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
        pendingActionPath: [],
        effects: [...state.effects, ...action.payload.effects].slice(-12),
        effectCursor: action.payload.effectCursor,
        viewerMode: action.payload.viewerMode,
        pendingOperation: null,
      };
    case "actionApplied":
      return {
        ...state,
        staleToken: action.staleToken,
        diagnostic: null,
        pendingActionPath: [],
        pendingOperation: "applyAction",
      };
    case "pendingActionPathChanged":
      return {
        ...state,
        pendingActionPath: action.path,
      };
    case "pendingActionPathCleared":
      return {
        ...state,
        pendingActionPath: [],
      };
    case "staleDiagnostic":
      return {
        ...state,
        diagnostic: action.diagnostic,
        pendingActionPath: [],
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
        pendingActionPath: [],
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

function viewerModeForPlayMode(playMode: SetupPlayMode): ViewerMode {
  return playMode === "bot_vs_bot" ? { kind: "observer" } : { kind: "seat", seat: "seat_0" };
}
