import { useId, useMemo, useState } from "react";
import {
  isOutcomeExplanationTemplateKey,
  outcomeExplanationTemplates,
  type OutcomeExplanationTemplate,
} from "./outcomeExplanationTemplates";

type OutcomeValue = string | number | boolean | null;

export type OutcomeExplanationParams = Record<string, OutcomeValue>;

export type OutcomeExplanationStanding = {
  id: string;
  label: string;
  result?: string;
  emphasized?: boolean;
  values: readonly OutcomeExplanationField[];
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
};

export type OutcomeExplanationSurfaceData = {
  gameId: string;
  heading: string;
  resultKind: string;
  decisiveCause: string;
  templateKey: string;
  templateParams?: OutcomeExplanationParams;
  finalStanding: readonly OutcomeExplanationStanding[];
  breakdownSections?: readonly OutcomeExplanationBreakdownSection[];
  ruleIds?: readonly string[];
};

export type OutcomeExplanationSourceRationale = {
  result_kind?: string;
  decisive_cause?: string;
  template_key?: string;
  template_params?: OutcomeExplanationParams;
  decisive_rule_ids?: readonly string[];
  final_standing?: readonly {
    seat: string;
    label?: string;
    result?: string;
    emphasized?: boolean;
    values?: readonly OutcomeExplanationField[];
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

  const params = explanation.templateParams ?? {};
  const summary = template ? renderTemplate(template.summary, params) : explanation.decisiveCause;
  const sections = explanation.breakdownSections ?? [];

  return (
    <section
      className={`outcome-explanation-panel${reducedMotion ? " reduced" : ""}`}
      aria-labelledby={headingId}
      data-outcome-game={explanation.gameId}
    >
      <div className="outcome-summary" role="status" aria-live="polite">
        <p className="eyebrow">Outcome</p>
        <h2 id={headingId}>{explanation.heading}</h2>
        <p>{summary}</p>
      </div>

      <div className="outcome-standing" aria-label="Final standing">
        {explanation.finalStanding.map((standing) => (
          <article
            className={`outcome-standing-row${standing.emphasized ? " emphasized" : ""}`}
            key={standing.id}
            aria-label={standing.result ? `${standing.label}, ${standing.result}` : standing.label}
          >
            <header>
              <strong>{standing.label}</strong>
              {standing.result ? <span>{standing.result}</span> : null}
            </header>
            <dl>
              {standing.values.map((field) => (
                <FieldRow field={field} key={`${standing.id}-${field.label}`} />
              ))}
            </dl>
          </article>
        ))}
      </div>

      {sections.length > 0 ? (
        <div className="outcome-breakdown" id={detailsId}>
          {sections.map((section) => {
            const sectionOpen = expandedSections[section.id] ?? initiallyExpanded;
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
                      [section.id]: !(current[section.id] ?? initiallyExpanded),
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

      {explanation.ruleIds?.length ? (
        <footer className="outcome-rule-refs" aria-label="Outcome rule references">
          <span>{template?.ruleRefLabel ?? "Rule references"}</span>
          <ul>
            {explanation.ruleIds.map((ruleId) => (
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
  return {
    gameId: input.gameId,
    heading: input.heading,
    resultKind: input.rationale?.result_kind ?? input.resultKind,
    decisiveCause: input.rationale?.decisive_cause ?? input.decisiveCause,
    templateKey: input.rationale?.template_key ?? input.templateKey,
    templateParams: input.rationale?.template_params ?? input.templateParams,
    finalStanding: input.rationale?.final_standing?.length
      ? input.rationale.final_standing.map((standing) => ({
          id: standing.seat,
          label: standing.label ?? standing.seat,
          result: standing.result,
          emphasized: standing.emphasized,
          values: standing.values ?? [],
        }))
      : input.finalStanding,
    breakdownSections: input.rationale?.breakdown_sections?.length
      ? input.rationale.breakdown_sections
      : input.breakdownSections,
    ruleIds: input.rationale?.decisive_rule_ids ?? input.ruleIds,
  };
}

function FieldRow({ field }: { field: OutcomeExplanationField }) {
  return (
    <div className={field.emphasized ? "emphasized" : ""}>
      <dt>{field.label}</dt>
      <dd>
        {formatValue(field.value)}
        {field.ruleId ? <small>{field.ruleId}</small> : null}
      </dd>
    </div>
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
  return String(value);
}
