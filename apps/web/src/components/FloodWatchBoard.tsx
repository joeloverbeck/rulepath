import { useMemo } from "react";
import type { ActionChoice, ActionTree, EffectEntry, FloodWatchDistrictView, FloodWatchPublicView } from "../wasm/client";
import { DeckFlowPanel } from "./DeckFlowPanel";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type FloodWatchBoardProps = {
  view: FloodWatchPublicView;
  actionTree: ActionTree | null;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onPathSubmit?: (path: string[]) => void;
};

export function FloodWatchBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onPathSubmit,
}: FloodWatchBoardProps) {
  const choices = actionTree?.choices ?? [];
  const districtChoices = useMemo(() => districtChoiceMap(choices), [choices]);
  const forecastChoice = choices.find((choice) => choice.segment === "forecast") ?? null;
  const endTurnChoice = choices.find((choice) => choice.segment === "end_turn") ?? null;
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const terminal = view.terminal.kind !== "non_terminal";
  const terminalSummary = view.terminal.kind === "complete" ? view.terminal.summary : null;
  const stormActive = effects.some((entry) => isStormEffect(entry.effect.payload.type));
  const canAct = Boolean(interactive && !pending && !terminal);
  const latestDrawn = view.drawn_cards.at(-1) ?? null;
  const outcomeExplanation = terminalSummary
    ? outcomeSurfaceData({
        gameId: "flood_watch",
        heading: view.terminal.outcome === "won" ? "Shared win" : "Shared loss",
        rationale: view.terminal_rationale ?? null,
        resultKind: view.terminal.outcome === "won" ? "win" : "loss",
        decisiveCause: terminalSummary.rule_id,
        templateKey:
          terminalSummary.rule_id === "FW-END-001"
            ? "flood_watch.shared_loss_inundation"
            : "flood_watch.shared_win_deck_exhausted",
        templateParams: {
          drawn_card_count: terminalSummary.drawn_card_count,
          rule_id: terminalSummary.rule_id,
        },
        finalStanding: [
          {
            id: "team",
            label: "Team",
            result: view.terminal.outcome === "won" ? "win" : "loss",
            emphasized: true,
            values: [
              { label: "Drawn cards", value: terminalSummary.drawn_card_count },
              { label: "Undrawn cards", value: view.undrawn_count },
            ],
          },
        ],
        breakdownSections: [
          {
            id: "public-summary",
            heading: "Public summary",
            summary: terminalSummary.public_summary,
            rows: terminalSummary.surviving_levels.map((entry) => ({
              label: districtLabel(view, entry.district),
              value: entry.count,
            })),
          },
        ],
        ruleIds: [terminalSummary.rule_id],
      })
    : null;

  return (
    <section
      className={`plain-tricks-board flood-watch-board ${terminal ? "terminal" : ""}${stormActive ? " reveal" : ""}${
        reducedMotion ? " reduced" : ""
      }`}
      aria-labelledby="flood-watch-heading"
      data-testid="flood-watch-board"
    >
      <div className="plain-tricks-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="flood-watch-heading">{statusLabel(view)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {terminal ? view.terminal.outcome : statusLabel(view)}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {view.display_name}, {statusLabel(view)}, {choices.length} legal choices, {view.forecast ? `${view.ui.forecast_label} ${view.forecast.label}` : "no forecast"}, {view.undrawn_count} {view.ui.face_down_label}.
      </p>

      <div className="plain-tricks-metrics" aria-label="Flood Watch status">
        <Metric label="Turn" value={String(view.turn_number)} />
        <Metric label="Budget" value={budgetLabel(view)} />
        <Metric label="Undrawn" value={String(view.undrawn_count)} />
      </div>

      <DeckFlowPanel
        label={view.ui.event_deck_label}
        currentLabel={view.ui.drawn_label}
        nextLabel={view.ui.forecast_label}
        discardLabel={view.ui.drawn_label}
        faceDownLabel={view.ui.face_down_label}
        faceDownSummary={view.ui.face_down_summary}
        current={latestDrawn}
        next={view.forecast}
        discard={view.drawn_cards}
        faceDownCount={view.undrawn_count}
      />

      <div className="plain-tricks-table" aria-label="Flood Watch districts">
        {view.districts.map((district) => {
          const legal = districtChoices.get(district.district) ?? {};
          return (
            <section
              className="plain-seat"
              aria-label={`${district.label} district`}
              data-testid={`flood-watch-district-${district.district}`}
              key={district.district}
            >
              <div className="plain-section-heading">
                <span>{district.label}</span>
                <strong>{district.flood_level >= 5 ? "Inundation risk" : `${district.flood_level} flood`}</strong>
              </div>
              <div className="plain-tricks-metrics" aria-label={`${district.label} public counters`}>
                <Metric label="Flood" value={String(district.flood_level)} />
                <Metric label="Levees" value={String(district.levees)} />
              </div>
              <div className="action-list">
                <DistrictButton choice={legal.bail} disabled={!canAct || !legal.bail} onPathSubmit={onPathSubmit} fallback="Bail" />
                <DistrictButton
                  choice={legal.reinforce}
                  disabled={!canAct || !legal.reinforce}
                  onPathSubmit={onPathSubmit}
                  fallback="Reinforce"
                />
              </div>
            </section>
          );
        })}
      </div>

      <section className="plain-history" aria-label="Team roles">
        <div className="plain-section-heading">
          <span>Roles</span>
          <strong>Public team powers</strong>
        </div>
        <ol>
          {view.roles.map((role) => (
            <li key={role.seat}>
              <span>{seatLabel(role.seat)}</span>
              <strong>{role.label}</strong>
              <small>Cooperative role</small>
            </li>
          ))}
        </ol>
      </section>

      <div className="plain-latest" role="status">
        <span>{outcomeExplanation ? "Outcome" : feedback?.title ?? "Waiting"}</span>
        <strong>
          {outcomeExplanation
            ? outcomeAnnouncementText(outcomeExplanation)
            : feedback?.detail ?? "Cooperative actions and public storm accounting will update here."}
        </strong>
      </div>

      {!terminal ? (
        <div className="action-list" aria-label="Flood Watch turn actions">
          {forecastChoice ? (
            <button type="button" disabled={!canAct} aria-label={forecastChoice.accessibility_label} onClick={() => onPathSubmit?.(["forecast"])}>
              {forecastChoice.label}
            </button>
          ) : null}
          {endTurnChoice ? (
            <button type="button" disabled={!canAct} aria-label={endTurnChoice.accessibility_label} onClick={() => onPathSubmit?.(["end_turn"])}>
              {endTurnChoice.label}
            </button>
          ) : null}
        </div>
      ) : null}

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function DistrictButton({
  choice,
  disabled,
  fallback,
  onPathSubmit,
}: {
  choice?: ActionChoice;
  disabled: boolean;
  fallback: string;
  onPathSubmit?: (path: string[]) => void;
}) {
  return (
    <button type="button" disabled={disabled} aria-label={choice?.accessibility_label ?? fallback} onClick={() => choice && onPathSubmit?.([choice.segment])}>
      {choice?.label ?? fallback}
    </button>
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

function districtChoiceMap(choices: ActionChoice[]): Map<string, { bail?: ActionChoice; reinforce?: ActionChoice }> {
  const map = new Map<string, { bail?: ActionChoice; reinforce?: ActionChoice }>();
  for (const choice of choices) {
    const [family, district] = choice.segment.split("/");
    if (!district || (family !== "bail" && family !== "reinforce")) {
      continue;
    }
    const entry = map.get(district) ?? {};
    entry[family] = choice;
    map.set(district, entry);
  }
  return map;
}

function isStormEffect(type: string): boolean {
  return type === "event_drawn" || type === "flood_level_rose" || type === "district_inundated" || type === "terminal";
}

function statusLabel(view: FloodWatchPublicView): string {
  if (view.terminal.kind !== "non_terminal") {
    return view.terminal.summary.public_summary;
  }
  return `${seatLabel(view.active_seat)} to act`;
}

function budgetLabel(view: FloodWatchPublicView): string {
  return view.phase.kind === "action" ? String(view.phase.budget_remaining) : "Terminal";
}

function districtLabel(view: FloodWatchPublicView, district: string): string {
  return view.districts.find((item) => item.district === district)?.label ?? district;
}

function seatLabel(seat: string): string {
  return seat === "seat_0" ? "Seat 0" : seat === "seat_1" ? "Seat 1" : seat;
}
