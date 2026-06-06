import { feedbackForEffect } from "./effectFeedback";
import type { ReplaySessionState } from "../state/shellReducer";
import type { EffectEntry, PublicView, ThreeMarksPublicView } from "../wasm/client";
import { ThreeMarksBoard } from "./ThreeMarksBoard";

type ReplayViewerProps = {
  replay: ReplaySessionState | null;
  reducedMotion: boolean;
  onStep: () => void;
  onReset: () => void;
};

export function ReplayViewer({ replay, reducedMotion, onStep, onReset }: ReplayViewerProps) {
  const step = replay?.step ?? null;
  const effects = step?.effects ?? [];
  const latestReplayEffect = effects.at(-1);
  const latestEntry: EffectEntry | null = latestReplayEffect && step ? { cursor: step.cursor, effect: latestReplayEffect } : null;
  const threeMarksView: ThreeMarksPublicView | null = step && isThreeMarksView(step.view) ? step.view : null;

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
            {snapshotItems(step.view, step.done).map((item) => (
              <div key={item.label}>
                <span>{item.label}</span>
                <strong>{item.value}</strong>
              </div>
            ))}
          </div>

          {threeMarksView ? (
            <div className="replay-board">
              <ThreeMarksBoard
                view={threeMarksView}
                latestEffect={latestEntry}
                reducedMotion={reducedMotion}
                pending={false}
                interactive={false}
              />
              {replay ? <PlacementSequence replay={replay} /> : null}
            </div>
          ) : null}

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

function PlacementSequence({ replay }: { replay: ReplaySessionState }) {
  const commands = replay.document?.commands ?? [];
  if (commands.length === 0) {
    return null;
  }

  return (
    <ol className="placement-sequence" aria-label="Replay command sequence">
      {commands.map((command) => {
        const isCurrent = command.index < replay.cursor;
        return (
          <li key={command.index} className={isCurrent ? "current" : ""}>
            <span>{command.index + 1}</span>
            <strong>{command.actor_seat}</strong>
            <code>{command.action_path.join("/")}</code>
          </li>
        );
      })}
    </ol>
  );
}

function isThreeMarksView(view: PublicView | null): view is ThreeMarksPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "three_marks");
}

function snapshotItems(view: PublicView, done: boolean): { label: string; value: string }[] {
  if ("counter" in view) {
    return [
      { label: "Counter", value: `${view.counter} / ${view.target}` },
      { label: "Turn", value: view.winner ? `${view.winner} won` : view.active_seat },
      { label: "Status", value: done ? "Complete" : "In progress" },
    ];
  }

  return [
    { label: "Board", value: `${view.board_rows} x ${view.board_columns}` },
    { label: "Turn", value: view.terminal_kind === "win" ? `${view.winning_seat} won` : view.active_seat },
    { label: "Status", value: view.status_label },
  ];
}
