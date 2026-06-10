import { useMemo } from "react";
import type { ActionChoice, ActionTree, EffectEntry, SecretDraftItemView, SecretDraftPublicView, SeatId } from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type SecretDraftBoardProps = {
  view: SecretDraftPublicView;
  actionTree: ActionTree | null;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onChoice?: (choice: ActionChoice) => void;
};

export function SecretDraftBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onChoice,
}: SecretDraftBoardProps) {
  const terminal = view.terminal.terminal;
  const choices = useMemo(() => actionTree?.choices ?? [], [actionTree]);
  const choiceByItem = useMemo(() => choicesByItem(choices), [choices]);
  const canAct = Boolean(interactive && !pending && !terminal && view.active_seat && choices.length > 0);
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const revealActive = effects.some((entry) => isRevealEffect(entry.effect.payload.type));
  const outcomeExplanation = terminal
    ? outcomeSurfaceData({
        gameId: "secret_draft",
        heading: terminalLabel(view),
        rationale: view.terminal_rationale,
        resultKind: view.terminal.draw ? "draw" : "win",
        decisiveCause: "rust_terminal_rationale",
        templateKey: view.terminal.draw ? "secret_draft.all_tied_draw" : "secret_draft.score_win",
        templateParams: { winner: view.terminal.winner ?? "" },
        finalStanding: [
          draftStanding("seat_0", view.terminal.winner, view),
          draftStanding("seat_1", view.terminal.winner, view),
        ],
        breakdownSections: [
          {
            id: "public-draft",
            heading: "Public drafted collections",
            rows: [
              { label: "seat_0 drafted", value: view.drafted.seat_0.map((item) => item.label).join(", ") || "None" },
              { label: "seat_1 drafted", value: view.drafted.seat_1.map((item) => item.label).join(", ") || "None" },
              { label: "Resolved rounds", value: view.revealed_history.length },
            ],
          },
        ],
      })
    : null;

  return (
    <section
      className={`secret-draft-board ${terminal ? "terminal" : ""}${revealActive ? " reveal" : ""}${
        reducedMotion ? " reduced" : ""
      }`}
      aria-labelledby="secret-draft-heading"
      data-testid="secret-draft-board"
    >
      <div className="secret-draft-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="secret-draft-heading">{statusLabel(view)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {terminalLabel(view)}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {boardSummary(view)}
      </p>

      <div className="secret-draft-metrics" aria-label="Draft status">
        <Metric label="Round" value={`${view.round_number} / ${view.round_limit}`} />
        <Metric label={view.ui.score_label} value={`${view.scores.seat_0} - ${view.scores.seat_1}`} />
        <Metric label="Priority" value={seatLabel(view.priority_seat)} />
        <Metric label="Pool" value={`${view.visible_pool.length} visible`} />
      </div>

      <div className="secret-draft-table" aria-label={view.ui.table_label}>
        <SeatDraftPanel view={view} seat="seat_0" />

        <section className="secret-pool" aria-label={view.ui.visible_pool_label}>
          <div className="secret-section-heading">
            <span>{view.ui.visible_pool_label}</span>
            <strong>{canAct ? "Rust legal choices" : pendingCopy(view)}</strong>
          </div>
          <div className="secret-pool-grid">
            {view.visible_pool.map((item, index) => {
              const choice = choiceByItem.get(item.item_id) ?? null;
              return (
                <button
                  type="button"
                  key={item.item_id}
                  className={`secret-item thread-${item.thread}`}
                  disabled={!canAct || !choice}
                  aria-label={choice?.accessibility_label ?? item.accessibility_label}
                  data-testid={`secret-draft-choice-${view.round_number}-${index}`}
                  onClick={() => choice && onChoice?.(choice)}
                >
                  <span>{item.thread}</span>
                  <strong>{item.label}</strong>
                  <small>Value {item.value}</small>
                </button>
              );
            })}
          </div>
        </section>

        <SeatDraftPanel view={view} seat="seat_1" />
      </div>

      <section className="secret-pending" aria-label={view.ui.pending_label}>
        <PendingSeat view={view} seat="seat_0" />
        <PendingSeat view={view} seat="seat_1" />
      </section>

      <section className="secret-reveals" aria-label="Reveal history">
        <div className="secret-section-heading">
          <span>Reveal history</span>
          <strong>{view.revealed_history.length} resolved</strong>
        </div>
        {view.revealed_history.length === 0 ? (
          <p className="muted">Resolved drafts will appear after both seats commit.</p>
        ) : (
          <ol>
            {view.revealed_history.map((round) => (
              <li key={round.round_number} className={round.contested ? "contested" : ""}>
                <span>Round {round.round_number}</span>
                <strong>
                  {round.seat_0_award.label} / {round.seat_1_award.label}
                </strong>
                <small>{round.contested ? `${seatLabel(round.priority_seat)} won the conflict` : "Both choices resolved"}</small>
              </li>
            ))}
          </ol>
        )}
      </section>

      <div className="secret-latest" role="status">
        <span>{outcomeExplanation ? "Outcome" : feedback?.title ?? "Waiting"}</span>
        <strong>
          {outcomeExplanation
            ? outcomeAnnouncementText(outcomeExplanation)
            : feedback?.detail ?? "Commitments stay hidden until Rust emits the reveal batch."}
        </strong>
      </div>

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function SeatDraftPanel({ view, seat }: { view: SecretDraftPublicView; seat: SeatId }) {
  const drafted = seat === "seat_0" ? view.drafted.seat_0 : view.drafted.seat_1;
  const active = view.active_seat === seat;
  const score = seat === "seat_0" ? view.scores.seat_0 : view.scores.seat_1;

  return (
    <section className={`secret-seat ${active ? "active" : ""}`} aria-label={`${seatLabel(seat)} drafted collection`}>
      <div className="secret-section-heading">
        <span>{seatLabel(seat)}</span>
        <strong>{active ? "Active" : `Score ${score}`}</strong>
      </div>
      {drafted.length === 0 ? (
        <p className="muted">No drafted items yet.</p>
      ) : (
        <div className="secret-drafted-list">
          {drafted.map((item) => (
            <DraftedItem key={item.item_id} item={item} />
          ))}
        </div>
      )}
    </section>
  );
}

function DraftedItem({ item }: { item: SecretDraftItemView }) {
  return (
    <div className={`secret-drafted-item thread-${item.thread}`} aria-label={item.accessibility_label}>
      <span>{item.thread}</span>
      <strong>{item.label}</strong>
      <small>{item.value}</small>
    </div>
  );
}

function PendingSeat({ view, seat }: { view: SecretDraftPublicView; seat: SeatId }) {
  const commitment = seat === "seat_0" ? view.commitments.seat_0 : view.commitments.seat_1;
  return (
    <div
      className={`secret-pending-seat ${commitment.committed ? "committed" : "waiting"}`}
      data-testid={`secret-draft-pending-${seat}-round-${view.round_number}`}
    >
      <span>{seatLabel(seat)}</span>
      <strong>{commitment.committed ? "Committed" : "Waiting"}</strong>
      <small>{commitment.committed ? "Choice hidden" : "No commitment yet"}</small>
    </div>
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

function choicesByItem(choices: ActionChoice[]): Map<string, ActionChoice> {
  const map = new Map<string, ActionChoice>();
  for (const choice of choices) {
    if (choice.segment.startsWith("commit/")) {
      map.set(choice.segment.slice("commit/".length), choice);
    }
  }
  return map;
}

function isRevealEffect(type: string): boolean {
  return type === "reveal_batch_started" || type === "choices_revealed" || type === "draft_resolved";
}

function statusLabel(view: SecretDraftPublicView): string {
  if (view.terminal.terminal) {
    return view.terminal.draw ? "Drawn draft" : `${seatLabel(view.terminal.winner ?? "seat_0")} wins`;
  }
  return view.active_seat ? `${seatLabel(view.active_seat)} to commit` : "Resolving reveal";
}

function terminalLabel(view: SecretDraftPublicView): string {
  if (view.terminal.terminal) {
    return view.terminal.draw ? "Draw" : `${view.terminal.winner} won`;
  }
  return view.active_seat ?? "Reveal";
}

function pendingCopy(view: SecretDraftPublicView): string {
  return view.commitments.copy.replaceAll("_", " ");
}

function boardSummary(view: SecretDraftPublicView): string {
  return `${view.display_name}, round ${view.round_number} of ${view.round_limit}, score ${view.scores.seat_0} to ${view.scores.seat_1}, ${pendingCopy(view)}.`;
}

function seatLabel(seat: SeatId): string {
  return seat === "seat_0" ? "Seat 0" : "Seat 1";
}

function draftStanding(seat: SeatId, winner: SeatId | null, view: SecretDraftPublicView) {
  const score = seat === "seat_0" ? view.scores.seat_0 : view.scores.seat_1;
  const drafted = seat === "seat_0" ? view.drafted.seat_0 : view.drafted.seat_1;
  return {
    id: seat,
    label: seatLabel(seat),
    result: winner === seat ? "Winner" : winner ? "Loss" : "Draw",
    emphasized: winner === seat,
    values: [
      { label: "Score", value: score },
      { label: "Drafted items", value: drafted.length },
    ],
  };
}
