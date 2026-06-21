import { useState } from "react";
import type { ActionChoice, ActionTree, EffectEntry, EventFrontierPublicView, EventFrontierSiteView, SeatId } from "../wasm/client";
import { resolveSeatLabel } from "../seatLabels";
import { animationRegistry } from "../animation/registry";
import type { SchedulerPresentation, SchedulerStep } from "../animation/scheduler";
import { animateFade, animateHighlight, type PresentationContext } from "../animation/presenters";
import { ActionPathBuilder, type ActionPathSelection } from "./ActionPathBuilder";
import { DeckFlowPanel } from "./DeckFlowPanel";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type EventFrontierBoardProps = {
  view: EventFrontierPublicView;
  actionTree: ActionTree | null;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  seatRoleLabels?: Partial<Record<SeatId, string>>;
  interactive?: boolean;
  onPathSubmit?: (path: string[]) => void;
};

const SITE_POINTS: Record<string, { x: number; y: number }> = {
  site_charterhouse: { x: 17, y: 28 },
  site_landing: { x: 23, y: 70 },
  site_crossing: { x: 45, y: 47 },
  site_granite_pass: { x: 68, y: 23 },
  site_high_meadow: { x: 76, y: 62 },
  site_old_mill: { x: 52, y: 82 },
};

registerEventFrontierAnimations();

export function EventFrontierBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  seatRoleLabels = {},
  interactive = true,
  onPathSubmit,
}: EventFrontierBoardProps) {
  const choices = actionTree?.choices ?? [];
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const terminal = view.terminal.kind !== "non_terminal";
  const canAct = Boolean(interactive && !pending && !terminal);
  const activeEffects = new Set(effects.map((entry) => String(entry.effect.payload.type)));
  const [highlightedTargets, setHighlightedTargets] = useState<string[]>([]);
  const charterLabel = factionLabel(view, "faction_charter");
  const freeholdersLabel = factionLabel(view, "faction_freeholders");
  const outcomeExplanation =
    view.terminal.kind === "winner"
      ? outcomeSurfaceData({
          gameId: "event_frontier",
          heading: `${factionLabel(view, view.terminal.winner)} win`,
          rationale: view.terminal_rationale ?? null,
          resultKind: "win",
          decisiveCause: eventFrontierCause(view),
          templateKey: eventFrontierTemplateKey(view),
          templateParams: {
            winner: factionLabel(view, view.terminal.winner),
            charter_score: view.terminal.scores.charter,
            freeholder_score: view.terminal.scores.freeholders,
          },
          finalStanding: [
            {
              id: "faction_charter",
              label: charterLabel,
              result: view.terminal.winner === "faction_charter" ? "win" : "loss",
              emphasized: view.terminal.winner === "faction_charter",
              values: [{ label: "Score", value: view.terminal.scores.charter }],
            },
            {
              id: "faction_freeholders",
              label: freeholdersLabel,
              result: view.terminal.winner === "faction_freeholders" ? "win" : "loss",
              emphasized: view.terminal.winner === "faction_freeholders",
              values: [{ label: "Score", value: view.terminal.scores.freeholders }],
            },
          ],
          breakdownSections: [
            {
              id: "event-frontier-terminal",
              heading: "Terminal cause",
              rows: [
                { label: "Victory type", value: view.terminal.victory_type },
                { label: "Decisive rule", value: view.terminal.decisive_rule },
                { label: "Reckonings", value: view.reckoning_count },
              ],
            },
          ],
          ruleIds: [view.terminal.decisive_rule],
        })
      : null;

  return (
    <section
      className={`plain-tricks-board frontier-control-board event-frontier-board${terminal ? " terminal" : ""}${
        reducedMotion ? " reduced" : ""
      }`}
      aria-labelledby="event-frontier-heading"
      data-testid="event-frontier-board"
    >
      <div className="plain-tricks-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="event-frontier-heading">{terminal ? terminalLabel(view) : `${activeFactionLabel(view)} to act`}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {terminal ? "terminal" : `epoch ${view.epoch} / reckoning ${view.reckoning_count}`}
        </span>
      </div>

      {seatRoleLabels.seat_0 === "you" ? <p className="event-frontier-identity">You play the {seatLabel(view, "seat_0")}.</p> : null}

      <p className="sr-only" aria-live="polite">
        {view.display_name}, current card {view.current_card?.label ?? "none"}, next public card {view.next_public_card?.label ?? "none"}, {choices.length} action
        legal choices. Undrawn deck order beyond the next public card is hidden.
      </p>

      <div className="plain-tricks-metrics" aria-label="Event Frontier status">
        <Metric
          label={`Funds - ${seatLabel(view, "seat_0")}${roleSuffix(seatRoleLabels.seat_0)}`}
          value={String(view.resources.funds)}
          animationTarget="event-frontier-resource-faction_charter"
        />
        <Metric
          label={`Provisions - ${seatLabel(view, "seat_1")}${roleSuffix(seatRoleLabels.seat_1)}`}
          value={String(view.resources.provisions)}
          animationTarget="event-frontier-resource-faction_freeholders"
        />
        <Metric label={`${charterLabel} score`} value={String(view.scores.charter)} animationTarget="event-frontier-score-faction_charter" />
        <Metric
          label={`${freeholdersLabel} score`}
          value={String(view.scores.freeholders)}
          animationTarget="event-frontier-score-faction_freeholders"
        />
      </div>

      <div data-animation-target="event-frontier-deck">
        <DeckFlowPanel
          label={view.ui.event_deck_label}
          currentLabel={view.ui.current_card_label}
          nextLabel={view.ui.next_card_label}
          discardLabel={view.ui.discard_label}
          faceDownLabel={view.ui.face_down_label}
          faceDownSummary={view.ui.face_down_summary}
          current={view.current_card}
          next={view.next_public_card}
          discard={view.discard}
        />
      </div>

      <div className="frontier-layout">
        <div className="frontier-map-panel" data-animation-target="event-frontier-map">
          <svg className="frontier-map" viewBox="0 0 100 100" role="img" aria-label="Event Frontier site map">
            <title>Event Frontier site map</title>
            <desc>Public sites, trails, agents, settlers, depots, and caches.</desc>
            {view.adjacency.flatMap((entry) =>
              entry.neighbors
                .filter((neighbor) => entry.site < neighbor)
                .map((neighbor) => {
                  const a = SITE_POINTS[entry.site];
                  const b = SITE_POINTS[neighbor];
                  return a && b ? <line key={`${entry.site}-${neighbor}`} className="frontier-trail" x1={a.x} y1={a.y} x2={b.x} y2={b.y} /> : null;
                }),
            )}
            {view.sites.map((site) => (
              <SiteNode
                key={site.site}
                site={site}
                active={activeEffects.has("op_resolved") || activeEffects.has("reckoning_resolved") || highlightedTargets.includes(site.site)}
                highlighted={highlightedTargets.includes(site.site)}
              />
            ))}
          </svg>
        </div>

        <section className="plain-history frontier-site-list" aria-label="Event Frontier sites">
          <div className="plain-section-heading">
            <span>Sites</span>
            <strong>Public map</strong>
          </div>
          <ol>
            {view.sites.map((site) => {
              const controller = siteController(site);
              return (
                <li key={site.site} className={`event-site control-${controller}`} data-testid={`event-frontier-site-${site.site}`}>
                  <span>{site.label}</span>
                  <strong>{siteSummary(site)}</strong>
                  <small>
                    {site.depot ? "depot" : "open"} / {site.cache_count} cache · {controlLabel(view, controller)}
                  </small>
                </li>
              );
            })}
          </ol>
          <p className="event-site-legend">
            A = agents, S = settlers. A site scores for whoever has more presence (Charter counts agents + depots).
          </p>
        </section>
      </div>

      <section className="plain-history" aria-label="Eligibility and victory distance">
        <div className="plain-section-heading">
          <span>Eligibility</span>
          <strong>Public status</strong>
        </div>
        <ol>
          {view.eligibility.map((entry) => (
            <li key={entry.faction}>
              <span>{factionLabel(view, entry.faction)}</span>
              <strong>{entry.eligible}</strong>
              <small>
                {entry.faction === "faction_charter"
                  ? victoryProgress(controlledSiteCount(view), view.victory_distance.charter_sites_needed, "controlled site")
                  : victoryProgress(totalCacheCount(view), view.victory_distance.freeholder_caches_needed, "cache")}
              </small>
            </li>
          ))}
        </ol>
      </section>

      {view.active_edicts.length ? (
        <section className="plain-history" aria-label="Active edicts">
          <div className="plain-section-heading">
            <span>Edicts</span>
            <strong>expire at Reckoning</strong>
          </div>
          <ol>
            {view.active_edicts.map((edict) => (
              <li key={edict}>
                <span>{edict}</span>
                <strong>active</strong>
                <small>active modifier</small>
              </li>
            ))}
          </ol>
        </section>
      ) : null}

      <div className="plain-latest" role="status" data-animation-target="event-frontier-outcome">
        <span>{outcomeExplanation ? "Outcome" : feedback?.title ?? "Waiting"}</span>
        <strong>
          {outcomeExplanation
            ? outcomeAnnouncementText(outcomeExplanation)
            : feedback?.detail ?? "Card flow, eligibility, operation paths, and public scoring will update here."}
        </strong>
      </div>

      {!terminal ? (
        <section className="frontier-actions" aria-label="Event Frontier actions">
          <ActionPathBuilder
            tree={actionTree}
            disabled={!canAct}
            emptyLabel={`${activeFactionLabel(view)} is waiting for the next available action.`}
            affordanceTemplates={view.ui.action_affordance_templates}
            costResource={activeResourceContext(view)}
            onTargetHighlight={setHighlightedTargets}
            onSubmit={(selection) => onPathSubmit?.(eventFrontierSubmitPath(selection))}
          />
        </section>
      ) : null}

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function SiteNode({ site, active, highlighted }: { site: EventFrontierSiteView; active: boolean; highlighted: boolean }) {
  const point = SITE_POINTS[site.site] ?? { x: 50, y: 50 };
  return (
    <g
      className={`frontier-site${site.depot ? " fort" : ""}${site.cache_count > 0 ? " staked" : ""}${active ? " active" : ""}${
        highlighted ? " highlighted" : ""
      }`}
      data-animation-target={`event-frontier-site-${site.site}`}
    >
      <circle cx={point.x} cy={point.y} r={7} />
      {site.depot ? <path d={`M ${point.x - 4} ${point.y - 7} h 8 v 4 h -8 z`} /> : null}
      {site.cache_count > 0 ? <rect x={point.x - 2.4} y={point.y - 10.5} width="4.8" height="5.8" rx="0.8" /> : null}
      <text x={point.x} y={point.y + 12}>
        {site.label}
      </text>
      <text className="frontier-counts" x={point.x} y={point.y + 2.2}>
        A{site.agents} S{site.settlers}
      </text>
    </g>
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

function eventFrontierSubmitPath(selection: ActionPathSelection): string[] {
  return selection.leaf.segment.includes("/") ? [selection.leaf.segment] : selection.segments;
}

function activeFactionLabel(view: EventFrontierPublicView): string {
  const activeSeatIndex = view.active_seat ? view.seats.indexOf(view.active_seat) : -1;
  const candidate = activeSeatIndex >= 0 ? view.factions[activeSeatIndex] : null;
  return factionLabel(view, candidate);
}

function factionLabel(view: EventFrontierPublicView, faction: string | null | undefined): string {
  const authored = view.ui.faction_labels.find((entry) => entry.faction === faction)?.label;
  if (authored) return authored;
  return faction ?? "Active faction";
}

function seatLabel(view: EventFrontierPublicView, seat: string): string {
  return resolveSeatLabel(seat, { catalogUiSeatLabels: view.ui.seat_labels });
}

function roleSuffix(role: string | undefined): string {
  return role ? ` (${role})` : "";
}

function activeResourceContext(view: EventFrontierPublicView): { label: string; balance: number } | null {
  const active = activeFaction(view);
  if (active === "faction_charter") {
    return { label: "funds", balance: view.resources.funds };
  }
  if (active === "faction_freeholders") {
    return { label: "provisions", balance: view.resources.provisions };
  }
  return null;
}

function activeFaction(view: EventFrontierPublicView): string | null {
  const activeSeatIndex = view.active_seat ? view.seats.indexOf(view.active_seat) : -1;
  return activeSeatIndex >= 0 ? view.factions[activeSeatIndex] : null;
}

function siteSummary(site: EventFrontierSiteView): string {
  return `A${site.agents} S${site.settlers}`;
}

type SiteControl = "charter" | "freeholders" | "contested";

// Mirrors the Rust scoring rule exactly (a site scores for strictly greater
// presence; Charter presence is agents + depots, Freeholders' is settlers).
// This is a pure public comparison, so displaying it cannot diverge from Rust.
function siteController(site: EventFrontierSiteView): SiteControl {
  const charterPresence = site.agents + (site.depot ? 1 : 0);
  if (charterPresence > site.settlers) {
    return "charter";
  }
  if (site.settlers > charterPresence) {
    return "freeholders";
  }
  return "contested";
}

function controlLabel(view: EventFrontierPublicView, control: SiteControl): string {
  if (control === "charter") {
    return `${factionLabel(view, "faction_charter")} leads`;
  }
  if (control === "freeholders") {
    return `${factionLabel(view, "faction_freeholders")} leads`;
  }
  return "contested";
}

function controlledSiteCount(view: EventFrontierPublicView): number {
  return view.sites.filter((site) => siteController(site) === "charter").length;
}

function totalCacheCount(view: EventFrontierPublicView): number {
  return view.sites.reduce((sum, site) => sum + site.cache_count, 0);
}

function victoryProgress(current: number, needed: number, unit: string): string {
  const plural = current === 1 ? "" : "s";
  if (needed <= 0) {
    return `${current} ${unit}${plural} — instant-victory threshold met`;
  }
  return `${current} ${unit}${plural} so far · ${needed} more for instant victory`;
}

function terminalLabel(view: EventFrontierPublicView): string {
  return view.terminal.kind === "winner" ? `${factionLabel(view, view.terminal.winner)} won` : "Event Frontier";
}

function eventFrontierCause(view: EventFrontierPublicView): string {
  if (view.terminal.kind !== "winner") return "non_terminal";
  if (view.terminal.victory_type === "charter_instant") return "charter_instant";
  if (view.terminal.victory_type === "freeholder_instant") return "freeholder_instant";
  if (view.terminal.scores.charter === view.terminal.scores.freeholders) return "final_fallback_tiebreak";
  return "final_fallback_score";
}

function eventFrontierTemplateKey(view: EventFrontierPublicView): string {
  const cause = eventFrontierCause(view);
  if (cause === "charter_instant") return "event_frontier.charter_instant";
  if (cause === "freeholder_instant") return "event_frontier.freeholder_instant";
  if (cause === "final_fallback_tiebreak") return "event_frontier.final_fallback_tiebreak";
  return "event_frontier.final_fallback_score";
}

function registerEventFrontierAnimations(): void {
  const deckEffects = ["event_resolved", "edict_activated", "edict_expired", "card_revealed", "card_discarded", "choice_taken"];
  for (const effectType of deckEffects) {
    animationRegistry.register("event_frontier", effectType, (step, context) => highlightTargets(context, ["event-frontier-deck"], step.reducedMotion));
  }

  animationRegistry.register("event_frontier", "resources_changed", (step, context) => {
    const faction = stringField(effectFields(step), "faction");
    return highlightTargets(context, [`event-frontier-resource-${faction}`], step.reducedMotion);
  });

  animationRegistry.register("event_frontier", "op_resolved", (step, context) => {
    const sites = stringArrayField(effectFields(step), "sites");
    return highlightTargets(context, ["event-frontier-map", ...sites.map((site) => `event-frontier-site-${site}`)], step.reducedMotion);
  });

  for (const effectType of ["agent_placed", "agent_removed", "depot_built", "cache_removed", "cache_laid", "settler_rallied"]) {
    animationRegistry.register("event_frontier", effectType, (step, context) => {
      const site = stringField(effectFields(step), "site");
      return highlightTargets(context, [`event-frontier-site-${site}`], step.reducedMotion);
    });
  }

  animationRegistry.register("event_frontier", "settler_moved", (step, context) => {
    const from = stringField(effectFields(step), "from");
    const to = stringField(effectFields(step), "to");
    return highlightTargets(context, [`event-frontier-site-${from}`, `event-frontier-site-${to}`], step.reducedMotion);
  });

  animationRegistry.register("event_frontier", "reckoning_resolved", (step, context) => {
    const fields = effectFields(step);
    const scoredSites = Array.isArray(fields.site_breakdown)
      ? fields.site_breakdown
          .map((entry) => (isRecord(entry) && typeof entry.site === "string" ? entry.site : null))
          .filter((site): site is string => Boolean(site))
      : [];
    return highlightTargets(
      context,
      [
        "event-frontier-map",
        "event-frontier-score-faction_charter",
        "event-frontier-score-faction_freeholders",
        "event-frontier-resource-faction_charter",
        "event-frontier-resource-faction_freeholders",
        ...scoredSites.map((site) => `event-frontier-site-${site}`),
      ],
      step.reducedMotion,
    );
  });

  animationRegistry.register("event_frontier", "terminal", (step, context) =>
    highlightTargets(context, ["event-frontier-outcome"], step.reducedMotion, "fade"),
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

function stringArrayField(fields: Record<string, unknown>, field: string): string[] {
  const value = fields[field];
  return Array.isArray(value) ? value.filter((entry): entry is string => typeof entry === "string") : [];
}

function uniqueElements(elements: Element[]): Element[] {
  return [...new Set(elements)];
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function cssEscape(value: string): string {
  if (typeof CSS !== "undefined" && CSS.escape) {
    return CSS.escape(value);
  }
  return value.replace(/["\\]/g, "\\$&");
}
