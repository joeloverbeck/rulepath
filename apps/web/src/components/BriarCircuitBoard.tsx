import { useMemo } from "react";
import type {
  ActionChoice,
  ActionTree,
  BriarCircuitPublicView,
  BriarCircuitSeatId,
  EffectEntry,
} from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type BriarCircuitBoardProps = {
  view: BriarCircuitPublicView;
  actionTree: ActionTree | null;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onPathSubmit?: (path: string[]) => void;
};

type PathChoice = {
  path: string[];
  choice: ActionChoice;
};

const SEATS: BriarCircuitSeatId[] = ["seat_0", "seat_1", "seat_2", "seat_3"];

export function BriarCircuitBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onPathSubmit,
}: BriarCircuitBoardProps) {
  const paths = useMemo(() => flattenActionTree(actionTree), [actionTree]);
  const passSelect = useMemo(() => new Map(paths.filter((entry) => isPassSelect(entry.path)).map((entry) => [entry.path[2], entry])), [paths]);
  const passUnselect = useMemo(
    () => new Map(paths.filter((entry) => isPassUnselect(entry.path)).map((entry) => [entry.path[2], entry])),
    [paths],
  );
  const passConfirm = paths.find((entry) => entry.path[0] === "pass" && entry.path[1] === "confirm") ?? null;
  const playChoices = useMemo(() => new Map(paths.filter((entry) => entry.path[0] === "play").map((entry) => [entry.path[1], entry])), [paths]);
  const selected = new Set(view.pass?.own_selection.map((card) => card.card_id) ?? []);
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const canAct = Boolean(interactive && !pending && view.private_view_status === "seat" && paths.length > 0 && view.phase !== "terminal");
  const trickChanged = effects.some((entry) => ["card_played", "trick_captured", "hearts_broken"].includes(String(entry.effect.payload.type)));
  const outcomeExplanation =
    view.phase === "terminal"
      ? outcomeSurfaceData({
          gameId: "briar_circuit",
          heading: "Briar Circuit result",
          rationale: null,
          resultKind: "lowest_score",
          decisiveCause: "match_threshold",
          templateKey: "briar_circuit.low_score_win",
          templateParams: { winner: lowestScoreSeat(view) },
          finalStanding: SEATS.map((seat) => ({
            id: seat,
            label: seatLabel(seat),
            result: seat === lowestScoreSeat(view) ? "win" : "loss",
            emphasized: seat === lowestScoreSeat(view),
            values: [{ label: "Score", value: view.cumulative_scores[seat] }],
          })),
          breakdownSections: [
            {
              id: "scores",
              heading: "Final scores",
              rows: SEATS.map((seat) => ({ label: seatLabel(seat), value: view.cumulative_scores[seat] })),
            },
          ],
        })
      : null;

  return (
    <section
      className={`briar-board ${view.phase === "terminal" ? "terminal" : ""}${trickChanged ? " reveal" : ""}${reducedMotion ? " reduced" : ""}`}
      aria-labelledby="briar-heading"
      data-testid="briar-circuit-board"
    >
      <div className="briar-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="briar-heading">{statusLabel(view)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {turnLabel(view)}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {view.display_name}, {view.phase}, hand {view.hand_index + 1}, {view.current_trick.length} cards in the current trick.
      </p>

      <div className="briar-metrics" aria-label="Briar Circuit status">
        <Metric label="Hand" value={String(view.hand_index + 1)} />
        <Metric label={view.ui.score_label} value={scoreLine(view)} />
        <Metric label="Dealer" value={seatLabel(view.dealer)} />
        <Metric label="Hearts" value={view.hearts_broken ? "Broken" : "Closed"} />
      </div>

      <div className="briar-table" aria-label={view.ui.table_label}>
        <section className="briar-seat-rail" aria-label="Seats">
          {SEATS.map((seat) => (
            <SeatPanel key={seat} view={view} seat={seat} />
          ))}
        </section>

        <section className="briar-trick" aria-label={view.ui.current_trick_label}>
          <div className="briar-section-heading">
            <span>{view.ui.current_trick_label}</span>
            <strong>{view.current_trick.length ? `${view.current_trick.length} / 4 played` : "No cards played"}</strong>
          </div>
          <div className="briar-trick-cards">
            {view.current_trick.length === 0 ? (
              <p className="muted">Waiting for a legal play.</p>
            ) : (
              view.current_trick.map((play) => (
                <div className="briar-played-card" key={`${play.seat}-${play.card}`}>
                  <span>{seatLabel(play.seat)}</span>
                  <strong>{formatCardId(play.card)}</strong>
                </div>
              ))
            )}
          </div>
        </section>

        <section className="briar-private" aria-label={view.ui.own_hand_label}>
          <div className="briar-section-heading">
            <span>{view.ui.own_hand_label}</span>
            <strong>{privateHeading(view)}</strong>
          </div>
          {view.private_view_status === "seat" ? (
            <div className="briar-hand" data-testid="briar-circuit-own-hand">
              {view.own_hand.map((card) => {
                const selectChoice = passSelect.get(card.card_id) ?? null;
                const unselectChoice = passUnselect.get(card.card_id) ?? null;
                const playChoice = playChoices.get(card.card_id) ?? null;
                const selectedForPass = selected.has(card.card_id);
                const action = selectedForPass ? unselectChoice : selectChoice ?? playChoice;
                return (
                  <button
                    type="button"
                    key={card.card_id}
                    className={`briar-card ${card.suit} ${action ? "legal" : ""}${selectedForPass ? " selected" : ""}`}
                    disabled={!canAct || !action}
                    aria-pressed={selectedForPass}
                    aria-label={action?.choice.accessibility_label ?? card.accessibility_label}
                    onClick={() => action && onPathSubmit?.(action.path)}
                  >
                    <span>{card.suit}</span>
                    <strong>{card.rank}</strong>
                    <small>{selectedForPass ? "Selected" : action ? actionLabel(action.path) : "Held"}</small>
                  </button>
                );
              })}
            </div>
          ) : (
            <div className="briar-hidden-hand" data-testid="briar-circuit-observer-hand">
              <span>Hidden</span>
              <strong>{SEATS.reduce((total, seat) => total + view.hand_counts[seat], 0)} cards held privately</strong>
            </div>
          )}
        </section>

        {view.pass ? (
          <section className="briar-pass" aria-label="Pass progress">
            <div className="briar-section-heading">
              <span>Pass</span>
              <strong>
                {view.pass.direction} {view.pass.own_selection.length}/3
              </strong>
            </div>
            <div className="briar-pass-meter" aria-label={`${view.pass.committed_count} seats committed`}>
              <span style={{ inlineSize: `${(view.pass.committed_count / 4) * 100}%` }} />
            </div>
            <button
              type="button"
              className="briar-confirm"
              disabled={!canAct || !passConfirm || view.pass.own_committed}
              onClick={() => passConfirm && onPathSubmit?.(passConfirm.path)}
            >
              {view.pass.own_committed ? "Pass committed" : "Confirm pass"}
            </button>
          </section>
        ) : null}
      </div>

      <section className="briar-history" aria-label={view.ui.captured_tricks_label}>
        <div className="briar-section-heading">
          <span>{view.ui.captured_tricks_label}</span>
          <strong>{view.captured_tricks.length} captured</strong>
        </div>
        {view.captured_tricks.length === 0 ? (
          <p className="muted">Captured tricks will appear here.</p>
        ) : (
          <ol>
            {view.captured_tricks.slice(-6).map((trick) => (
              <li key={`${trick.hand_index}-${trick.trick_index}`}>
                <span>Trick {trick.trick_index + 1}</span>
                <strong>{seatLabel(trick.winner)}</strong>
                <small>{trick.plays.map((play) => `${seatLabel(play.seat)} ${formatCardId(play.card)}`).join(" / ")}</small>
              </li>
            ))}
          </ol>
        )}
      </section>

      <div className="briar-latest" role="status">
        <span>{outcomeExplanation ? "Outcome" : feedback?.title ?? "Waiting"}</span>
        <strong>
          {outcomeExplanation
            ? outcomeAnnouncementText(outcomeExplanation)
            : feedback?.detail ?? "Legal pass and play controls are ready."}
        </strong>
      </div>

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function flattenActionTree(tree: ActionTree | null): PathChoice[] {
  const result: PathChoice[] = [];
  const visit = (choices: ActionChoice[] | undefined, prefix: string[]) => {
    for (const choice of choices ?? []) {
      const path = [...prefix, choice.segment];
      if (choice.next?.choices?.length) {
        visit(choice.next.choices, path);
      } else {
        result.push({ path, choice });
      }
    }
  };
  visit(tree?.choices, []);
  return result;
}

function isPassSelect(path: string[]): boolean {
  return path[0] === "pass" && path[1] === "select" && Boolean(path[2]);
}

function isPassUnselect(path: string[]): boolean {
  return path[0] === "pass" && path[1] === "unselect" && Boolean(path[2]);
}

function SeatPanel({ view, seat }: { view: BriarCircuitPublicView; seat: BriarCircuitSeatId }) {
  const active = view.active_seat === seat;
  const viewer = view.viewer_seat === seat;
  return (
    <article className={`briar-seat ${active ? "active" : ""}${viewer ? " viewer" : ""}`}>
      <span>{seatLabel(seat)}</span>
      <strong>{view.cumulative_scores[seat]}</strong>
      <small>
        {view.hand_counts[seat]} cards{seat === view.dealer ? " / dealer" : ""}
      </small>
    </article>
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

function statusLabel(view: BriarCircuitPublicView): string {
  if (view.phase === "passing" && view.pass) {
    return `Pass ${view.pass.direction}: ${view.pass.committed_count} committed`;
  }
  if (view.phase === "playing") {
    return view.active_seat ? `${seatLabel(view.active_seat)} to play` : "Playing";
  }
  return view.phase;
}

function turnLabel(view: BriarCircuitPublicView): string {
  return view.active_seat ? seatLabel(view.active_seat) : view.pass ? `${view.pass.pending_count} pending` : view.phase;
}

function privateHeading(view: BriarCircuitPublicView): string {
  if (view.private_view_status !== "seat") {
    return "Observer";
  }
  return view.pass ? `${view.pass.own_selection.length} of 3 selected` : `${view.own_hand.length} cards`;
}

function actionLabel(path: string[]): string {
  if (path[0] === "pass" && path[1] === "select") return "Select";
  if (path[0] === "pass" && path[1] === "unselect") return "Unselect";
  if (path[0] === "play") return "Play";
  return "Legal";
}

function scoreLine(view: BriarCircuitPublicView): string {
  return SEATS.map((seat) => view.cumulative_scores[seat]).join(" / ");
}

function lowestScoreSeat(view: BriarCircuitPublicView): BriarCircuitSeatId {
  return SEATS.reduce((best, seat) => (view.cumulative_scores[seat] < view.cumulative_scores[best] ? seat : best), SEATS[0]);
}

function seatLabel(seat: BriarCircuitSeatId): string {
  return `Seat ${Number(seat.slice(-1)) + 1}`;
}

function formatCardId(cardId: string): string {
  return cardId
    .split("_")
    .map((part) => part.slice(0, 1).toUpperCase() + part.slice(1))
    .join(" ");
}
