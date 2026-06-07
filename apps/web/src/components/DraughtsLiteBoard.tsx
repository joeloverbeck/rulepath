import { useMemo, type KeyboardEvent } from "react";
import type { ActionChoice, ActionTree, DraughtsLiteCellView, DraughtsLitePublicView, EffectEntry } from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";

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
                className={`draughts-cell ${cell.playable ? "playable" : "non-playable"} ${cell.owner ?? "empty"} ${
                  choice ? "legal" : ""
                } ${selected ? "selected" : ""} ${previewCapture ? "capture-preview" : ""} ${
                  recentOrigin ? "recent-origin" : ""
                } ${recentLanding ? "recent-landing" : ""} ${recentCaptured ? "recent-captured" : ""} ${
                  promotes ? "promotes" : ""
                }`}
                role="gridcell"
                disabled={!choice || !canPlay}
                aria-label={choice?.accessibility_label ?? cell.accessibility_label}
                data-testid={`draughts-cell-${cell.cell}`}
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
        <Cue label="Captures" value={countTagged(current.choices, "capture").toString()} />
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
          {feedback
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

function latestEffectOfType(entries: EffectEntry[], type: string): EffectEntry | null {
  for (let index = entries.length - 1; index >= 0; index -= 1) {
    const entry = entries[index];
    if (entry.effect.payload.type === type) {
      return entry;
    }
  }
  return null;
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
