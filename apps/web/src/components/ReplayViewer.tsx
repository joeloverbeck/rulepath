import { feedbackForEffect } from "./effectFeedback";
import type { ReplaySessionState } from "../state/shellReducer";
import type { EffectEntry } from "../wasm/client";

type ReplayViewerProps = {
  replay: ReplaySessionState | null;
  reducedMotion: boolean;
  onStep: () => void;
  onReset: () => void;
};

export function ReplayViewer({ replay, reducedMotion, onStep, onReset }: ReplayViewerProps) {
  const step = replay?.step ?? null;
  const effects = step?.effects ?? [];

  return (
    <section className="replay-viewer" aria-labelledby="replay-viewer-heading">
      <div className="region-heading">
        <p className="eyebrow">Viewer</p>
        <h2 id="replay-viewer-heading">Replay viewer</h2>
      </div>

      {step ? (
        <>
          <div className="replay-progress">
            <span>
              Cursor {step.cursor} / {step.command_count}
            </span>
            <progress value={step.cursor} max={Math.max(step.command_count, 1)} />
          </div>

          <div className="replay-snapshot">
            <div>
              <span>Counter</span>
              <strong>{step.view.counter} / {step.view.target}</strong>
            </div>
            <div>
              <span>Turn</span>
              <strong>{step.view.winner ? `${step.view.winner} won` : step.view.active_seat}</strong>
            </div>
            <div>
              <span>Status</span>
              <strong>{step.done ? "Complete" : "In progress"}</strong>
            </div>
          </div>

          <ol className="replay-effects">
            {effects.length === 0 ? (
              <li>No replay effects at this cursor.</li>
            ) : (
              effects.map((effect, index) => {
                const entry: EffectEntry = { cursor: index + 1, effect };
                const feedback = feedbackForEffect(entry);
                return (
                  <li key={`${step.cursor}-${index}`} className={reducedMotion ? "reduced" : ""}>
                    <strong>{feedback.title}</strong>
                    <span>{feedback.detail}</span>
                  </li>
                );
              })
            )}
          </ol>
        </>
      ) : (
        <p className="muted">Export or import a replay to inspect it here.</p>
      )}

      <div className="replay-actions">
        <button type="button" onClick={onReset} disabled={!step || step.cursor === 0}>
          Reset
        </button>
        <button type="button" onClick={onStep} disabled={!step || step.done}>
          Step
        </button>
      </div>
    </section>
  );
}
