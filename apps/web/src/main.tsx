import React, { useCallback, useEffect, useMemo, useReducer } from "react";
import { createRoot } from "react-dom/client";
import "./styles.css";
import { AppShell } from "./components/AppShell";
import { ActionControls } from "./components/ActionControls";
import { GamePicker } from "./components/GamePicker";
import { MatchSetup } from "./components/MatchSetup";
import { RaceBoard } from "./components/RaceBoard";
import { initialShellState, shellReducer, type RefreshPayload } from "./state/shellReducer";
import { loadApi, type ActionChoice, type ApiError, type EffectEntry, type PublicView } from "./wasm/client";

type AppTextState = {
  mode: "loading" | "ready" | "playing" | "error";
  version: string;
  matchId: string | null;
  view: Pick<PublicView, "counter" | "target" | "active_seat" | "winner" | "freshness_token"> | null;
  choices: string[];
  effects: string[];
  diagnostic: ApiError | null;
};

function describeEffect(entry: EffectEntry): string {
  const payload = entry.effect.payload;
  switch (payload.type) {
    case "action_started":
      return `${entry.cursor}: ${payload.actor} started add-${payload.amount}`;
    case "counter_advanced":
      return `${entry.cursor}: ${payload.actor} moved ${payload.from} to ${payload.to}`;
    case "turn_changed":
      return `${entry.cursor}: turn changed to ${payload.next_actor}`;
    case "game_ended":
      return `${entry.cursor}: ${payload.winner} won`;
    case "action_completed":
      return `${entry.cursor}: ${payload.actor} completed`;
    default:
      return `${entry.cursor}: ${payload.type}`;
  }
}

function App() {
  const [state, dispatch] = useReducer(shellReducer, initialShellState);
  const { api, version, matchId, view, actionTree, effects, effectCursor, diagnostic } = state;
  const selectedGame = state.catalog.find((game) => game.game_id === state.selectedGameId) ?? null;
  const latestEffect = effects.at(-1) ?? null;

  const refresh = useCallback(
    (loadedApi: NonNullable<typeof api>, loadedMatchId: string, sinceCursor: number) => {
      const nextView = loadedApi.getView(loadedMatchId);
      const nextEffects = loadedApi.getEffects(loadedMatchId, sinceCursor);
      const newestCursor = nextEffects.reduce((cursor, entry) => Math.max(cursor, entry.cursor), sinceCursor);
      const nextTree =
        nextView.active_seat === "seat_0" && nextView.winner === null
          ? loadedApi.getActionTree(loadedMatchId, "seat_0")
          : { freshness_token: nextView.freshness_token, choices: [] };

      const payload: RefreshPayload = {
        view: nextView,
        actionTree: nextTree,
        effects: nextEffects,
        effectCursor: newestCursor,
      };
      dispatch({ type: "refreshed", payload });
    },
    [],
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
      dispatch({ type: "diagnosticCleared" });
      const tokenBeforeAction = view.freshness_token;
      try {
        const afterHuman = api.applyAction(matchId, "seat_0", choice.segment, tokenBeforeAction);
        dispatch({ type: "actionApplied", staleToken: tokenBeforeAction });
        if (afterHuman.winner === null && afterHuman.active_seat === "seat_1") {
          api.runBotTurn(matchId, "seat_1", tokenBeforeAction + 101);
        }
        refresh(api, matchId, effectCursor);
      } catch (error: unknown) {
        dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
      }
    },
    [api, effectCursor, matchId, refresh, view],
  );

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
      effects: effects.map(describeEffect),
      diagnostic,
    }),
    [actionTree, diagnostic, effects, matchId, state.mode, version, view],
  );

  useEffect(() => {
    window.render_game_to_text = () => JSON.stringify(textState);
    window.advanceTime = () => undefined;
  }, [textState]);

  return (
    <AppShell version={version}>
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
          pending={state.pendingOperation !== null}
          onChoice={playChoice}
          onRestart={start}
        />

        {diagnostic ? (
          <div className="diagnostic" role="status" data-testid="diagnostic">
            <strong>{diagnostic.code}</strong>
            <span>{diagnostic.message}</span>
          </div>
        ) : null}
      </section>

      <section className="effects" aria-label="semantic effects">
        <h2>Effects</h2>
        <ol data-testid="effects">
          {effects.length === 0 ? <li>No effects yet</li> : effects.map((entry) => <li key={entry.cursor}>{describeEffect(entry)}</li>)}
        </ol>
      </section>
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
