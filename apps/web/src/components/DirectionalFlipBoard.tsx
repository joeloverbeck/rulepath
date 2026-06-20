import { useMemo, useRef, useState, type KeyboardEvent } from "react";
import type {
  ActionChoice,
  DirectionalFlipCellView,
  DirectionalFlipLegalTargetView,
  DirectionalFlipPublicView,
  EffectEntry,
  SeatId,
} from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type DirectionalFlipBoardProps = {
  view: DirectionalFlipPublicView;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  seatRoleLabels?: Partial<Record<SeatId, string>>;
  onChoice?: (choice: ActionChoice) => void;
};

type CellCoordinate = {
  row: number;
  column: number;
};

export function DirectionalFlipBoard({
  view,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  seatRoleLabels = {},
  onChoice,
}: DirectionalFlipBoardProps) {
  const cells = useMemo(
    () => [...view.cells].sort((left, right) => left.row - right.row || left.column - right.column),
    [view.cells],
  );
  const [focusedCell, setFocusedCell] = useState(() => cells[0]?.cell ?? "");
  const [previewCell, setPreviewCell] = useState<string | null>(null);
  const buttonRefs = useRef(new Map<string, HTMLButtonElement>());
  const terminal = view.terminal_kind !== "non_terminal";
  const legalByCell = useMemo(
    () =>
      new Map(
        view.legal_targets
          .filter((target) => target.cell !== null)
          .map((target) => [target.cell as string, target] as const),
      ),
    [view.legal_targets],
  );
  const coordinateByCell = useMemo(
    () => new Map(cells.map((cell) => [cell.cell, { row: cell.row, column: cell.column }] as const)),
    [cells],
  );
  const cellByCoordinate = useMemo(
    () => new Map(cells.map((cell) => [`${cell.row}:${cell.column}`, cell.cell] as const)),
    [cells],
  );
  const forcedPassTarget = view.legal_targets.find((target) => target.action_kind === "forced_pass") ?? null;
  const activePreview = previewCell ? legalByCell.get(previewCell)?.preview ?? null : null;
  const previewFlips = useMemo(
    () => new Set(activePreview?.ordered_flip_cells ?? []),
    [activePreview?.ordered_flip_cells],
  );
  const placedCell = stringPayload(latestEffectOfType(effects, "disc_placed"), "cell");
  const flippedCells = useMemo(() => flipCellsFromEffect(latestEffectOfType(effects, "discs_flipped")), [effects]);
  const botEffect = latestEffectOfType(effects, "bot_chose_action");
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const canPlay = interactive && !pending && !terminal;
  const seat0Role = seatRoleLabels.seat_0;
  const seat1Role = seatRoleLabels.seat_1;
  const showIdentity = interactive && seat0Role === "you" && seat1Role === "bot";
  const scoreLeader =
    view.score.seat_0 > view.score.seat_1
      ? "seat_0"
      : view.score.seat_1 > view.score.seat_0
        ? "seat_1"
        : null;
  const outcomeExplanation = terminal
    ? outcomeSurfaceData({
        gameId: "directional_flip",
        heading: terminalLabel(view),
        rationale: view.terminal_rationale,
        resultKind: view.terminal_kind === "draw" ? "draw" : "win",
        decisiveCause: "final_score",
        templateKey:
          view.terminal_kind === "draw" ? "directional_flip.final_score_draw" : "directional_flip.final_score_win",
        templateParams: { winner: view.winning_seat ?? "" },
        finalStanding: [
          scoreStanding("seat_0", view.winning_seat, view.final_score?.seat_0 ?? view.score.seat_0),
          scoreStanding("seat_1", view.winning_seat, view.final_score?.seat_1 ?? view.score.seat_1),
        ],
        breakdownSections: [
          {
            id: "final-score",
            heading: "Final score",
            rows: [
              { label: "seat_0", value: view.final_score?.seat_0 ?? view.score.seat_0 },
              { label: "seat_1", value: view.final_score?.seat_1 ?? view.score.seat_1 },
            ],
          },
        ],
      })
    : null;

  const chooseTarget = (target: DirectionalFlipLegalTargetView | null) => {
    if (!target || !canPlay || !onChoice) {
      return;
    }
    onChoice({
      segment: target.action_segment,
      label: target.label,
      accessibility_label: target.accessibility_label,
    });
  };

  const moveFocus = (fromCell: string, delta: CellCoordinate) => {
    const from = coordinateByCell.get(fromCell);
    if (!from) {
      return;
    }
    const nextRow = Math.min(Math.max(from.row + delta.row, 1), view.board_rows);
    const nextColumn = Math.min(Math.max(from.column + delta.column, 1), view.board_columns);
    const nextCell = cellByCoordinate.get(`${nextRow}:${nextColumn}`);
    if (!nextCell) {
      return;
    }
    setFocusedCell(nextCell);
    buttonRefs.current.get(nextCell)?.focus();
  };

  return (
    <section
      className={`directional-flip-board ${terminal ? "terminal" : ""}${reducedMotion ? " reduced" : ""}`}
      aria-labelledby="directional-flip-heading"
    >
      <div className="directional-flip-banner">
        <div>
          <p className="eyebrow">Directional Flip</p>
          <h2 id="directional-flip-heading">{playerFacingText(view.status_label)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {terminalLabel(view)}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {boardSummary(view)}
      </p>

      {showIdentity ? (
        <p className="directional-identity" data-testid="directional-identity">
          You play the {view.ui.first_disc_shape_label} (Player 1). The bot plays the{" "}
          {view.ui.second_disc_shape_label} (Player 2).
        </p>
      ) : null}

      <div className="directional-score" aria-label="Score">
        <div className={scoreLeader === "seat_0" ? "leading" : ""} data-testid="directional-score-seat_0">
          <span>
            {view.ui.first_disc_shape_label}
            {roleSuffix(seat0Role)}
          </span>
          <strong>{view.score.seat_0}</strong>
          <small className="directional-lead-note">{leadNote("seat_0", scoreLeader)}</small>
        </div>
        <div className={scoreLeader === "seat_1" ? "leading" : ""} data-testid="directional-score-seat_1">
          <span>
            {view.ui.second_disc_shape_label}
            {roleSuffix(seat1Role)}
          </span>
          <strong>{view.score.seat_1}</strong>
          <small className="directional-lead-note">{leadNote("seat_1", scoreLeader)}</small>
        </div>
        <div>
          <span>Ply</span>
          <strong>{view.ply_count}</strong>
        </div>
      </div>

      <div className="directional-grid-shell">
        <div
          className="directional-grid"
          role="grid"
          aria-label={view.ui.board_label}
          data-testid="directional-flip-board"
          style={{
            gridTemplateColumns: `repeat(${view.board_columns}, minmax(0, 1fr))`,
            gridTemplateRows: `repeat(${view.board_rows}, minmax(0, 1fr))`,
          }}
        >
          {cells.map((cell) => {
            const target = legalByCell.get(cell.cell) ?? null;
            const previewTarget = activePreview?.target_cell === cell.cell;
            const previewFlip = previewFlips.has(cell.cell);
            const placed = placedCell === cell.cell;
            const flipped = flippedCells.has(cell.cell);
            const ariaDisabled = !canPlay || !target;
            return (
              <button
                type="button"
                key={cell.cell}
                ref={(node) => {
                  if (node) {
                    buttonRefs.current.set(cell.cell, node);
                  } else {
                    buttonRefs.current.delete(cell.cell);
                  }
                }}
                className={`directional-cell ${cell.owner ?? "empty"} ${target ? "legal" : ""} ${
                  previewTarget ? "preview-target" : ""
                } ${previewFlip ? "preview-flip" : ""} ${placed ? "placed" : ""} ${flipped ? "flipped" : ""}`}
                role="gridcell"
                tabIndex={focusedCell === cell.cell ? 0 : -1}
                aria-disabled={ariaDisabled}
                aria-label={target?.accessibility_label ?? cellLabel(cell)}
                data-testid={`directional-cell-${cell.cell}`}
                onFocus={() => {
                  setFocusedCell(cell.cell);
                  setPreviewCell(cell.cell);
                }}
                onBlur={() => setPreviewCell(null)}
                onMouseEnter={() => setPreviewCell(cell.cell)}
                onMouseLeave={() => setPreviewCell(null)}
                onKeyDown={(event) => {
                  handleGridKey(event, cell, view, moveFocus, () => chooseTarget(target), () => setPreviewCell(null));
                }}
                onClick={() => chooseTarget(target)}
              >
                {cell.owner ? (
                  <Disc owner={cell.owner} label={discLabel(cell)} />
                ) : (
                  <span className="directional-target-dot" aria-hidden="true" />
                )}
                <span className="directional-coordinate">{cell.cell_id}</span>
              </button>
            );
          })}
        </div>
      </div>

      <div className="directional-controls">
        {forcedPassTarget ? (
          <button
            type="button"
            className="primary"
            disabled={!canPlay}
            aria-label={forcedPassTarget.accessibility_label}
            onClick={() => chooseTarget(forcedPassTarget)}
          >
            {view.ui.forced_pass_label}
          </button>
        ) : null}
        <div className="directional-preview" aria-live="polite">
          <span>Preview</span>
          <strong>{activePreview ? activePreview.explanation : interactive ? "Focus a legal cell." : "Replay view."}</strong>
        </div>
      </div>

      {botEffect || view.bot_rationale ? (
        <div className="bot-note" data-testid="bot-explanation">
          <span>Bot</span>
          <strong>{String(botEffect?.effect.payload.rationale ?? view.bot_rationale ?? "The bot selected a move.")}</strong>
        </div>
      ) : null}

      <div className="board-status" role="status">
        <span>
          {outcomeExplanation
            ? outcomeAnnouncementText(outcomeExplanation)
            : feedback
              ? playerFacingText(feedback.detail)
            : forcedPassTarget
              ? "A forced pass is required."
              : interactive
                ? "Choose a highlighted cell."
                : "Replay board reflects this cursor."}
        </span>
      </div>

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function handleGridKey(
  event: KeyboardEvent<HTMLButtonElement>,
  cell: DirectionalFlipCellView,
  view: DirectionalFlipPublicView,
  moveFocus: (fromCell: string, delta: CellCoordinate) => void,
  choose: () => void,
  clearPreview: () => void,
) {
  switch (event.key) {
    case "ArrowUp":
      event.preventDefault();
      moveFocus(cell.cell, { row: -1, column: 0 });
      break;
    case "ArrowDown":
      event.preventDefault();
      moveFocus(cell.cell, { row: 1, column: 0 });
      break;
    case "ArrowLeft":
      event.preventDefault();
      moveFocus(cell.cell, { row: 0, column: -1 });
      break;
    case "ArrowRight":
      event.preventDefault();
      moveFocus(cell.cell, { row: 0, column: 1 });
      break;
    case "Home":
      event.preventDefault();
      moveFocus(cell.cell, { row: 1 - cell.row, column: 1 - cell.column });
      break;
    case "End":
      event.preventDefault();
      moveFocus(cell.cell, { row: view.board_rows - cell.row, column: view.board_columns - cell.column });
      break;
    case "Enter":
    case " ":
      event.preventDefault();
      choose();
      break;
    case "Escape":
      event.preventDefault();
      clearPreview();
      break;
  }
}

function Disc({ owner, label }: { owner: SeatId; label: string }) {
  if (owner === "seat_0") {
    return (
      <svg className="directional-disc directional-disc-seat-0" viewBox="0 0 100 100" role="img" aria-label={label}>
        <circle cx="50" cy="50" r="39" />
        <circle className="directional-disc-mark" cx="50" cy="50" r="15" />
      </svg>
    );
  }
  return (
    <svg className="directional-disc directional-disc-seat-1" viewBox="0 0 100 100" role="img" aria-label={label}>
      <circle cx="50" cy="50" r="39" />
      <path className="directional-disc-mark" d="M 28 36 H 72 M 28 50 H 72 M 28 64 H 72" />
    </svg>
  );
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

function stringPayload(entry: EffectEntry | null, key: string): string | null {
  const value = entry?.effect.payload[key];
  return typeof value === "string" ? value : null;
}

function flipCellsFromEffect(entry: EffectEntry | null): Set<string> {
  const flips = entry?.effect.payload.flips;
  if (!Array.isArray(flips)) {
    return new Set();
  }
  return new Set(
    flips.flatMap((flip) => {
      if (typeof flip === "object" && flip !== null && "cell" in flip && typeof flip.cell === "string") {
        return [flip.cell];
      }
      return [];
    }),
  );
}

function terminalLabel(view: DirectionalFlipPublicView): string {
  if (view.terminal_kind === "draw") {
    return "Draw";
  }
  if (view.terminal_kind === "win") {
    return `${seatLabel(view.winning_seat)} wins`;
  }
  return `${seatLabel(view.active_seat)} to move`;
}

function boardSummary(view: DirectionalFlipPublicView): string {
  const occupied = view.cells
    .filter((cell) => cell.owner)
    .map((cell) => `${cell.cell_id} ${seatLabel(cell.owner)}`)
    .join(", ");
  const legal = view.legal_targets
    .filter((target) => target.cell)
    .map((target) => target.cell)
    .join(", ");
  return `${playerFacingText(view.status_label)}. Score ${view.score.seat_0} to ${view.score.seat_1}. Occupied: ${
    occupied || "none"
  }. Legal targets: ${legal || "none"}.`;
}

function cellLabel(cell: DirectionalFlipCellView): string {
  if (cell.owner) {
    return `${cell.cell_id}, occupied by ${seatLabel(cell.owner)}`;
  }
  return `${cell.cell_id}, empty`;
}

function seatLabel(seat: string | null): string {
  return seat === "seat_0" ? "Player 1" : seat === "seat_1" ? "Player 2" : "No player";
}

function roleSuffix(role: string | undefined): string {
  return role ? ` (${role})` : "";
}

function leadNote(seat: "seat_0" | "seat_1", leader: "seat_0" | "seat_1" | null): string {
  if (leader === null) {
    return "Tied";
  }
  return leader === seat ? "Leading" : "Behind";
}

function playerFacingText(value: string): string {
  return value.replace(/\bseat_0\b/g, "Player 1").replace(/\bseat_1\b/g, "Player 2");
}

function discLabel(cell: DirectionalFlipCellView): string {
  return cell.disc_shape_label ?? cell.disc_pattern_label ?? cell.owner ?? "Disc";
}

function scoreStanding(seat: "seat_0" | "seat_1", winner: "seat_0" | "seat_1" | null, score: number) {
  return {
    id: seat,
    label: seat,
    result: winner === seat ? "Winner" : winner ? "Loss" : "Draw",
    emphasized: winner === seat,
    values: [{ label: "Final score", value: score }],
  };
}
