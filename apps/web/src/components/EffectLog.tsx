import { feedbackForEffect, type ReducedMotionOverride } from "./effectFeedback";
import type { EffectEntry } from "../wasm/client";
import { segmentResolutionBursts } from "../animation/bursts";

type EffectLogProps = {
  effects: EffectEntry[];
  reducedMotion: boolean;
  override: ReducedMotionOverride;
  onOverrideChange: (override: ReducedMotionOverride) => void;
};

export function EffectLog({ effects, reducedMotion, override, onOverrideChange }: EffectLogProps) {
  const newestCursor = effects.at(-1)?.cursor ?? null;
  const bursts = segmentResolutionBursts(effects);

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

      <ol data-testid="effects" className="effect-bursts" aria-live="polite">
        {bursts.length === 0 ? (
          <li>No effects yet</li>
        ) : (
          bursts.map((burst) => {
            const newestInBurst = burst.entries.some((entry) => entry.cursor === newestCursor);
            return (
              <li key={burst.id} className={`effect-burst${newestInBurst ? " newest" : ""}`}>
                <div className="effect-burst-heading">
                  <strong>{burst.label}</strong>
                  <span>{burst.visibleEntries.length} effect{burst.visibleEntries.length === 1 ? "" : "s"}</span>
                </div>
                <ol>
                  {burst.visibleEntries.length === 0 ? (
                    <li className={`effect-entry neutral${reducedMotion ? " reduced" : ""}`}>
                      <span className="effect-cursor">{burst.marker?.cursor ?? "-"}</span>
                      <strong>Marker recorded</strong>
                      <span>Rust marked a resolution boundary.</span>
                    </li>
                  ) : (
                    burst.visibleEntries.map((entry) => {
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
