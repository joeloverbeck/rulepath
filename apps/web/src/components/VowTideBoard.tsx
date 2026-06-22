import { useMemo } from "react";
import type { ActionChoice, ActionTree, EffectEntry, VowTideCardView, VowTidePublicView, VowTideSeatId } from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type VowTideBoardProps = {
  view: VowTidePublicView;
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

const SEATS: VowTideSeatId[] = ["seat_0", "seat_1", "seat_2", "seat_3", "seat_4", "seat_5", "seat_6"];
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

export function VowTideBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onPathSubmit,
}: VowTideBoardProps) {
  const seats = activeSeats(view);
  const paths = useMemo(() => flattenActionTree(actionTree), [actionTree]);
  const choicesByCard = useMemo(
    () => new Map(paths.filter((entry) => entry.path[0] === "play").map((entry) => [entry.path[1], entry])),
    [paths],
  );
  const canAct = Boolean(interactive && !pending && view.terminal.kind === "non_terminal" && paths.length > 0);
  const feedback = vowFeedback(latestEffect) ?? (latestEffect ? feedbackForEffect(latestEffect) : null);
  const changed = effects.some((entry) => ["bid_accepted", "card_played", "trick_captured", "hand_scored"].includes(vowEffectKind(entry)));
  const outcomeExplanation =
    view.terminal.kind === "terminal"
      ? outcomeSurfaceData({
          gameId: "vow_tide",
          heading: view.terminal.winners.length > 1 ? "Vow Tide shared win" : "Vow Tide result",
          rationale: view.terminal_rationale ?? null,
          resultKind: view.terminal.winners.length > 1 ? "shared_high_score" : "high_score_win",
          decisiveCause: "final_schedule",
          templateKey: view.terminal.winners.length > 1 ? "vow_tide.shared_high_score" : "vow_tide.high_score_win",
          templateParams:
            view.terminal.winners.length === 1 ? { winner: seatLabel(view.terminal.winners[0] ?? "seat_0") } : {},
          finalStanding: view.terminal.standings.map((standing) => ({
            id: standing.seat,
            label: seatLabel(standing.seat),
            result: standing.is_winner ? "win" : "loss",
            emphasized: standing.is_winner,
            values: [
              { label: "Rank", value: standing.rank },
              { label: "Score", value: standing.score },
            ],
          })),
          breakdownSections: [
            {
              id: "schedule",
              heading: "Completed schedule",
              rows: [
                { label: "Hands played", value: view.terminal.hands_played },
                { label: "Winner count", value: view.terminal.winners.length },
              ],
            },
          ],
          ruleIds: ["VT-STANDINGS-001", "VT-TERMINAL-001"],
        })
      : null;

  return (
    <section
      className={`vow-tide-board ${view.terminal.kind === "terminal" ? "terminal" : ""}${changed ? " reveal" : ""}${reducedMotion ? " reduced" : ""}`}
      aria-labelledby="vow-tide-heading"
      data-testid="vow-tide-board"
    >
      <div className="vow-tide-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="vow-tide-heading">{statusLabel(view)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {view.active_seat ? seatLabel(view.active_seat) : "Complete"}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {view.display_name}, {view.phase}, hand {view.hand_index + 1}, {seats.length} seats, {paths.length} Rust legal choices.
      </p>

      <div className="vow-tide-metrics" aria-label="Vow Tide status">
        <Metric label="Hand" value={`${view.hand_index + 1} / ${view.hand_schedule.length}`} />
        <Metric label="Hand size" value={String(view.hand_size)} />
        <Metric label="Dealer" value={seatLabel(view.dealer)} />
        <Metric label="Hidden stock" value={String(view.hidden_stock_count)} />
      </div>

      <div className="vow-tide-table">
        <section className="vow-tide-seat-rail" aria-label="Vow Tide seats">
          {seats.map((seat) => (
            <SeatPanel key={seat} view={view} seat={seat} />
          ))}
        </section>

        <section className="vow-tide-center" aria-label="Current trick and trump">
          <div className="vow-tide-trump">
            <div className="vow-tide-trump-text">
              <span>Trump indicator</span>
              <strong className={`vow-tide-trump-suit ${suitTone(view.trump_indicator.suit)}`}>
                Trump: {suitName(view.trump_indicator.suit)} {SUIT_GLYPH[view.trump_indicator.suit] ?? ""}
              </strong>
            </div>
            <CardFace card={view.trump_indicator} />
          </div>
          <div className="vow-tide-trick">
            <div className="vow-tide-section-heading">
              <span>Current trick</span>
              <strong>
                {view.current_trick.length
                  ? `Led ${suitName(view.current_trick[0].card.suit)} · ${view.current_trick.length} played`
                  : "No cards played"}
              </strong>
            </div>
            <div className="vow-tide-played-cards">
              {view.current_trick.length === 0 ? (
                <div className="vow-tide-facedown">
                  <span>Waiting</span>
                  <strong>{view.active_seat ? `${seatLabel(view.active_seat)} to act` : "Complete"}</strong>
                </div>
              ) : (
                view.current_trick.map((play) => (
                  <div key={`${play.seat}-${play.card.card_id}`} className="vow-tide-played-card">
                    <CardFace card={play.card} />
                    <small>{seatLabel(play.seat)}</small>
                  </div>
                ))
              )}
            </div>
          </div>
        </section>
      </div>

      <section className="vow-tide-private" aria-label="Private hand">
        <div className="vow-tide-section-heading">
          <span>Private hand</span>
          <strong>{view.private_view_status === "seat" ? `${view.own_hand.length} cards` : "Hidden for observer"}</strong>
        </div>
        <div className="vow-tide-hand">
          {view.private_view_status !== "seat" ? (
            <div className="vow-tide-facedown" data-testid="vow-tide-private-hidden">
              <span>Hidden</span>
              <strong>Seat hand only</strong>
            </div>
          ) : (
            view.own_hand.map((card) => {
              const action = choicesByCard.get(card.card_id);
              return (
                <button
                  key={card.card_id}
                  type="button"
                  className={`vow-tide-card ${suitTone(card.suit)} ${action ? "legal" : ""}`}
                  disabled={!canAct || !action}
                  aria-label={action?.choice.accessibility_label ?? card.label}
                  onClick={() => action && onPathSubmit?.(action.path)}
                >
                  <CardFace card={card} />
                  <small>{action ? "Play" : "Held"}</small>
                </button>
              );
            })
          )}
        </div>
      </section>

      <section className="vow-tide-actions" aria-label="Vow Tide actions">
        <div className="vow-tide-section-heading">
          <span>Actions</span>
          <strong>{canAct ? "Available choices" : pending ? "Working" : "Waiting"}</strong>
        </div>
        <div className="vow-tide-action-grid">
          {paths.length === 0 ? (
            <p className="muted">No actions available.</p>
          ) : (
            paths.map((entry) => (
              <button
                key={entry.path.join(">")}
                type="button"
                disabled={!canAct}
                data-testid={`vow-tide-action-${entry.path.join("-")}`}
                onClick={() => onPathSubmit?.(entry.path)}
              >
                <strong>{actionLabel(entry)}</strong>
                <small>{entry.choice.accessibility_label}</small>
              </button>
            ))
          )}
        </div>
      </section>

      <section className="vow-tide-scoreboard" aria-label="Vow Tide scores">
        <div className="vow-tide-section-heading">
          <span>Scores and bids</span>
          <strong>{view.completed_hand_count} hands complete</strong>
        </div>
        <table>
          <thead>
            <tr>
              <th>Seat</th>
              <th>Bid</th>
              <th>Tricks</th>
              <th>Score</th>
            </tr>
          </thead>
          <tbody>
            {seats.map((seat) => (
              <tr key={seat} className={view.active_seat === seat ? "active" : undefined}>
                <th scope="row">{seatLabel(seat)}</th>
                <td>{formatBid(view.public_bids[seat])}</td>
                <td>{view.trick_counts[seat] ?? 0}</td>
                <td>{view.cumulative_scores[seat] ?? 0}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </section>

      <div className="vow-tide-latest" role="status" data-animation-target="vow-tide-status">
        <span>{outcomeExplanation ? "Outcome" : feedback?.title ?? "Waiting"}</span>
        <strong>
          {outcomeExplanation ? outcomeAnnouncementText(outcomeExplanation) : feedback?.detail ?? "Bids and trick play will update here."}
        </strong>
      </div>

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function SeatPanel({ view, seat }: { view: VowTidePublicView; seat: VowTideSeatId }) {
  return (
    <article className={`vow-tide-seat ${view.active_seat === seat ? "active" : ""}`}>
      <span>{seatLabel(seat)}</span>
      <strong>{view.cumulative_scores[seat] ?? 0}</strong>
      <small>
        bid {formatBid(view.public_bids[seat])} · tricks {view.trick_counts[seat] ?? 0} · cards {view.hand_counts[seat] ?? 0}
      </small>
    </article>
  );
}

function CardFace({ card }: { card: VowTideCardView }) {
  return (
    <span className={`vow-tide-card-face ${suitTone(card.suit)}`} aria-hidden="true">
      <span>{RANK_GLYPH[card.rank] ?? card.rank}</span>
      <b>{SUIT_GLYPH[card.suit] ?? card.suit}</b>
    </span>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div className="vow-tide-metric">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function activeSeats(view: VowTidePublicView): VowTideSeatId[] {
  return SEATS.filter((seat) => view.hand_counts[seat] !== undefined);
}

function flattenActionTree(tree: ActionTree | null): PathChoice[] {
  return tree?.choices.flatMap((choice) => flattenChoice(choice, [])) ?? [];
}

function flattenChoice(choice: ActionChoice, prefix: string[]): PathChoice[] {
  const path = [...prefix, choice.segment];
  const children = choice.next?.choices ?? [];
  if (children.length === 0) {
    return [{ path, choice }];
  }
  return children.flatMap((child) => flattenChoice(child, path));
}

function actionLabel(entry: PathChoice): string {
  if (entry.path[0] === "bid") return `Bid ${entry.path[1] ?? ""}`;
  if (entry.path[0] === "play") return `Play ${entry.choice.label}`;
  return entry.choice.label;
}

function statusLabel(view: VowTidePublicView): string {
  if (view.terminal.kind === "terminal") {
    return view.terminal.winners.length > 1
      ? `${view.terminal.winners.length} seats share the win`
      : `${seatLabel(view.terminal.winners[0] ?? "seat_0")} wins`;
  }
  if (view.phase === "bidding") return `${view.active_seat ? seatLabel(view.active_seat) : "Seat"} to bid`;
  if (view.phase === "playing_trick") return `${view.active_seat ? seatLabel(view.active_seat) : "Seat"} to play`;
  return view.phase;
}

function vowFeedback(entry: EffectEntry | null): { title: string; detail: string; tone: "neutral" | "movement" | "turn" | "terminal" } | null {
  if (!entry) return null;
  const kind = vowEffectKind(entry);
  const summary = String(entry.effect.payload.summary ?? "");
  switch (kind) {
    case "bid_accepted":
      return { title: "Bid accepted", detail: summary || "Recorded the public bid.", tone: "turn" };
    case "card_played":
      return { title: "Card played", detail: summary || "Played a card to the trick.", tone: "movement" };
    case "trick_captured":
      return { title: "Trick captured", detail: summary || "Awarded the trick.", tone: "movement" };
    case "hand_scored":
      return { title: "Hand scored", detail: summary || "Scored exact bids.", tone: "turn" };
    case "match_completed":
      return { title: "Match completed", detail: summary || "Finalized standings.", tone: "terminal" };
    default:
      return null;
  }
}

function vowEffectKind(entry: EffectEntry): string {
  return String(entry.effect.payload.kind ?? entry.effect.payload.type ?? "");
}

function seatLabel(seat: VowTideSeatId): string {
  const index = Number(seat.replace("seat_", ""));
  return Number.isFinite(index) ? `Tide ${index + 1}` : seat;
}

function formatBid(value: number | null | undefined): string {
  return value === null || value === undefined ? "—" : String(value);
}

function suitTone(suit: string): string {
  return suit === "hearts" || suit === "diamonds" ? "red" : "black";
}

function suitName(suit: string): string {
  return suit.length > 0 ? suit.charAt(0).toUpperCase() + suit.slice(1) : suit;
}
