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
  const canAct = Boolean(interactive && !pending && view.private_view_status === "seat" && paths.length > 0 && view.phase.kind !== "terminal");
  const changed = effects.some((entry) =>
    ["card_played", "trick_captured", "bid_accepted", "hand_scored", "deal_completed"].includes(
      String(entry.effect.payload.type),
    ),
  );
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
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
        {view.display_name}, {phaseLabel(view)}, hand {view.hand_index + 1}, {paths.length} Rust legal choices.
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
          <strong>{view.private_view_status === "seat" ? `${view.own_hand?.length ?? 0} cards` : "Hidden for observer"}</strong>
        </div>
        <div className="blackglass-hand">
          {view.private_view_status !== "seat" ? (
            <div className="blackglass-facedown" data-testid="blackglass-private-hidden">
              <span>Hidden</span>
              <strong>Seat hand only</strong>
            </div>
          ) : (
            (view.own_hand ?? []).map((card) => {
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
          <strong>{feedback.detail}</strong>
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
        <thead>
          <tr>
            <th>Team</th>
            <th>Contract</th>
            <th>Tricks</th>
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
    </section>
  );
}

function CardFace({ card }: { card: BlackglassPactCardView }) {
  return (
    <span className={`blackglass-card-face ${suitTone(card.suit)}`} aria-hidden="true">
      <b>{card.label}</b>
      <small>{SUIT_GLYPH[card.suit] ?? card.suit}</small>
    </span>
  );
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
