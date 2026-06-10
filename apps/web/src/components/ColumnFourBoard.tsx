import { useMemo, useState } from "react";
import type { ActionChoice, ColumnFourPublicView, EffectEntry } from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type ColumnFourBoardProps = {
  view: ColumnFourPublicView;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onChoice?: (choice: ActionChoice) => void;
};

const CELL_SIZE = 64;
const CELL_GAP = 10;
const BOARD_PAD = 18;

export function ColumnFourBoard({
  view,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onChoice,
}: ColumnFourBoardProps) {
  const [previewCell, setPreviewCell] = useState<string | null>(null);
  const terminal = view.terminal_kind !== "non_terminal";
  const winningCells = useMemo(() => new Set(view.winning_line), [view.winning_line]);
  const legalByColumn = useMemo(
    () => new Map(view.legal_targets.map((target) => [target.column, target] as const)),
    [view.legal_targets],
  );
  const cells = useMemo(
    () => [...view.cells].sort((left, right) => right.row - left.row || left.column - right.column),
    [view.cells],
  );
  const boardWidth = BOARD_PAD * 2 + view.board_columns * CELL_SIZE + (view.board_columns - 1) * CELL_GAP;
  const boardHeight = BOARD_PAD * 2 + view.board_rows * CELL_SIZE + (view.board_rows - 1) * CELL_GAP;
  const botEffect = latestEffectOfType(effects, "bot_chose_action");
  const landedCell = landedCellFromEffects(effects);
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const outcomeExplanation = terminal
    ? outcomeSurfaceData({
        gameId: "column_four",
        heading: terminalLabel(view),
        rationale: view.terminal_rationale,
        resultKind: view.terminal_kind === "draw" ? "draw" : "win",
        decisiveCause: view.terminal_kind === "draw" ? "full_board_draw" : "line_completed",
        templateKey: view.terminal_kind === "draw" ? "column_four.full_board_draw" : "column_four.line_completed",
        templateParams: {
          winner: view.winning_seat ?? "",
          line_label: view.winning_line.join(", "),
        },
        finalStanding: [standing("seat_0", view.winning_seat), standing("seat_1", view.winning_seat)],
        breakdownSections: [
          {
            id: "terminal-line",
            heading: "Terminal detail",
            rows: [
              { label: "Kind", value: view.terminal_kind },
              { label: "Winning line", value: view.winning_line.join(", ") || "None" },
            ],
          },
        ],
      })
    : null;

  return (
    <section
      className={`column-four-board ${terminal ? "terminal" : ""}${reducedMotion ? " reduced" : ""}`}
      aria-labelledby="column-four-heading"
    >
      <div className="column-four-banner">
        <div>
          <p className="eyebrow">Column Four</p>
          <h2 id="column-four-heading">{view.status_label}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {terminalLabel(view)}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {boardSummary(view)}
      </p>

      <div className="column-controls" role="group" aria-label="Column choices">
        {view.columns.map((column) => {
          const legal = legalByColumn.get(column.column) ?? null;
          const disabled = pending || terminal || !legal || !interactive;
          return (
            <button
              type="button"
              key={column.column}
              className="column-choice"
              disabled={disabled}
              aria-label={legal?.accessibility_label ?? `${column.label}, not available`}
              data-testid={`column-four-control-${column.column}`}
              onFocus={() => setPreviewCell(column.landing_preview)}
              onBlur={() => setPreviewCell(null)}
              onMouseEnter={() => setPreviewCell(column.landing_preview)}
              onMouseLeave={() => setPreviewCell(null)}
              onClick={() => {
                if (legal && onChoice) {
                  onChoice({
                    segment: legal.action_segment,
                    label: legal.label,
                    accessibility_label: legal.accessibility_label,
                  });
                }
              }}
            >
              {column.column_id}
            </button>
          );
        })}
      </div>

      <div className="column-four-stage" role="img" aria-label={boardSummary(view)} data-testid="column-four-board">
        <svg viewBox={`0 0 ${boardWidth} ${boardHeight}`} aria-hidden="true">
          <rect className="column-four-backdrop" x="0" y="0" width={boardWidth} height={boardHeight} rx="8" />
          {cells.map((cell) => {
            const x = BOARD_PAD + (cell.column - 1) * (CELL_SIZE + CELL_GAP);
            const y = BOARD_PAD + (view.board_rows - cell.row) * (CELL_SIZE + CELL_GAP);
            const preview = previewCell === cell.cell && cell.owner === null && !terminal;
            const winning = winningCells.has(cell.cell);
            const landed = landedCell === cell.cell;
            return (
              <g
                key={cell.cell}
                className={`column-cell ${cell.owner ?? "empty"} ${preview ? "preview" : ""} ${winning ? "winning" : ""} ${landed ? "landed" : ""}`}
              >
                <circle cx={x + CELL_SIZE / 2} cy={y + CELL_SIZE / 2} r={CELL_SIZE / 2 - 4} />
                {cell.owner ? <Piece owner={cell.owner} x={x} y={y} label={cell.piece_shape_label ?? cell.owner} /> : null}
                <title>{cellLabel(cell)}</title>
              </g>
            );
          })}
        </svg>
      </div>

      <div className="column-four-status">
        <div>
          <span>Active</span>
          <strong>{view.active_seat ?? "terminal"}</strong>
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
          <strong>{String(botEffect.effect.payload.rationale ?? "Rust bot selected a column.")}</strong>
        </div>
      ) : null}

      <div className="board-status" role="status">
        <span>
          {outcomeExplanation
            ? outcomeAnnouncementText(outcomeExplanation)
            : feedback
              ? feedback.detail
            : interactive
              ? "Choose a column above the board."
              : "Replay board is projected by Rust at this cursor."}
        </span>
      </div>

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function landedCellFromEffects(entries: EffectEntry[]): string | null {
  const entry = latestEffectOfType(entries, "piece_landed");
  if (!entry) {
    return null;
  }
  const cell = entry.effect.payload.cell;
  return typeof cell === "string" ? cell : null;
}

function latestEffectOfType(entries: EffectEntry[], type: string): EffectEntry | null {
  for (let index = entries.length - 1; index >= 0; index -= 1) {
    const entry = entries[index];
    if (entry.effect.payload.type === type) {
      return entry;
    }
  }
  return null;
}

function Piece({ owner, x, y, label }: { owner: "seat_0" | "seat_1"; x: number; y: number; label: string }) {
  const cx = x + CELL_SIZE / 2;
  const cy = y + CELL_SIZE / 2;
  if (owner === "seat_0") {
    return (
      <g className="column-piece seat-0" role="img" aria-label={label}>
        <circle cx={cx} cy={cy} r={CELL_SIZE / 2 - 11} />
        <circle className="piece-mark" cx={cx} cy={cy} r="11" />
      </g>
    );
  }
  return (
    <g className="column-piece seat-1" role="img" aria-label={label}>
      <circle cx={cx} cy={cy} r={CELL_SIZE / 2 - 11} />
      <path className="piece-mark" d={`M ${cx - 14} ${cy} L ${cx + 14} ${cy} M ${cx} ${cy - 14} L ${cx} ${cy + 14}`} />
    </g>
  );
}

function terminalLabel(view: ColumnFourPublicView): string {
  if (view.terminal_kind === "draw") {
    return "Draw";
  }
  if (view.terminal_kind === "win") {
    return `${view.winning_seat} wins`;
  }
  return `${view.active_seat} to move`;
}

function boardSummary(view: ColumnFourPublicView): string {
  const occupied = view.cells
    .filter((cell) => cell.owner)
    .map((cell) => `${cell.cell} ${cell.owner}`)
    .join(", ");
  const legal = view.legal_targets.map((target) => target.column).join(", ");
  return `${view.status_label}. Occupied: ${occupied || "none"}. Legal columns: ${legal || "none"}.`;
}

function cellLabel(cell: ColumnFourPublicView["cells"][number]): string {
  if (cell.owner) {
    return `${cell.cell}, occupied by ${cell.owner}`;
  }
  return `${cell.cell}, empty`;
}

function standing(seat: "seat_0" | "seat_1", winner: "seat_0" | "seat_1" | null) {
  return {
    id: seat,
    label: seat,
    result: winner === seat ? "Winner" : winner ? "Loss" : "Draw",
    emphasized: winner === seat,
    values: [{ label: "Result", value: winner === seat ? "win" : winner ? "loss" : "draw" }],
  };
}
