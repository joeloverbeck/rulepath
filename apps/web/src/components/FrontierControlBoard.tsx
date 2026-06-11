import { useMemo } from "react";
import type { ActionChoice, ActionTree, EffectEntry, FrontierControlPublicView, FrontierControlSiteView } from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type FrontierControlBoardProps = {
  view: FrontierControlPublicView;
  actionTree: ActionTree | null;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onPathSubmit?: (path: string[]) => void;
};

type SitePoint = { x: number; y: number };

const SITE_POINTS: Record<string, SitePoint> = {
  site_gatehouse: { x: 16, y: 23 },
  site_signal_hill: { x: 41, y: 14 },
  site_quarry: { x: 45, y: 43 },
  site_ford: { x: 28, y: 63 },
  site_base_camp: { x: 47, y: 84 },
  site_timberline: { x: 73, y: 67 },
  site_goldfield: { x: 78, y: 26 },
};

// Presentation-only trail layout. Legality and supply connectivity are Rust-owned.
const STANDARD_TRAILS: Array<[string, string]> = [
  ["site_gatehouse", "site_ford"],
  ["site_gatehouse", "site_quarry"],
  ["site_gatehouse", "site_signal_hill"],
  ["site_signal_hill", "site_quarry"],
  ["site_signal_hill", "site_goldfield"],
  ["site_quarry", "site_ford"],
  ["site_quarry", "site_timberline"],
  ["site_ford", "site_base_camp"],
  ["site_timberline", "site_base_camp"],
  ["site_timberline", "site_goldfield"],
];

const HIGHLANDS_TRAILS: Array<[string, string]> = [
  ["site_gatehouse", "site_signal_hill"],
  ["site_gatehouse", "site_ford"],
  ["site_signal_hill", "site_quarry"],
  ["site_signal_hill", "site_goldfield"],
  ["site_quarry", "site_goldfield"],
  ["site_quarry", "site_timberline"],
  ["site_quarry", "site_ford"],
  ["site_ford", "site_base_camp"],
  ["site_timberline", "site_base_camp"],
  ["site_timberline", "site_goldfield"],
];

export function FrontierControlBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onPathSubmit,
}: FrontierControlBoardProps) {
  const choices = actionTree?.choices ?? [];
  const grouped = useMemo(() => groupChoices(choices), [choices]);
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const terminal = view.terminal.kind !== "non_terminal";
  const canAct = Boolean(interactive && !pending && !terminal);
  const activeFaction = factionLabel(view.active_faction);
  const trails = view.variant_id === "frontier_control_highlands" ? HIGHLANDS_TRAILS : STANDARD_TRAILS;
  const activeEffects = new Set(effects.map((entry) => entry.effect.payload.type));
  const outcomeExplanation =
    view.terminal.kind === "winner"
      ? outcomeSurfaceData({
          gameId: "frontier_control",
          heading: `${factionLabel(view.terminal.winner)} win`,
          rationale: view.terminal_rationale ?? null,
          resultKind: "win",
          decisiveCause: view.terminal.garrison_tiebreak ? "garrison_tiebreak" : "score_compare",
          templateKey: view.terminal.garrison_tiebreak
            ? "frontier_control.garrison_tiebreak"
            : "frontier_control.score_compare",
          templateParams: {
            winner: factionLabel(view.terminal.winner),
            garrison_score: view.terminal.scores.garrison,
            prospector_score: view.terminal.scores.prospectors,
          },
          finalStanding: [
            {
              id: "faction_garrison",
              label: "Garrison",
              result: view.terminal.winner === "faction_garrison" ? "win" : "loss",
              emphasized: view.terminal.winner === "faction_garrison",
              values: [{ label: "Score", value: view.terminal.scores.garrison }],
            },
            {
              id: "faction_prospectors",
              label: "Prospectors",
              result: view.terminal.winner === "faction_prospectors" ? "win" : "loss",
              emphasized: view.terminal.winner === "faction_prospectors",
              values: [{ label: "Score", value: view.terminal.scores.prospectors }],
            },
          ],
          breakdownSections: [
            {
              id: "terminal-summary",
              heading: "Rust terminal summary",
              summary: view.terminal.summary,
              rows: [
                { label: "Garrison tiebreak", value: view.terminal.garrison_tiebreak ? "applied" : "not applied" },
                { label: "Round", value: view.round_number },
              ],
            },
          ],
          ruleIds: view.terminal.garrison_tiebreak
            ? ["FC-TERM-GARRISON-TIEBREAK", "FC-SCORE-COMPARABLE-TRACK"]
            : ["FC-TERM-SCORE-COMPARE", "FC-SCORE-COMPARABLE-TRACK"],
        })
      : null;

  return (
    <section
      className={`plain-tricks-board frontier-control-board${terminal ? " terminal" : ""}${reducedMotion ? " reduced" : ""}`}
      aria-labelledby="frontier-control-heading"
      data-testid="frontier-control-board"
    >
      <div className="plain-tricks-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="frontier-control-heading">{terminal ? terminalLabel(view) : `${activeFaction} to act`}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {terminal ? "terminal" : `${activeFaction} round ${view.round_number}`}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {view.display_name}, round {view.round_number}, {activeFaction} active, {choices.length} Rust legal choices. Scores:
        Garrison {view.scores.garrison}, Prospectors {view.scores.prospectors}.
      </p>

      <div className="plain-tricks-metrics" aria-label="Frontier Control status">
        <Metric label="Round" value={String(view.round_number)} />
        <Metric label="Budget" value={budgetLabel(view)} />
        <Metric label="Garrison" value={String(view.scores.garrison)} />
        <Metric label="Prospectors" value={String(view.scores.prospectors)} />
      </div>

      <div className="frontier-layout">
        <div className="frontier-map-panel">
          <svg className="frontier-map" viewBox="0 0 100 100" role="img" aria-label="Frontier trail map">
            <title>Frontier trail map</title>
            <desc>Sites, trails, public units, forts, stakes, and Rust-projected supplied state.</desc>
            {trails.map(([from, to]) => {
              const a = SITE_POINTS[from];
              const b = SITE_POINTS[to];
              if (!a || !b) return null;
              return <line key={`${from}-${to}`} className="frontier-trail" x1={a.x} y1={a.y} x2={b.x} y2={b.y} />;
            })}
            {view.sites.map((site) => (
              <SiteNode key={site.site} site={site} active={activeEffects.has("crew_marched") || activeEffects.has("guard_patrolled")} />
            ))}
          </svg>
        </div>

        <section className="plain-history frontier-site-list" aria-label="Frontier sites">
          <div className="plain-section-heading">
            <span>Sites</span>
            <strong>Rust public view</strong>
          </div>
          <ol>
            {view.sites.map((site) => (
              <li key={site.site}>
                <span>{site.label}</span>
                <strong>{siteSummary(site)}</strong>
                <small>{siteStatus(site)}</small>
              </li>
            ))}
          </ol>
        </section>
      </div>

      <section className="plain-history" aria-label="Faction seats">
        <div className="plain-section-heading">
          <span>Factions</span>
          <strong>{terminal ? "Complete" : `${activeFaction} active`}</strong>
        </div>
        <ol>
          {view.factions.map((faction) => (
            <li key={faction.seat}>
              <span>{faction.label}</span>
              <strong>{faction.seat}</strong>
              <small>{faction.faction === view.active_faction && !terminal ? "active" : "waiting"}</small>
            </li>
          ))}
        </ol>
      </section>

      <div className="plain-latest" role="status">
        <span>{outcomeExplanation ? "Outcome" : feedback?.title ?? "Waiting"}</span>
        <strong>
          {outcomeExplanation
            ? outcomeAnnouncementText(outcomeExplanation)
            : feedback?.detail ?? "Rust/WASM supplies legal faction actions and public supply status."}
        </strong>
      </div>

      {!terminal ? (
        <section className="frontier-actions" aria-label="Frontier Control actions">
          {Object.entries(grouped).map(([family, familyChoices]) => (
            <div className="frontier-action-group" key={family}>
              <h3>{actionFamilyLabel(family)}</h3>
              <div className="action-list">
                {familyChoices.map((choice) => (
                  <button
                    type="button"
                    key={choice.segment}
                    disabled={!canAct}
                    aria-label={choice.accessibility_label}
                    data-testid={`frontier-choice-${choice.segment.replaceAll("/", "-")}`}
                    onClick={() => onPathSubmit?.([choice.segment])}
                  >
                    {choice.label}
                  </button>
                ))}
              </div>
            </div>
          ))}
          {choices.length === 0 ? <p className="muted">No Rust legal actions available.</p> : null}
        </section>
      ) : null}

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function SiteNode({ site, active }: { site: FrontierControlSiteView; active: boolean }) {
  const point = SITE_POINTS[site.site] ?? { x: 50, y: 50 };
  const suppliedClass = site.supplied === true ? " supplied" : site.supplied === false ? " cut" : "";
  return (
    <g className={`frontier-site${site.fort ? " fort" : ""}${site.stake ? " staked" : ""}${suppliedClass}${active ? " active" : ""}`}>
      <circle cx={point.x} cy={point.y} r={6.8} />
      {site.fort ? <path d={`M ${point.x - 4} ${point.y - 7} h 8 v 4 h -8 z`} /> : null}
      {site.stake ? <rect x={point.x - 2.4} y={point.y - 10.5} width="4.8" height="5.8" rx="0.8" /> : null}
      <text x={point.x} y={point.y + 12}>
        {site.label}
      </text>
      <text className="frontier-counts" x={point.x} y={point.y + 2.2}>
        G{site.guards} C{site.crews}
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

function groupChoices(choices: ActionChoice[]): Record<string, ActionChoice[]> {
  const grouped: Record<string, ActionChoice[]> = {};
  for (const choice of choices) {
    const family = metadataValue(choice, "action_family") ?? choice.segment.split("/")[0];
    grouped[family] = [...(grouped[family] ?? []), choice];
  }
  return grouped;
}

function metadataValue(choice: ActionChoice, key: string): string | null {
  return choice.metadata?.find((entry) => entry.key === key)?.value ?? null;
}

function actionFamilyLabel(family: string): string {
  return family.replaceAll("-", " ").replaceAll("_", " ");
}

function factionLabel(faction: string): string {
  if (faction === "faction_garrison") return "Garrison";
  if (faction === "faction_prospectors") return "Prospectors";
  return faction;
}

function budgetLabel(view: FrontierControlPublicView): string {
  return view.phase.kind === "action" ? String(view.phase.budget_remaining) : "Terminal";
}

function terminalLabel(view: FrontierControlPublicView): string {
  return view.terminal.kind === "winner" ? view.terminal.summary : "Match complete";
}

function siteSummary(site: FrontierControlSiteView): string {
  const parts = [];
  if (site.fort) parts.push("fort");
  if (site.stake) parts.push(`stake ${site.stake_value}`);
  if (parts.length === 0) parts.push(site.stake_value > 0 ? `value ${site.stake_value}` : "route");
  return parts.join(", ");
}

function siteStatus(site: FrontierControlSiteView): string {
  const supplied = site.supplied === true ? "supplied" : site.supplied === false ? "cut" : "supply n/a";
  return `guards ${site.guards}, crews ${site.crews}, ${supplied}`;
}
