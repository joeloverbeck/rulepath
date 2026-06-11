import { feedbackForEffect, type ReducedMotionOverride } from "./effectFeedback";
import type { EffectEntry } from "../wasm/client";

type EffectLogProps = {
  effects: EffectEntry[];
  reducedMotion: boolean;
  override: ReducedMotionOverride;
  onOverrideChange: (override: ReducedMotionOverride) => void;
};

export function EffectLog({ effects, reducedMotion, override, onOverrideChange }: EffectLogProps) {
  const newestCursor = effects.at(-1)?.cursor ?? null;

  return (
    <section className="effects" aria-label="semantic effects">
      <div className="effect-heading">
        <div>
          <p className="eyebrow">Effects</p>
          <h2>Effect log</h2>
        </div>
        <label className="motion-field">
          <span>Motion</span>
          <select value={override} onChange={(event) => onOverrideChange(event.currentTarget.value as ReducedMotionOverride)}>
            <option value="system">System</option>
            <option value="reduce">Reduced</option>
            <option value="motion">Motion</option>
          </select>
        </label>
      </div>

      <ol data-testid="effects" aria-live="polite">
        {effects.length === 0 ? (
          <li>No effects yet</li>
        ) : (
          effects.map((entry) => {
            const feedback = feedbackForEffect(entry);
            const isNewest = entry.cursor === newestCursor;
            return (
              <li
                key={entry.cursor}
                className={`effect-entry ${feedback.tone}${isNewest ? " newest" : ""}${isRevealBatch(entry) ? " reveal-batch" : ""}${
                  reducedMotion ? " reduced" : ""
                }`}
              >
                <span className="effect-cursor">{entry.cursor}</span>
                <strong>{feedback.title}</strong>
                <span>{feedback.detail}</span>
              </li>
            );
          })
        )}
      </ol>
    </section>
  );
}

function isRevealBatch(entry: EffectEntry): boolean {
  return (
    entry.effect.payload.type === "reveal_batch_started" ||
    entry.effect.payload.type === "choices_revealed" ||
    entry.effect.payload.type === "draft_resolved" ||
    entry.effect.payload.type === "mask_revealed"
  );
}
