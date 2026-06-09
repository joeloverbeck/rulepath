import { useMemo } from "react";
import type { ActionChoice, ActionTree, EffectEntry, PlainTricksCardView, PlainTricksPublicView, SeatId } from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type PlainTricksBoardProps = {
  view: PlainTricksPublicView;
  actionTree: ActionTree | null;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onPathSubmit?: (path: string[]) => void;
};

export function PlainTricksBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onPathSubmit,
}: PlainTricksBoardProps) {
  const playChoices = useMemo(() => cardChoices(actionTree), [actionTree]);
  const legalCards = useMemo(() => new Map(playChoices.map((choice) => [choice.segment, choice] as const)), [playChoices]);
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const terminal = view.terminal.kind !== "non_terminal";
  const canAct = Boolean(interactive && !pending && !terminal && view.private_view.status === "seat" && playChoices.length > 0);
  const revealActive = effects.some((entry) => isTrickEffect(entry.effect.payload.type));
  const ownSeat = view.private_view.status === "seat" ? view.private_view.seat : null;
  const opponentSeat = ownSeat === "seat_0" ? "seat_1" : "seat_0";
  const opponentCount = ownSeat ? view.hand_counts[opponentSeat] : view.hand_counts.seat_0 + view.hand_counts.seat_1;

  return (
    <section
      className={`plain-tricks-board ${terminal ? "terminal" : ""}${revealActive ? " reveal" : ""}${
        reducedMotion ? " reduced" : ""
      }`}
      aria-labelledby="plain-tricks-heading"
      data-testid="plain-tricks-board"
    >
      <div className="plain-tricks-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="plain-tricks-heading">{statusLabel(view)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {terminalLabel(view)}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {boardSummary(view, playChoices.length)}
      </p>

      <div className="plain-tricks-metrics" aria-label="Plain Tricks status">
        <Metric label="Round" value={`${view.round_index + 1} / 2`} />
        <Metric label="Trick" value={`${view.trick_index + 1} / 6`} />
        <Metric label={view.ui.score_label} value={`${view.total_trick_counts.seat_0} - ${view.total_trick_counts.seat_1}`} />
        <Metric label="Lead" value={seatLabel(view.current_leader)} />
      </div>

      <div className="plain-tricks-table" aria-label={view.ui.table_label}>
        <section className="plain-seat active" aria-label={ownSeat ? `${seatLabel(ownSeat)} private hand` : "Observer hand state"}>
          <div className="plain-section-heading">
            <span>{view.ui.own_hand_label}</span>
            <strong>{privateHeading(view)}</strong>
          </div>
          {view.private_view.status === "seat" ? (
            <div className="plain-hand" data-testid="plain-tricks-own-hand">
              {view.private_view.own_hand.map((card, index) => {
                const choice = legalCards.get(card.card_id) ?? null;
                return (
                  <button
                    type="button"
                    key={card.card_id}
                    className={`plain-card ${card.suit} ${choice ? "legal" : ""}`}
                    disabled={!canAct || !choice}
                    aria-label={choice?.accessibility_label ?? card.accessibility_label}
                    data-testid={`choice-plain-tricks-trick-${view.trick_index}-${index}`}
                    onClick={() => choice && onPathSubmit?.(["play", choice.segment])}
                  >
                    <span>{card.suit}</span>
                    <strong>{card.rank}</strong>
                    <small>{choice ? "Legal" : "Held"}</small>
                  </button>
                );
              })}
            </div>
          ) : (
            <FaceDownCount count={opponentCount} label={view.ui.observer_disabled_reason} testId="plain-tricks-observer-hand" />
          )}
        </section>

        <section className="plain-trick-surface" aria-label={view.ui.current_trick_label}>
          <div className="plain-section-heading">
            <span>{view.ui.current_trick_label}</span>
            <strong>{view.current_trick.led_suit ? `Led ${view.current_trick.led_suit}` : "No suit led"}</strong>
          </div>
          <div className="plain-played-row">
            {view.current_trick.plays.length === 0 ? (
              <p className="muted">Waiting for the first card.</p>
            ) : (
              view.current_trick.plays.map((play) => (
                <div key={`${play.seat}-${play.card.card_id}`} className={`plain-played-card ${play.card.suit}`}>
                  <span>{seatLabel(play.seat)}</span>
                  <strong>{play.card.label}</strong>
                  <small>{play.card.accessibility_label}</small>
                </div>
              ))
            )}
          </div>
        </section>

        <section className="plain-seat opponent" aria-label={view.ui.opponent_hand_label}>
          <div className="plain-section-heading">
            <span>{view.ui.opponent_hand_label}</span>
            <strong>{ownSeat ? seatLabel(opponentSeat) : "Both seats"}</strong>
          </div>
          <FaceDownCount count={opponentCount} label="Cards hidden" testId="plain-tricks-opponent-hand" />
        </section>
      </div>

      <section className="plain-history" aria-label={view.ui.trick_history_label}>
        <div className="plain-section-heading">
          <span>{view.ui.trick_history_label}</span>
          <strong>{view.trick_history.length} resolved</strong>
        </div>
        {view.trick_history.length === 0 ? (
          <p className="muted">Resolved tricks will appear here.</p>
        ) : (
          <ol>
            {view.trick_history.slice(-6).map((trick) => (
              <li key={`${trick.round_index}-${trick.trick_index}`}>
                <span>
                  R{trick.round_index + 1} T{trick.trick_index + 1}
                </span>
                <strong>{seatLabel(trick.winner)}</strong>
                <small>
                  {trick.plays.map((play) => `${seatLabel(play.seat)} ${play.card.label}`).join(" / ")}
                </small>
              </li>
            ))}
          </ol>
        )}
      </section>

      <div className="plain-latest" role="status">
        <span>{feedback?.title ?? "Waiting"}</span>
        <strong>{feedback?.detail ?? "Rust/WASM supplies legal cards, trick results, and redacted views."}</strong>
      </div>

      {terminal ? (
        <OutcomeExplanationPanel
          reducedMotion={reducedMotion}
          explanation={outcomeSurfaceData({
            gameId: "plain_tricks",
            heading: terminalLabel(view),
            rationale: view.terminal_rationale ?? null,
            resultKind: view.terminal.draw ? "draw" : "win",
            decisiveCause: view.terminal.kind,
            templateKey: view.terminal.kind === "split" ? "plain_tricks.split" : "plain_tricks.trick_win",
            templateParams: { winner: view.terminal.winner ?? "" },
            finalStanding: [
              plainStanding("seat_0", view),
              plainStanding("seat_1", view),
            ],
            breakdownSections: [
              {
                id: "tricks",
                heading: "Trick totals",
                rows: [
                  { label: "seat_0 tricks", value: view.total_trick_counts.seat_0 },
                  { label: "seat_1 tricks", value: view.total_trick_counts.seat_1 },
                  { label: "Resolved tricks", value: view.trick_history.length },
                ],
              },
            ],
          })}
        />
      ) : null}
    </section>
  );
}

function cardChoices(actionTree: ActionTree | null): ActionChoice[] {
  return actionTree?.choices.find((choice) => choice.segment === "play")?.next?.choices ?? [];
}

function FaceDownCount({ count, label, testId }: { count: number; label: string; testId: string }) {
  return (
    <div className="plain-facedown" data-testid={testId}>
      <span>{label}</span>
      <strong>{count}</strong>
      <small>{count === 1 ? "card" : "cards"}</small>
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

function plainStanding(seat: SeatId, view: PlainTricksPublicView) {
  const total = view.total_trick_counts[seat];
  const winner = view.terminal.kind !== "non_terminal" ? view.terminal.winner : null;
  return {
    id: seat,
    label: seatLabel(seat),
    result: winner === seat ? "win" : view.terminal.draw ? "split" : "loss",
    emphasized: winner === seat,
    values: [{ label: "Tricks", value: total }],
  };
}

function isTrickEffect(type: string): boolean {
  return type === "card_played" || type === "trick_resolved" || type === "round_scored" || type === "deal_rotated";
}

function privateHeading(view: PlainTricksPublicView): string {
  return view.private_view.status === "seat" ? `${seatLabel(view.private_view.seat)} view` : "Observer";
}

function statusLabel(view: PlainTricksPublicView): string {
  if (view.terminal.kind === "split") {
    return "Split match";
  }
  if (view.terminal.kind === "trick_win") {
    return `${seatLabel(view.terminal.winner)} wins`;
  }
  return view.active_seat ? `${seatLabel(view.active_seat)} to play` : "Resolving trick";
}

function terminalLabel(view: PlainTricksPublicView): string {
  if (view.terminal.kind === "split") {
    return "Split";
  }
  if (view.terminal.kind === "trick_win") {
    return `${view.terminal.winner} won`;
  }
  return view.active_seat ?? "Terminal";
}

function boardSummary(view: PlainTricksPublicView, legalCount: number): string {
  return `${view.display_name}, ${statusLabel(view)}, round ${view.round_index + 1}, trick ${
    view.trick_index + 1
  }, ${legalCount} Rust legal card choices.`;
}

function seatLabel(seat: SeatId): string {
  return seat === "seat_0" ? "Seat 0" : "Seat 1";
}
