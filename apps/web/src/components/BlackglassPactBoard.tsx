import { useMemo } from "react";
import type {
  ActionChoice,
  ActionTree,
  BlackglassPactBidView,
  BlackglassPactCardView,
  BlackglassPactPublicView,
  BlackglassPactSeatId,
  BlackglassPactTeamId,
  EffectEntry,
} from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type BlackglassPactBoardProps = {
  view: BlackglassPactPublicView;
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

const SEATS: BlackglassPactSeatId[] = ["seat_0", "seat_1", "seat_2", "seat_3"];
const TEAMS: BlackglassPactTeamId[] = ["team_0", "team_1"];
const TEAM_SEATS: Record<BlackglassPactTeamId, BlackglassPactSeatId[]> = {
  team_0: ["seat_0", "seat_2"],
  team_1: ["seat_1", "seat_3"],
};
const SUIT_GLYPH: Record<string, string> = { clubs: "C", diamonds: "D", hearts: "H", spades: "S" };
const SUIT_PIP: Record<string, string> = { clubs: "♣", diamonds: "♦", hearts: "♥", spades: "♠" };
// Presentation-only hand ordering. Spades are trump, so they group at one end;
// colors alternate (red/black/red/black) so adjacent suits stay easy to tell apart.
// Rust still owns card identity, legality, and every game decision.
const SUIT_SORT: Record<string, number> = { diamonds: 0, clubs: 1, hearts: 2, spades: 3 };
const RANK_SORT: Record<string, number> = {
  two: 0,
  three: 1,
  four: 2,
  five: 3,
  six: 4,
  seven: 5,
  eight: 6,
  nine: 7,
  ten: 8,
  jack: 9,
  queen: 10,
  king: 11,
  ace: 12,
};

export function BlackglassPactBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onPathSubmit,
}: BlackglassPactBoardProps) {
  const paths = useMemo(() => flattenActionTree(actionTree), [actionTree]);
  const playChoices = useMemo(
    () => new Map(paths.filter((entry) => entry.path[0] === "play").map((entry) => [entry.path[1], entry])),
    [paths],
  );
  const nonCardChoices = paths.filter((entry) => entry.path[0] !== "play");
  const sortedHand = useMemo(
    () =>
      [...(view.own_hand ?? [])].sort(
        (a, b) =>
          (SUIT_SORT[a.suit] ?? 0) - (SUIT_SORT[b.suit] ?? 0) ||
          (RANK_SORT[a.rank] ?? 0) - (RANK_SORT[b.rank] ?? 0),
      ),
    [view.own_hand],
  );
  const canAct = Boolean(interactive && !pending && view.private_view_status === "seat" && paths.length > 0 && view.phase.kind !== "terminal");
  const changed = effects.some((entry) =>
    ["card_played", "trick_captured", "bid_accepted", "hand_scored", "deal_completed"].includes(
      String(entry.effect.payload.type),
    ),
  );
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const latestDetail = blackglassLatestDetail(latestEffect, feedback?.detail ?? null);
  const terminalTeam = view.phase.kind === "terminal" ? view.phase.winning_team : null;
  const outcomeExplanation =
    terminalTeam
      ? outcomeSurfaceData({
          gameId: "blackglass_pact",
          heading: "Blackglass Pact result",
          rationale: view.outcome_rationale ?? null,
          resultKind: "team_score_win",
          decisiveCause: "terminal_score_threshold",
          templateKey: "blackglass_pact.team_score_win",
          templateParams: { winner: teamLabel(terminalTeam) },
          finalStanding: TEAMS.map((team) => ({
            id: team,
            label: teamLabel(team),
            result: team === terminalTeam ? "win" : "loss",
            emphasized: team === terminalTeam,
            values: [
              { label: "Score", value: view.team_scores[team] },
              { label: "Bags", value: view.team_bags[team] },
            ],
          })),
          breakdownSections: [
            {
              id: "teams",
              heading: "Final team totals",
              rows: TEAMS.map((team) => ({
                label: teamLabel(team),
                value: `${view.team_scores[team]} points, ${view.team_bags[team]} bags`,
              })),
            },
          ],
          ruleIds: ["BP-END-001", "BP-END-002", "BP-END-003"],
        })
      : null;

  return (
    <section
      className={`blackglass-board ${view.phase.kind === "terminal" ? "terminal" : ""}${changed ? " reveal" : ""}${reducedMotion ? " reduced" : ""}`}
      aria-labelledby="blackglass-heading"
      data-testid="blackglass-pact-board"
    >
      <div className="blackglass-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="blackglass-heading">{statusLabel(view)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {view.active_seat ? seatLabel(view.active_seat) : view.phase.kind === "terminal" ? "Complete" : "Resolving"}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {view.display_name}, {phaseLabel(view)}, hand {view.hand_index + 1}, {paths.length} legal choices.
      </p>

      <div className="blackglass-metrics" aria-label="Blackglass Pact status">
        <Metric label="Hand" value={String(view.hand_index + 1)} />
        <Metric label="Dealer" value={seatLabel(view.dealer)} />
        <Metric label="Phase" value={phaseLabel(view)} />
        <Metric label="Spades" value={view.spades_broken ? "Broken" : "Unbroken"} />
      </div>

      <section className="blackglass-teams" aria-label="Partnerships">
        {TEAMS.map((team) => (
          <article key={team} className={`blackglass-team ${team === "team_0" ? "north-south" : "east-west"}`}>
            <div className="blackglass-section-heading">
              <span>{teamLabel(team)}</span>
              <strong>{TEAM_SEATS[team].map(seatLabel).join(" + ")}</strong>
            </div>
            <dl>
              <div>
                <dt>Score</dt>
                <dd>{view.team_scores[team]}</dd>
              </div>
              <div>
                <dt>Bags</dt>
                <dd>{view.team_bags[team]}</dd>
              </div>
              <div>
                <dt>Contract</dt>
                <dd>{contractForTeam(view, team)}</dd>
              </div>
            </dl>
          </article>
        ))}
      </section>

      <div className="blackglass-table">
        <section className="blackglass-seat-rail" aria-label="Seats">
          {SEATS.map((seat) => (
            <SeatPanel key={seat} view={view} seat={seat} />
          ))}
        </section>

        <section className="blackglass-trick" aria-label="Current trick">
          <div className="blackglass-section-heading">
            <span>Current trick</span>
            <strong>{view.current_trick.length ? `${view.current_trick.length} / 4 played` : "Waiting"}</strong>
          </div>
          <div className="blackglass-played-cards">
            {view.current_trick.length === 0 ? (
              <div className="blackglass-facedown">
                <span>No card played</span>
                <strong>{view.active_seat ? `${seatLabel(view.active_seat)} to act` : "No active seat"}</strong>
              </div>
            ) : (
              view.current_trick.map((play) => (
                <div key={`${play.seat}-${play.card.id}`} className="blackglass-played-card">
                  <CardFace card={play.card} />
                  <small>{seatLabel(play.seat)}</small>
                </div>
              ))
            )}
          </div>
        </section>
      </div>

      <section className="blackglass-private" aria-label="Private hand">
        <div className="blackglass-section-heading">
          <span>Private hand</span>
          <strong>
            {view.private_view_status !== "seat"
              ? "Hidden for observer"
              : view.phase.kind === "blind_nil_commitment"
                ? "Not dealt yet"
                : `${view.own_hand?.length ?? 0} cards`}
          </strong>
        </div>
        <div className="blackglass-hand">
          {view.private_view_status !== "seat" ? (
            <div className="blackglass-facedown" data-testid="blackglass-private-hidden">
              <span>Hidden</span>
              <strong>Seat hand only</strong>
            </div>
          ) : sortedHand.length === 0 ? (
            <div className="blackglass-facedown" data-testid="blackglass-private-empty">
              <span>{view.phase.kind === "blind_nil_commitment" ? "No cards yet" : "No cards in hand"}</span>
              <strong>
                {view.phase.kind === "blind_nil_commitment"
                  ? "Blind nil is committed before the deal"
                  : "Your hand is empty"}
              </strong>
            </div>
          ) : (
            sortedHand.map((card) => {
              const action = playChoices.get(card.id);
              return (
                <button
                  key={card.id}
                  type="button"
                  className={`blackglass-card ${suitTone(card.suit)} ${action ? "legal" : ""}`}
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

      <section className="blackglass-actions" aria-label="Blackglass Pact actions">
        <div className="blackglass-section-heading">
          <span>Actions</span>
          <strong>{canAct ? "Available choices" : pending ? "Working" : "Waiting"}</strong>
        </div>
        <div className="blackglass-action-grid">
          {nonCardChoices.length === 0 ? (
            <p className="muted">No blind or bid action available.</p>
          ) : (
            nonCardChoices.map((entry) => (
              <button
                key={entry.path.join(">")}
                type="button"
                disabled={!canAct}
                data-testid={`blackglass-action-${entry.path.join("-")}`}
                onClick={() => onPathSubmit?.(entry.path)}
              >
                <span>{entry.path[0] === "blind_nil" ? "Blind nil" : "Bid"}</span>
                <strong>{entry.choice.label}</strong>
              </button>
            ))
          )}
        </div>
      </section>

      {view.last_hand_score ? <HandScorePanel view={view} /> : null}

      {feedback ? (
        <section className="blackglass-latest" aria-label="Latest event">
          <span>{feedback.title}</span>
          <strong>{latestDetail ?? feedback.detail}</strong>
        </section>
      ) : null}

      {outcomeExplanation ? (
        <>
          <OutcomeExplanationPanel explanation={outcomeExplanation} reducedMotion={reducedMotion} />
          <p className="sr-only" aria-live="polite">
            {outcomeAnnouncementText(outcomeExplanation)}
          </p>
        </>
      ) : null}
    </section>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div className="blackglass-metric">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function SeatPanel({ view, seat }: { view: BlackglassPactPublicView; seat: BlackglassPactSeatId }) {
  const active = view.active_seat === seat;
  const bid = view.bids.find((row) => row.seat === seat)?.bid ?? null;
  return (
    <article className={`blackglass-seat ${active ? "active" : ""} ${view.viewer_seat === seat ? "viewer" : ""}`}>
      <div className="blackglass-section-heading">
        <span>{seatLabel(seat)}</span>
        <strong>{teamLabel(teamForSeat(seat))}</strong>
      </div>
      <dl>
        <div>
          <dt>Cards</dt>
          <dd>{handCount(view, seat)}</dd>
        </div>
        <div>
          <dt>Bid</dt>
          <dd>{bidLabel(bid)}</dd>
        </div>
      </dl>
    </article>
  );
}

function HandScorePanel({ view }: { view: BlackglassPactPublicView }) {
  const score = view.last_hand_score;
  if (!score) return null;
  return (
    <section className="blackglass-score-panel" aria-label="Last hand score">
      <div className="blackglass-section-heading">
        <span>Last hand score</span>
        <strong>Hand {score.hand_index + 1}</strong>
      </div>
      <table>
        <caption>Teams</caption>
        <thead>
          <tr>
            <th>Team</th>
            <th>Contract</th>
            <th>Ord. tricks</th>
            <th>Delta</th>
            <th>Score</th>
            <th>Bags</th>
          </tr>
        </thead>
        <tbody>
          {score.teams.map((team) => (
            <tr key={team.team}>
              <th>{teamLabel(team.team)}</th>
              <td>{team.contract}</td>
              <td>{team.ordinary_tricks}</td>
              <td>{team.hand_delta}</td>
              <td>
                {team.prior_score} to {team.next_score}
              </td>
              <td>{team.next_bags}</td>
            </tr>
          ))}
        </tbody>
      </table>
      <table className="blackglass-seat-score">
        <caption>Seats</caption>
        <thead>
          <tr>
            <th>Seat</th>
            <th>Team</th>
            <th>Bid</th>
            <th>Tricks</th>
            <th>Nil result</th>
          </tr>
        </thead>
        <tbody>
          {score.seats.map((seat) => (
            <tr key={seat.seat}>
              <th>{seatLabel(seat.seat)}</th>
              <td>{teamLabel(seat.team)}</td>
              <td>{bidLabel(seat.bid)}</td>
              <td>{seat.tricks}</td>
              <td>{nilResultLabel(seat.nil_result)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}

function nilResultLabel(result: string | null): string {
  if (result === "Made") return "Made";
  if (result === "Failed") return "Failed";
  return "—";
}

function CardFace({ card }: { card: BlackglassPactCardView }) {
  return (
    <span
      className={`blackglass-card-face ${suitTone(card.suit)}`}
      data-card-label={card.label}
      aria-hidden="true"
    >
      <b>{rankSymbol(card)}</b>
      <small>{SUIT_PIP[card.suit] ?? SUIT_GLYPH[card.suit] ?? card.suit}</small>
    </span>
  );
}

// The Rust public label is rank+suit-letter (e.g. "7S", "10D", "AS"). Show the rank
// prominently and render the suit as a standard pip, dropping the redundant letter.
function rankSymbol(card: BlackglassPactCardView): string {
  const glyph = SUIT_GLYPH[card.suit];
  if (glyph && card.label.endsWith(glyph)) {
    return card.label.slice(0, card.label.length - glyph.length);
  }
  return card.label;
}

function flattenActionTree(tree: ActionTree | null): PathChoice[] {
  if (!tree) return [];
  const paths: PathChoice[] = [];
  for (const root of tree.choices) {
    if (!root.next?.choices.length) {
      paths.push({ path: [root.segment], choice: root });
      continue;
    }
    for (const leaf of root.next.choices) {
      paths.push({ path: [root.segment, leaf.segment], choice: leaf });
    }
  }
  return paths;
}

// The shared effect feedback falls back to a generic "Rust awarded the trick."
// because Rust does not attach a summary string to the trick_captured effect.
// The public payload does carry the winning seat, so name it here using the
// board's own seat labels (UI.md: the latest event should name the trick winner).
// Presentation-only: the winner and trick index are public Rust-projected facts.
function blackglassLatestDetail(entry: EffectEntry | null, fallback: string | null): string | null {
  if (!entry) return fallback;
  const payload = entry.effect.payload as {
    type?: string;
    winner?: unknown;
    seat?: unknown;
    dealer?: unknown;
    team?: unknown;
    card?: unknown;
    bid?: unknown;
    hand_index?: unknown;
    points_deducted?: unknown;
    trick_index?: unknown;
  };
  if (payload.type === "bid_accepted" && typeof payload.seat === "string") {
    const seat = payload.seat as BlackglassPactSeatId;
    const bid = payload.bid as BlackglassPactBidView | undefined;
    if (SEATS.includes(seat) && bid) {
      const text = bid.kind === "blind_nil" ? "blind nil" : bid.kind === "nil" ? "nil" : String(bid.value);
      return `${seatLabel(seat)} bid ${text}.`;
    }
  }
  if (payload.type === "card_played" && typeof payload.seat === "string") {
    const seat = payload.seat as BlackglassPactSeatId;
    const card = payload.card as BlackglassPactCardView | undefined;
    if (SEATS.includes(seat) && card?.label) {
      return `${seatLabel(seat)} played ${rankSymbol(card)}${SUIT_PIP[card.suit] ?? ""}.`;
    }
  }
  if (payload.type === "trick_captured" && typeof payload.winner === "string") {
    const seat = payload.winner as BlackglassPactSeatId;
    if (!SEATS.includes(seat)) return fallback;
    const trickNumber = typeof payload.trick_index === "number" ? payload.trick_index + 1 : null;
    return trickNumber ? `${seatLabel(seat)} won trick ${trickNumber}.` : `${seatLabel(seat)} won the trick.`;
  }
  if (payload.type === "spades_broken" && typeof payload.seat === "string") {
    const seat = payload.seat as BlackglassPactSeatId;
    if (SEATS.includes(seat)) return `${seatLabel(seat)} broke spades; spades may now be led.`;
  }
  if (payload.type === "blind_nil_declared" && typeof payload.seat === "string") {
    const seat = payload.seat as BlackglassPactSeatId;
    if (SEATS.includes(seat)) return `${seatLabel(seat)} committed to blind nil.`;
  }
  if (payload.type === "blind_nil_declined" && typeof payload.seat === "string") {
    const seat = payload.seat as BlackglassPactSeatId;
    if (SEATS.includes(seat)) return `${seatLabel(seat)} declined blind nil.`;
  }
  if (payload.type === "dealer_advanced" && typeof payload.dealer === "string") {
    const seat = payload.dealer as BlackglassPactSeatId;
    if (SEATS.includes(seat)) {
      const handNumber = typeof payload.hand_index === "number" ? payload.hand_index + 1 : null;
      return handNumber ? `${seatLabel(seat)} deals hand ${handNumber}.` : `${seatLabel(seat)} deals the next hand.`;
    }
  }
  if (payload.type === "bag_penalty_applied" && typeof payload.team === "string") {
    const team = payload.team as BlackglassPactTeamId;
    if (TEAMS.includes(team)) {
      const points = Math.abs(Number(payload.points_deducted) || 0);
      return `${teamLabel(team)} lost ${points} points to a bag penalty.`;
    }
  }
  return fallback;
}

function statusLabel(view: BlackglassPactPublicView): string {
  if (view.phase.kind === "terminal") return `${teamLabel(view.phase.winning_team)} wins`;
  if (view.phase.kind === "blind_nil_commitment") return view.phase.active ? `${seatLabel(view.phase.active)} blind nil` : "Blind nil window";
  if (view.phase.kind === "bidding") return `${seatLabel(view.phase.next)} bidding`;
  if (view.phase.kind === "playing_trick") return `${seatLabel(view.phase.next)} to play`;
  return "Scoring hand";
}

function phaseLabel(view: BlackglassPactPublicView): string {
  return view.phase.kind.replaceAll("_", " ");
}

function seatLabel(seat: BlackglassPactSeatId): string {
  return ({ seat_0: "North", seat_1: "East", seat_2: "South", seat_3: "West" } as const)[seat];
}

function teamLabel(team: BlackglassPactTeamId): string {
  return team === "team_0" ? "North-South" : "East-West";
}

function teamForSeat(seat: BlackglassPactSeatId): BlackglassPactTeamId {
  return seat === "seat_0" || seat === "seat_2" ? "team_0" : "team_1";
}

function contractForTeam(view: BlackglassPactPublicView, team: BlackglassPactTeamId): number {
  return view.team_contracts.find((row) => row.team === team)?.ordinary_contract ?? 0;
}

function handCount(view: BlackglassPactPublicView, seat: BlackglassPactSeatId): number {
  return view.hand_counts.find((row) => row.seat === seat)?.count ?? 0;
}

function bidLabel(bid: BlackglassPactBidView | null): string {
  if (!bid) return "None";
  if (bid.kind === "blind_nil") return "Blind nil";
  if (bid.kind === "nil") return "Nil";
  return String(bid.value);
}

function suitTone(suit: string): "red" | "black" {
  return suit === "hearts" || suit === "diamonds" ? "red" : "black";
}
