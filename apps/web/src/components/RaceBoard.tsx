import type { EffectEntry, PublicView } from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";

type RaceBoardProps = {
  view: PublicView | null;
  latestEffect: EffectEntry | null;
};

export function RaceBoard({ view, latestEffect }: RaceBoardProps) {
  const counter = view?.counter ?? 0;
  const target = view?.target ?? 21;
  const progress = target > 0 ? Math.min(100, (counter / target) * 100) : 0;
  const status = view?.winner ? `${view.winner} won` : view ? `${view.active_seat} to move` : "Ready";

  return (
    <section className="race-board" aria-label="Current match">
      <div className="scoreboard">
        <div>
          <span>Counter</span>
          <strong data-testid="counter">{view ? `${counter} / ${target}` : "-- / 21"}</strong>
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

      <div className="counter-stage" aria-label={`Counter progress ${counter} of ${target}`}>
        <svg viewBox="0 0 320 112" role="img" aria-label={status}>
          <rect className="counter-rail" x="20" y="50" width="280" height="18" rx="9" />
          <rect className="counter-fill" x="20" y="50" width={(280 * progress) / 100} height="18" rx="9" />
          <circle className="counter-token" cx={20 + (280 * progress) / 100} cy="59" r="18" />
          <text x="20" y="95">
            0
          </text>
          <text x="300" y="95" textAnchor="end">
            {target}
          </text>
          <text className="counter-value" x="160" y="36" textAnchor="middle">
            {counter}
          </text>
        </svg>
      </div>

      <div className="board-status" role="status">
        <span>{latestEffect ? effectSummary(latestEffect) : "No action yet"}</span>
      </div>
    </section>
  );
}

function effectSummary(entry: EffectEntry): string {
  return feedbackForEffect(entry).detail;
}
