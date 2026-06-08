import type { ActionChoice, ActionTree, PublicView, SeatId } from "../wasm/client";

type ActionControlsProps = {
  actionTree: ActionTree | null;
  view: PublicView | null;
  actorSeat: SeatId | null;
  pending: boolean;
  onChoice: (choice: ActionChoice) => void;
  onRestart: () => void;
};

export function ActionControls({ actionTree, view, actorSeat, pending, onChoice, onRestart }: ActionControlsProps) {
  const choices = actionTree?.choices ?? [];
  const isTerminal = isTerminalView(view);
  const isActorTurn = actorSeat !== null && view?.active_seat === actorSeat;
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
          choices.map((choice, index) => (
            <button
              type="button"
              key={choice.segment}
              onClick={() => onChoice(choice)}
              disabled={controlsDisabled}
              aria-label={choice.accessibility_label}
              data-testid={choiceTestId(view, choice, index)}
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

function choiceTestId(view: PublicView | null, choice: ActionChoice, index: number): string {
  if (view && "game_id" in view && view.game_id === "secret_draft") {
    const round = "round_number" in view ? view.round_number : 0;
    return `choice-secret-draft-round-${round}-${index}`;
  }
  return `choice-${choice.segment}`;
}

function isTerminalView(view: PublicView | null): boolean {
  if (!view) {
    return false;
  }
  if ("winner" in view) {
    return view.winner !== null;
  }
  if ("terminal" in view) {
    return view.terminal.terminal;
  }
  return view.terminal_kind !== "non_terminal";
}
