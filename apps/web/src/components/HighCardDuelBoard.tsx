import { useMemo } from "react";
import type {
  ActionChoice,
  ActionTree,
  EffectEntry,
  HighCardDuelCardView,
  HighCardDuelPublicView,
  SeatId,
  ViewerMode,
} from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type HighCardDuelBoardProps = {
  view: HighCardDuelPublicView;
  actionTree: ActionTree | null;
  viewerMode: ViewerMode;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onChoice?: (choice: ActionChoice) => void;
  onViewerModeChange?: (viewerMode: ViewerMode) => void;
};

const VIEWER_OPTIONS: Array<{ label: string; mode: ViewerMode }> = [
  { label: "Seat 0", mode: { kind: "seat", seat: "seat_0" } },
  { label: "Seat 1", mode: { kind: "seat", seat: "seat_1" } },
  { label: "Observer", mode: { kind: "observer" } },
];

export function HighCardDuelBoard({
  view,
  actionTree,
  viewerMode,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onChoice,
  onViewerModeChange,
}: HighCardDuelBoardProps) {
  const terminal = view.terminal_kind !== "non_terminal";
  const viewerSeat = viewerMode.kind === "seat" ? viewerMode.seat : null;
  const canAct = Boolean(interactive && !pending && !terminal && viewerSeat && viewerSeat === view.active_seat);
  const choices = canAct ? actionTree?.choices ?? [] : [];
  const commitChoices = useMemo(() => choices.map(commitChoiceSummary), [choices]);
  const revealEffect = latestEffectOfType(effects, "cards_revealed");
  const commitEffect = latestEffectOfType(effects, "commit_face_down");
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const viewerLabel = viewerMode.kind === "observer" ? "Observer" : seatLabel(viewerMode.seat);

  return (
    <section
      className={`high-card-duel-board ${terminal ? "terminal" : ""}${revealEffect ? " reveal" : ""}${
        reducedMotion ? " reduced" : ""
      }`}
      aria-labelledby="high-card-duel-heading"
    >
      <div className="high-card-duel-banner">
        <div>
          <p className="eyebrow">High Card Duel</p>
          <h2 id="high-card-duel-heading">{statusLabel(view)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {terminalLabel(view)}
        </span>
      </div>

      <div className="high-card-viewer-controls" role="group" aria-label="Viewer mode">
        {VIEWER_OPTIONS.map((option) => {
          const selected = sameViewerMode(option.mode, viewerMode);
          return (
            <button
              type="button"
              key={option.label}
              className={selected ? "selected" : ""}
              aria-pressed={selected}
              onClick={() => onViewerModeChange?.(option.mode)}
            >
              {option.label}
            </button>
          );
        })}
      </div>

      <p className="sr-only" aria-live="polite">
        {boardSummary(view, viewerLabel)}
      </p>

      <div className="high-card-score" aria-label="Match status">
        <Metric label="Viewer" value={viewerLabel} />
        <Metric label="Round" value={`${view.round_number} / ${view.round_limit}`} />
        <Metric label="Score" value={`${view.score.seat_0} - ${view.score.seat_1}`} />
        <Metric label="Deck" value={`${view.deck_count}`} />
      </div>

      <div className="duel-table" data-testid="high-card-duel-board">
        <SeatPanel
          seat="seat_0"
          view={view}
          viewerSeat={viewerSeat}
          active={view.active_seat === "seat_0"}
          revealEffect={Boolean(revealEffect)}
        />
        <div className="duel-center" aria-label="Commitments">
          <CommitmentSlot seat="seat_0" view={view} />
          <div className="duel-vs" aria-hidden="true">
            vs
          </div>
          <CommitmentSlot seat="seat_1" view={view} />
        </div>
        <SeatPanel
          seat="seat_1"
          view={view}
          viewerSeat={viewerSeat}
          active={view.active_seat === "seat_1"}
          revealEffect={Boolean(revealEffect)}
        />
      </div>

      <div className="high-card-actions" aria-label="Rust legal commit actions">
        <div>
          <span>Action source</span>
          <strong>{canAct ? "Rust legal tree" : viewerSeat ? "Waiting for active seat" : "Observer only"}</strong>
        </div>
        {commitChoices.length === 0 ? (
          <p className="muted">No private commit actions for this viewer.</p>
        ) : (
          <div className="high-card-hand-actions">
            {commitChoices.map(({ choice, label, description }, index) => (
              <button
                type="button"
                key={choice.segment}
                disabled={!canAct}
                aria-label={description}
                data-testid={`high-card-commit-${index}`}
                onClick={() => onChoice?.(choice)}
              >
                {label}
              </button>
            ))}
          </div>
        )}
      </div>

      <div className="board-status" role="status">
        <span>
          {feedback
            ? feedback.detail
            : commitEffect
              ? "A face-down commitment was recorded by Rust."
              : "Choose only from the Rust-provided private hand actions."}
        </span>
      </div>

      {terminal ? (
        <OutcomeExplanationPanel
          reducedMotion={reducedMotion}
          explanation={outcomeSurfaceData({
            gameId: "high_card_duel",
            heading: terminalLabel(view),
            rationale: view.terminal_rationale,
            resultKind: view.terminal_kind === "draw" ? "draw" : "win",
            decisiveCause: "final_score",
            templateKey:
              view.terminal_kind === "draw" ? "high_card_duel.final_score_draw" : "high_card_duel.final_score_win",
            templateParams: { winner: view.winning_seat ?? "" },
            finalStanding: [
              scoreStanding("seat_0", view.winning_seat, view.score.seat_0),
              scoreStanding("seat_1", view.winning_seat, view.score.seat_1),
            ],
            breakdownSections: [
              {
                id: "revealed-rounds",
                heading: "Public revealed rounds",
                rows: [
                  { label: "Revealed rounds", value: view.revealed_cards.length },
                  { label: "seat_0 score", value: view.score.seat_0 },
                  { label: "seat_1 score", value: view.score.seat_1 },
                ],
              },
            ],
          })}
        />
      ) : null}
    </section>
  );
}

function SeatPanel({
  seat,
  view,
  viewerSeat,
  active,
  revealEffect,
}: {
  seat: SeatId;
  view: HighCardDuelPublicView;
  viewerSeat: SeatId | null;
  active: boolean;
  revealEffect: boolean;
}) {
  const ownPrivate =
    view.private_view.status === "seat" && view.private_view.seat === seat ? view.private_view : null;
  const handCount = seat === "seat_0" ? view.hand_counts.seat_0 : view.hand_counts.seat_1;
  const visibleHand = ownPrivate?.hand ?? [];
  const isViewer = viewerSeat === seat;

  return (
    <section className={`duel-seat ${seat} ${active ? "active" : ""}`} aria-label={`${seatLabel(seat)} area`}>
      <div className="duel-seat-heading">
        <span>{seatLabel(seat)}</span>
        <strong>{active ? "Active" : isViewer ? "Viewer" : `${handCount} held`}</strong>
      </div>
      <div className="duel-hand" aria-label={isViewer ? `${seatLabel(seat)} private hand` : `${seatLabel(seat)} hand count`}>
        {isViewer
          ? visibleHand.map((card, index) => <CardFace key={card.card_id} card={card} reveal={revealEffect} index={index} />)
          : Array.from({ length: handCount }, (_, index) => <CardBack key={index} index={index} />)}
      </div>
    </section>
  );
}

function CommitmentSlot({ seat, view }: { seat: SeatId; view: HighCardDuelPublicView }) {
  const commitment = seat === "seat_0" ? view.commitments.seat_0 : view.commitments.seat_1;
  return (
    <div className={`duel-commitment ${commitment.status}`} aria-label={`${seatLabel(seat)} commitment ${commitment.status}`}>
      <span>{seatLabel(seat)}</span>
      {commitment.card ? <CardFace card={commitment.card} reveal index={0} compact /> : <div className="duel-card back" />}
      <strong>{commitment.status === "empty" ? "Open" : commitment.status === "face_down" ? "Face down" : "Committed"}</strong>
    </div>
  );
}

function CardFace({
  card,
  reveal,
  index,
  compact = false,
}: {
  card: HighCardDuelCardView;
  reveal: boolean;
  index: number;
  compact?: boolean;
}) {
  return (
    <div
      className={`duel-card face ${reveal ? "revealed" : ""} ${compact ? "compact" : ""}`}
      aria-label={`Private rank ${card.rank} ${card.sigil} card`}
      style={{ ["--card-index" as string]: index }}
    >
      <span>{card.rank}</span>
      <small>{card.sigil}</small>
    </div>
  );
}

function CardBack({ index }: { index: number }) {
  return (
    <div className="duel-card back" aria-label="Hidden card back" style={{ ["--card-index" as string]: index }}>
      <span aria-hidden="true" />
    </div>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function commitChoiceSummary(choice: ActionChoice): { choice: ActionChoice; label: string; description: string } {
  const rank = metadataValue(choice, "rank") ?? "?";
  const sigil = metadataValue(choice, "sigil") ?? "card";
  return {
    choice,
    label: `Rank ${rank} ${sigil}`,
    description: `Commit rank ${rank} ${sigil} face-down`,
  };
}

function metadataValue(choice: ActionChoice, key: string): string | null {
  return choice.metadata?.find((entry) => entry.key === key)?.value ?? null;
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

function statusLabel(view: HighCardDuelPublicView): string {
  if (view.terminal_kind === "win") {
    return `${seatLabel(view.winning_seat ?? "seat_0")} wins`;
  }
  if (view.terminal_kind === "draw") {
    return "Drawn duel";
  }
  return `${phaseLabel(view.phase)} - round ${view.round_number}`;
}

function terminalLabel(view: HighCardDuelPublicView): string {
  if (view.terminal_kind === "draw") {
    return "Draw";
  }
  if (view.terminal_kind === "win") {
    return `${view.winning_seat} wins`;
  }
  return view.active_seat ? `${view.active_seat} to commit` : "Resolving";
}

function boardSummary(view: HighCardDuelPublicView, viewerLabel: string): string {
  return `${viewerLabel} view. ${statusLabel(view)}. Score ${view.score.seat_0} to ${view.score.seat_1}. Hand counts ${view.hand_counts.seat_0} and ${view.hand_counts.seat_1}.`;
}

function phaseLabel(phase: HighCardDuelPublicView["phase"]): string {
  switch (phase) {
    case "lead_commit":
      return "Lead commit";
    case "reply_commit":
      return "Reply commit";
    case "revealed":
      return "Revealed";
    case "terminal":
      return "Terminal";
  }
}

function seatLabel(seat: SeatId): string {
  return seat === "seat_0" ? "Seat 0" : "Seat 1";
}

function sameViewerMode(left: ViewerMode, right: ViewerMode): boolean {
  if (left.kind !== right.kind) {
    return false;
  }
  return left.kind === "observer" || left.seat === (right.kind === "seat" ? right.seat : null);
}

function scoreStanding(seat: SeatId, winner: SeatId | null, score: number) {
  return {
    id: seat,
    label: seatLabel(seat),
    result: winner === seat ? "Winner" : winner ? "Loss" : "Draw",
    emphasized: winner === seat,
    values: [{ label: "Score", value: score }],
  };
}
