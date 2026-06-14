import { useMemo } from "react";
import type {
  ActionChoice,
  ActionTree,
  EffectEntry,
  RiverLedgerCardView,
  RiverLedgerPublicView,
  RiverLedgerSeatId,
  RiverLedgerSeatView,
} from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";

type RiverLedgerBoardProps = {
  view: RiverLedgerPublicView;
  actionTree: ActionTree | null;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onChoice?: (choice: ActionChoice) => void;
};

export function RiverLedgerBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onChoice,
}: RiverLedgerBoardProps) {
  const choices = useMemo(() => actionTree?.choices ?? [], [actionTree]);
  const canAct = Boolean(interactive && !pending && !view.terminal.terminal && view.active_seat && choices.length > 0);
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const boardChanged = effects.some((entry) => String(entry.effect.payload.type).includes("board"));

  return (
    <section
      className={`river-ledger-board ${view.terminal.terminal ? "terminal" : ""}${boardChanged ? " reveal" : ""}${
        reducedMotion ? " reduced" : ""
      }`}
      aria-labelledby="river-ledger-heading"
      data-testid="river-ledger-board"
    >
      <div className="river-ledger-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="river-ledger-heading">{statusLabel(view)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {view.active_seat ? seatLabel(view.active_seat) : terminalLabel(view)}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {view.display_name}, {phaseLabel(view.phase)}, pot {view.pot_total}, {view.board.length} public cards.
      </p>

      <div className="river-ledger-metrics" aria-label="River Ledger status">
        <Metric label="Pot" value={String(view.pot_total)} />
        <Metric label="Street" value={phaseLabel(view.phase)} />
        <Metric label="Button" value={seatLabel(view.button)} />
        <Metric label="Blinds" value={`${seatLabel(view.small_blind)} / ${seatLabel(view.big_blind)}`} />
      </div>

      <div className="river-ledger-layout" aria-label={view.ui.surface_label}>
        <section className="river-ledger-seats" aria-label={view.ui.seat_metadata_label}>
          {view.seats.map((seat) => (
            <SeatLedger key={seat.seat} view={view} seat={seat} />
          ))}
        </section>

        <section className="river-ledger-center" aria-label={view.ui.board_label}>
          <div className="river-ledger-section-heading">
            <span>Board</span>
            <strong>{view.board.length ? `${view.board.length} public` : "No public cards"}</strong>
          </div>
          <div className="river-ledger-board-cards">
            {view.board.map((card) => (
              <RiverCard key={card.card_id} card={card} tone="board" />
            ))}
            {Array.from({ length: Math.max(0, 5 - view.board.length) }, (_, index) => (
              <div className="river-ledger-card hidden" key={`hidden-board-${index}`}>
                <span>Hidden</span>
                <strong>Pending</strong>
              </div>
            ))}
          </div>
          <ContributionTrack seats={view.seats} />
        </section>
      </div>

      <section className="river-ledger-private" aria-label="Private view">
        <div className="river-ledger-section-heading">
          <span>Private view</span>
          <strong>{privateHeading(view)}</strong>
        </div>
        {view.private_view.status === "seat" ? (
          <div className="river-ledger-private-cards">
            {view.private_view.hole_cards.map((card) => (
              <RiverCard key={card.card_id} card={card} tone="private" />
            ))}
          </div>
        ) : (
          <div className="river-ledger-hidden" data-testid="river-ledger-private-hidden">
            <span>Hidden</span>
            <strong>{view.ui.hidden_hole_label}</strong>
          </div>
        )}
      </section>

      <section className="river-ledger-actions" aria-label={view.ui.action_hint_label}>
        <div className="river-ledger-section-heading">
          <span>Actions</span>
          <strong>{canAct ? "Available choices" : actionStatus(view, pending)}</strong>
        </div>
        <div className="river-ledger-action-grid">
          {choices.length === 0 ? (
            <p className="muted">No actions available.</p>
          ) : (
            choices.map((choice, index) => (
              <button
                type="button"
                key={choice.segment}
                className="river-ledger-action"
                disabled={!canAct}
                aria-label={choice.accessibility_label}
                data-testid={`choice-river-ledger-${index}`}
                onClick={() => onChoice?.(choice)}
              >
                <strong>{choice.label}</strong>
                <small>{actionDetail(choice)}</small>
              </button>
            ))
          )}
        </div>
      </section>

      {view.terminal.terminal ? <OutcomePanel view={view} /> : null}

      <div className="river-ledger-latest" role="status">
        <span>{view.terminal.terminal ? "Outcome" : feedback?.title ?? "Waiting"}</span>
        <strong>{view.terminal.terminal ? terminalLabel(view) : feedback?.detail ?? "Visible state changes will update here."}</strong>
      </div>
    </section>
  );
}

function SeatLedger({ view, seat }: { view: RiverLedgerPublicView; seat: RiverLedgerSeatView }) {
  const active = view.active_seat === seat.seat;
  const markers = [
    view.button === seat.seat ? "Button" : null,
    view.small_blind === seat.seat ? "SB" : null,
    view.big_blind === seat.seat ? "BB" : null,
  ].filter(Boolean);

  return (
    <section className={`river-ledger-seat ${active ? "active" : ""}`} aria-label={`${seatLabel(seat.seat)} ledger`}>
      <div className="river-ledger-section-heading">
        <span>{seatLabel(seat.seat)}</span>
        <strong>{active ? "Active" : seatStatusLabel(seat.status)}</strong>
      </div>
      {markers.length ? <div className="river-ledger-markers">{markers.map((marker) => <b key={marker}>{marker}</b>)}</div> : null}
      <Metric label="Street" value={String(seat.street_contribution)} />
      <Metric label="Total" value={String(seat.total_contribution)} />
      <Metric label="Private" value={String(seat.hidden_hole_count)} />
    </section>
  );
}

function ContributionTrack({ seats }: { seats: RiverLedgerSeatView[] }) {
  const maxContribution = Math.max(1, ...seats.map((seat) => seat.total_contribution));
  return (
    <div className="river-ledger-track" aria-label="Contribution ledger">
      {seats.map((seat) => (
        <div key={seat.seat}>
          <span>{seatLabel(seat.seat)}</span>
          <div className="river-ledger-track-bar">
            <span style={{ inlineSize: `${Math.max(8, (seat.total_contribution / maxContribution) * 100)}%` }} />
          </div>
          <strong>{seat.total_contribution}</strong>
        </div>
      ))}
    </div>
  );
}

function OutcomePanel({ view }: { view: RiverLedgerPublicView }) {
  return (
    <section className="river-ledger-outcome" aria-label={view.ui.outcome_explanation_label}>
      <div className="river-ledger-section-heading">
        <span>Outcome</span>
        <strong>{terminalLabel(view)}</strong>
      </div>
      <div className="river-ledger-allocations">
        {view.terminal.allocations.map((allocation) => (
          <div key={allocation.seat}>
            <span>{seatLabel(allocation.seat)}</span>
            <strong>{allocation.amount}</strong>
          </div>
        ))}
      </div>
      {view.terminal.explanations.length ? (
        <ul className="river-ledger-explanations">
          {view.terminal.explanations.map((explanation) => (
            <li key={explanation}>{explanation}</li>
          ))}
        </ul>
      ) : null}
    </section>
  );
}

function RiverCard({ card, tone }: { card: RiverLedgerCardView; tone: "board" | "private" }) {
  return (
    <div className={`river-ledger-card ${tone}`} aria-label={card.accessibility_label}>
      <span>{card.suit}</span>
      <strong>{card.label}</strong>
      <small>{card.rank}</small>
    </div>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div className="river-ledger-metric">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function actionDetail(choice: ActionChoice): string {
  const metadata = choice.metadata ?? [];
  const adds = metadata.find((entry) => entry.key === "adds_to_pot")?.value;
  const required = metadata.find((entry) => entry.key === "required_to_call")?.value;
  const potAfter = metadata.find((entry) => entry.key === "pot_after")?.value;
  if (adds && adds !== "0") {
    return potAfter ? `Adds ${adds}, pot ${potAfter}` : `Adds ${adds}`;
  }
  if (required && required !== "0") {
    return `Matches ${required}`;
  }
  return "No units added";
}

function actionStatus(view: RiverLedgerPublicView, pending: boolean): string {
  if (pending) return "Applying";
  if (view.terminal.terminal) return "Complete";
  return view.active_seat ? `${seatLabel(view.active_seat)} to choose` : "Waiting";
}

function statusLabel(view: RiverLedgerPublicView): string {
  if (view.terminal.terminal) return terminalLabel(view);
  return view.active_seat ? `${seatLabel(view.active_seat)} to choose` : "Resolving";
}

function terminalLabel(view: RiverLedgerPublicView): string {
  if (!view.terminal.terminal) return "In progress";
  if (view.terminal.winners.length === 0) return "Complete";
  if (view.terminal.winners.length === 1) return `${seatLabel(view.terminal.winners[0])} wins`;
  return `${view.terminal.winners.length} seats split`;
}

function privateHeading(view: RiverLedgerPublicView): string {
  if (view.private_view.status === "seat") return `${seatLabel(view.private_view.seat)} view`;
  return "Observer view";
}

function phaseLabel(phase: string): string {
  return phase.replace(/_/g, " ").replace(/\b\w/g, (letter) => letter.toUpperCase());
}

function seatStatusLabel(status: string): string {
  return phaseLabel(status);
}

function seatLabel(seat: RiverLedgerSeatId): string {
  return `Seat ${seat.replace("seat_", "")}`;
}
