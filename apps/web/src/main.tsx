import React, { useCallback, useEffect, useMemo, useReducer } from "react";
import { createRoot } from "react-dom/client";
import "./styles.css";
import { AppShell } from "./components/AppShell";
import { ActionControls } from "./components/ActionControls";
import { EffectLog } from "./components/EffectLog";
import { summarizeEffect, useReducedMotionPreference } from "./components/effectFeedback";
import { GamePicker } from "./components/GamePicker";
import { MatchSetup } from "./components/MatchSetup";
import { ModeControls } from "./components/ModeControls";
import { RaceBoard } from "./components/RaceBoard";
import { initialShellState, shellReducer, type RefreshPayload, type SetupPlayMode } from "./state/shellReducer";
import { loadApi, type ActionChoice, type ApiError, type PublicView } from "./wasm/client";

type AppTextState = {
  mode: "loading" | "ready" | "playing" | "error";
  version: string;
  matchId: string | null;
  view: Pick<PublicView, "counter" | "target" | "active_seat" | "winner" | "freshness_token"> | null;
  choices: string[];
  effects: string[];
  diagnostic: ApiError | null;
};

function App() {
  const [state, dispatch] = useReducer(shellReducer, initialShellState);
  const motion = useReducedMotionPreference();
  const { api, version, matchId, view, actionTree, effects, effectCursor, diagnostic } = state;
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
        nextActorSeat && nextView.winner === null
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
        dispatch({ type: "wasmLoaded", api: loadedApi, version: loadedApi.version(), catalog: loadedApi.listGames() });
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
        if (
          state.setup.playMode === "human_vs_bot" &&
          afterHuman.winner === null &&
          botSeatForMode(state.setup.playMode, afterHuman.active_seat)
        ) {
          api.runBotTurn(matchId, afterHuman.active_seat, botSeed(afterHuman));
        }
        refresh(api, matchId, effectCursor);
      } catch (error: unknown) {
        dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
      }
    },
    [api, effectCursor, matchId, refresh, state.setup.playMode, view],
  );

  const runBotStep = useCallback(() => {
    if (!api || !matchId || !view || view.winner !== null || !botSeatForMode(state.setup.playMode, view.active_seat)) {
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

  useEffect(() => {
    if (
      !state.autoplay.running ||
      state.setup.playMode !== "bot_vs_bot" ||
      state.pendingOperation !== null ||
      !view ||
      view.winner !== null
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
      view: view
        ? {
            counter: view.counter,
            target: view.target,
            active_seat: view.active_seat,
            winner: view.winner,
            freshness_token: view.freshness_token,
          }
        : null,
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
        <RaceBoard view={view} latestEffect={latestEffect} />

        <ActionControls
          actionTree={actionTree}
          view={view}
          actorSeat={humanActorSeat}
          pending={state.pendingOperation !== null}
          onChoice={playChoice}
          onRestart={start}
        />

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
        </>
      )}
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

function humanSeatForMode(playMode: SetupPlayMode, view: PublicView): "seat_0" | "seat_1" | null {
  if (view.winner !== null) {
    return null;
  }
  if (playMode === "hotseat") {
    return view.active_seat;
  }
  if (playMode === "human_vs_bot" && view.active_seat === "seat_0") {
    return "seat_0";
  }
  return null;
}

function botSeatForMode(playMode: SetupPlayMode, seat: "seat_0" | "seat_1"): boolean {
  if (playMode === "bot_vs_bot") {
    return true;
  }
  return playMode === "human_vs_bot" && seat === "seat_1";
}

function botSeed(view: PublicView): number {
  return view.freshness_token + (view.active_seat === "seat_0" ? 101 : 211);
}
