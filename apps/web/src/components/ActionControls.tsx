import type { ActionChoice, ActionTree, PublicView } from "../wasm/client";

type ActionControlsProps = {
  actionTree: ActionTree | null;
  view: PublicView | null;
  pending: boolean;
  onChoice: (choice: ActionChoice) => void;
  onRestart: () => void;
};

export function ActionControls({ actionTree, view, pending, onChoice, onRestart }: ActionControlsProps) {
  const choices = actionTree?.choices ?? [];
  const isTerminal = view?.winner !== null && view?.winner !== undefined;
  const isActorTurn = view?.active_seat === "seat_0";
  const controlsDisabled = pending || !isActorTurn || isTerminal;

  return (
    <section className="action-panel" aria-label="Rust action choices">
      <div className="action-panel-heading">
        <h2>Actions</h2>
        <p>{isTerminal ? "Match complete" : isActorTurn ? "Choose a Rust-supplied action" : "Waiting for next turn"}</p>
      </div>

      <div className="controls">
        {choices.length === 0 ? (
          <p className="muted">No actions available.</p>
        ) : (
          choices.map((choice) => (
            <button
              type="button"
              key={choice.segment}
              onClick={() => onChoice(choice)}
              disabled={controlsDisabled}
              aria-label={choice.accessibility_label}
              data-testid={`choice-${choice.segment}`}
            >
              {choice.label}
            </button>
          ))
        )}
        <button type="button" onClick={onRestart}>
          Restart
        </button>
      </div>
    </section>
  );
}
