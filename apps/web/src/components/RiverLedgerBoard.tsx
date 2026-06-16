import { useMemo } from "react";
import type {
  ActionChoice,
  ActionTree,
  EffectEntry,
  RiverLedgerHandRankingMetadata,
  RiverLedgerOutcomeStanding,
  RiverLedgerPublicView,
  RiverLedgerSeatId,
  RiverLedgerSeatView,
} from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";
import { RiverLedgerCard, riverLedgerCardGroupLabel } from "./RiverLedgerCard";

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
  const outcomeExplanation = view.terminal.terminal
    ? outcomeSurfaceData({
        gameId: "river_ledger",
        heading: terminalLabel(view),
        rationale: view.terminal_rationale ?? null,
        resultKind: view.terminal.kind,
        decisiveCause: view.terminal.kind,
        templateKey: riverTemplateKey(view),
        finalStanding: view.seats.map((seat) => riverStanding(view, seat)),
        breakdownSections: [
          {
            id: "ledger",
            heading: "Public ledger",
            rows: [
              { label: "Terminal kind", value: view.terminal.kind },
              { label: "Ledger total", value: view.terminal.pot_total },
              { label: "Winner count", value: view.terminal.winners.length },
            ],
          },
        ],
        ruleIds: view.terminal.kind === "last_live_hand" ? ["RL-END-LAST-LIVE", "RL-SCORE-POT-AWARD"] : ["RL-SCORE-SHOWDOWN", "RL-END-SHOWDOWN"],
      })
    : null;

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
        {view.display_name}, {phaseLabel(view.phase)}, ledger total {view.pot_total}, {view.board.length} public cards.
      </p>

      <div className="river-ledger-metrics" aria-label="River Ledger status">
        <Metric label="Ledger" value={String(view.pot_total)} />
        <Metric label="Street" value={phaseLabel(view.phase)} />
        <Metric label="Button" value={seatLabel(view.button)} />
        <Metric label="Blinds" value={`${seatLabel(view.small_blind)} / ${seatLabel(view.big_blind)}`} />
      </div>

      <StreetStrip view={view} />

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
          <div className="river-ledger-board-cards" aria-label={riverLedgerCardGroupLabel(view.board, "Public board cards")}>
            {view.board_slots.map((slot) =>
              slot.card ? (
                <RiverLedgerCard key={slot.slot} card={slot.card} tone="board" />
              ) : (
                <div className="river-ledger-card hidden" key={slot.slot} aria-label={slot.accessibility_label}>
                  <span>{slot.street_label}</span>
                  <strong>{slot.visual_placeholder_label}</strong>
                </div>
              ),
            )}
          </div>
        </section>
      </div>

      <section className="river-ledger-private" aria-label="Private view">
        <div className="river-ledger-section-heading">
          <span>Private view</span>
          <strong>{privateHeading(view)}</strong>
        </div>
        {view.private_view.status === "seat" ? (
          <div
            className="river-ledger-private-cards"
            aria-label={riverLedgerCardGroupLabel(view.private_view.hole_cards, `${seatLabel(view.private_view.seat)} private cards`)}
          >
            {view.private_view.hole_cards.map((card) => (
              <RiverLedgerCard key={card.card_id} card={card} tone="private" />
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
                <ActionChoiceDetails choice={choice} />
              </button>
            ))
          )}
        </div>
      </section>

      <HandRankingReference
        currentCategory={currentShowdownCategory(view)}
        rankings={view.ui.hand_rankings}
        terminal={view.terminal.terminal}
      />

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}

      <div className="river-ledger-latest" role="status">
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

function SeatLedger({ view, seat }: { view: RiverLedgerPublicView; seat: RiverLedgerSeatView }) {
  const active = view.active_seat === seat.seat;
  const display = seat.ledger_display;
  const metrics = [display.round_contribution, display.hand_contribution, display.hole_card_summary];

  return (
    <section className={`river-ledger-seat ${active ? "active" : ""}`} aria-label={`${seatLabel(seat.seat)} ledger`}>
      <div className="river-ledger-section-heading">
        <span>{seatLabel(seat.seat)}</span>
        <strong>{display.status_label}</strong>
      </div>
      {display.role_badges.length ? (
        <div className="river-ledger-markers">
          {display.role_badges.map((badge) => (
            <b key={badge}>{badge}</b>
          ))}
        </div>
      ) : null}
      {metrics.map((metric) => (
        <Metric key={metric.label} label={metric.label} value={metric.value} ariaLabel={metric.accessibility_label} />
      ))}
    </section>
  );
}

function StreetStrip({ view }: { view: RiverLedgerPublicView }) {
  const current = streetStripPhase(view);
  const currentIndex = streetSteps.findIndex((step) => step.id === current);

  return (
    <nav className="river-ledger-street-strip" aria-label="Street progression">
      <ol>
        {streetSteps.map((step, index) => {
          const state = index < currentIndex ? "complete" : index === currentIndex ? "current" : "upcoming";
          return (
            <li className={state} key={step.id} aria-current={state === "current" ? "step" : undefined}>
              <span aria-hidden="true">{state === "complete" ? "✓" : state === "current" ? ">" : "·"}</span>
              <strong>{step.label}</strong>
            </li>
          );
        })}
      </ol>
    </nav>
  );
}

function HandRankingReference({
  currentCategory,
  rankings,
  terminal,
}: {
  currentCategory: string | null;
  rankings: RiverLedgerHandRankingMetadata[];
  terminal: boolean;
}) {
  if (rankings.length === 0) {
    return null;
  }

  return (
    <section className="river-ledger-hand-rankings" aria-label="Hand ranking reference">
      <details key={terminal ? "terminal-rankings" : "play-rankings"} open={terminal ? true : undefined}>
        <summary>Hand ranking reference</summary>
        <ol>
          {rankings.map((ranking) => {
            const current = ranking.category === currentCategory;
            return (
              <li className={current ? "current" : ""} key={ranking.category} aria-current={current ? "true" : undefined}>
                <strong>{ranking.label}</strong>
                <span>{ranking.definition}</span>
              </li>
            );
          })}
        </ol>
      </details>
    </section>
  );
}

function Metric({ label, value, ariaLabel }: { label: string; value: string; ariaLabel?: string }) {
  return (
    <div className="river-ledger-metric" aria-label={ariaLabel}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function ActionChoiceDetails({ choice }: { choice: ActionChoice }) {
  const presentation = choice.presentation;

  return (
    <small className="river-ledger-action-detail">
      {presentation?.helper_text ? <span>{presentation.helper_text}</span> : null}
      {presentation?.display_rows.map((row) => (
        <span className={`river-ledger-choice-row ${row.tone}`} key={`${choice.segment}-${row.label}`}>
          {row.label} {row.value}
        </span>
      ))}
    </small>
  );
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

function riverTemplateKey(view: RiverLedgerPublicView): string {
  if (view.terminal.kind === "last_live_hand") {
    return "river_ledger.last_live_fold_win";
  }
  return view.terminal.winners.length > 1 ? "river_ledger.showdown_split_pot" : "river_ledger.showdown_best_hand_win";
}

function riverStanding(view: RiverLedgerPublicView, seat: RiverLedgerSeatView) {
  const allocation = view.terminal.allocations.find((share) => share.seat === seat.seat)?.amount ?? 0;
  const winner = view.terminal.winners.some((winnerSeat) => winnerSeat === seat.seat);
  return {
    id: seat.seat,
    label: seatLabel(seat.seat),
    result: winner ? (view.terminal.winners.length > 1 ? "split" : "win") : seatStatusLabel(seat.status),
    emphasized: winner,
    values: [
      { label: "Contribution", value: seat.total_contribution },
      { label: "Allocation", value: allocation },
    ],
  };
}

function currentShowdownCategory(view: RiverLedgerPublicView): string | null {
  const standings = view.terminal_rationale?.final_standing as RiverLedgerOutcomeStanding[] | undefined;
  return standings?.find((standing) => standing.emphasized && standing.strength)?.strength?.category ?? null;
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

const streetSteps = [
  { id: "preflop", label: "Preflop" },
  { id: "flop", label: "Flop" },
  { id: "turn", label: "Turn" },
  { id: "river", label: "River" },
  { id: "showdown", label: "Showdown" },
] as const;

function streetStripPhase(view: RiverLedgerPublicView): (typeof streetSteps)[number]["id"] {
  if (view.terminal.terminal) {
    return "showdown";
  }
  return streetSteps.some((step) => step.id === view.phase) ? (view.phase as (typeof streetSteps)[number]["id"]) : "preflop";
}
