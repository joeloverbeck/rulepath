import type {
  ActionTree,
  ApiError,
  BotDecisionPublicExplanation,
  BotTurnResult,
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

export type BotDecisionSummary = {
  policyId: string;
  policyVersion: number | null;
  rationale: string;
  publicExplanation: BotDecisionPublicExplanation | null;
};

export type RulesPanelStatus = "idle" | "loading" | "loaded" | "error";

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
    variantId: string | null;
    seatCount: number | null;
  };
  matchId: string | null;
  actorSeat: "seat_0" | "seat_1";
  viewerSeat: string | null;
  viewerMode: ViewerMode;
  view: PublicView | null;
  actionTree: ActionTree | null;
  pendingActionPath: string[];
  effects: EffectEntry[];
  effectCursor: number;
  diagnostic: ApiError | null;
  lastBotDecision: BotDecisionSummary | null;
  staleToken: number | null;
  replay: ReplaySessionState | null;
  autoplay: {
    running: boolean;
  };
  orchestration: {
    paused: boolean;
    rate: number;
  };
  rulesPanelOpen: boolean;
  rulesPanelGameId: string | null;
  rulesPanelStatus: RulesPanelStatus;
  rulesPanelMarkdown: string | null;
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
  | { type: "setupVariantChanged"; variantId: string }
  | { type: "setupSeatCountChanged"; seatCount: number }
  | { type: "viewerModeChanged"; viewerMode: ViewerMode }
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
  | { type: "botTurnCompleted"; result: BotTurnResult }
  | { type: "autoplayStarted" }
  | { type: "autoplayPaused" }
  | { type: "orchestrationPaused" }
  | { type: "orchestrationResumed" }
  | { type: "orchestrationRateChanged"; rate: number }
  | { type: "rulesPanelOpened"; gameId: string }
  | { type: "rulesPanelClosed" }
  | { type: "rulesPanelLoadStarted"; gameId: string }
  | { type: "rulesPanelLoaded"; gameId: string; markdown: string }
  | { type: "rulesPanelFailed"; gameId: string }
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
    variantId: null,
    seatCount: null,
  },
  matchId: null,
  actorSeat: "seat_0",
  viewerSeat: "seat_0",
  viewerMode: { kind: "seat", seat: "seat_0" },
  view: null,
  actionTree: null,
  pendingActionPath: [],
  effects: [],
  effectCursor: 0,
  diagnostic: null,
  lastBotDecision: null,
  staleToken: null,
  replay: null,
  autoplay: {
    running: false,
  },
  orchestration: {
    paused: false,
    rate: 1,
  },
  rulesPanelOpen: false,
  rulesPanelGameId: null,
  rulesPanelStatus: "idle",
  rulesPanelMarkdown: null,
  devPanelOpen: false,
  reducedMotion: false,
  pendingOperation: "loadWasm",
};

export function shellReducer(state: ShellState, action: ShellAction): ShellState {
  switch (action.type) {
    case "wasmLoaded": {
      const catalog = action.catalog ?? state.catalog;
      const selectedGameId = state.selectedGameId || catalog[0]?.game_id || "";
      const selectedGame = catalog.find((game) => game.game_id === selectedGameId) ?? null;
      return {
        ...state,
        mode: catalog.length > 0 ? "setup" : "ready",
        api: action.api,
        version: action.version,
        catalog,
        featureReport: action.featureReport ?? state.featureReport,
        selectedGameId,
        setup: {
          ...state.setup,
          variantId: state.setup.variantId ?? selectedGame?.variants?.[0]?.id ?? null,
          seatCount: resolveSeatCount(selectedGame, state.setup.seatCount),
        },
        pendingOperation: null,
      };
    }
    case "wasmLoadFailed":
      return {
        ...state,
        mode: "error",
        version: action.message,
        diagnostic: { code: "wasm_load_failed", message: action.message },
        pendingOperation: null,
      };
    case "gameSelected": {
      const selectedGame = state.catalog.find((game) => game.game_id === action.gameId) ?? null;
      return {
        ...state,
        mode: "setup",
        selectedGameId: action.gameId,
        setup: {
          ...state.setup,
          variantId: selectedGame?.variants?.[0]?.id ?? null,
          seatCount: resolveSeatCount(selectedGame, null),
        },
        matchId: null,
        view: null,
        actionTree: null,
        pendingActionPath: [],
        effects: [],
        effectCursor: 0,
        viewerMode: viewerModeForPlayMode(state.setup.playMode),
        viewerSeat: viewerSeatForMode(viewerModeForPlayMode(state.setup.playMode)),
        diagnostic: null,
        lastBotDecision: null,
        staleToken: null,
        replay: null,
        autoplay: { running: false },
        orchestration: { paused: false, rate: state.orchestration.rate },
      };
    }
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
        viewerSeat: viewerSeatForMode(viewerModeForPlayMode(action.playMode)),
      };
    case "setupVariantChanged":
      return {
        ...state,
        setup: {
          ...state.setup,
          variantId: action.variantId,
        },
      };
    case "setupSeatCountChanged":
      return {
        ...state,
        setup: {
          ...state.setup,
          seatCount: action.seatCount,
        },
      };
    case "viewerModeChanged":
      return {
        ...state,
        viewerMode: action.viewerMode,
        viewerSeat: viewerSeatForMode(action.viewerMode),
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
        viewerSeat: viewerSeatForMode(viewerModeForPlayMode(state.setup.playMode)),
        diagnostic: null,
        lastBotDecision: null,
        staleToken: null,
        replay: null,
        autoplay: { running: false },
        orchestration: { paused: false, rate: state.orchestration.rate },
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
        viewerSeat: viewerSeatForMode(action.payload.viewerMode),
        pendingOperation: null,
      };
    case "actionApplied":
      return {
        ...state,
        staleToken: action.staleToken,
        diagnostic: null,
        lastBotDecision: null,
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
        lastBotDecision: null,
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
        lastBotDecision: null,
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
    case "botTurnCompleted":
      return {
        ...state,
        lastBotDecision: botDecisionSummary(action.result),
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
    case "orchestrationPaused":
      return {
        ...state,
        orchestration: { ...state.orchestration, paused: true },
      };
    case "orchestrationResumed":
      return {
        ...state,
        orchestration: { ...state.orchestration, paused: false },
      };
    case "orchestrationRateChanged":
      return {
        ...state,
        orchestration: { ...state.orchestration, rate: action.rate },
      };
    case "rulesPanelOpened":
      return {
        ...state,
        rulesPanelOpen: true,
        rulesPanelGameId: action.gameId,
        rulesPanelStatus: "loading",
        rulesPanelMarkdown: null,
      };
    case "rulesPanelClosed":
      return {
        ...state,
        rulesPanelOpen: false,
        rulesPanelGameId: null,
        rulesPanelStatus: "idle",
        rulesPanelMarkdown: null,
      };
    case "rulesPanelLoadStarted":
      return {
        ...state,
        rulesPanelGameId: action.gameId,
        rulesPanelStatus: "loading",
        rulesPanelMarkdown: null,
      };
    case "rulesPanelLoaded":
      if (state.rulesPanelGameId !== action.gameId) {
        return state;
      }
      return {
        ...state,
        rulesPanelStatus: "loaded",
        rulesPanelMarkdown: action.markdown,
      };
    case "rulesPanelFailed":
      if (state.rulesPanelGameId !== action.gameId) {
        return state;
      }
      return {
        ...state,
        rulesPanelStatus: "error",
        rulesPanelMarkdown: null,
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

function resolveSeatCount(game: GameCatalogEntry | null, current: number | null): number | null {
  const supportedSeats = game?.supported_seats ?? [];
  if (current !== null && supportedSeats.includes(current)) {
    return current;
  }
  if (typeof game?.default_seats === "number" && supportedSeats.includes(game.default_seats)) {
    return game.default_seats;
  }
  return supportedSeats[0] ?? null;
}

function botDecisionSummary(result: BotTurnResult): BotDecisionSummary | null {
  if (!result.policy_id || !result.rationale) {
    return null;
  }
  return {
    policyId: result.policy_id,
    policyVersion: result.policy_version ?? null,
    rationale: result.rationale,
    publicExplanation: result.bot_explanation ?? null,
  };
}

function viewerModeForPlayMode(playMode: SetupPlayMode): ViewerMode {
  return playMode === "bot_vs_bot" ? { kind: "observer" } : { kind: "seat", seat: "seat_0" };
}

function viewerSeatForMode(viewerMode: ViewerMode): string | null {
  return viewerMode.kind === "seat" ? viewerMode.seat : null;
}
