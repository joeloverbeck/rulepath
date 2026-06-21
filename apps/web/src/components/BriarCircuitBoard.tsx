import { useMemo, useState } from "react";
import type {
  ActionChoice,
  ActionTree,
  BriarCircuitHandSummaryView,
  BriarCircuitPublicView,
  BriarCircuitSeatId,
  EffectEntry,
} from "../wasm/client";
import { resolveSeatLabel } from "../seatLabels";
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
  onRestart?: () => void;
};

type PathChoice = {
  path: string[];
  choice: ActionChoice;
};

const SEATS: BriarCircuitSeatId[] = ["seat_0", "seat_1", "seat_2", "seat_3"];

const SUIT_GLYPH: Record<string, string> = { clubs: "♣", diamonds: "♦", hearts: "♥", spades: "♠" };
const RANK_GLYPH: Record<string, string> = {
  two: "2",
  three: "3",
  four: "4",
  five: "5",
  six: "6",
  seven: "7",
  eight: "8",
  nine: "9",
  ten: "10",
  jack: "J",
  queen: "Q",
  king: "K",
  ace: "A",
};
// Display-only suit grouping. Alternating red/black keeps adjacent suit groups
// visually distinct when the owner hand is sorted.
const SUIT_SORT: Record<string, number> = { clubs: 0, diamonds: 1, spades: 2, hearts: 3 };

function isRedSuit(suit: string): boolean {
  return suit === "hearts" || suit === "diamonds";
}

// Penalty cards under briar-circuit-rules-v1 scoring: each heart is 1, the queen
// of spades is 13. This is a presentation-only highlight of an already-known card
// identity; Rust remains the scoring authority and TypeScript decides no legality.
function pointValue(suit: string, rank: string): number | null {
  if (suit === "spades" && rank === "queen") return 13;
  if (suit === "hearts") return 1;
  return null;
}

function splitCardId(cardId: string): { rank: string; suit: string } {
  const idx = cardId.lastIndexOf("_");
  if (idx < 0) return { rank: cardId, suit: "" };
  return { rank: cardId.slice(0, idx), suit: cardId.slice(idx + 1) };
}

function CardFace({ suit, rank }: { suit: string; rank: string }) {
  return (
    <span className="briar-card-face" aria-hidden="true">
      <span className="briar-card-rank">{RANK_GLYPH[rank] ?? rank}</span>
      <span className="briar-card-suit">{SUIT_GLYPH[suit] ?? suit}</span>
    </span>
  );
}

export function BriarCircuitBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onPathSubmit,
  onRestart,
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
  // Sort the owner hand for display only (suit groups, then ascending rank). Pure
  // presentation: the underlying Rust action leaves are looked up by card id, so
  // reordering the buttons never changes which plays are legal.
  const sortedHand = useMemo(
    () => [...view.own_hand].sort((a, b) => SUIT_SORT[a.suit] - SUIT_SORT[b.suit] || a.rank_value - b.rank_value),
    [view.own_hand],
  );
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const canAct = Boolean(interactive && !pending && view.private_view_status === "seat" && paths.length > 0 && view.phase !== "terminal");
  // Between-hands scoring summary: the most recently completed hand, retained by Rust.
  // Each hand has a unique cumulative-score signature, so dismissal is keyed by it; a
  // newly completed hand produces a new signature and re-shows the panel.
  const summary = view.last_hand_summary;
  const summarySignature = summary ? SEATS.map((seat) => summary.cumulative_after[seat]).join("-") : null;
  const [dismissedSignature, setDismissedSignature] = useState<string | null>(null);
  const showHandSummary = Boolean(summary && view.phase !== "terminal" && summarySignature !== dismissedSignature);
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
        <div className="briar-banner-actions">
          <span className="turn-pill" data-testid="turn">
            {turnLabel(view)}
          </span>
          {onRestart ? (
            <button type="button" className="briar-restart" data-testid="briar-circuit-restart" onClick={onRestart}>
              Restart
            </button>
          ) : null}
        </div>
      </div>

      {showHandSummary && summary ? (
        <HandSummaryPanel
          summary={summary}
          handNumber={view.hand_index}
          onDismiss={() => setDismissedSignature(summarySignature)}
        />
      ) : null}

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
              view.current_trick.map((play) => {
                const { rank, suit } = splitCardId(play.card);
                const penalty = pointValue(suit, rank);
                const leadSuit = view.current_trick[0] ? splitCardId(view.current_trick[0].card).suit : suit;
                return (
                  <div
                    className={`briar-played-card ${isRedSuit(suit) ? "red" : "black"}${penalty !== null ? " point" : ""}${
                      suit === leadSuit ? " lead" : ""
                    }`}
                    key={`${play.seat}-${play.card}`}
                  >
                    <span className="briar-played-seat">{seatLabel(play.seat)}</span>
                    <CardFace suit={suit} rank={rank} />
                    <span className="sr-only">{formatCardId(play.card)}</span>
                  </div>
                );
              })
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
              {sortedHand.map((card) => {
                const selectChoice = passSelect.get(card.card_id) ?? null;
                const unselectChoice = passUnselect.get(card.card_id) ?? null;
                const playChoice = playChoices.get(card.card_id) ?? null;
                const selectedForPass = selected.has(card.card_id);
                const action = selectedForPass ? unselectChoice : selectChoice ?? playChoice;
                const penalty = pointValue(card.suit, card.rank);
                return (
                  <button
                    type="button"
                    key={card.card_id}
                    className={`briar-card suit-${card.suit} ${isRedSuit(card.suit) ? "red" : "black"}${
                      penalty !== null ? " point" : ""
                    } ${action ? "legal" : ""}${selectedForPass ? " selected" : ""}`}
                    disabled={!canAct || !action}
                    aria-pressed={selectedForPass}
                    aria-label={action?.choice.accessibility_label ?? card.accessibility_label}
                    onClick={() => action && onPathSubmit?.(action.path)}
                  >
                    <CardFace suit={card.suit} rank={card.rank} />
                    {penalty !== null ? (
                      <span className="briar-card-penalty" aria-hidden="true">
                        {penalty}
                      </span>
                    ) : null}
                    <small className="briar-card-state">
                      {selectedForPass ? "Selected" : action ? actionLabel(action.path) : "Held"}
                    </small>
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
            {view.captured_tricks.slice(-6).map((trick) => {
              const trickPoints = trick.plays.reduce((total, play) => {
                const { rank, suit } = splitCardId(play.card);
                return total + (pointValue(suit, rank) ?? 0);
              }, 0);
              return (
                <li key={`${trick.hand_index}-${trick.trick_index}`}>
                  <span>Trick {trick.trick_index + 1}</span>
                  <strong>
                    {seatLabel(trick.winner)}
                    {trickPoints > 0 ? <em className="briar-trick-points"> +{trickPoints}</em> : null}
                  </strong>
                  <small className="briar-history-cards">
                    {trick.plays.map((play) => {
                      const { rank, suit } = splitCardId(play.card);
                      const penalty = pointValue(suit, rank);
                      return (
                        <span
                          key={play.card}
                          className={`briar-mini-card ${isRedSuit(suit) ? "red" : "black"}${penalty !== null ? " point" : ""}`}
                        >
                          <span className="briar-mini-seat">{seatLabel(play.seat)}</span>
                          <span className="briar-mini-glyph" aria-hidden="true">
                            {RANK_GLYPH[rank] ?? rank}
                            {SUIT_GLYPH[suit] ?? suit}
                          </span>
                          <span className="sr-only">{formatCardId(play.card)}</span>
                        </span>
                      );
                    })}
                  </small>
                </li>
              );
            })}
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

function HandSummaryPanel({
  summary,
  handNumber,
  onDismiss,
}: {
  summary: BriarCircuitHandSummaryView;
  handNumber: number;
  onDismiss: () => void;
}) {
  const moonShooter = summary.moon_shooter;
  return (
    <section className="briar-hand-summary" role="status" aria-live="polite" data-testid="briar-hand-summary">
      <div className="briar-hand-summary-head">
        <strong>Hand {handNumber} complete</strong>
        {moonShooter ? <span className="briar-moon-flag">{seatLabel(moonShooter)} shot the moon</span> : null}
      </div>
      <table className="briar-hand-summary-table">
        <thead>
          <tr>
            <th scope="col">Seat</th>
            <th scope="col">Took</th>
            <th scope="col">Added</th>
            <th scope="col">Total</th>
          </tr>
        </thead>
        <tbody>
          {SEATS.map((seat) => (
            <tr key={seat} className={seat === moonShooter ? "moon" : ""}>
              <th scope="row">{seatLabel(seat)}</th>
              <td>{summary.raw_points[seat]}</td>
              <td>+{summary.hand_additions[seat]}</td>
              <td>{summary.cumulative_after[seat]}</td>
            </tr>
          ))}
        </tbody>
      </table>
      <button type="button" className="briar-hand-summary-continue" data-testid="briar-hand-summary-continue" onClick={onDismiss}>
        Continue
      </button>
    </section>
  );
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
  return resolveSeatLabel(seat);
}

function formatCardId(cardId: string): string {
  return cardId
    .split("_")
    .map((part) => part.slice(0, 1).toUpperCase() + part.slice(1))
    .join(" ");
}
