import React, { useCallback, useEffect, useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import "./styles.css";
import { RulepathApi, loadApi, type ActionChoice, type ActionTree, type ApiError, type EffectEntry, type PublicView } from "./wasm/client";

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
  const [api, setApi] = useState<RulepathApi | null>(null);
  const [version, setVersion] = useState("Loading wasm-api...");
  const [matchId, setMatchId] = useState<string | null>(null);
  const [view, setView] = useState<PublicView | null>(null);
  const [tree, setTree] = useState<ActionTree | null>(null);
  const [effects, setEffects] = useState<EffectEntry[]>([]);
  const [lastCursor, setLastCursor] = useState(0);
  const [diagnostic, setDiagnostic] = useState<ApiError | null>(null);
  const [staleToken, setStaleToken] = useState<number | null>(null);
  const [mode, setMode] = useState<AppTextState["mode"]>("loading");

  const refresh = useCallback(
    (loadedApi: RulepathApi, loadedMatchId: string, sinceCursor: number) => {
      const nextView = loadedApi.getView(loadedMatchId);
      const nextEffects = loadedApi.getEffects(loadedMatchId, sinceCursor);
      const newestCursor = nextEffects.reduce((cursor, entry) => Math.max(cursor, entry.cursor), sinceCursor);
      const nextTree =
        nextView.active_seat === "seat_0" && nextView.winner === null
          ? loadedApi.getActionTree(loadedMatchId, "seat_0")
          : { freshness_token: nextView.freshness_token, choices: [] };

      setView(nextView);
      setTree(nextTree);
      setEffects((current) => [...current, ...nextEffects].slice(-12));
      setLastCursor(newestCursor);
      setMode("playing");
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
        setApi(loadedApi);
        setVersion(loadedApi.version());
        setMode("ready");
      })
      .catch((error: unknown) => {
        if (!cancelled) {
          setVersion(error instanceof Error ? error.message : "Unable to load wasm-api artifact");
          setMode("error");
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
    setDiagnostic(null);
    const created = api.newMatch("race_to_n", 1);
    setMatchId(created.match_id);
    setEffects([]);
    setLastCursor(0);
    setStaleToken(null);
    refresh(api, created.match_id, 0);
  }, [api, refresh]);

  const playChoice = useCallback(
    (choice: ActionChoice) => {
      if (!api || !matchId || !view) {
        return;
      }
      setDiagnostic(null);
      const tokenBeforeAction = view.freshness_token;
      try {
        const afterHuman = api.applyAction(matchId, "seat_0", choice.segment, tokenBeforeAction);
        setStaleToken(tokenBeforeAction);
        if (afterHuman.winner === null && afterHuman.active_seat === "seat_1") {
          api.runBotTurn(matchId, "seat_1", tokenBeforeAction + 101);
        }
        refresh(api, matchId, lastCursor);
      } catch (error: unknown) {
        setDiagnostic(error as ApiError);
      }
    },
    [api, lastCursor, matchId, refresh, view],
  );

  const submitStale = useCallback(() => {
    if (!api || !matchId) {
      return;
    }
    try {
      api.applyAction(matchId, "seat_0", "add-1", staleToken ?? 0);
    } catch (error: unknown) {
      setDiagnostic(error as ApiError);
    }
    refresh(api, matchId, lastCursor);
  }, [api, lastCursor, matchId, refresh, staleToken]);

  const textState = useMemo<AppTextState>(
    () => ({
      mode,
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
      choices: tree?.choices.map((choice) => choice.segment) ?? [],
      effects: effects.map(describeEffect),
      diagnostic,
    }),
    [diagnostic, effects, matchId, mode, tree, version, view],
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
              {(tree?.choices ?? []).map((choice) => (
                <button
                  type="button"
                  key={choice.segment}
                  onClick={() => playChoice(choice)}
                  disabled={view?.active_seat !== "seat_0" || view.winner !== null}
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
