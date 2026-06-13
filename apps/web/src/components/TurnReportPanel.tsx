import type { EffectEntry } from "../wasm/client";
import { latestResolutionBurst } from "../animation/bursts";
import { feedbackForEffect } from "./effectFeedback";

type TurnReportPanelProps = {
  gameId: string;
  effects: EffectEntry[];
};

const ADOPTED_GAMES = new Set(["event_frontier", "flood_watch"]);

export function TurnReportPanel({ gameId, effects }: TurnReportPanelProps) {
  if (!ADOPTED_GAMES.has(gameId)) {
    return null;
  }

  const report = latestResolutionBurst(effects);
  const reportEntries = report?.entries ?? [];

  return (
    <section className="turn-report-panel" aria-label="Turn report" data-testid="turn-report-panel">
      <div className="plain-section-heading">
        <span>Turn report</span>
        <strong>{reportEntries.length ? report?.label ?? "Latest resolution" : "Waiting"}</strong>
      </div>
      <ol aria-live="polite">
        {reportEntries.length ? (
          reportEntries.map((entry) => {
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
