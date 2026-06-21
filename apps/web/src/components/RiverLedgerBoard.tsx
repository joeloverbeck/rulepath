import { useMemo } from "react";
import type { SchedulerPresentation, SchedulerStep } from "../animation/scheduler";
import { animationRegistry } from "../animation/registry";
import { animateFade, animateHighlight, type PresentationContext } from "../animation/presenters";
import type { BotDecisionSummary } from "../state/shellReducer";
import type {
  ActionChoice,
  ActionTree,
  EffectEntry,
  RiverLedgerHandRankingMetadata,
  RiverLedgerOutcomeStanding,
  RiverLedgerPublicView,
  RiverLedgerSeatId,
  RiverLedgerSeatView,
  SeatDisplayLabel,
} from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import {
  OutcomeExplanationPanel,
  outcomeAnnouncementText,
  outcomeSurfaceData,
  type OutcomeExplanationBreakdownSection,
} from "./OutcomeExplanationPanel";
import { RiverLedgerCard, riverLedgerCardGroupLabel } from "./RiverLedgerCard";
import { resolveSeatLabel } from "../seatLabels";

registerRiverLedgerAnimations();

type RiverLedgerBoardProps = {
  view: RiverLedgerPublicView;
  actionTree: ActionTree | null;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  lastBotDecision?: BotDecisionSummary | null;
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
  lastBotDecision = null,
  interactive = true,
  onChoice,
}: RiverLedgerBoardProps) {
  const choices = useMemo(() => actionTree?.choices ?? [], [actionTree]);
  const canAct = Boolean(interactive && !pending && !view.terminal.terminal && view.active_seat && choices.length > 0);
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const botExplanation = lastBotDecision?.publicExplanation ?? null;
  const boardChanged = effects.some((entry) => String(entry.effect.payload.type).includes("board"));
  const labelForSeat = useMemo(() => seatLabeler(view.ui.seat_labels), [view.ui.seat_labels]);
  const outcomeExplanation = view.terminal.terminal
    ? outcomeSurfaceData({
        gameId: "river_ledger",
        heading: terminalLabel(view, labelForSeat),
        rationale: view.terminal_rationale ?? null,
        resultKind: view.terminal.kind,
        decisiveCause: view.terminal.kind,
        templateKey: riverTemplateKey(view),
        finalStanding: view.seats.map((seat) => riverStanding(view, seat, labelForSeat)),
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
          ...potBreakdownSections(view, labelForSeat),
        ],
        ruleIds: view.terminal.kind === "last_live_hand" ? ["RL-END-LAST-LIVE", "RL-SCORE-POT-AWARD"] : ["RL-SCORE-SHOWDOWN", "RL-END-SHOWDOWN"],
        riverLedgerShowdownV2: view.terminal.kind === "showdown" ? view.terminal.presentation_v2 ?? null : null,
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
          {view.active_seat ? labelForSeat(view.active_seat) : terminalLabel(view, labelForSeat)}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {view.display_name}, {phaseLabel(view.phase)}, ledger total {view.pot_total}, {view.board.length} public cards.
      </p>

      <div className="river-ledger-metrics" aria-label="River Ledger status">
        <Metric label="Ledger" value={String(view.pot_total)} />
        <Metric label="Street" value={phaseLabel(view.phase)} />
        <Metric label="Button" value={labelForSeat(view.button)} />
        <Metric label="Blinds" value={`${labelForSeat(view.small_blind)} / ${labelForSeat(view.big_blind)}`} />
      </div>

      <div className="river-ledger-table-shell" aria-label={view.ui.surface_label}>
        <StreetStrip view={view} />
        <PotLedger view={view} labelForSeat={labelForSeat} />

        <div className="river-ledger-layout">
          <section className="river-ledger-board-well" aria-label={view.ui.board_label}>
            <div className="river-ledger-section-heading">
              <span>Board</span>
              <strong>{view.board.length ? `${view.board.length} public` : "No public cards"}</strong>
            </div>
            <div
              className="river-ledger-board-cards"
              aria-label={riverLedgerCardGroupLabel(view.board, "Public board cards")}
              data-animation-target="river-ledger-board-reveal"
            >
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

          <section className="river-ledger-seat-rail" aria-label={view.ui.seat_metadata_label}>
            {view.seats.map((seat) => (
              <SeatLedger key={seat.seat} view={view} seat={seat} labelForSeat={labelForSeat} />
            ))}
          </section>
        </div>

        <div className="river-ledger-action-band">
          <section className="river-ledger-private" aria-label="Private view">
            <div className="river-ledger-section-heading">
              <span>Private view</span>
              <strong>{privateHeading(view)}</strong>
            </div>
            {view.private_view.status === "seat" ? (
              <div
                className="river-ledger-private-cards"
                aria-label={riverLedgerCardGroupLabel(view.private_view.hole_cards, `${labelForSeat(view.private_view.seat)} private cards`)}
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
              <strong>{canAct ? "Available choices" : actionStatus(view, pending, labelForSeat)}</strong>
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

          <div className="river-ledger-latest" role="status" data-animation-target="river-ledger-status">
            <span>{outcomeExplanation ? "Outcome" : feedback?.title ?? "Waiting"}</span>
            <strong>
              {outcomeExplanation
                ? outcomeAnnouncementText(outcomeExplanation)
                : feedback?.detail ?? "Visible state changes will update here."}
            </strong>
          </div>

          {botExplanation ? <RiverLedgerBotWhy explanation={botExplanation} /> : null}
        </div>
      </div>

      <HandRankingReference
        currentCategory={currentShowdownCategory(view)}
        rankings={view.ui.hand_rankings}
        terminal={view.terminal.terminal}
      />

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}

    </section>
  );
}

function RiverLedgerBotWhy({ explanation }: { explanation: NonNullable<BotDecisionSummary["publicExplanation"]> }) {
  return (
    <details
      className="river-ledger-bot-why bot-note bot-why"
      data-testid="river-ledger-bot-explanation"
      aria-label={`Why ${explanation.seat_label} chose ${explanation.action_label}`}
    >
      <summary>Why?</summary>
      <strong>{explanation.short_reason}</strong>
      <dl>
        {explanation.public_facts.map((fact) => (
          <div key={`${fact.label}:${fact.value}`}>
            <dt>{fact.label}</dt>
            <dd>{fact.value}</dd>
          </div>
        ))}
      </dl>
      <p>{explanation.hidden_information_notice}</p>
    </details>
  );
}

function SeatLedger({
  view,
  seat,
  labelForSeat,
}: {
  view: RiverLedgerPublicView;
  seat: RiverLedgerSeatView;
  labelForSeat: (seat: RiverLedgerSeatId) => string;
}) {
  const active = view.active_seat === seat.seat;
  const display = seat.ledger_display;
  const allIn = seat.is_all_in || seat.remaining_stack === 0;
  const metrics = [
    display.round_contribution,
    display.hand_contribution,
    {
      label: "Stack",
      value: `${seat.remaining_stack} / ${seat.starting_stack}`,
      accessibility_label: `${labelForSeat(seat.seat)} has ${seat.remaining_stack} remaining from ${seat.starting_stack} starting stack.`,
    },
    display.hole_card_summary,
  ];

  return (
    <section
      className={`river-ledger-seat ${active ? "active" : ""}${allIn ? " all-in" : ""}`}
      aria-label={`${labelForSeat(seat.seat)} ledger`}
      data-animation-target={`river-ledger-seat-${seat.seat}`}
    >
      <div className="river-ledger-section-heading">
        <span>{labelForSeat(seat.seat)}</span>
        <strong>{display.status_label}</strong>
      </div>
      {display.role_badges.length ? (
        <div className="river-ledger-markers">
          {display.role_badges.map((badge) => (
            <b key={badge}>{badge}</b>
          ))}
        </div>
      ) : null}
      {allIn ? <span className="river-ledger-all-in">All-in</span> : null}
      {metrics.map((metric) => (
        <Metric key={metric.label} label={metric.label} value={metric.value} ariaLabel={metric.accessibility_label} />
      ))}
    </section>
  );
}

function PotLedger({ view, labelForSeat }: { view: RiverLedgerPublicView; labelForSeat: (seat: RiverLedgerSeatId) => string }) {
  return (
    <section className="river-ledger-pot-ledger" aria-label="Public pot tiers" data-animation-target="river-ledger-pot-ledger">
      <div className="river-ledger-section-heading">
        <span>Pot tiers</span>
        <strong>{view.pot_tiers.length ? `${view.pot_tiers.length} tier${view.pot_tiers.length === 1 ? "" : "s"}` : "No tiers"}</strong>
      </div>
      <div className="river-ledger-pot-grid" data-testid="river-ledger-pot-tiers">
        {view.pot_tiers.length ? (
          view.pot_tiers.map((tier, index) => (
            <article className="river-ledger-pot-tier" key={`${tier.pot_id}-${index}`}>
              <header>
                <strong>{potTierLabel(tier.pot_id, index)}</strong>
                <span>{tier.amount}</span>
              </header>
              <dl>
                <div>
                  <dt>Cap</dt>
                  <dd>{tier.cap ?? "Open"}</dd>
                </div>
                <div>
                  <dt>Contributors</dt>
                  <dd>{seatList(tier.contributors, labelForSeat)}</dd>
                </div>
                <div>
                  <dt>Eligible</dt>
                  <dd>{seatList(tier.eligible, labelForSeat)}</dd>
                </div>
              </dl>
            </article>
          ))
        ) : (
          <p className="muted">Rust has not formed a public pot tier.</p>
        )}
      </div>
      {view.uncalled_returns.length ? (
        <div className="river-ledger-returns" data-testid="river-ledger-uncalled-returns">
          <span>Uncalled returns</span>
          <ul>
            {view.uncalled_returns.map((refund) => (
              <li key={`${refund.seat}-${refund.amount}`}>
                {labelForSeat(refund.seat)} receives {refund.amount}
              </li>
            ))}
          </ul>
        </div>
      ) : null}
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

function actionStatus(view: RiverLedgerPublicView, pending: boolean, labelForSeat: (seat: RiverLedgerSeatId) => string): string {
  if (pending) return "Applying";
  if (view.terminal.terminal) return "Complete";
  return view.active_seat ? `${labelForSeat(view.active_seat)} to choose` : "Waiting";
}

function registerRiverLedgerAnimations(): void {
  animationRegistry.register("river_ledger", "river_ledger_street_advanced", (step, context) =>
    highlightRiverTargets(context, ["river-ledger-board-reveal", "river-ledger-status"], step.reducedMotion),
  );
  for (const effectType of [
    "river_ledger_stack_changed",
    "river_ledger_seat_became_all_in",
    "river_ledger_uncalled_contribution_returned",
    "river_ledger_pot_resolved",
    "river_ledger_pot_awarded",
  ]) {
    animationRegistry.register("river_ledger", effectType, (step, context) =>
      highlightRiverTargets(context, ["river-ledger-pot-ledger", "river-ledger-status"], step.reducedMotion),
    );
  }
  animationRegistry.register("river_ledger", "river_ledger_showdown_resolved", (step, context) =>
    stagedRiverShowdown(step, context),
  );
}

function stagedRiverShowdown(step: SchedulerStep, context: PresentationContext): SchedulerPresentation {
  const reducedMotion = context.reducedMotion ?? step.reducedMotion;
  return highlightRiverTargets(
    context,
    ["river-ledger-showdown-banner", "river-ledger-showdown-board", "river-ledger-showdown-standings", "river-ledger-status"],
    reducedMotion,
    "staged",
  );
}

function highlightRiverTargets(
  context: PresentationContext,
  targetIds: string[],
  reducedMotion: boolean,
  kind: "highlight" | "staged" = "highlight",
): SchedulerPresentation {
  const root = context.root ?? document;
  const targets = uniqueElements(targetIds.flatMap((targetId) => [...root.querySelectorAll(targetSelector(targetId))]));
  const animations = targets.map((element, index) => {
    if (kind === "staged" && index === 0) {
      return animateFade(element, reducedMotion);
    }
    return animateHighlight(element, reducedMotion);
  });
  return { animations };
}

function targetSelector(targetId: string): string {
  return `[data-animation-target="${cssEscape(targetId)}"]`;
}

function uniqueElements(elements: Element[]): Element[] {
  return [...new Set(elements)];
}

function cssEscape(value: string): string {
  if (typeof CSS !== "undefined" && CSS.escape) {
    return CSS.escape(value);
  }
  return value.replace(/["\\]/g, "\\$&");
}

function statusLabel(view: RiverLedgerPublicView): string {
  const labelForSeat = seatLabeler(view.ui.seat_labels);
  if (view.terminal.terminal) return terminalLabel(view, labelForSeat);
  return view.active_seat ? `${labelForSeat(view.active_seat)} to choose` : "Resolving";
}

function terminalLabel(view: RiverLedgerPublicView, labelForSeat: (seat: RiverLedgerSeatId) => string): string {
  if (!view.terminal.terminal) return "In progress";
  if (view.terminal.winners.length === 0) return "Complete";
  if (view.terminal.winners.length === 1) return `${labelForSeat(view.terminal.winners[0])} wins`;
  return `${view.terminal.winners.length} seats split`;
}

function riverTemplateKey(view: RiverLedgerPublicView): string {
  if (view.terminal.kind === "last_live_hand") {
    return "river_ledger.last_live_fold_win";
  }
  return view.terminal.winners.length > 1 ? "river_ledger.showdown_split_pot" : "river_ledger.showdown_best_hand_win";
}

function riverStanding(
  view: RiverLedgerPublicView,
  seat: RiverLedgerSeatView,
  labelForSeat: (seat: RiverLedgerSeatId) => string,
) {
  const allocation = view.terminal.allocations.find((share) => share.seat === seat.seat)?.amount ?? 0;
  const winner = view.terminal.winners.some((winnerSeat) => winnerSeat === seat.seat);
  return {
    id: seat.seat,
    label: labelForSeat(seat.seat),
    result: winner ? (view.terminal.winners.length > 1 ? "split" : "win") : seatStatusLabel(seat.status),
    emphasized: winner,
    values: [
      { label: "Contribution", value: seat.total_contribution },
      { label: "Allocation", value: allocation },
    ],
  };
}

function potBreakdownSections(
  view: RiverLedgerPublicView,
  labelForSeat: (seat: RiverLedgerSeatId) => string,
): OutcomeExplanationBreakdownSection[] {
  const sections: OutcomeExplanationBreakdownSection[] = view.pot_tiers.map((tier, index) => ({
    id: `pot-tier-${index}`,
    heading: potTierLabel(tier.pot_id, index),
    rows: [
      { label: "Amount", value: tier.amount },
      { label: "Cap", value: tier.cap ?? "Open" },
      { label: "Contributors", value: seatList(tier.contributors, labelForSeat) },
      { label: "Eligible", value: seatList(tier.eligible, labelForSeat) },
    ],
  }));
  if (view.uncalled_returns.length) {
    sections.push({
      id: "uncalled-returns",
      heading: "Uncalled returns",
      rows: view.uncalled_returns.map((refund) => ({
        label: labelForSeat(refund.seat),
        value: refund.amount,
      })),
    });
  }
  if (view.terminal.terminal && view.terminal.allocations.length) {
    sections.push({
      id: "terminal-allocations",
      heading: "Terminal allocations",
      rows: view.terminal.allocations.map((allocation) => ({
        label: labelForSeat(allocation.seat),
        value: allocation.amount,
      })),
      defaultOpen: true,
    });
  }
  return sections;
}

function potTierLabel(potId: string, index: number): string {
  if (potId === "main_pot") {
    return "Main pot";
  }
  return `Side pot ${index}`;
}

function seatList(seats: RiverLedgerSeatId[], labelForSeat: (seat: RiverLedgerSeatId) => string): string {
  return seats.length ? seats.map(labelForSeat).join(", ") : "None";
}

function currentShowdownCategory(view: RiverLedgerPublicView): string | null {
  const standings = view.terminal_rationale?.final_standing as RiverLedgerOutcomeStanding[] | undefined;
  return standings?.find((standing) => standing.emphasized && standing.strength)?.strength?.category ?? null;
}

function privateHeading(view: RiverLedgerPublicView): string {
  const labelForSeat = seatLabeler(view.ui.seat_labels);
  if (view.private_view.status === "seat") return `${labelForSeat(view.private_view.seat)} view`;
  return "Observer view";
}

function phaseLabel(phase: string): string {
  return phase.replace(/_/g, " ").replace(/\b\w/g, (letter) => letter.toUpperCase());
}

function seatStatusLabel(status: string): string {
  return phaseLabel(status);
}

function seatLabeler(labels: SeatDisplayLabel[]): (seat: RiverLedgerSeatId) => string {
  return (seat) => resolveSeatLabel(seat, { catalogUiSeatLabels: labels });
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
