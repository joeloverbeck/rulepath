import type { ActionChoice, ActionTree, EffectEntry, EventFrontierPublicView, EventFrontierSiteView } from "../wasm/client";
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

export function EventFrontierBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onPathSubmit,
}: EventFrontierBoardProps) {
  const choices = actionTree?.choices ?? [];
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const terminal = view.terminal.kind !== "non_terminal";
  const canAct = Boolean(interactive && !pending && !terminal);
  const activeEffects = new Set(effects.map((entry) => String(entry.effect.payload.type)));
  const outcomeExplanation =
    view.terminal.kind === "winner"
      ? outcomeSurfaceData({
          gameId: "event_frontier",
          heading: `${factionLabel(view.terminal.winner)} win`,
          rationale: view.terminal_rationale ?? null,
          resultKind: "win",
          decisiveCause: eventFrontierCause(view),
          templateKey: eventFrontierTemplateKey(view),
          templateParams: {
            winner: factionLabel(view.terminal.winner),
            charter_score: view.terminal.scores.charter,
            freeholder_score: view.terminal.scores.freeholders,
          },
          finalStanding: [
            {
              id: "faction_charter",
              label: "Charter",
              result: view.terminal.winner === "faction_charter" ? "win" : "loss",
              emphasized: view.terminal.winner === "faction_charter",
              values: [{ label: "Score", value: view.terminal.scores.charter }],
            },
            {
              id: "faction_freeholders",
              label: "Freeholders",
              result: view.terminal.winner === "faction_freeholders" ? "win" : "loss",
              emphasized: view.terminal.winner === "faction_freeholders",
              values: [{ label: "Score", value: view.terminal.scores.freeholders }],
            },
          ],
          breakdownSections: [
            {
              id: "event-frontier-terminal",
              heading: "Rust terminal cause",
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

      <p className="sr-only" aria-live="polite">
        {view.display_name}, current card {view.current_card?.label ?? "none"}, next public card {view.next_public_card?.label ?? "none"}, {choices.length} action
        legal choices. Undrawn deck order beyond the next public card is hidden.
      </p>

      <div className="plain-tricks-metrics" aria-label="Event Frontier status">
        <Metric label="Funds" value={String(view.resources.funds)} />
        <Metric label="Provisions" value={String(view.resources.provisions)} />
        <Metric label="Charter score" value={String(view.scores.charter)} />
        <Metric label="Freeholder score" value={String(view.scores.freeholders)} />
      </div>

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

      <div className="frontier-layout">
        <div className="frontier-map-panel">
          <svg className="frontier-map" viewBox="0 0 100 100" role="img" aria-label="Event Frontier site map">
            <title>Event Frontier site map</title>
            <desc>Public sites, trails, agents, settlers, depots, and caches from the Rust view.</desc>
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
              <SiteNode key={site.site} site={site} active={activeEffects.has("op_resolved") || activeEffects.has("reckoning_resolved")} />
            ))}
          </svg>
        </div>

        <section className="plain-history frontier-site-list" aria-label="Event Frontier sites">
          <div className="plain-section-heading">
            <span>Sites</span>
            <strong>Public map</strong>
          </div>
          <ol>
            {view.sites.map((site) => (
              <li key={site.site}>
                <span>{site.label}</span>
                <strong>{siteSummary(site)}</strong>
                <small>{site.depot ? "depot" : "open"} / {site.cache_count} cache</small>
              </li>
            ))}
          </ol>
        </section>
      </div>

      <section className="plain-history" aria-label="Eligibility and victory distance">
        <div className="plain-section-heading">
          <span>Eligibility</span>
          <strong>Rust projection</strong>
        </div>
        <ol>
          {view.eligibility.map((entry) => (
            <li key={entry.faction}>
              <span>{factionLabel(entry.faction)}</span>
              <strong>{entry.eligible}</strong>
              <small>
                {entry.faction === "faction_charter"
                  ? `${view.victory_distance.charter_sites_needed} sites needed`
                  : `${view.victory_distance.freeholder_caches_needed} caches needed`}
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
                <small>Rust modifier</small>
              </li>
            ))}
          </ol>
        </section>
      ) : null}

      <div className="plain-latest" role="status">
        <span>{outcomeExplanation ? "Outcome" : feedback?.title ?? "Waiting"}</span>
        <strong>
          {outcomeExplanation
            ? outcomeAnnouncementText(outcomeExplanation)
            : feedback?.detail ?? "Rust/WASM supplies card flow, eligibility, operation paths, and public scoring."}
        </strong>
      </div>

      {!terminal ? (
        <section className="frontier-actions" aria-label="Event Frontier actions">
          <ActionPathBuilder
            tree={actionTree}
            disabled={!canAct}
            onSubmit={(selection) => onPathSubmit?.(eventFrontierSubmitPath(selection))}
          />
        </section>
      ) : null}

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function SiteNode({ site, active }: { site: EventFrontierSiteView; active: boolean }) {
  const point = SITE_POINTS[site.site] ?? { x: 50, y: 50 };
  return (
    <g className={`frontier-site${site.depot ? " fort" : ""}${site.cache_count > 0 ? " staked" : ""}${active ? " active" : ""}`}>
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

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div>
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
  return factionLabel(candidate);
}

function factionLabel(faction: string | null | undefined): string {
  if (faction === "faction_charter") return "Charter";
  if (faction === "faction_freeholders") return "Freeholders";
  return faction ?? "Rust";
}

function siteSummary(site: EventFrontierSiteView): string {
  return `A${site.agents} S${site.settlers}`;
}

function terminalLabel(view: EventFrontierPublicView): string {
  return view.terminal.kind === "winner" ? `${factionLabel(view.terminal.winner)} won` : "Event Frontier";
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
