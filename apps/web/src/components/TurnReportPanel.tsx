import type { EffectEntry } from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";

type TurnReportPanelProps = {
  gameId: string;
  effects: EffectEntry[];
};

const ADOPTED_GAMES = new Set(["event_frontier", "flood_watch"]);
const MAX_REPORT_EVENTS = 6;

export function TurnReportPanel({ gameId, effects }: TurnReportPanelProps) {
  if (!ADOPTED_GAMES.has(gameId)) {
    return null;
  }

  const report = latestReportBurst(effects);

  return (
    <section className="turn-report-panel" aria-label="Turn report" data-testid="turn-report-panel">
      <div className="plain-section-heading">
        <span>Turn report</span>
        <strong>{report.length ? "Latest resolution" : "Waiting"}</strong>
      </div>
      <ol aria-live="polite">
        {report.length ? (
          report.map((entry) => {
            const feedback = feedbackForEffect(entry);
            return (
              <li key={entry.cursor}>
                <strong>{feedback.title}</strong>
                <span>{feedback.detail}</span>
              </li>
            );
          })
        ) : (
          <li>
            <strong>No report yet</strong>
            <span>Resolved Rust effects will appear here after the next decision.</span>
          </li>
        )}
      </ol>
    </section>
  );
}

function latestReportBurst(effects: EffectEntry[]): EffectEntry[] {
  const visible = effects.filter((entry) => !isDecisionMarker(entry));
  return visible.slice(-MAX_REPORT_EVENTS);
}

function isDecisionMarker(entry: EffectEntry): boolean {
  const type = entry.effect.payload.type;
  return type === "action_started" || type === "choice_taken" || type === "bot_chose_action" || type === "bot_chose_action_public";
}
