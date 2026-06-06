import React, { useCallback, useEffect, useMemo, useReducer } from "react";
import { createRoot } from "react-dom/client";
import "./styles.css";
import { AppShell } from "./components/AppShell";
import { ActionControls } from "./components/ActionControls";
import { ColumnFourBoard } from "./components/ColumnFourBoard";
import { DevPanel } from "./components/DevPanel";
import { EffectLog } from "./components/EffectLog";
import { summarizeEffect, useReducedMotionPreference } from "./components/effectFeedback";
import { GamePicker } from "./components/GamePicker";
import { MatchSetup } from "./components/MatchSetup";
import { ModeControls } from "./components/ModeControls";
import { RaceBoard } from "./components/RaceBoard";
import { ReplayImportExport } from "./components/ReplayImportExport";
import { ReplayViewer } from "./components/ReplayViewer";
import { ThreeMarksBoard } from "./components/ThreeMarksBoard";
import { initialShellState, shellReducer, type RefreshPayload, type SetupPlayMode } from "./state/shellReducer";
import {
  loadApi,
  type ActionChoice,
  type ApiError,
  type ColumnFourPublicView,
  type PublicView,
  type RacePublicView,
  type ReplayDocument,
  type SeatId,
  type ThreeMarksPublicView,
} from "./wasm/client";

type AppTextState = {
  mode: "loading" | "ready" | "playing" | "error";
  version: string;
  matchId: string | null;
  view:
    | {
        game_id: string;
        active_seat: SeatId;
        freshness_token: number;
        status: string;
      }
    | null;
  choices: string[];
  effects: string[];
  diagnostic: ApiError | null;
};

function App() {
  const [state, dispatch] = useReducer(shellReducer, initialShellState);
  const motion = useReducedMotionPreference();
  const { api, version, matchId, view, actionTree, effects, effectCursor, diagnostic, staleToken } = state;
  const selectedGame = state.catalog.find((game) => game.game_id === state.selectedGameId) ?? null;
  const latestEffect = effects.at(-1) ?? null;
  const humanActorSeat = view ? humanSeatForMode(state.setup.playMode, view) : null;

  const refresh = useCallback(
    (loadedApi: NonNullable<typeof api>, loadedMatchId: string, sinceCursor: number) => {
      const nextView = loadedApi.getView(loadedMatchId);
      const nextEffects = loadedApi.getEffects(loadedMatchId, sinceCursor);
      const newestCursor = nextEffects.reduce((cursor, entry) => Math.max(cursor, entry.cursor), sinceCursor);
      const nextActorSeat = humanSeatForMode(state.setup.playMode, nextView);
      const nextTree =
        nextActorSeat && !isTerminalView(nextView)
          ? loadedApi.getActionTree(loadedMatchId, nextActorSeat)
          : { freshness_token: nextView.freshness_token, choices: [] };

      const payload: RefreshPayload = {
        view: nextView,
        actionTree: nextTree,
        effects: nextEffects,
        effectCursor: newestCursor,
      };
      dispatch({ type: "refreshed", payload });
    },
    [state.setup.playMode],
  );

  useEffect(() => {
    let cancelled = false;

    loadApi()
      .then((loadedApi) => {
        if (cancelled) {
          return;
        }
        dispatch({
          type: "wasmLoaded",
          api: loadedApi,
          version: loadedApi.version(),
          catalog: loadedApi.listGames(),
          featureReport: loadedApi.featureReport(),
        });
      })
      .catch((error: unknown) => {
        if (!cancelled) {
          dispatch({
            type: "wasmLoadFailed",
            message: error instanceof Error ? error.message : "Unable to load wasm-api artifact",
          });
        }
      });

    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    dispatch({ type: "reducedMotionChanged", reducedMotion: motion.reducedMotion });
  }, [motion.reducedMotion]);

  const start = useCallback(() => {
    if (!api || !state.selectedGameId) {
      return;
    }
    dispatch({ type: "matchStarting" });
    const created = api.newMatch(state.selectedGameId, state.setup.seed);
    dispatch({ type: "matchStarted", matchId: created.match_id });
    refresh(api, created.match_id, 0);
  }, [api, refresh, state.selectedGameId, state.setup.seed]);

  const playChoice = useCallback(
    (choice: ActionChoice) => {
      if (!api || !matchId || !view) {
        return;
      }
      const actorSeat = humanSeatForMode(state.setup.playMode, view);
      if (!actorSeat) {
        return;
      }
      dispatch({ type: "diagnosticCleared" });
      const tokenBeforeAction = view.freshness_token;
      try {
        const afterHuman = api.applyAction(matchId, actorSeat, choice.segment, tokenBeforeAction);
        dispatch({ type: "actionApplied", staleToken: tokenBeforeAction });
        const afterHumanSeat = afterHuman.active_seat;
        if (
          state.setup.playMode === "human_vs_bot" &&
          !isTerminalView(afterHuman) &&
          afterHumanSeat &&
          botSeatForMode(state.setup.playMode, afterHumanSeat)
        ) {
          api.runBotTurn(matchId, afterHumanSeat, botSeed(afterHuman));
        }
        refresh(api, matchId, effectCursor);
      } catch (error: unknown) {
        dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
      }
    },
    [api, effectCursor, matchId, refresh, state.setup.playMode, view],
  );

  const runBotStep = useCallback(() => {
    if (
      !api ||
      !matchId ||
      !view ||
      isTerminalView(view) ||
      !view.active_seat ||
      !botSeatForMode(state.setup.playMode, view.active_seat)
    ) {
      return;
    }
    dispatch({ type: "botTurnStarted" });
    try {
      api.runBotTurn(matchId, view.active_seat, botSeed(view));
      refresh(api, matchId, effectCursor);
    } catch (error: unknown) {
      dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
    }
  }, [api, effectCursor, matchId, refresh, state.setup.playMode, view]);

  const exportCurrentReplay = useCallback(() => {
    if (!api || !matchId) {
      throw { code: "no_match", message: "Start a match before exporting a replay." } satisfies ApiError;
    }
    return api.exportReplay(matchId);
  }, [api, matchId]);

  const importReplay = useCallback(
    (documentText: string) => {
      if (!api) {
        throw { code: "wasm_not_ready", message: "WASM API is not ready." } satisfies ApiError;
      }
      dispatch({ type: "pendingOperationChanged", pendingOperation: "importReplay" });
      try {
        const parsedDocument = parseReplayDocument(documentText);
        const imported = api.importReplay(documentText);
        const step = api.replayReset(imported.replay_id);
        dispatch({ type: "replayImported", replayId: imported.replay_id, document: parsedDocument, step });
      } catch (error: unknown) {
        dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
        throw error;
      }
    },
    [api],
  );

  const stepReplay = useCallback(() => {
    if (!api || !state.replay) {
      return;
    }
    dispatch({ type: "pendingOperationChanged", pendingOperation: "stepReplay" });
    const step = api.replayStep(state.replay.replayId, state.replay.cursor + 1);
    dispatch({ type: "replayStepped", step });
  }, [api, state.replay]);

  const resetReplay = useCallback(() => {
    if (!api || !state.replay) {
      return;
    }
    dispatch({ type: "pendingOperationChanged", pendingOperation: "stepReplay" });
    const step = api.replayReset(state.replay.replayId);
    dispatch({ type: "replayReset", step });
  }, [api, state.replay]);

  const submitStale = useCallback(() => {
    if (!api || !matchId || staleToken === null) {
      return;
    }
    try {
      api.applyAction(matchId, "seat_0", actionTree?.choices[0]?.segment ?? "add-1", staleToken);
    } catch (error: unknown) {
      dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
    }
    refresh(api, matchId, effectCursor);
  }, [actionTree, api, effectCursor, matchId, refresh, staleToken]);

  useEffect(() => {
    if (
      !state.autoplay.running ||
      state.setup.playMode !== "bot_vs_bot" ||
      state.pendingOperation !== null ||
      !view ||
      isTerminalView(view)
    ) {
      return;
    }
    const delay = state.reducedMotion ? 80 : 520;
    const timer = window.setTimeout(runBotStep, delay);
    return () => window.clearTimeout(timer);
  }, [runBotStep, state.autoplay.running, state.pendingOperation, state.reducedMotion, state.setup.playMode, view]);

  const textState = useMemo<AppTextState>(
    () => ({
      mode: state.mode === "play" || state.mode === "replay" ? "playing" : state.mode === "setup" ? "ready" : state.mode,
      version,
      matchId,
      view: view ? textView(view, state.selectedGameId) : null,
      choices: actionTree?.choices.map((choice) => choice.segment) ?? [],
      effects: effects.map(summarizeEffect),
      diagnostic,
    }),
    [actionTree, diagnostic, effects, matchId, state.mode, version, view],
  );

  useEffect(() => {
    window.render_game_to_text = () => JSON.stringify(textState);
    window.advanceTime = () => undefined;
  }, [textState]);

  return (
    <AppShell version={version} reducedMotion={state.reducedMotion}>
      {!matchId ? (
        <>
          <GamePicker
            games={state.catalog}
            selectedGameId={state.selectedGameId}
            onSelect={(gameId) => dispatch({ type: "gameSelected", gameId })}
          />
          <MatchSetup
            selectedGame={selectedGame}
            seed={state.setup.seed}
            playMode={state.setup.playMode}
            canStart={Boolean(api && state.selectedGameId)}
            onSeedChange={(seed) => dispatch({ type: "setupSeedChanged", seed })}
            onPlayModeChange={(playMode) => dispatch({ type: "setupPlayModeChanged", playMode })}
            onStart={start}
          />
        </>
      ) : (
        <>

      <section className="play-surface" aria-label={`${selectedGame?.display_name ?? "Selected game"} play surface`}>
        {state.selectedGameId === "race_to_n" ? (
          <RaceBoard view={isRaceView(view) ? view : null} latestEffect={latestEffect} />
        ) : isColumnFourView(view) ? (
          <ColumnFourBoard
            view={view}
            latestEffect={latestEffect}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            onChoice={playChoice}
          />
        ) : isThreeMarksView(view) ? (
          <ThreeMarksBoard
            view={view}
            latestEffect={latestEffect}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            onChoice={playChoice}
          />
        ) : (
          <GenericGameSurface view={view} selectedGameName={selectedGame?.display_name ?? "Selected game"} />
        )}

        {isColumnFourView(view) ? null : (
          <ActionControls
            actionTree={actionTree}
            view={view}
            actorSeat={humanActorSeat}
            pending={state.pendingOperation !== null}
            onChoice={playChoice}
            onRestart={start}
          />
        )}

        <ModeControls
          playMode={state.setup.playMode}
          view={view}
          autoplayRunning={state.autoplay.running}
          pending={state.pendingOperation !== null}
          onBotStep={runBotStep}
          onAutoplayStart={() => dispatch({ type: "autoplayStarted" })}
          onAutoplayPause={() => dispatch({ type: "autoplayPaused" })}
        />

        {diagnostic ? (
          <div className="diagnostic" role="status" data-testid="diagnostic">
            <strong>{diagnostic.code}</strong>
            <span>{diagnostic.message}</span>
          </div>
        ) : null}
      </section>

      <EffectLog
        effects={effects}
        reducedMotion={state.reducedMotion}
        override={motion.override}
        onOverrideChange={motion.setOverride}
      />
      <ReplayImportExport canExport={Boolean(matchId)} onExport={exportCurrentReplay} onImport={importReplay} />
      <ReplayViewer
        replay={state.replay}
        reducedMotion={state.reducedMotion}
        onStep={stepReplay}
        onReset={resetReplay}
      />
        </>
      )}
      <DevPanel
        open={state.devPanelOpen}
        featureReport={state.featureReport}
        selectedGameName={selectedGame?.display_name ?? null}
        matchId={matchId}
        seed={state.setup.seed}
        playMode={state.setup.playMode}
        view={view}
        actionTree={actionTree}
        effectCursor={effectCursor}
        effectCount={effects.length}
        pendingOperation={state.pendingOperation}
        replayId={state.replay?.replayId ?? null}
        replayCursor={state.replay?.cursor ?? null}
        diagnostic={diagnostic}
        canSubmitStale={Boolean(api && matchId && staleToken !== null)}
        onToggle={() => dispatch({ type: "devPanelToggled" })}
        onSubmitStale={submitStale}
      />
    </AppShell>
  );
}

declare global {
  interface Window {
    render_game_to_text?: () => string;
    advanceTime?: (ms: number) => void;
  }
}

const rootElement = document.getElementById("root");
if (!rootElement) {
  throw new Error("Missing #root element");
}

createRoot(rootElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

function humanSeatForMode(playMode: SetupPlayMode, view: PublicView): SeatId | null {
  if (isTerminalView(view)) {
    return null;
  }
  if (playMode === "hotseat") {
    return view.active_seat ?? null;
  }
  if (playMode === "human_vs_bot" && view.active_seat === "seat_0") {
    return "seat_0";
  }
  return null;
}

function botSeatForMode(playMode: SetupPlayMode, seat: SeatId): boolean {
  if (playMode === "bot_vs_bot") {
    return true;
  }
  return playMode === "human_vs_bot" && seat === "seat_1";
}

function botSeed(view: PublicView): number {
  return view.freshness_token + (view.active_seat === "seat_0" ? 101 : 211);
}

function parseReplayDocument(documentText: string): ReplayDocument | null {
  try {
    return JSON.parse(documentText) as ReplayDocument;
  } catch {
    return null;
  }
}

function isRaceView(view: PublicView | null): view is RacePublicView {
  return Boolean(view && "counter" in view);
}

function isThreeMarksView(view: PublicView | null): view is ThreeMarksPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "three_marks");
}

function isColumnFourView(view: PublicView | null): view is ColumnFourPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "column_four");
}

function isTerminalView(view: PublicView): boolean {
  if ("winner" in view) {
    return view.winner !== null;
  }
  return view.terminal_kind !== "non_terminal";
}

function textView(view: PublicView, fallbackGameId: string): AppTextState["view"] {
  if ("counter" in view) {
    return {
      game_id: fallbackGameId || "race_to_n",
      active_seat: view.active_seat,
      freshness_token: view.freshness_token,
      status: view.winner ? `${view.winner} won` : `${view.counter} / ${view.target}`,
    };
  }
  return {
    game_id: view.game_id,
    active_seat: view.active_seat ?? "seat_0",
    freshness_token: view.freshness_token,
    status: view.status_label,
  };
}

function GenericGameSurface({
  view,
  selectedGameName,
}: {
  view: PublicView | null;
  selectedGameName: string;
}) {
  const boardShape = view && "board_rows" in view ? `${view.board_rows} x ${view.board_columns}` : "Rust view";
  const status = view ? ("status_label" in view ? view.status_label : `${view.active_seat} to move`) : "Ready";

  return (
    <section className="race-board generic-board" aria-label={`${selectedGameName} match`}>
      <div className="scoreboard">
        <div>
          <span>Game</span>
          <strong>{selectedGameName}</strong>
        </div>
        <div>
          <span>Status</span>
          <strong data-testid="turn">{status}</strong>
        </div>
        <div>
          <span>Token</span>
          <strong>{view?.freshness_token ?? "--"}</strong>
        </div>
      </div>

      <div className="board-status" role="status">
        <span data-testid="three-marks-started">{view ? `${boardShape} match started from Rust` : "Waiting for Rust view"}</span>
      </div>
    </section>
  );
}
