import type { EffectEntry, RacePublicView } from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type RaceBoardProps = {
  view: RacePublicView | null;
  latestEffect: EffectEntry | null;
};

export function RaceBoard({ view, latestEffect }: RaceBoardProps) {
  const counter = view?.counter ?? 0;
  const target = view?.target ?? 21;
  const progress = target > 0 ? Math.min(100, (counter / target) * 100) : 0;
  const status = view?.winner ? `${seatLabel(view.winner)} won` : view ? `${seatLabel(view.active_seat)} to move` : "Ready";
  const outcomeExplanation = view?.winner
    ? outcomeSurfaceData({
        gameId: "race_to_n",
        heading: `${seatLabel(view.winner)} wins`,
        rationale: view.terminal_rationale,
        resultKind: "win",
        decisiveCause: "exact_target_reached",
        templateKey: "race_to_n.exact_target_reached",
        templateParams: { winner: view.winner, target },
        finalStanding: [
          {
            id: view.winner,
            label: seatLabel(view.winner),
            result: "Winner",
            emphasized: true,
            values: [{ label: "Counter", value: counter }],
          },
        ],
        breakdownSections: [
          {
            id: "target",
            heading: "Target",
            rows: [
              { label: "Counter", value: counter },
              { label: "Target", value: target },
            ],
          },
        ],
        ruleIds: ["RACE-END-001"],
      })
    : null;

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
          <span>Remaining</span>
          <strong data-testid="remaining">{view ? Math.max(0, target - counter) : "--"}</strong>
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

      {view && !view.winner ? (
        <div className="race-reach" aria-label="Move preview">
          <span className="race-reach-label">Each add reaches</span>
          {[1, 2, 3].map((add) => {
            const resulting = counter + add;
            const legal = resulting <= target;
            const wins = resulting === target;
            return (
              <span
                key={add}
                className={`race-reach-chip ${wins ? "win" : legal ? "" : "overshoot"}`}
                data-testid={`race-reach-${add}`}
              >
                +{add} → {resulting}
                {wins ? " · wins" : legal ? "" : " · over 21"}
              </span>
            );
          })}
        </div>
      ) : null}

      <div className="board-status" role="status">
        <span>{outcomeExplanation ? outcomeAnnouncementText(outcomeExplanation) : latestEffect ? effectSummary(latestEffect) : "No action yet"}</span>
      </div>

      {outcomeExplanation ? <OutcomeExplanationPanel explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function seatLabel(seat: string): string {
  return seat === "seat_0" ? "Player 1" : seat === "seat_1" ? "Player 2" : seat;
}

function effectSummary(entry: EffectEntry): string {
  return feedbackForEffect(entry).detail.replace(/\bseat_0\b/g, "Player 1").replace(/\bseat_1\b/g, "Player 2");
}
