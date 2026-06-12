import { useEffect, useMemo, useState } from "react";
import type { ActionChoice, ActionTree } from "../wasm/client";

type ActionPathBuilderProps = {
  tree: ActionTree | null;
  disabled?: boolean;
  label?: string;
  emptyLabel?: string;
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
  onSubmit,
}: ActionPathBuilderProps) {
  const rootChoices = useMemo(() => tree?.choices ?? [], [tree]);
  const [selectedChoices, setSelectedChoices] = useState<ActionChoice[]>([]);
  const currentChoices = selectedChoices.at(-1)?.next?.choices ?? rootChoices;
  const leaf = selectedChoices.at(-1) ?? null;
  const canConfirm = Boolean(leaf && !leaf.next?.choices?.length);

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
          <button type="button" disabled={disabled} onClick={confirm} aria-label={`Confirm ${leaf.accessibility_label}`}>
            Confirm
          </button>
        </div>
      ) : currentChoices.length ? (
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
            </button>
          ))}
        </div>
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
