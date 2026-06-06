import React, { useCallback, useEffect, useMemo, useReducer } from "react";
import { createRoot } from "react-dom/client";
import "./styles.css";
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
  const { api, version, matchId, view, actionTree, effects, effectCursor, diagnostic, staleToken } = state;

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
        dispatch({ type: "wasmLoaded", api: loadedApi, version: loadedApi.version() });
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
    if (!api) {
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

  const submitStale = useCallback(() => {
    if (!api || !matchId) {
      return;
    }
    try {
      api.applyAction(matchId, "seat_0", "add-1", staleToken ?? 0);
    } catch (error: unknown) {
      dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
    }
    refresh(api, matchId, effectCursor);
  }, [api, effectCursor, matchId, refresh, staleToken]);

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
    <main className="shell">
      <section className="topbar" aria-label="WASM status">
        <div>
          <p className="eyebrow">Rulepath</p>
          <h1>race_to_n</h1>
        </div>
        <p className="wasm-status" data-testid="wasm-status">
          {version}
        </p>
      </section>

      <section className="play-surface" aria-label="race_to_n play surface">
        <div className="scoreboard">
          <div>
            <span>Counter</span>
            <strong data-testid="counter">{view ? `${view.counter} / ${view.target}` : "-- / 21"}</strong>
          </div>
          <div>
            <span>Turn</span>
            <strong data-testid="turn">{view?.winner ? `${view.winner} won` : view?.active_seat ?? "--"}</strong>
          </div>
          <div>
            <span>Token</span>
            <strong>{view?.freshness_token ?? "--"}</strong>
          </div>
        </div>

        <div className="board" aria-label="counter track">
          <div className="track">
            <div
              className="track-fill"
              style={{ width: `${view ? (view.counter / view.target) * 100 : 0}%` }}
            />
          </div>
          <div className="marker-row">
            <span>0</span>
            <span>21</span>
          </div>
        </div>

        <div className="controls" aria-label="Rust action choices">
          {!matchId ? (
            <button type="button" className="primary" onClick={start} disabled={!api} data-testid="start-match">
              Start Match
            </button>
          ) : (
            <>
              {(actionTree?.choices ?? []).map((choice) => (
                <button
                  type="button"
                  key={choice.segment}
                  onClick={() => playChoice(choice)}
                  disabled={view?.active_seat !== "seat_0" || view?.winner !== null}
                  data-testid={`choice-${choice.segment}`}
                >
                  {choice.label}
                </button>
              ))}
              <button
                type="button"
                onClick={submitStale}
                disabled={staleToken === null}
                data-testid="stale-action"
              >
                Submit Stale
              </button>
              <button type="button" onClick={start}>
                Restart
              </button>
            </>
          )}
        </div>

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
    </main>
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
