import { useEffect, useMemo, useRef, useState, type KeyboardEvent } from "react";
import type { ActionChoice, ActionTree, DraughtsLiteCellView, DraughtsLitePublicView, EffectEntry } from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type DraughtsLiteBoardProps = {
  view: DraughtsLitePublicView;
  actionTree: ActionTree | null;
  pendingPath: string[];
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onPendingPathChange?: (path: string[]) => void;
  onPendingPathClear?: () => void;
  onPathSubmit?: (path: string[]) => void;
};

type CurrentNode = {
  choices: ActionChoice[];
  selectedChoice: ActionChoice | null;
  selectedCell: string | null;
  validPath: boolean;
};

export function DraughtsLiteBoard({
  view,
  actionTree,
  pendingPath,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onPendingPathChange,
  onPendingPathClear,
  onPathSubmit,
}: DraughtsLiteBoardProps) {
  const cells = useMemo(
    () => [...view.cells].sort((left, right) => left.row - right.row || left.column - right.column),
    [view.cells],
  );
  const [focusedCell, setFocusedCell] = useState(() => cells[0]?.cell ?? "");
  const buttonRefs = useRef(new Map<string, HTMLButtonElement>());
  const current = useMemo(() => currentNode(actionTree, pendingPath), [actionTree, pendingPath]);
  const choicesByCell = useMemo(() => choicesForCells(current.choices), [current.choices]);
  const legalCells = useMemo(() => new Set(choicesByCell.keys()), [choicesByCell]);
  const terminal = view.terminal_kind !== "non_terminal";
  const canPlay = interactive && !pending && !terminal && current.validPath;
  const capturePreview = metadataSet(current.choices, "preview_captured_cell");
  const promotionCells = metadataSet(
    current.choices.filter((choice) => metadataValue(choice, "would_promote") === "true"),
    "cell_id",
  );
  const recent = useMemo(() => recentEffectCells(effects), [effects]);
  const botEffect = latestEffectOfType(effects, "bot_chose_action");
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const pendingLabel = pendingPath.length > 0 ? pendingPath.join(" > ") : "None";
  const captureChoiceCount = countCaptureChoices(current.choices);
  const coordinateByCell = useMemo(
    () => new Map(cells.map((cell) => [cell.cell, { row: cell.row, column: cell.column }] as const)),
    [cells],
  );
  const cellByCoordinate = useMemo(
    () => new Map(cells.map((cell) => [`${cell.row}:${cell.column}`, cell.cell] as const)),
    [cells],
  );
  const liveStatus = liveAnnouncement(view, current, pendingPath, feedback, botEffect);
  const outcomeExplanation = terminal && view.winning_seat
    ? outcomeSurfaceData({
        gameId: "draughts_lite",
        heading: `${view.winning_seat} wins`,
        rationale: view.terminal_rationale,
        resultKind: "win",
        decisiveCause: "terminal_position",
        templateKey: "draughts_lite.opponent_no_pieces",
        templateParams: {
          winner: view.winning_seat,
          loser: otherSeat(view.winning_seat),
        },
        finalStanding: [
          pieceStanding("seat_0", view.winning_seat, view.cells),
          pieceStanding("seat_1", view.winning_seat, view.cells),
        ],
        breakdownSections: [
          {
            id: "material",
            heading: "Public material",
            rows: [
              { label: "seat_0 pieces", value: pieceCount("seat_0", view.cells) },
              { label: "seat_1 pieces", value: pieceCount("seat_1", view.cells) },
            ],
          },
        ],
      })
    : null;

  useEffect(() => {
    if (!focusedCell && cells[0]) {
      setFocusedCell(cells[0].cell);
    }
  }, [cells, focusedCell]);

  const chooseCell = (cell: DraughtsLiteCellView) => {
    const choice = choicesByCell.get(cell.cell);
    if (!choice || !canPlay) {
      return;
    }
    const nextPath = [...pendingPath, choice.segment];
    if (choice.next?.choices?.length) {
      onPendingPathChange?.(nextPath);
    } else {
      onPathSubmit?.(nextPath);
    }
  };

  const cancelPending = () => {
    if (pendingPath.length > 0) {
      onPendingPathClear?.();
    }
  };

  const moveFocus = (fromCell: string, target: { row: number; column: number }) => {
    const from = coordinateByCell.get(fromCell);
    if (!from) {
      return;
    }
    const nextRow = Math.min(Math.max(target.row, 1), view.board_rows);
    const nextColumn = Math.min(Math.max(target.column, 1), view.board_columns);
    const nextCell = cellByCoordinate.get(`${nextRow}:${nextColumn}`);
    if (!nextCell) {
      return;
    }
    setFocusedCell(nextCell);
    buttonRefs.current.get(nextCell)?.focus();
  };

  return (
    <section
      className={`draughts-lite-board ${terminal ? "terminal" : ""}${reducedMotion ? " reduced" : ""}`}
      aria-labelledby="draughts-lite-heading"
      onKeyDown={(event: KeyboardEvent<HTMLElement>) => {
        if (event.key === "Escape") {
          event.preventDefault();
          cancelPending();
        }
      }}
    >
      <div className="draughts-lite-banner">
        <div>
          <p className="eyebrow">Draughts Lite</p>
          <h2 id="draughts-lite-heading">{view.status_label}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {terminalLabel(view)}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {boardSummary(view, current.choices)}
      </p>

      <div className="draughts-lite-status" aria-label="Match status">
        <div>
          <span>Active</span>
          <strong>{view.active_seat ?? "terminal"}</strong>
        </div>
        <div>
          <span>Ply</span>
          <strong>{view.ply_count}</strong>
        </div>
        <div>
          <span>Path</span>
          <strong>{pendingPath.length > 0 ? `${pendingPath.length} step${pendingPath.length === 1 ? "" : "s"}` : "ready"}</strong>
        </div>
      </div>

      <div className="draughts-lite-grid-shell">
        <div
          className="draughts-lite-grid"
          role="grid"
          aria-label={view.ui.board_label}
          aria-activedescendant={focusedCell ? `draughts-cell-${focusedCell}` : undefined}
          data-testid="draughts-lite-board"
          style={{
            gridTemplateColumns: `repeat(${view.board_columns}, minmax(0, 1fr))`,
            gridTemplateRows: `repeat(${view.board_rows}, minmax(0, 1fr))`,
          }}
        >
          {cells.map((cell) => {
            const choice = choicesByCell.get(cell.cell) ?? null;
            const selected = current.selectedCell === cell.cell;
            const recentOrigin = recent.origins.has(cell.cell);
            const recentLanding = recent.landings.has(cell.cell);
            const recentCaptured = recent.captures.has(cell.cell);
            const previewCapture = capturePreview.has(cell.cell);
            const promotes = promotionCells.has(cell.cell);
            return (
              <button
                type="button"
                key={cell.cell}
                id={`draughts-cell-${cell.cell}`}
                ref={(node) => {
                  if (node) {
                    buttonRefs.current.set(cell.cell, node);
                  } else {
                    buttonRefs.current.delete(cell.cell);
                  }
                }}
                className={`draughts-cell ${cell.playable ? "playable" : "non-playable"} ${cell.owner ?? "empty"} ${
                  choice ? "legal" : ""
                } ${selected ? "selected" : ""} ${previewCapture ? "capture-preview" : ""} ${
                  recentOrigin ? "recent-origin" : ""
                } ${recentLanding ? "recent-landing" : ""} ${recentCaptured ? "recent-captured" : ""} ${
                  promotes ? "promotes" : ""
                }`}
                role="gridcell"
                tabIndex={focusedCell === cell.cell ? 0 : -1}
                aria-disabled={!choice || !canPlay}
                aria-label={choice?.accessibility_label ?? cell.accessibility_label}
                data-testid={`draughts-cell-${cell.cell}`}
                onFocus={() => setFocusedCell(cell.cell)}
                onKeyDown={(event) => {
                  handleCellKey(event, cell, view, moveFocus, () => chooseCell(cell), cancelPending);
                }}
                onClick={() => chooseCell(cell)}
              >
                {cell.owner ? <Piece cell={cell} /> : <span className="draughts-target-dot" aria-hidden="true" />}
                <span className="draughts-coordinate">{cell.cell_id}</span>
                {legalCells.has(cell.cell) ? <span className="draughts-legal-ring" aria-hidden="true" /> : null}
              </button>
            );
          })}
        </div>
      </div>

      <div className="draughts-lite-controls">
        <div className="draughts-path" aria-live="polite">
          <span>Pending path</span>
          <code>{pendingLabel}</code>
        </div>
        <button type="button" onClick={cancelPending} disabled={pendingPath.length === 0}>
          Cancel
        </button>
      </div>

      <div className="draughts-lite-cues" aria-label="Rust-provided move cues">
        <Cue label="Choices" value={current.choices.length.toString()} />
        <Cue label="Captures" value={captureChoiceCount.toString()} />
        <Cue label="Promotion" value={countTagged(current.choices, "promotion").toString()} />
        <Cue label="Continuation" value={current.choices.some((choice) => choice.next?.choices?.length) ? "available" : "none"} />
      </div>

      {botEffect ? (
        <div className="bot-note" data-testid="bot-explanation">
          <span>Bot</span>
          <strong>{String(botEffect.effect.payload.rationale ?? "Rust bot selected a complete path.")}</strong>
        </div>
      ) : null}

      <div className="board-status" role="status">
        <span>
          {outcomeExplanation
            ? outcomeAnnouncementText(outcomeExplanation)
            : feedback
              ? feedback.detail
            : current.choices.some((choice) => metadataValue(choice, "forced_by_continuation") === "true")
              ? "Rust requires this capture path to continue."
              : pendingPath.length > 0
                ? "Choose one of the Rust-provided continuations."
                : interactive
                  ? "Choose a highlighted origin."
                  : "Replay board is projected by Rust at this cursor."}
        </span>
      </div>

      <p className="sr-only" aria-live="polite" data-testid="draughts-live-status">
        {liveStatus}
      </p>

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function Piece({ cell }: { cell: DraughtsLiteCellView }) {
  return (
    <span className={`draughts-piece ${cell.owner ?? ""} ${cell.piece_kind ?? ""}`} aria-label={cell.piece_accessibility_label ?? cell.piece_label ?? "Piece"}>
      <span className="draughts-piece-mark">{cell.piece_kind === "crown" ? "K" : ""}</span>
    </span>
  );
}

function Cue({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function handleCellKey(
  event: KeyboardEvent<HTMLButtonElement>,
  cell: DraughtsLiteCellView,
  view: DraughtsLitePublicView,
  moveFocus: (fromCell: string, target: { row: number; column: number }) => void,
  choose: () => void,
  cancel: () => void,
) {
  switch (event.key) {
    case "ArrowUp":
      event.preventDefault();
      moveFocus(cell.cell, { row: cell.row - 1, column: cell.column });
      break;
    case "ArrowDown":
      event.preventDefault();
      moveFocus(cell.cell, { row: cell.row + 1, column: cell.column });
      break;
    case "ArrowLeft":
      event.preventDefault();
      moveFocus(cell.cell, { row: cell.row, column: cell.column - 1 });
      break;
    case "ArrowRight":
      event.preventDefault();
      moveFocus(cell.cell, { row: cell.row, column: cell.column + 1 });
      break;
    case "Home":
      event.preventDefault();
      moveFocus(cell.cell, {
        row: event.ctrlKey ? 1 : cell.row,
        column: 1,
      });
      break;
    case "End":
      event.preventDefault();
      moveFocus(cell.cell, {
        row: event.ctrlKey ? view.board_rows : cell.row,
        column: view.board_columns,
      });
      break;
    case "Enter":
    case " ":
      event.preventDefault();
      choose();
      break;
    case "Escape":
      event.preventDefault();
      cancel();
      break;
  }
}

function currentNode(actionTree: ActionTree | null, pendingPath: string[]): CurrentNode {
  let choices = actionTree?.choices ?? [];
  let selectedChoice: ActionChoice | null = null;
  for (const segment of pendingPath) {
    const choice = choices.find((candidate) => candidate.segment === segment) ?? null;
    if (!choice) {
      return { choices: [], selectedChoice: null, selectedCell: null, validPath: false };
    }
    selectedChoice = choice;
    choices = choice.next?.choices ?? [];
  }
  return {
    choices,
    selectedChoice,
    selectedCell: selectedChoice ? metadataValue(selectedChoice, "cell_id") : null,
    validPath: true,
  };
}

function choicesForCells(choices: ActionChoice[]): Map<string, ActionChoice> {
  return new Map(
    choices.flatMap((choice) => {
      const cell = metadataValue(choice, "cell_id");
      return cell ? [[cell, choice] as const] : [];
    }),
  );
}

function metadataValue(choice: ActionChoice, key: string): string | null {
  return choice.metadata?.find((entry) => entry.key === key)?.value ?? null;
}

function metadataSet(choices: ActionChoice[], key: string): Set<string> {
  return new Set(choices.flatMap((choice) => metadataValue(choice, key) ?? []));
}

function countTagged(choices: ActionChoice[], tag: string): number {
  return choices.filter((choice) => choice.tags?.includes(tag)).length;
}

function countCaptureChoices(choices: ActionChoice[]): number {
  return choices.filter((choice) => choice.tags?.includes("capture") || metadataValue(choice, "is_capture") === "true").length;
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

function liveAnnouncement(
  view: DraughtsLitePublicView,
  current: CurrentNode,
  pendingPath: string[],
  feedback: ReturnType<typeof feedbackForEffect> | null,
  botEffect: EffectEntry | null,
): string {
  if (view.terminal_kind === "win") {
    return `${view.winning_seat} won. ${view.status_label}`;
  }

  const mandatoryCapture = current.choices.some((choice) => metadataValue(choice, "capture_mandatory") === "true");
  const forcedContinuation = current.choices.some((choice) => metadataValue(choice, "forced_by_continuation") === "true");
  const promotions = countTagged(current.choices, "promotion");
  const captures = countCaptureChoices(current.choices);
  const effectText = feedback
    ? `${feedback.detail} `
    : botEffect
      ? `${String(botEffect.effect.payload.rationale ?? "Rust bot selected a complete path.")} `
      : "";
  const selected = current.selectedCell ? `Selected ${current.selectedCell}. ` : "";
  const captureText = mandatoryCapture ? "Capture is mandatory. " : captures > 0 ? `${captures} capture destination${captures === 1 ? "" : "s"}. ` : "";
  const continuationText = forcedContinuation ? "Forced continuation is required. " : "";
  const promotionText = promotions > 0 ? `${promotions} promotion destination${promotions === 1 ? "" : "s"}. ` : "";
  const pathText = pendingPath.length > 0 ? `Pending path ${pendingPath.join(" then ")}. ` : "";

  return `${effectText}${view.active_seat} to move. ${selected}${pathText}${captureText}${continuationText}${promotionText}${current.choices.length} Rust-provided choice${current.choices.length === 1 ? "" : "s"}.`;
}

function recentEffectCells(entries: EffectEntry[]): { origins: Set<string>; landings: Set<string>; captures: Set<string> } {
  const origins = new Set<string>();
  const landings = new Set<string>();
  const captures = new Set<string>();
  for (const entry of entries.slice(-6)) {
    const payload = entry.effect.payload;
    addString(origins, payload.start_cell);
    addString(origins, payload.origin);
    addString(landings, payload.final_cell);
    addString(landings, payload.landing);
    addString(landings, payload.cell);
    addString(captures, payload.captured_cell);
  }
  return { origins, landings, captures };
}

function addString(target: Set<string>, value: unknown) {
  if (typeof value === "string") {
    target.add(value);
  }
}

function terminalLabel(view: DraughtsLitePublicView): string {
  if (view.terminal_kind === "win") {
    return `${view.winning_seat} wins`;
  }
  return `${view.active_seat} to move`;
}

function boardSummary(view: DraughtsLitePublicView, choices: ActionChoice[]): string {
  const occupied = view.cells
    .filter((cell) => cell.owner)
    .map((cell) => `${cell.cell_id} ${cell.owner} ${cell.piece_kind}`)
    .join(", ");
  const legal = choices.map((choice) => choice.label).join(", ");
  return `${view.status_label}. Occupied: ${occupied || "none"}. Legal choices: ${legal || "none"}.`;
}

function otherSeat(seat: "seat_0" | "seat_1"): "seat_0" | "seat_1" {
  return seat === "seat_0" ? "seat_1" : "seat_0";
}

function pieceCount(seat: "seat_0" | "seat_1", cells: DraughtsLiteCellView[]): number {
  return cells.filter((cell) => cell.owner === seat).length;
}

function pieceStanding(seat: "seat_0" | "seat_1", winner: "seat_0" | "seat_1", cells: DraughtsLiteCellView[]) {
  return {
    id: seat,
    label: seat,
    result: winner === seat ? "Winner" : "Loss",
    emphasized: winner === seat,
    values: [{ label: "Pieces", value: pieceCount(seat, cells) }],
  };
}
