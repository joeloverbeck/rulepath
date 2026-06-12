import { useMemo } from "react";
import type { ActionChoice, ActionTree, EffectEntry, PokerLiteCardView, PokerLitePublicView, SeatId } from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type PokerLiteBoardProps = {
  view: PokerLitePublicView;
  actionTree: ActionTree | null;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onChoice?: (choice: ActionChoice) => void;
};

export function PokerLiteBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onChoice,
}: PokerLiteBoardProps) {
  const choices = useMemo(() => actionTree?.choices ?? [], [actionTree]);
  const canAct = Boolean(interactive && !pending && !view.terminal.terminal && view.active_seat && choices.length > 0);
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const revealActive = effects.some((entry) => isRevealEffect(entry.effect.payload.type));
  const outcomeExplanation = view.terminal.terminal
    ? outcomeSurfaceData({
        gameId: "poker_lite",
        heading: terminalLabel(view),
        rationale: view.terminal_rationale,
        resultKind: view.terminal.draw ? "split" : "win",
        decisiveCause: view.terminal.kind,
        templateKey: pokerTemplateKey(view),
        templateParams: {
          winner: view.terminal.winner ?? "",
          loser: view.terminal.kind === "yield_win" ? view.terminal.loser : "",
        },
        finalStanding: [
          pokerStanding("seat_0", view.terminal.winner, view),
          pokerStanding("seat_1", view.terminal.winner, view),
        ],
        breakdownSections: [
          {
            id: "ledger",
            heading: "Public ledger",
            rows: [
              { label: "Terminal kind", value: view.terminal.kind },
              { label: "Shared pool", value: view.terminal.shared_pool },
              { label: "seat_0 contribution", value: view.contributions.seat_0 },
              { label: "seat_1 contribution", value: view.contributions.seat_1 },
            ],
          },
        ],
      })
    : null;

  return (
    <section
      className={`poker-lite-board ${view.terminal.terminal ? "terminal" : ""}${revealActive ? " reveal" : ""}${
        reducedMotion ? " reduced" : ""
      }`}
      aria-labelledby="poker-lite-heading"
      data-testid="poker-lite-board"
    >
      <div className="poker-lite-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="poker-lite-heading">{statusLabel(view)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {view.active_seat ?? terminalLabel(view)}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {boardSummary(view)}
      </p>

      <div className="poker-lite-metrics" aria-label="Crest Ledger status">
        <Metric label={view.ui.shared_pool_label} value={String(view.shared_pool)} />
        <Metric label="Round" value={`${view.round.round_index + 1} / 2`} />
        <Metric label="Round unit" value={String(view.round.round_unit)} />
        <Metric label="To answer" value={view.round.outstanding_actor ? seatLabel(view.round.outstanding_actor) : "None"} />
      </div>

      <div className="poker-lite-table" aria-label={view.ui.surface_label}>
        <SeatLedger view={view} seat="seat_0" />

        <section className="poker-lite-center" aria-label="Center crest">
          <div className="poker-lite-section-heading">
            <span>Center</span>
            <strong>{view.center.status === "revealed" ? "Revealed" : view.ui.hidden_center_label}</strong>
          </div>
          {view.center.status === "revealed" && view.center.card ? (
            <CrestCard card={view.center.card} tone="center" />
          ) : (
            <div className="poker-lite-hidden" data-testid="poker-lite-center-hidden">
              <span>Hidden</span>
              <strong>Center crest</strong>
            </div>
          )}
          <ContributionTrack view={view} />
        </section>

        <SeatLedger view={view} seat="seat_1" />
      </div>

      <section className="poker-lite-private" aria-label="Private crest">
        <div className="poker-lite-section-heading">
          <span>Private view</span>
          <strong>{privateHeading(view)}</strong>
        </div>
        {view.private_view.status === "seat" && view.private_view.own_private ? (
          <CrestCard card={view.private_view.own_private} tone="private" />
        ) : (
          <div className="poker-lite-hidden" data-testid="poker-lite-private-hidden">
            <span>Hidden</span>
            <strong>{view.ui.hidden_private_label}</strong>
          </div>
        )}
      </section>

      <section className="poker-lite-actions" aria-label="Legal Crest Ledger actions">
        <div className="poker-lite-section-heading">
          <span>Actions</span>
          <strong>{canAct ? "Available choices" : actionStatus(view, pending)}</strong>
        </div>
        <div className="poker-lite-action-grid">
          {choices.length === 0 ? (
            <p className="muted">No actions available.</p>
          ) : (
            choices.map((choice, index) => (
              <button
                type="button"
                key={choice.segment}
                className="poker-lite-action"
                disabled={!canAct}
                aria-label={choice.accessibility_label}
                data-testid={`choice-poker-lite-round-${view.round.round_index}-${index}`}
                onClick={() => onChoice?.(choice)}
              >
                <strong>{choice.label}</strong>
                <small>{actionDetail(choice)}</small>
              </button>
            ))
          )}
        </div>
      </section>

      {view.showdown ? <ShowdownPanel view={view} /> : null}
      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}

      <div className="poker-lite-latest" role="status">
        <span>{outcomeExplanation ? "Outcome" : feedback?.title ?? "Waiting"}</span>
        <strong>
          {outcomeExplanation
            ? outcomeAnnouncementText(outcomeExplanation)
            : feedback?.detail ?? "Visible state changes will update here."}
        </strong>
      </div>
    </section>
  );
}

function SeatLedger({ view, seat }: { view: PokerLitePublicView; seat: SeatId }) {
  const active = view.active_seat === seat;
  const contribution = seat === "seat_0" ? view.contributions.seat_0 : view.contributions.seat_1;
  const privateCount = seat === "seat_0" ? view.private_counts.seat_0 : view.private_counts.seat_1;

  return (
    <section className={`poker-lite-seat ${active ? "active" : ""}`} aria-label={`${seatLabel(seat)} ledger`}>
      <div className="poker-lite-section-heading">
        <span>{seatLabel(seat)}</span>
        <strong>{active ? "Active" : "Waiting"}</strong>
      </div>
      <Metric label="Contribution" value={String(contribution)} />
      <Metric label="Private crests" value={String(privateCount)} />
    </section>
  );
}

function ContributionTrack({ view }: { view: PokerLitePublicView }) {
  const maxContribution = Math.max(view.contributions.seat_0, view.contributions.seat_1, 1);
  return (
    <div className="poker-lite-track" aria-label="Contribution ledger">
      {(["seat_0", "seat_1"] as const).map((seat) => {
        const amount = view.contributions[seat];
        return (
          <div key={seat}>
            <span>{seatLabel(seat)}</span>
            <div className="poker-lite-track-bar">
              <span style={{ inlineSize: `${Math.max(12, (amount / maxContribution) * 100)}%` }} />
            </div>
            <strong>{amount}</strong>
          </div>
        );
      })}
    </div>
  );
}

function ShowdownPanel({ view }: { view: PokerLitePublicView }) {
  if (!view.showdown) {
    return null;
  }
  return (
    <section className="poker-lite-showdown" aria-label="Grouped showdown reveal">
      <div className="poker-lite-section-heading">
        <span>Showdown</span>
        <strong>{view.showdown.winner ? `${seatLabel(view.showdown.winner)} leads` : "Split strength"}</strong>
      </div>
      <div className="poker-lite-showdown-grid">
        <CrestCard card={view.showdown.seat_0_private} label="Seat 0" tone="revealed" />
        <CrestCard card={view.showdown.center} label="Center" tone="center" />
        <CrestCard card={view.showdown.seat_1_private} label="Seat 1" tone="revealed" />
      </div>
    </section>
  );
}

function CrestCard({ card, label, tone }: { card: PokerLiteCardView; label?: string; tone: "center" | "private" | "revealed" }) {
  return (
    <div className={`poker-lite-card ${tone}`} aria-label={card.accessibility_label}>
      {label ? <span>{label}</span> : null}
      <strong>{card.label}</strong>
      <small>
        {card.rank} / {card.copy}
      </small>
    </div>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div className="poker-lite-metric">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function actionDetail(choice: ActionChoice): string {
  const metadata = choice.metadata ?? [];
  const adds = metadata.find((entry) => entry.key === "adds_to_pool")?.value;
  const required = metadata.find((entry) => entry.key === "required_to_match")?.value;
  if (adds && adds !== "0") {
    return `Adds ${adds}`;
  }
  if (required && required !== "0") {
    return `Answers ${required}`;
  }
  return "No marker added";
}

function actionStatus(view: PokerLitePublicView, pending: boolean): string {
  if (pending) {
    return "Applying";
  }
  if (view.terminal.terminal) {
    return "Complete";
  }
  return view.active_seat ? `${seatLabel(view.active_seat)} to choose` : "Waiting";
}

function statusLabel(view: PokerLitePublicView): string {
  if (view.terminal.terminal) {
    return terminalLabel(view);
  }
  return view.active_seat ? `${seatLabel(view.active_seat)} to choose` : "Resolving";
}

function terminalLabel(view: PokerLitePublicView): string {
  if (!view.terminal.terminal) {
    return "In progress";
  }
  if (view.terminal.draw) {
    return "Split ledger";
  }
  return `${seatLabel(view.terminal.winner ?? "seat_0")} wins`;
}

function privateHeading(view: PokerLitePublicView): string {
  if (view.private_view.status === "seat") {
    return `${seatLabel(view.private_view.seat)} view`;
  }
  return "Observer view";
}

function boardSummary(view: PokerLitePublicView): string {
  return `${view.display_name}, shared pool ${view.shared_pool}, ${view.active_seat ?? "no seat"} active, center ${view.center.status}.`;
}

function isRevealEffect(type: string): boolean {
  return type === "center_reveal_started" || type === "center_revealed" || type === "showdown_revealed";
}

function seatLabel(seat: SeatId): string {
  return seat === "seat_0" ? "Seat 0" : "Seat 1";
}

function pokerTemplateKey(view: PokerLitePublicView): string {
  switch (view.terminal.kind) {
    case "yield_win":
      return "poker_lite.yield_win_no_reveal";
    case "showdown_win":
      return "poker_lite.pair_beats_high_card";
    case "split":
      return "poker_lite.equal_strength_split";
    case "non_terminal":
      return "poker_lite.equal_strength_split";
  }
}

function pokerStanding(seat: SeatId, winner: SeatId | null, view: PokerLitePublicView) {
  return {
    id: seat,
    label: seatLabel(seat),
    result: winner === seat ? "Winner" : winner ? "Loss" : "Split",
    emphasized: winner === seat,
    values: [
      { label: "Contribution", value: view.contributions[seat] },
      { label: "Private crests", value: view.private_counts[seat] },
    ],
  };
}
