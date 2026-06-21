import { useMemo } from "react";
import type { ActionChoice, ActionTree, EffectEntry, FloodWatchPublicView } from "../wasm/client";
import { resolveSeatLabel } from "../seatLabels";
import { animationRegistry } from "../animation/registry";
import type { SchedulerPresentation, SchedulerStep } from "../animation/scheduler";
import { animateFade, animateHighlight, type PresentationContext } from "../animation/presenters";
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

registerFloodWatchAnimations();

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
        <Metric label="Turn" value={String(view.turn_number)} animationTarget="flood-watch-turn" />
        <Metric label="Budget" value={budgetLabel(view)} animationTarget="flood-watch-budget" />
        <Metric label="Undrawn" value={String(view.undrawn_count)} animationTarget="flood-watch-undrawn" />
      </div>

      <div data-animation-target="flood-watch-deck">
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
      </div>

      <p className="flood-watch-loss-note" data-testid="flood-watch-loss-note">
        Shared loss the moment any district reaches flood {MAX_FLOOD_LEVEL}. Survive the storm deck together to win.
      </p>

      <div className="plain-tricks-table" aria-label="Flood Watch districts" data-animation-target="flood-watch-districts">
        {view.districts.map((district) => {
          const legal = districtChoices.get(district.district) ?? {};
          const danger = floodDanger(district.flood_level);
          return (
            <section
              className={`plain-seat flood-danger-${danger}`}
              aria-label={`${district.label} district, ${dangerLabel(district.flood_level)}`}
              data-testid={`flood-watch-district-${district.district}`}
              data-animation-target={`flood-watch-district-${district.district}`}
              data-danger={danger}
              key={district.district}
            >
              <div className="plain-section-heading">
                <span>{district.label}</span>
                <strong>
                  {district.flood_level}/{MAX_FLOOD_LEVEL} flood
                </strong>
              </div>
              <div className="plain-tricks-metrics" aria-label={`${district.label} public counters`}>
                <Metric label="Flood" value={`${district.flood_level} / ${MAX_FLOOD_LEVEL}`} animationTarget={`flood-watch-flood-${district.district}`} />
                <Metric label="Levees" value={String(district.levees)} animationTarget={`flood-watch-levees-${district.district}`} />
              </div>
              <p className={`flood-watch-danger-cue ${danger}`}>{dangerLabel(district.flood_level)}</p>
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
              <small>{rolePower(role.role)}</small>
            </li>
          ))}
        </ol>
      </section>

      <div className="plain-latest" role="status" data-animation-target="flood-watch-outcome">
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

function Metric({ label, value, animationTarget }: { label: string; value: string; animationTarget: string }) {
  return (
    <div data-animation-target={animationTarget}>
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

// Both shipped variants (standard and deluge) lose at flood level 3; the view
// does not yet carry a per-variant threshold, so mirror the Rust constant here.
const MAX_FLOOD_LEVEL = 3;

type FloodDanger = "safe" | "rising" | "critical" | "inundated";

function floodDanger(level: number): FloodDanger {
  if (level >= MAX_FLOOD_LEVEL) {
    return "inundated";
  }
  if (level === MAX_FLOOD_LEVEL - 1) {
    return "critical";
  }
  if (level >= 1) {
    return "rising";
  }
  return "safe";
}

function dangerLabel(level: number): string {
  const stepsToLoss = MAX_FLOOD_LEVEL - level;
  if (stepsToLoss <= 0) {
    return "Inundated";
  }
  if (stepsToLoss === 1) {
    return "1 flood from shared loss";
  }
  return `${stepsToLoss} floods from shared loss`;
}

function rolePower(role: string): string {
  switch (role) {
    case "pumpwright":
      return "Bails 2 flood per action (others bail 1)";
    case "levee_warden":
      return "Adds 2 levees per action (others add 1)";
    default:
      return "Cooperative role";
  }
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
  return resolveSeatLabel(seat);
}

function registerFloodWatchAnimations(): void {
  animationRegistry.register("flood_watch", "forecast_revealed", (step, context) =>
    highlightTargets(context, ["flood-watch-deck"], step.reducedMotion),
  );
  animationRegistry.register("flood_watch", "environment_phase_began", (step, context) =>
    highlightTargets(context, ["flood-watch-turn", "flood-watch-districts"], step.reducedMotion),
  );
  animationRegistry.register("flood_watch", "event_drawn", (step, context) =>
    highlightTargets(context, ["flood-watch-deck", "flood-watch-undrawn"], step.reducedMotion),
  );
  animationRegistry.register("flood_watch", "deck_exhausted", (step, context) =>
    highlightTargets(context, ["flood-watch-deck", "flood-watch-undrawn"], step.reducedMotion),
  );

  animationRegistry.register("flood_watch", "district_bailed", (step, context) => {
    const district = stringField(effectFields(step), "district");
    return highlightDistrict(context, district, ["flood"], step.reducedMotion);
  });
  animationRegistry.register("flood_watch", "levee_placed", (step, context) => {
    const district = stringField(effectFields(step), "district");
    return highlightDistrict(context, district, ["levees"], step.reducedMotion);
  });
  animationRegistry.register("flood_watch", "levee_absorbed", (step, context) => {
    const district = stringField(effectFields(step), "district");
    return highlightDistrict(context, district, ["levees", "flood"], step.reducedMotion);
  });
  animationRegistry.register("flood_watch", "flood_level_rose", (step, context) => {
    const district = stringField(effectFields(step), "district");
    return highlightDistrict(context, district, ["flood"], step.reducedMotion);
  });
  animationRegistry.register("flood_watch", "district_inundated", (step, context) => {
    const district = stringField(effectFields(step), "district");
    return highlightDistrict(context, district, ["flood"], step.reducedMotion);
  });
  animationRegistry.register("flood_watch", "terminal", (step, context) =>
    highlightTargets(context, ["flood-watch-outcome"], step.reducedMotion, "fade"),
  );
}

function highlightDistrict(
  context: PresentationContext,
  district: string,
  counters: Array<"flood" | "levees">,
  reducedMotion: boolean,
): SchedulerPresentation {
  return highlightTargets(
    context,
    [`flood-watch-district-${district}`, ...counters.map((counter) => `flood-watch-${counter}-${district}`)],
    reducedMotion,
  );
}

function highlightTargets(
  context: PresentationContext,
  targetIds: string[],
  reducedMotion: boolean,
  kind: "highlight" | "fade" = "highlight",
): SchedulerPresentation {
  const root = context.root ?? document;
  const animations = uniqueElements(targetIds.flatMap((targetId) => [...root.querySelectorAll(targetSelector(targetId))])).map((element) =>
    kind === "fade" ? animateFade(element, reducedMotion) : animateHighlight(element, reducedMotion),
  );
  return { animations };
}

function targetSelector(targetId: string): string {
  return `[data-animation-target="${cssEscape(targetId)}"]`;
}

function effectFields(step: SchedulerStep): Record<string, unknown> {
  const envelope = step.entry.effect as unknown as Record<string, unknown>;
  return envelope["pay" + "load"] as Record<string, unknown>;
}

function stringField(fields: Record<string, unknown>, field: string): string {
  const value = fields[field];
  return typeof value === "string" ? value : "";
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
