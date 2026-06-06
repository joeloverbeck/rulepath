import type { ActionChoice, EffectEntry, ThreeMarksPublicView } from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";

type ThreeMarksBoardProps = {
  view: ThreeMarksPublicView;
  latestEffect: EffectEntry | null;
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onChoice?: (choice: ActionChoice) => void;
};

type CellModel = {
  id: string;
  row: number;
  column: number;
  occupancy: string;
  owner: "seat_0" | "seat_1" | null;
  token: string | null;
  shape: string | null;
};

type LegalTarget = {
  cell: string;
  actionSegment: string;
  label: string;
  accessibilityLabel: string;
  freshnessToken: number;
};

export function ThreeMarksBoard({
  view,
  latestEffect,
  reducedMotion,
  pending,
  interactive = true,
  onChoice,
}: ThreeMarksBoardProps) {
  const cells = view.cells.map(parseCell).sort((left, right) => left.row - right.row || left.column - right.column);
  const legalTargets = new Map(view.legal_targets.map((target) => {
    const parsed = parseLegalTarget(target);
    return [parsed.cell, parsed] as const;
  }));
  const terminal = view.terminal_kind !== "non_terminal";
  const winningCells = new Set(view.winning_line);
  const summary = boardSummary(cells, view);
  const botEffect = latestEffect?.effect.payload.type === "bot_chose_action" ? latestEffect : null;
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;

  return (
    <section
      className={`three-marks-board ${terminal ? "terminal" : ""}${reducedMotion ? " reduced" : ""}`}
      aria-labelledby="three-marks-heading"
    >
      <div className="three-marks-banner">
        <div>
          <p className="eyebrow">Three Marks</p>
          <h2 id="three-marks-heading">{view.status_label}</h2>
        </div>
        <span className="turn-pill">{terminalLabel(view)}</span>
      </div>

      <p className="sr-only" aria-live="polite">
        {summary}
      </p>

      <div className="three-marks-grid" role="group" aria-label={summary} data-testid="three-marks-board">
        {cells.map((cell) => {
          const legal = legalTargets.get(cell.id) ?? null;
          const occupied = cell.owner !== null;
          const winning = winningCells.has(cell.id);
          const disabled = pending || terminal || !legal || !interactive;
          return (
            <button
              type="button"
              key={cell.id}
              className={`three-cell ${occupied ? "occupied" : "empty"} ${cell.owner ?? ""} ${winning ? "winning" : ""}`}
              disabled={disabled}
              aria-label={legal?.accessibilityLabel ?? cellLabel(cell)}
              data-testid={`three-cell-${cell.id}`}
              onClick={() => {
                if (legal && onChoice) {
                  onChoice({
                    segment: legal.actionSegment,
                    label: legal.label,
                    accessibility_label: legal.accessibilityLabel,
                  });
                }
              }}
            >
              {cell.owner ? <Mark owner={cell.owner} label={cell.shape ?? cell.owner} /> : <span className="empty-dot" />}
              <span className="cell-coord">{cell.id}</span>
            </button>
          );
        })}
      </div>

      <div className="three-marks-status">
        <div>
          <span>Active</span>
          <strong>{view.active_seat}</strong>
        </div>
        <div>
          <span>Ply</span>
          <strong>{view.ply_count}</strong>
        </div>
        <div>
          <span>Legal</span>
          <strong>{terminal ? "0" : view.legal_targets.length}</strong>
        </div>
      </div>

      {botEffect ? (
        <div className="bot-note" data-testid="bot-explanation">
          <span>Bot</span>
          <strong>{String(botEffect.effect.payload.explanation ?? "Rust bot selected a move.")}</strong>
        </div>
      ) : null}

      <div className="board-status" role="status">
        <span>
          {feedback
            ? feedback.detail
            : interactive
              ? "Choose a highlighted cell to place a mark."
              : "Replay board is projected by Rust at this cursor."}
        </span>
      </div>
    </section>
  );
}

function Mark({ owner, label }: { owner: "seat_0" | "seat_1"; label: string }) {
  if (owner === "seat_0") {
    return (
      <svg className="mark mark-seat-0" viewBox="0 0 100 100" role="img" aria-label={label}>
        <circle cx="50" cy="50" r="30" />
        <circle className="mark-cutout" cx="50" cy="50" r="13" />
      </svg>
    );
  }

  return (
    <svg className="mark mark-seat-1" viewBox="0 0 100 100" role="img" aria-label={label}>
      <path d="M24 24 L76 76 M76 24 L24 76" />
    </svg>
  );
}

function parseCell(encoded: string): CellModel {
  const [id, row, column, occupancy, owner, token, shape] = encoded.split("|");
  return {
    id,
    row: Number(row),
    column: Number(column),
    occupancy,
    owner: owner === "seat_0" || owner === "seat_1" ? owner : null,
    token: token === "none" ? null : token,
    shape: shape === "none" ? null : shape,
  };
}

function parseLegalTarget(encoded: string): LegalTarget {
  const [cell, actionSegment, label, accessibilityLabel, freshnessToken] = encoded.split("|");
  return {
    cell,
    actionSegment,
    label,
    accessibilityLabel,
    freshnessToken: Number(freshnessToken),
  };
}

function cellLabel(cell: CellModel): string {
  if (cell.owner) {
    return `${cell.id}, occupied by ${cell.owner}`;
  }
  return `${cell.id}, empty`;
}

function terminalLabel(view: ThreeMarksPublicView): string {
  if (view.terminal_kind === "draw") {
    return "Draw";
  }
  if (view.terminal_kind === "win") {
    return `${view.winning_seat} wins`;
  }
  return `${view.active_seat} to move`;
}

function boardSummary(cells: CellModel[], view: ThreeMarksPublicView): string {
  const occupied = cells
    .filter((cell) => cell.owner)
    .map((cell) => `${cell.id} ${cell.owner}`)
    .join(", ");
  const legal = view.legal_targets
    .map(parseLegalTarget)
    .map((target) => target.cell)
    .join(", ");
  return `${view.status_label}. Occupied: ${occupied || "none"}. Legal targets: ${legal || "none"}.`;
}
