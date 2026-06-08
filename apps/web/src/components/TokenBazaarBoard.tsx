import { useMemo } from "react";
import type {
  ActionChoice,
  ActionTree,
  EffectEntry,
  SeatId,
  TokenBazaarLegalActionView,
  TokenBazaarPublicView,
  TokenBazaarResourceCounts,
} from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";

type TokenBazaarBoardProps = {
  view: TokenBazaarPublicView;
  actionTree: ActionTree | null;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onChoice?: (choice: ActionChoice) => void;
};

const RESOURCE_ORDER: Array<keyof TokenBazaarResourceCounts> = ["amber", "jade", "iron"];

export function TokenBazaarBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onChoice,
}: TokenBazaarBoardProps) {
  const terminal = view.terminal.terminal;
  const choices = useMemo(() => actionTreeChoices(actionTree, view.legal_actions), [actionTree, view.legal_actions]);
  const canAct = Boolean(interactive && !pending && !terminal && view.active_seat && choices.length > 0);
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const recentEffects = view.recent_effects.length > 0 ? view.recent_effects : effects.map(effectSummary);

  return (
    <section
      className={`token-bazaar-board ${terminal ? "terminal" : ""}${reducedMotion ? " reduced" : ""}`}
      aria-labelledby="token-bazaar-heading"
      data-testid="token-bazaar-board"
    >
      <div className="token-bazaar-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="token-bazaar-heading">{statusLabel(view)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {terminalLabel(view)}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {boardSummary(view)}
      </p>

      <div className="token-bazaar-metrics" aria-label="Match accounting">
        <Metric label={view.ui.score_label} value={`${view.scores.seat_0} - ${view.scores.seat_1}`} />
        <Metric label={view.ui.turn_counter_label} value={`${view.turns_taken.seat_0} / ${view.turns_taken.turns_per_seat}`} />
        <Metric label="Seat 1 turns" value={`${view.turns_taken.seat_1} / ${view.turns_taken.turns_per_seat}`} />
        <Metric label="Queue" value={`${view.queue_remaining} contracts`} />
      </div>

      <div className="token-bazaar-table" aria-label={view.ui.table_label}>
        <SeatInventory view={view} seat="seat_0" />

        <section className="token-supply" aria-label={view.ui.supply_label}>
          <div className="token-section-heading">
            <span>{view.ui.supply_label}</span>
            <strong>Public</strong>
          </div>
          <ResourceChips counts={view.supply} />
        </section>

        <SeatInventory view={view} seat="seat_1" />
      </div>

      <section className="token-market" aria-label={view.ui.market_label}>
        <div className="token-section-heading">
          <span>{view.ui.market_label}</span>
          <strong>Rust market slots</strong>
        </div>
        <div className="token-market-row">
          {view.market_slots.map((slot) => (
            <article key={slot.slot_id} className={`token-contract ${slot.is_empty ? "empty" : ""}`} aria-label={slot.accessibility_label}>
              <span>{slot.slot}</span>
              {slot.contract ? (
                <>
                  <strong>{slot.contract.label}</strong>
                  <ResourceChips counts={slot.contract.cost} compact label="Cost" />
                  <small>{slot.contract.points} points</small>
                </>
              ) : (
                <>
                  <strong>Empty slot</strong>
                  <small>No queued contract remains.</small>
                </>
              )}
            </article>
          ))}
        </div>
      </section>

      <section className="token-actions" aria-label="Rust legal economy actions">
        <div className="token-section-heading">
          <span>Actions</span>
          <strong>{canAct ? "Rust legal tree" : terminal ? "Match complete" : "Waiting"}</strong>
        </div>
        {choices.length === 0 ? (
          <p className="muted">No Rust-supplied legal actions.</p>
        ) : (
          <div className="token-action-grid">
            {choices.map((choice) => (
              <button
                type="button"
                key={choice.segment}
                disabled={!canAct}
                aria-label={choice.accessibility_label}
                data-testid={`token-action-${choice.segment.replaceAll("/", "-")}`}
                onClick={() => onChoice?.(choice)}
              >
                <span>{choice.label}</span>
                <ActionMetadata choice={choice} />
              </button>
            ))}
          </div>
        )}
      </section>

      <div className="token-recent" aria-label="Recent public accounting">
        <div>
          <span>Latest effect</span>
          <strong>{feedback?.title ?? recentEffects.at(-1)?.kind ?? "None yet"}</strong>
        </div>
        <p>{feedback?.detail ?? recentEffects.at(-1)?.summary ?? "Accounting effects will appear after a Rust action resolves."}</p>
        {recentEffects.length > 0 ? (
          <ol>
            {recentEffects.map((effect, index) => (
              <li key={`${effect.kind}-${index}`}>
                <strong>{effect.kind.replace(/^tb_/, "").replaceAll("_", " ")}</strong>
                <span>{effect.summary}</span>
              </li>
            ))}
          </ol>
        ) : null}
      </div>
    </section>
  );
}

function SeatInventory({ view, seat }: { view: TokenBazaarPublicView; seat: SeatId }) {
  const index = seat === "seat_0" ? 0 : 1;
  const inventory = view.inventories[index];
  const fulfilled = seat === "seat_0" ? view.fulfilled.seat_0 : view.fulfilled.seat_1;
  const active = view.active_seat === seat;

  return (
    <section className={`token-seat ${active ? "active" : ""}`} aria-label={`${seatLabel(seat)} inventory`}>
      <div className="token-section-heading">
        <span>{seatLabel(seat)}</span>
        <strong>{active ? "Active" : `${fulfilled.length} fulfilled`}</strong>
      </div>
      <ResourceChips counts={inventory.resources} />
      <div className="token-seat-footer">
        <span>Score {seat === "seat_0" ? view.scores.seat_0 : view.scores.seat_1}</span>
        <span>Contracts {fulfilled.length ? fulfilled.join(", ") : "none"}</span>
      </div>
    </section>
  );
}

function ResourceChips({
  counts,
  compact = false,
  label,
}: {
  counts: TokenBazaarResourceCounts;
  compact?: boolean;
  label?: string;
}) {
  return (
    <div className={`resource-chips ${compact ? "compact" : ""}`} aria-label={label ?? "Resource counts"}>
      {RESOURCE_ORDER.map((resource) => (
        <span key={resource} className={`resource-chip ${resource}`}>
          <b>{resourceCode(resource)}</b>
          <span>{resource}</span>
          <strong>{counts[resource]}</strong>
        </span>
      ))}
    </div>
  );
}

function ActionMetadata({ choice }: { choice: ActionChoice }) {
  const cost = parseCounts(metadataValue(choice, "cost"));
  const gain = parseCounts(metadataValue(choice, "gain"));
  const points = metadataValue(choice, "points");
  return (
    <small>
      {cost ? <ResourceChips counts={cost} compact label="Action cost" /> : null}
      {gain ? <ResourceChips counts={gain} compact label="Action gain" /> : null}
      {points ? <span>{points} points</span> : null}
    </small>
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

function actionTreeChoices(actionTree: ActionTree | null, legalActions: TokenBazaarLegalActionView[]): ActionChoice[] {
  if (actionTree?.choices.length) {
    return actionTree.choices;
  }
  return legalActions.map((action) => ({
    segment: action.action_segment,
    label: action.label,
    accessibility_label: action.accessibility_label,
    metadata: action.metadata,
  }));
}

function metadataValue(choice: ActionChoice, key: string): string | null {
  return choice.metadata?.find((entry) => entry.key === key)?.value ?? null;
}

function parseCounts(value: string | null): TokenBazaarResourceCounts | null {
  if (!value) {
    return null;
  }
  const counts: TokenBazaarResourceCounts = { amber: 0, jade: 0, iron: 0 };
  for (const part of value.split(",")) {
    const [key, raw] = part.split("=");
    if (key === "amber" || key === "jade" || key === "iron") {
      counts[key] = Number(raw) || 0;
    }
  }
  return counts;
}

function effectSummary(entry: EffectEntry): { kind: string; summary: string } {
  return {
    kind: entry.effect.payload.type,
    summary: feedbackForEffect(entry).detail,
  };
}

function statusLabel(view: TokenBazaarPublicView): string {
  if (view.terminal.terminal) {
    return view.terminal.draw ? "Drawn bazaar" : `${seatLabel(view.terminal.winner ?? "seat_0")} wins`;
  }
  return `${seatLabel(view.active_seat ?? "seat_0")} to act`;
}

function terminalLabel(view: TokenBazaarPublicView): string {
  if (view.terminal.terminal) {
    return view.terminal.draw ? "Draw" : `${seatLabel(view.terminal.winner ?? "seat_0")} won`;
  }
  return `${view.turns_taken.seat_0 + view.turns_taken.seat_1} turns taken`;
}

function boardSummary(view: TokenBazaarPublicView): string {
  return `Token Bazaar score ${view.scores.seat_0} to ${view.scores.seat_1}; supply amber ${view.supply.amber}, jade ${view.supply.jade}, iron ${view.supply.iron}; ${terminalLabel(view)}.`;
}

function seatLabel(seat: SeatId): string {
  return seat === "seat_0" ? "Seat 0" : "Seat 1";
}

function resourceCode(resource: keyof TokenBazaarResourceCounts): string {
  switch (resource) {
    case "amber":
      return "AM";
    case "jade":
      return "JA";
    case "iron":
      return "IR";
  }
}
