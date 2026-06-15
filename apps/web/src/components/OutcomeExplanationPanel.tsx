import { useId, useMemo, useState } from "react";
import {
  isOutcomeExplanationTemplateKey,
  outcomeDisplayText,
  outcomeDisplayValue,
  outcomeExplanationTemplates,
  seatDisplayLabel,
  type OutcomeExplanationTemplate,
} from "./outcomeExplanationTemplates";
import { RiverLedgerCard, riverLedgerCardGroupLabel, type RiverLedgerCardLike } from "./RiverLedgerCard";

type OutcomeValue = string | number | boolean | null;

export type OutcomeExplanationParams = Record<string, OutcomeValue>;

export type OutcomeExplanationStanding = {
  id: string;
  label: string;
  result?: string;
  emphasized?: boolean;
  values: readonly OutcomeExplanationField[];
  showdownStrength?: RiverLedgerShowdownStrength | null;
};

export type OutcomeExplanationField = {
  label: string;
  value: OutcomeValue;
  emphasized?: boolean;
  ruleId?: string;
};

export type OutcomeExplanationBreakdownSection = {
  id: string;
  heading: string;
  summary?: string;
  rows?: readonly OutcomeExplanationField[];
  defaultOpen?: boolean;
};

export type OutcomeExplanationSurfaceData = {
  gameId: string;
  heading: string;
  resultKind: string;
  decisiveCause: string;
  templateKey: string;
  templateParams?: OutcomeExplanationParams;
  headline?: string;
  decisiveComparison?: string;
  comparisonBasis?: string;
  finalStanding: readonly OutcomeExplanationStanding[];
  breakdownSections?: readonly OutcomeExplanationBreakdownSection[];
  ruleIds?: readonly string[];
};

type RiverLedgerShowdownStrength = {
  category: string;
  tie_break_vector: readonly number[];
  best_five: readonly RiverLedgerCardLike[];
  result_label: string;
  hand_name: string;
  rank_explanation: string;
  comparison_note: string;
  best_five_accessibility_label: string;
};

export type OutcomeExplanationSourceRationale = {
  result_kind?: string;
  decisive_cause?: string;
  template_key?: string;
  template_params?: OutcomeExplanationParams;
  headline?: string | null;
  decisive_comparison?: string | null;
  comparison_basis?: string | null;
  decisive_rule_ids?: readonly string[];
  final_standing?: readonly {
    seat: string;
    label?: string;
    result?: string;
    emphasized?: boolean;
    values?: readonly OutcomeExplanationField[];
    strength?: RiverLedgerShowdownStrength | null;
  }[];
  breakdown_sections?: readonly OutcomeExplanationBreakdownSection[];
} | null;

export type OutcomeExplanationAdapterInput = {
  gameId: string;
  heading: string;
  rationale?: OutcomeExplanationSourceRationale;
  resultKind: string;
  decisiveCause: string;
  templateKey: string;
  templateParams?: OutcomeExplanationParams;
  finalStanding: readonly OutcomeExplanationStanding[];
  breakdownSections?: readonly OutcomeExplanationBreakdownSection[];
  ruleIds?: readonly string[];
};

type OutcomeExplanationPanelProps = {
  explanation: OutcomeExplanationSurfaceData | null;
  reducedMotion?: boolean;
  initiallyExpanded?: boolean;
};

export function OutcomeExplanationPanel({
  explanation,
  reducedMotion = false,
  initiallyExpanded = false,
}: OutcomeExplanationPanelProps) {
  const rootId = useId();
  const headingId = `${rootId}-heading`;
  const detailsId = `${rootId}-details`;
  const [expandedSections, setExpandedSections] = useState<Record<string, boolean>>({});
  const template = useMemo(() => templateFor(explanation?.templateKey ?? ""), [explanation?.templateKey]);

  if (!explanation) {
    return null;
  }

  const summary = outcomeSummaryText(explanation);
  const sections = explanation.breakdownSections ?? [];
  const riverShowdown = riverLedgerShowdownData(explanation);
  const ruleIds = explanation.ruleIds ?? [];
  const showRuleFooter = ruleIds.length > 0 && !riverShowdown;

  return (
    <section
      className={`outcome-explanation-panel${reducedMotion ? " reduced" : ""}`}
      aria-labelledby={headingId}
      data-outcome-game={explanation.gameId}
    >
      <div className="outcome-summary">
        <p className="eyebrow">Outcome</p>
        <h2 id={headingId}>{explanation.heading}</h2>
        <p>{summary}</p>
      </div>

      {riverShowdown ? <RiverLedgerShowdown explanation={explanation} /> : null}

      <div className="outcome-standing" aria-label="Final standing">
        {explanation.finalStanding.map((standing) => (
          <article
            className={`outcome-standing-row${standing.emphasized ? " emphasized" : ""}`}
            key={standing.id}
            aria-label={standing.result ? `${standing.label}, ${outcomeDisplayValue(standing.result)}` : standing.label}
          >
            <header>
              <strong>{standing.label}</strong>
              {standing.result ? <span>{outcomeDisplayValue(standing.result)}</span> : null}
            </header>
            <dl>
              {standing.values
                .filter((field) => standingFieldVisible(field, standing, explanation))
                .map((field) => (
                  <FieldRow field={field} key={`${standing.id}-${field.label}`} />
                ))}
            </dl>
          </article>
        ))}
      </div>

      {sections.length > 0 ? (
        <div className="outcome-breakdown" id={detailsId}>
          {sections.map((section) => {
            const defaultOpen = section.defaultOpen ?? (initiallyExpanded || isShortSection(section));
            const sectionOpen = expandedSections[section.id] ?? defaultOpen;
            const sectionId = `${rootId}-${section.id}`;
            const buttonId = `${sectionId}-button`;
            return (
              <section className="outcome-breakdown-section" key={section.id} aria-labelledby={buttonId}>
                <button
                  type="button"
                  id={buttonId}
                  aria-expanded={sectionOpen}
                  aria-controls={sectionId}
                  onClick={() =>
                    setExpandedSections((current) => ({
                      ...current,
                      [section.id]: !(current[section.id] ?? defaultOpen),
                    }))
                  }
                >
                  {section.heading}
                </button>
                <div id={sectionId} hidden={!sectionOpen}>
                  {section.summary ? <p>{section.summary}</p> : null}
                  {section.rows?.length ? (
                    <dl>
                      {section.rows.map((field) => (
                        <FieldRow field={field} key={`${section.id}-${field.label}`} />
                      ))}
                    </dl>
                  ) : null}
                </div>
              </section>
            );
          })}
        </div>
      ) : null}

      {showRuleFooter ? (
        <footer className="outcome-rule-refs" aria-label="Outcome rule references">
          <span>{template?.ruleRefLabel ?? "Rule references"}</span>
          <ul>
            {ruleIds.map((ruleId) => (
              <li key={ruleId}>
                <code>{ruleId}</code>
              </li>
            ))}
          </ul>
        </footer>
      ) : null}
    </section>
  );
}

export function outcomeSurfaceData(input: OutcomeExplanationAdapterInput): OutcomeExplanationSurfaceData {
  const rationaleStanding = input.rationale?.final_standing?.length
    ? input.rationale.final_standing.map((standing) => ({
        id: standing.seat,
        label: standing.label ? outcomeDisplayText(standing.label) : seatDisplayLabel(standing.seat),
        result: standing.result,
        emphasized: standing.emphasized,
        values: standing.values ?? [],
        showdownStrength: standing.strength ?? null,
      }))
    : null;

  const breakdownSections = input.rationale?.breakdown_sections?.length
    ? input.rationale.breakdown_sections
    : input.breakdownSections;

  return {
    gameId: input.gameId,
    heading: outcomeDisplayText(input.heading),
    resultKind: input.rationale?.result_kind ?? input.resultKind,
    decisiveCause: input.rationale?.decisive_cause ?? input.decisiveCause,
    templateKey: input.rationale?.template_key ?? input.templateKey,
    templateParams: input.rationale?.template_params ?? input.templateParams,
    headline: normalizeOptionalText(input.rationale?.headline),
    decisiveComparison: normalizeOptionalText(input.rationale?.decisive_comparison),
    comparisonBasis: normalizeOptionalText(input.rationale?.comparison_basis),
    finalStanding: orderStandings(rationaleStanding ?? input.finalStanding.map(normalizeStanding)),
    breakdownSections: breakdownSections?.map((section) => normalizeBreakdownSection(section, input.rationale?.decisive_cause ?? input.decisiveCause)),
    ruleIds: input.rationale?.decisive_rule_ids ?? input.ruleIds,
  };
}

export function outcomeSummaryText(explanation: OutcomeExplanationSurfaceData): string {
  const template = templateFor(explanation.templateKey);
  return template
    ? renderTemplate(template.summary, explanation.templateParams ?? {})
    : outcomeDisplayText(explanation.decisiveCause);
}

export function outcomeAnnouncementText(explanation: OutcomeExplanationSurfaceData): string {
  return `${explanation.heading} - ${outcomeSummaryText(explanation)}`;
}

function FieldRow({ field }: { field: OutcomeExplanationField }) {
  return (
    <div className={field.emphasized ? "emphasized" : ""}>
      <dt>{outcomeDisplayText(field.label)}</dt>
      <dd>
        {formatValue(field.value)}
        {field.ruleId ? <small>{field.ruleId}</small> : null}
      </dd>
    </div>
  );
}

function RiverLedgerShowdown({ explanation }: { explanation: OutcomeExplanationSurfaceData }) {
  const standings = riverLedgerShowdownData(explanation);
  if (!standings) {
    return null;
  }

  return (
    <section className="river-ledger-showdown-panel" aria-label="Showdown explanation">
      <div className="river-ledger-showdown-lead">
        {explanation.headline ? <strong>{explanation.headline}</strong> : null}
        {explanation.decisiveComparison ? <p>{explanation.decisiveComparison}</p> : null}
        {explanation.comparisonBasis ? <p>{explanation.comparisonBasis}</p> : null}
      </div>

      <div className="river-ledger-showdown-hands">
        {standings.map((standing) => {
          const strength = standing.showdownStrength;
          if (!strength) {
            return null;
          }
          return (
            <article
              className={`river-ledger-showdown-hand${standing.emphasized ? " emphasized" : ""}`}
              key={standing.id}
              aria-label={`${standing.label}, ${strength.best_five_accessibility_label}`}
            >
              <header>
                <span>{standing.label}</span>
                <strong>{strength.result_label}</strong>
              </header>
              <p>{strength.rank_explanation}</p>
              <p>{strength.comparison_note}</p>
              <div
                className="river-ledger-showdown-cards"
                aria-label={riverLedgerCardGroupLabel(strength.best_five, strength.best_five_accessibility_label)}
              >
                {strength.best_five.map((card) => (
                  <RiverLedgerCard
                    card={card}
                    className="river-ledger-showdown-card"
                    key={card.card_id}
                    tone="showdown"
                  />
                ))}
              </div>
            </article>
          );
        })}
      </div>

      <details className="river-ledger-showdown-details">
        <summary>Showdown details</summary>
        <dl>
          {standings.map((standing) => {
            const strength = standing.showdownStrength;
            if (!strength) {
              return null;
            }
            return (
              <div key={`${standing.id}-raw`}>
                <dt>{standing.label}</dt>
                <dd>
                  {outcomeDisplayText(strength.category)}; tie break {strength.tie_break_vector.join(", ")}
                </dd>
              </div>
            );
          })}
          {explanation.ruleIds?.length ? (
            <div>
              <dt>Rule references</dt>
              <dd>{explanation.ruleIds.join(", ")}</dd>
            </div>
          ) : null}
        </dl>
      </details>
    </section>
  );
}

function templateFor(key: string): OutcomeExplanationTemplate | null {
  if (!isOutcomeExplanationTemplateKey(key)) {
    return null;
  }
  return outcomeExplanationTemplates[key];
}

function renderTemplate(template: string, params: OutcomeExplanationParams): string {
  return template.replace(/\{([a-zA-Z0-9_]+)\}/g, (_match, key: string) => formatValue(params[key] ?? ""));
}

function formatValue(value: OutcomeValue): string {
  if (value === null) {
    return "None";
  }
  if (typeof value === "boolean") {
    return value ? "Yes" : "No";
  }
  return outcomeDisplayValue(String(value));
}

function normalizeStanding(standing: OutcomeExplanationStanding): OutcomeExplanationStanding {
  return {
    ...standing,
    label: outcomeDisplayText(standing.label),
  };
}

function normalizeOptionalText(value: string | null | undefined): string | undefined {
  const normalized = value?.trim();
  return normalized ? normalized : undefined;
}

function orderStandings(standings: readonly OutcomeExplanationStanding[]): OutcomeExplanationStanding[] {
  return [...standings].sort((left, right) => Number(Boolean(right.emphasized)) - Number(Boolean(left.emphasized)));
}

function normalizeBreakdownSection(
  section: OutcomeExplanationBreakdownSection,
  decisiveCause: string,
): OutcomeExplanationBreakdownSection {
  return {
    ...section,
    defaultOpen: section.defaultOpen ?? (section.id === decisiveCause || isShortSection(section)),
  };
}

function isShortSection(section: OutcomeExplanationBreakdownSection): boolean {
  return !section.summary && (section.rows?.length ?? 0) <= 2;
}

function isDuplicateResultField(field: OutcomeExplanationField, result: string | undefined): boolean {
  return Boolean(result) && field.label.trim().toLowerCase() === "result";
}

function standingFieldVisible(
  field: OutcomeExplanationField,
  standing: OutcomeExplanationStanding,
  explanation: OutcomeExplanationSurfaceData,
): boolean {
  if (isDuplicateResultField(field, standing.result)) {
    return false;
  }
  if (explanation.gameId !== "river_ledger" || !standing.showdownStrength) {
    return true;
  }
  return !["best five", "category", "tie break"].includes(field.label.trim().toLowerCase());
}

function riverLedgerShowdownData(
  explanation: OutcomeExplanationSurfaceData,
): readonly OutcomeExplanationStanding[] | null {
  if (explanation.gameId !== "river_ledger" || !explanation.templateKey.startsWith("river_ledger.showdown_")) {
    return null;
  }
  const standings = explanation.finalStanding.filter((standing) => Boolean(standing.showdownStrength));
  return standings.length ? standings : null;
}
