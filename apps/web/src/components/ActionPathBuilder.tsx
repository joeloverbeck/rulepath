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
  onTargetHighlight?: (targets: string[]) => void;
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
  onTargetHighlight,
  onSubmit,
}: ActionPathBuilderProps) {
  const rootChoices = useMemo(() => tree?.choices ?? [], [tree]);
  const [selectedChoices, setSelectedChoices] = useState<ActionChoice[]>([]);
  const [composedTargets, setComposedTargets] = useState<string[]>([]);
  const currentChoices = selectedChoices.at(-1)?.next?.choices ?? rootChoices;
  const leaf = selectedChoices.at(-1) ?? null;
  const canConfirm = Boolean(leaf && !leaf.next?.choices?.length);
  const composer = buildTargetComposer(currentChoices);
  const composedLeaf = composer ? leafForTargetSet(composer.leaves, composedTargets) : null;
  const stageConsequence = resolvedConsequence(currentChoices, affordanceTemplates);
  const confirmLeaf = composedLeaf ?? leaf;
  const confirmSummary = confirmLeaf ? actionConfirmSummary([...selectedChoices, confirmLeaf], confirmLeaf, affordanceTemplates, costResource) : null;

  useEffect(() => {
    setSelectedChoices([]);
    setComposedTargets([]);
    onTargetHighlight?.([]);
  }, [tree?.freshness_token]);

  useEffect(() => {
    setComposedTargets([]);
    onTargetHighlight?.([]);
  }, [currentChoices]);

  function choose(choice: ActionChoice) {
    setSelectedChoices((current) => [...current, choice]);
  }

  function back() {
    setSelectedChoices((current) => current.slice(0, -1));
    setComposedTargets([]);
    onTargetHighlight?.([]);
  }

  function cancel() {
    setSelectedChoices([]);
    setComposedTargets([]);
    onTargetHighlight?.([]);
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
    setComposedTargets([]);
    onTargetHighlight?.([]);
  }

  function confirmComposed() {
    if (!composedLeaf) {
      return;
    }
    onSubmit({
      choices: [...selectedChoices, composedLeaf],
      segments: [...selectedChoices.map((choice) => choice.segment), composedLeaf.segment],
      leaf: composedLeaf,
    });
    setSelectedChoices([]);
    setComposedTargets([]);
    onTargetHighlight?.([]);
  }

  function toggleTarget(targetId: string) {
    setComposedTargets((current) => {
      const next = current.includes(targetId) ? current.filter((id) => id !== targetId) : [...current, targetId];
      onTargetHighlight?.(next);
      return next;
    });
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

      {composer ? (
        <div className="action-target-composer" data-testid="action-target-composer">
          {stageConsequence ? <p className="action-path-consequence">{stageConsequence}</p> : null}
          <div className="action-target-options" role="group" aria-label={`${composer.operationLabel} targets`}>
            {composer.targets.map((target) => {
              const selected = composedTargets.includes(target.id);
              const nextTargets = selected ? composedTargets.filter((id) => id !== target.id) : [...composedTargets, target.id];
              const enabled = nextTargets.length === 0 || hasLeafContainingTargets(composer.leaves, nextTargets);
              return (
                <button
                  type="button"
                  key={target.id}
                  disabled={disabled || !enabled}
                  aria-pressed={selected}
                  data-testid={`action-target-toggle-${stableSegment(target.id)}`}
                  onClick={() => toggleTarget(target.id)}
                >
                  {target.label}
                </button>
              );
            })}
          </div>
          {confirmSummary ? (
            <p className="action-path-summary" data-testid="action-path-confirm-summary">
              {confirmSummary}
            </p>
          ) : null}
          {composedLeaf ? <span>Ready</span> : null}
          <button
            type="button"
            disabled={disabled || !composedLeaf}
            onClick={confirmComposed}
            data-testid="action-target-confirm"
            aria-label={`Confirm ${composedLeaf?.accessibility_label ?? composer.operationLabel}`}
          >
            Confirm
          </button>
        </div>
      ) : canConfirm && leaf ? (
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

type TargetComposer = {
  operationLabel: string;
  targets: Array<{ id: string; label: string }>;
  leaves: Array<{ choice: ActionChoice; targetIds: string[] }>;
};

function buildTargetComposer(choices: ActionChoice[]): TargetComposer | null {
  if (choices.length < 3 || !choices.every((choice) => choice.tags?.includes("operation-leaf"))) {
    return null;
  }
  const leaves = choices.map((choice) => ({ choice, targetIds: targetIdsForChoice(choice) }));
  if (leaves.some((leaf) => leaf.targetIds.some((id) => id.includes(">")))) {
    return null;
  }
  if (leaves.some((leaf) => leaf.targetIds.length === 0) || !leaves.some((leaf) => leaf.targetIds.length > 1)) {
    return null;
  }
  const operationLabel = operationLabelForChoice(choices[0]);
  if (!operationLabel || !choices.every((choice) => operationLabelForChoice(choice) === operationLabel)) {
    return null;
  }
  const singleLabels = new Map<string, string>();
  for (const leaf of leaves) {
    if (leaf.targetIds.length === 1) {
      singleLabels.set(leaf.targetIds[0], targetLabelFromChoice(operationLabel, leaf.choice));
    }
  }
  if (!leaves.every((leaf) => leaf.targetIds.every((id) => singleLabels.has(id)))) {
    return null;
  }
  const targets: Array<{ id: string; label: string }> = [];
  for (const leaf of leaves) {
    for (const id of leaf.targetIds) {
      if (!targets.some((target) => target.id === id)) {
        targets.push({ id, label: singleLabels.get(id) ?? id });
      }
    }
  }
  if (choices.length <= targets.length) {
    return null;
  }
  return { operationLabel, targets, leaves };
}

function targetIdsForChoice(choice: ActionChoice): string[] {
  const payload = choice.segment.split("/")[2] ?? "";
  return payload
    .split(",")
    .map((part) => part.trim())
    .filter(Boolean)
    .sort();
}

function operationLabelForChoice(choice: ActionChoice): string | null {
  const label = choice.label.trim();
  const firstSpace = label.indexOf(" ");
  return firstSpace > 0 ? label.slice(0, firstSpace) : null;
}

function targetLabelFromChoice(operationLabel: string, choice: ActionChoice): string {
  return choice.label.replace(new RegExp(`^${escapeRegExp(operationLabel)}\\s+`), "");
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function leafForTargetSet(leaves: TargetComposer["leaves"], targetIds: string[]): ActionChoice | null {
  const normalized = [...targetIds].sort();
  return leaves.find((leaf) => sameTargetSet(leaf.targetIds, normalized))?.choice ?? null;
}

function hasLeafContainingTargets(leaves: TargetComposer["leaves"], targetIds: string[]): boolean {
  return leaves.some((leaf) => targetIds.every((id) => leaf.targetIds.includes(id)));
}

function sameTargetSet(left: string[], right: string[]): boolean {
  return left.length === right.length && left.every((value, index) => value === right[index]);
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
