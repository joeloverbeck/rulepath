import { useEffect, useMemo, useState } from "react";
import type { ActionChoice, ActionTree } from "../wasm/client";

type ActionPathBuilderProps = {
  tree: ActionTree | null;
  disabled?: boolean;
  label?: string;
  emptyLabel?: string;
  affordanceTemplates?: Array<{ id: string; text: string }>;
  costResource?: {
    label: string;
    balance: number;
  } | null;
  onSubmit: (selection: ActionPathSelection) => void;
};

export type ActionPathSelection = {
  choices: ActionChoice[];
  segments: string[];
  leaf: ActionChoice;
};

export function ActionPathBuilder({
  tree,
  disabled = false,
  label = "Actions",
  emptyLabel = "No legal actions available.",
  affordanceTemplates = [],
  costResource = null,
  onSubmit,
}: ActionPathBuilderProps) {
  const rootChoices = useMemo(() => tree?.choices ?? [], [tree]);
  const [selectedChoices, setSelectedChoices] = useState<ActionChoice[]>([]);
  const currentChoices = selectedChoices.at(-1)?.next?.choices ?? rootChoices;
  const leaf = selectedChoices.at(-1) ?? null;
  const canConfirm = Boolean(leaf && !leaf.next?.choices?.length);
  const stageConsequence = resolvedConsequence(currentChoices, affordanceTemplates);
  const confirmSummary = leaf ? actionConfirmSummary(selectedChoices, leaf, affordanceTemplates, costResource) : null;

  useEffect(() => {
    setSelectedChoices([]);
  }, [tree?.freshness_token]);

  function choose(choice: ActionChoice) {
    setSelectedChoices((current) => [...current, choice]);
  }

  function back() {
    setSelectedChoices((current) => current.slice(0, -1));
  }

  function cancel() {
    setSelectedChoices([]);
  }

  function confirm() {
    if (!leaf || !canConfirm) {
      return;
    }
    onSubmit({
      choices: selectedChoices,
      segments: selectedChoices.map((choice) => choice.segment),
      leaf,
    });
    setSelectedChoices([]);
  }

  return (
    <section className="action-path-builder" aria-label={label} data-testid="action-path-builder">
      <div className="action-path-heading">
        <h3>{label}</h3>
        <span>{selectedChoices.length ? `${selectedChoices.length + 1} stages` : "Choose"}</span>
      </div>

      {selectedChoices.length ? (
        <ol className="action-path-trail" aria-label="Selected action path" data-testid="action-path-trail">
          {selectedChoices.map((choice, index) => (
            <li key={`${index}-${choice.segment}`}>{choice.label}</li>
          ))}
        </ol>
      ) : null}

      {canConfirm && leaf ? (
        <div className="action-path-confirm" data-testid="action-path-confirm">
          <span>Ready</span>
          <strong>{selectedChoices.map((choice) => choice.label).join(" / ")}</strong>
          {confirmSummary ? (
            <p className="action-path-summary" data-testid="action-path-confirm-summary">
              {confirmSummary}
            </p>
          ) : null}
          <button type="button" disabled={disabled} onClick={confirm} aria-label={`Confirm ${leaf.accessibility_label}`}>
            Confirm
          </button>
        </div>
      ) : currentChoices.length ? (
        <>
          {stageConsequence ? <p className="action-path-consequence">{stageConsequence}</p> : null}
          <div className="action-path-options" data-testid="action-path-options">
            {currentChoices.map((choice) => (
              <button
                type="button"
                key={choice.segment}
                disabled={disabled}
                aria-label={choice.accessibility_label}
                data-testid={`action-path-choice-${stableSegment(choice.segment)}`}
                onClick={() => choose(choice)}
              >
                <span>{choice.label}</span>
                <ChoiceCost choice={choice} costResourceLabel={costResource?.label ?? null} />
              </button>
            ))}
          </div>
        </>
      ) : (
        <p className="muted">{emptyLabel}</p>
      )}

      {selectedChoices.length ? (
        <div className="action-path-tools">
          <button type="button" disabled={disabled} onClick={back}>
            Back
          </button>
          <button type="button" disabled={disabled} onClick={cancel}>
            Cancel
          </button>
        </div>
      ) : null}
    </section>
  );
}

function stableSegment(segment: string): string {
  return segment.replaceAll("/", "-").replaceAll(",", "-").replaceAll(" ", "-");
}

function ChoiceCost({ choice, costResourceLabel }: { choice: ActionChoice; costResourceLabel: string | null }) {
  const cost = metadataValue(choice, "cost");
  if (!cost) {
    return null;
  }
  return (
    <small className="action-cost-chip" data-testid="action-cost-chip">
      {formatCost(cost, costResourceLabel)}
    </small>
  );
}

function actionConfirmSummary(
  selectedChoices: ActionChoice[],
  leaf: ActionChoice,
  templates: Array<{ id: string; text: string }>,
  costResource: { label: string; balance: number } | null,
): string {
  const pieces: string[] = [leaf.label];
  const cost = metadataValue(leaf, "cost");
  if (cost && costResource) {
    pieces.push(`Spends ${formatCost(cost, costResource.label)} of your ${costResource.balance} ${costResource.label}`);
  } else if (cost) {
    pieces.push(`Spends ${formatCost(cost, null)}`);
  }
  const consequence = resolvedTemplate(metadataValue(leaf, "eligibility_consequence"), templates);
  if (consequence) {
    pieces.push(consequence);
  }
  const costRule = selectedChoices
    .map((choice) => resolvedTemplate(metadataValue(choice, "cost_rule"), templates))
    .find((value): value is string => Boolean(value));
  if (costRule && cost) {
    pieces.push(costRule);
  }
  return pieces.join(" · ");
}

function resolvedConsequence(choices: ActionChoice[], templates: Array<{ id: string; text: string }>): string | null {
  const consequenceIds = new Set(choices.map((choice) => metadataValue(choice, "eligibility_consequence")).filter(Boolean));
  if (consequenceIds.size !== 1) {
    return null;
  }
  return resolvedTemplate(Array.from(consequenceIds)[0] ?? null, templates);
}

function resolvedTemplate(id: string | null, templates: Array<{ id: string; text: string }>): string | null {
  if (!id) {
    return null;
  }
  return templates.find((template) => template.id === id)?.text ?? null;
}

function metadataValue(choice: ActionChoice, key: string): string | null {
  return choice.metadata?.find((entry) => entry.key === key)?.value ?? null;
}

function formatCost(value: string, resourceLabel: string | null): string {
  const label = resourceLabel ? pluralizeResource(resourceLabel, value) : "resources";
  return `${value} ${label}`;
}

function pluralizeResource(resourceLabel: string, value: string): string {
  if (value !== "1") {
    return resourceLabel;
  }
  if (resourceLabel === "funds") {
    return "fund";
  }
  if (resourceLabel === "provisions") {
    return "provision";
  }
  return resourceLabel;
}
