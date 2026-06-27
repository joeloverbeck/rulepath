import { useMemo, useState } from "react";
import type {
  ActionChoice,
  ActionTree,
  EffectEntry,
  MeldfallLedgerMeldGroupView,
  MeldfallLedgerPublicView,
  MeldfallLedgerSeatId,
  MeldfallLedgerSettlementView,
  MeldfallLedgerStandingView,
  MeldfallLedgerTableCardView,
} from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type MeldfallLedgerBoardProps = {
  view: MeldfallLedgerPublicView;
  actionTree: ActionTree | null;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onPathSubmit?: (path: string[]) => void;
};

type GroupedChoices = {
  draw: ActionChoice[];
  table: ActionChoice[];
  discard: ActionChoice[];
  turn: ActionChoice[];
};

const SEATS: MeldfallLedgerSeatId[] = ["seat_0", "seat_1", "seat_2", "seat_3", "seat_4", "seat_5"];

// Cumulative match target for the pinned classic_500_single_deck_v1 variant
// (ML-MATCH-001). A fixed variant constant surfaced for presentation only.
const MATCH_TARGET = 500;

export function MeldfallLedgerBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onPathSubmit,
}: MeldfallLedgerBoardProps) {
  const choices = useMemo(() => actionTree?.choices ?? [], [actionTree]);
  const groupedChoices = useMemo(() => groupChoices(choices), [choices]);
  const [handSort, setHandSort] = useState<HandSort>("suit");
  const sortedHand = useMemo(
    () => [...view.own_hand].sort((a, b) => compareHandCards(a, b, handSort)),
    [view.own_hand, handSort],
  );

  const seats = SEATS.slice(0, view.hand_counts.length);
  const canAct = Boolean(interactive && !pending && !view.terminal && choices.length > 0);
  // A discard pickup is offered only when its chosen card has an immediate legal use
  // this turn; Rust omits the choice otherwise (ML-TURN-004). When this seat is making
  // a live draw decision we can explain why a disabled discard is not pickable.
  const drawDecision = canAct && !view.terminal && view.phase === "draw";
  const roundSettled = !view.terminal && view.phase === "round_settled";
  // Stock-exhaustion pressure (ML-TURN-009): when the stock runs out and no seat has a
  // legal draw, the round settles immediately and the discard pile is never reshuffled.
  // The raw count is public, so flagging the endgame is a presentation-only proximity
  // signal the strategy guide calls out. Threshold scales with table size: once fewer
  // cards remain than seats, the next go-around cannot give everyone a stock draw.
  const stockLive = !view.terminal && !roundSettled;
  const stockEmpty = stockLive && view.stock_count === 0;
  const stockLow = stockLive && view.stock_count > 0 && view.stock_count <= seats.length;
  // Next seat to act, clockwise by seat index (advance_active_seat uses
  // next_clockwise_index). Turn order is public (ML-SETUP-005), and the next seat is
  // the first to see — and most likely to pick up — your discard, which the strategy
  // guide flags as a discard-risk signal. Presentation-only derivation from public
  // seat order; null when the round is not actively in play or at a single seat.
  const activeIndex = seats.indexOf(view.active_seat);
  const nextSeat =
    stockLive && activeIndex >= 0 && seats.length > 1 ? seats[(activeIndex + 1) % seats.length] : null;
  // After a discard pickup, Rust offers only table plays that use the committed card
  // and withholds finish/discard until the commitment is satisfied (ML-TURN-004). In
  // the table phase that is the only reason finish is absent, so we can tell the player
  // why their turn is locked. Inferred from the Rust action set, not computed here.
  const pickupCommitmentPending =
    canAct &&
    view.phase === "table" &&
    groupedChoices.table.length > 0 &&
    groupedChoices.turn.length === 0 &&
    groupedChoices.discard.length === 0;
  // Go-out is offered only when the hand can be emptied without a final discard
  // (ML-TURN-007). Surfaced from the Rust action set so the board can explain that
  // choosing it ends the round immediately rather than passing the turn.
  const goOutAvailable = groupedChoices.turn.some((choice) => choice.segment === "go-out-without-discard");
  // Plain-language guidance for the active seat's current step. The turn runs
  // draw -> table -> discard, and in the table phase "Finish turn" advances to a
  // mandatory discard rather than ending the turn, which the bare button label does
  // not convey. Derived only from the public phase and the Rust-offered action kinds.
  const turnGuidance =
    canAct && !pickupCommitmentPending ? turnGuidanceText(view.phase, goOutAvailable, stockEmpty) : null;
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const tableChanged = effects.some((entry) => {
    const payload = entry.effect.payload;
    const kind = String(payload.type ?? payload.kind ?? "");
    return ["draw", "meld", "lay_off", "discard", "round_score", "match_terminal"].includes(kind);
  });
  const drawStockChoice = groupedChoices.draw.find((choice) => choice.segment === "draw-stock") ?? null;
  const outcomeExplanation = view.terminal
    ? outcomeSurfaceData({
        gameId: "meldfall_ledger",
        heading: terminalHeading(view.terminal.standings),
        rationale: view.terminal_rationale ?? null,
        resultKind: "win",
        decisiveCause: "unique_high_score_at_target",
        templateKey: "meldfall_ledger.high_score_win",
        templateParams: {
          winner: winnerStanding(view.terminal.standings)?.seat ?? "winner",
          target: MATCH_TARGET,
        },
        finalStanding: view.terminal.standings.map((standing) => ({
          id: standing.seat,
          label: seatLabel(standing.seat),
          result: standing.winner ? "win" : `rank ${standing.rank}`,
          emphasized: standing.winner,
          values: [
            { label: "Cumulative score", value: standing.cumulative_score, ruleId: "ML-MATCH-005" },
            { label: "Latest round delta", value: standing.latest_round_delta, ruleId: "ML-SCORE-004" },
          ],
        })),
        breakdownSections: [
          {
            id: "terminal-threshold",
            heading: "Target threshold",
            rows: [
              { label: "Target", value: MATCH_TARGET, ruleId: "ML-MATCH-001" },
              { label: "Winner rule", value: "Unique highest eligible score", ruleId: "ML-MATCH-002" },
            ],
          },
        ],
        ruleIds: ["ML-MATCH-001", "ML-MATCH-002", "ML-MATCH-005"],
      })
    : null;

  return (
    <section
      className={["meldfall-board", view.terminal ? "terminal" : "", tableChanged ? "reveal" : "", reducedMotion ? "reduced" : ""]
        .filter(Boolean)
        .join(" ")}
      aria-labelledby="meldfall-heading"
      data-testid="meldfall-ledger-board"
      data-animation-target="meldfall-board"
    >
      <div className="meldfall-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="meldfall-heading">{statusLabel(view)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {view.terminal
            ? terminalHeading(view.terminal.standings)
            : roundSettled
              ? roundEndLabel(view.round_end)
              : `${seatLabel(view.active_seat)} acts`}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {view.display_name}, {phaseLabel(view.phase)}, stock count {view.stock_count}, discard cards {view.discard.length}, meld groups {view.tableau.groups.length}.
      </p>

      <div className="meldfall-metrics" aria-label="Meldfall Ledger status">
        <Metric label="Phase" value={phaseLabel(view.phase)} />
        <Metric label="Dealer" value={seatLabel(view.dealer)} />
        <Metric label="Stock" value={`${view.stock_count} hidden`} />
        <Metric label="Discard" value={`${view.discard.length} public`} />
        <Metric label="Target" value={`${MATCH_TARGET} to win`} />
      </div>

      {view.last_settlement ? <SettlementSummary settlement={view.last_settlement} /> : null}

      <div className="meldfall-table-shell" aria-label="Meldfall Ledger table">
        <section className="meldfall-seat-rail" aria-label="Seat score ledger">
          {seats.map((seat, index) => (
            <SeatLedger key={seat} view={view} seat={seat} index={index} nextSeat={nextSeat} />
          ))}
        </section>

        <div className="meldfall-layout">
          <section className="meldfall-zones" aria-label="Stock and discard zones">
            <div className="meldfall-section-heading">
              <span>Draw zones</span>
              <strong>{view.discard.length ? "Discard is public oldest to newest" : "Discard empty"}</strong>
            </div>
            <div className="meldfall-draw-zones">
              <button
                type="button"
                className={`meldfall-stock${stockLow || stockEmpty ? " low" : ""}`}
                disabled={!canAct || !drawStockChoice}
                aria-label={drawStockChoice?.accessibility_label ?? `${view.stock_count} hidden stock cards`}
                data-testid="meldfall-stock"
                onClick={() => drawStockChoice && onPathSubmit?.([drawStockChoice.segment])}
              >
                <span>Stock</span>
                <strong>{view.stock_count}</strong>
                <small>{drawStockChoice ? "Draw from stock" : stockEmpty ? "Stock empty" : "Hidden order"}</small>
              </button>

              <div className="meldfall-discard" aria-label="Public discard pile" data-animation-target="meldfall-discard">
                {view.discard.length === 0 ? (
                  <div className="meldfall-empty">No public discard cards</div>
                ) : (
                  view.discard.map((card, index) => {
                    const choice = groupedChoices.draw.find((candidate) => candidate.segment === `draw-discard-${index}`) ?? null;
                    const position = index === view.discard.length - 1 ? "Top discard" : `Index ${index}`;
                    // Picking up index i takes that card plus every newer card above it.
                    const takeCount = view.discard.length - index;
                    const hint = choice
                      ? `Take ${takeCount} · use now`
                      : drawDecision
                        ? `${position} · no immediate use`
                        : position;
                    return (
                      <button
                        type="button"
                        className="meldfall-card discard"
                        key={`${card}-${index}`}
                        disabled={!canAct || !choice}
                        aria-label={choice?.accessibility_label ?? `Public discard ${cardLabel(card)}`}
                        data-testid="meldfall-discard-card"
                        onClick={() => choice && onPathSubmit?.([choice.segment])}
                      >
                        <CardFace card={card} />
                        <small>{hint}</small>
                      </button>
                    );
                  })
                )}
              </div>
            </div>
            {drawDecision && view.discard.length ? (
              <p className="meldfall-zone-hint">
                Taking a discard also takes every newer card above it, and the card you choose must be melded or
                laid off this turn before you can finish.
              </p>
            ) : null}
            {stockEmpty ? (
              <p className="meldfall-stock-warning" role="status">
                Stock is empty. The round ends the moment the active seat has no legal draw — the discard pile is
                not reshuffled.
              </p>
            ) : stockLow ? (
              <p className="meldfall-stock-warning" role="status">
                Stock is running low ({view.stock_count} left). When no one can draw, the round ends by exhaustion
                with no reshuffle — shed high cards or aim to go out.
              </p>
            ) : null}
          </section>

          <Tableau groups={view.tableau.groups} />
        </div>

        <div className="meldfall-action-band">
          <section className="meldfall-private" aria-label="Private hand">
            <div className="meldfall-section-heading">
              <span>Private hand</span>
              <strong>{privateHeading(view)}</strong>
            </div>
            {view.private_view_status === "seat" ? (
              <>
                {view.own_hand.length > 1 ? (
                  <div className="meldfall-hand-sort" role="group" aria-label="Sort your hand">
                    <span>Sort</span>
                    <button
                      type="button"
                      className={handSort === "suit" ? "active" : ""}
                      aria-pressed={handSort === "suit"}
                      onClick={() => setHandSort("suit")}
                    >
                      By suit
                    </button>
                    <button
                      type="button"
                      className={handSort === "rank" ? "active" : ""}
                      aria-pressed={handSort === "rank"}
                      onClick={() => setHandSort("rank")}
                    >
                      By rank
                    </button>
                  </div>
                ) : null}
                <div className="meldfall-hand" aria-label="Your private hand">
                  {sortedHand.map((card, index) => (
                    <div className="meldfall-card private" key={`${card}-${index}`} data-testid="meldfall-private-card">
                      <CardFace card={card} />
                    </div>
                  ))}
                </div>
              </>
            ) : (
              <div className="meldfall-hidden-hand" data-testid="meldfall-private-hidden">
                <span>Hidden</span>
                <strong>Choose a seat view to see that seat's hand.</strong>
              </div>
            )}
          </section>

          <section className="meldfall-actions" aria-label="Meldfall Ledger legal actions">
            <div className="meldfall-section-heading">
              <span>Actions</span>
              <strong>{canAct ? "Available choices" : actionStatus(view, pending)}</strong>
            </div>
            {pickupCommitmentPending ? (
              <p className="meldfall-commitment-note" role="status">
                Pickup commitment: meld or lay off the discard you took before you can finish this turn.
              </p>
            ) : null}
            {turnGuidance ? (
              <p className="meldfall-phase-guide" role="status">
                {turnGuidance}
              </p>
            ) : null}
            <ActionGroup title="Draw" choices={groupedChoices.draw} canAct={canAct} onPathSubmit={onPathSubmit} />
            <ActionGroup
              title="Table"
              choices={groupedChoices.table}
              canAct={canAct}
              onPathSubmit={onPathSubmit}
              fallbackHint={tablePlayHint}
            />
            <ActionGroup title="Discard" choices={groupedChoices.discard} canAct={canAct} onPathSubmit={onPathSubmit} />
            <ActionGroup
              title="Turn"
              choices={groupedChoices.turn}
              canAct={canAct}
              onPathSubmit={onPathSubmit}
              fallbackHint={turnChoiceHint}
            />
          </section>

          <div className="meldfall-status-col">
            <div className="meldfall-latest" role="status" data-animation-target="meldfall-status">
              <span>{outcomeExplanation ? "Outcome" : feedback?.title ?? "Waiting"}</span>
              <strong>
                {outcomeExplanation
                  ? outcomeAnnouncementText(outcomeExplanation)
                  : feedback?.detail ?? "Visible state changes will update here."}
              </strong>
            </div>
            <RecentActions effects={effects} />
          </div>
        </div>
      </div>

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function SettlementSummary({
  settlement,
}: {
  settlement: MeldfallLedgerSettlementView;
}) {
  return (
    <section className="meldfall-settlement" aria-label={`Round ${settlement.round_index + 1} settlement`}>
      <div className="meldfall-section-heading">
        <span>Last round settled</span>
        <strong>Round {settlement.round_index + 1}</strong>
      </div>
      <p className="meldfall-settlement-reason">{roundEndLabel(settlement.round_end_reason)}</p>
      <p className="meldfall-settlement-note">Round delta = tabled card points minus the value of cards still held.</p>
      <div className="meldfall-settlement-grid">
        {settlement.seats.map((seat) => {
          const leads = seat.rank === 1;
          return (
            <article className={`meldfall-settlement-seat${leads ? " leads" : ""}${seat.winner ? " winner" : ""}`} key={seat.seat}>
              <header>
                <strong>{seatLabel(seat.seat)}</strong>
                <span className="meldfall-settlement-tag">{seat.winner ? "Winner" : `Rank ${seat.rank}`}</span>
              </header>
              <p className={`meldfall-settlement-delta ${seat.delta >= 0 ? "gain" : "loss"}`}>
                {seat.delta >= 0 ? `+${seat.delta}` : `${seat.delta}`}
                <span className="sr-only"> round delta</span>
              </p>
              <p className="meldfall-settlement-breakdown">
                <span>{seat.tabled_positive} tabled</span>
                <span>{seat.in_hand_penalty} held penalty</span>
              </p>
              <p className="meldfall-settlement-remaining">{seat.remaining_hand_count} cards held at settlement</p>
              <p className="meldfall-settlement-total">
                {seat.cumulative_score} <small>/ {MATCH_TARGET}</small>
              </p>
            </article>
          );
        })}
      </div>
    </section>
  );
}

// Public per-turn action kinds worth replaying. Draws (including discard pickups, shown
// as "drew N discard cards"), melds, lay-offs, and discards are all public proximity
// signals the strategy guide calls out (ML-VIS-001). Round-score/terminal effects are
// omitted here because the persistent settlement and outcome panels already carry them.
const RECENT_ACTION_KINDS = new Set(["draw", "stock_draw_private", "meld", "lay_off", "discard"]);

function RecentActions({ effects }: { effects: EffectEntry[] }) {
  // A short feed of recent public moves so the player can review what each opponent did
  // between their own turns — the single latest-status line cannot show a full go-around.
  // The shell already viewer-filters these effects, and we render the same public-safe
  // copy used by the latest-status line, so no hidden card identities are exposed.
  const recent = effects
    .filter((entry) => RECENT_ACTION_KINDS.has(String(entry.effect.payload.type ?? entry.effect.payload.kind ?? "")))
    .slice(-6)
    .reverse();
  if (recent.length === 0) {
    return null;
  }
  return (
    <section className="meldfall-log" aria-label="Recent public actions">
      <div className="meldfall-section-heading">
        <span>Recent actions</span>
        <strong>Public moves</strong>
      </div>
      <ol className="meldfall-log-list">
        {recent.map((entry, index) => (
          <li key={index}>{feedbackForEffect(entry).detail}</li>
        ))}
      </ol>
    </section>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div className="meldfall-metric">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function SeatLedger({
  view,
  seat,
  index,
  nextSeat,
}: {
  view: MeldfallLedgerPublicView;
  seat: MeldfallLedgerSeatId;
  index: number;
  nextSeat: MeldfallLedgerSeatId | null;
}) {
  const handCount = view.hand_counts[index] ?? 0;
  // Public go-out threat: a seat one or two cards from empty can end the round soon,
  // settling everyone's in-hand penalties. Hand counts are public (ML-VIS-001), so
  // flagging the threat surfaces a legal proximity signal the strategy guide calls out.
  const nearGoOut = !view.terminal && handCount > 0 && handCount <= 2;
  const isNext = nextSeat === seat;
  return (
    <article
      className={`meldfall-seat${view.active_seat === seat ? " active" : ""}${view.dealer === seat ? " dealer" : ""}${
        isNext ? " next" : ""
      }${nearGoOut ? " near-goout" : ""}`}
    >
      <header>
        <strong>{seatLabel(seat)}</strong>
        <span>{view.active_seat === seat ? "Turn" : isNext ? "Up next" : view.dealer === seat ? "Dealer" : "Seat"}</span>
      </header>
      <dl>
        <div>
          <dt>Hand</dt>
          <dd>{handCount}</dd>
        </div>
        <div>
          <dt>Score</dt>
          <dd>{view.cumulative_scores[index] ?? 0}</dd>
        </div>
        <div>
          <dt>Tabled</dt>
          <dd>{view.round_played_scores[index] ?? 0}</dd>
        </div>
      </dl>
      {nearGoOut ? <p className="meldfall-goout-flag">Near go-out</p> : null}
    </article>
  );
}

function Tableau({ groups }: { groups: MeldfallLedgerMeldGroupView[] }) {
  return (
    <section className="meldfall-tableau" aria-label="Public meld tableau" data-animation-target="meldfall-tableau">
      <div className="meldfall-section-heading">
        <span>Tableau</span>
        <strong>{groups.length ? `${groups.length} public groups` : "No melds tabled"}</strong>
      </div>
      {groups.length === 0 ? (
        <div className="meldfall-empty">Public meld groups will appear here after Rust accepts table plays.</div>
      ) : (
        <div className="meldfall-groups">
          {groups.map((group) => (
            <article className="meldfall-group" key={group.id}>
              <header>
                <strong>{group.kind === "run" ? "Run" : group.kind === "set" ? "Set" : titleCase(group.kind)}</strong>
                <span>
                  {group.id} by {seatLabel(group.origin_seat)}
                </span>
              </header>
              <div className="meldfall-group-cards" aria-label={`${group.id} public cards`}>
                {group.cards.map((card, index) => (
                  <TableCard card={card} key={`${group.id}-${card.card}-${index}`} />
                ))}
              </div>
            </article>
          ))}
        </div>
      )}
    </section>
  );
}

function TableCard({ card }: { card: MeldfallLedgerTableCardView }) {
  return (
    <div className="meldfall-card tabled" data-testid="meldfall-table-card">
      <CardFace card={card.card} />
      <small>
        Played by {seatLabel(card.played_by)}
        {card.score_credit_owner !== card.played_by ? `; credit ${seatLabel(card.score_credit_owner)}` : ""}
      </small>
    </div>
  );
}

// Rust action labels carry terse card codes (e.g. "Meld new 2C 3C 4C", "Discard QH").
// Render those tokens with the same coloured suit glyphs used everywhere else so the
// action buttons read like cards. Visual only: the spoken accessibility_label stays
// the Rust string, and clicks still submit choice.segment unchanged.
const SUIT_LETTER: Record<string, { glyph: string; red: boolean }> = {
  C: { glyph: "♣", red: false },
  D: { glyph: "♦", red: true },
  H: { glyph: "♥", red: true },
  S: { glyph: "♠", red: false },
};
const CARD_CODE = /^(10|[2-9JQKA])([CDHS])$/;

function renderActionLabel(label: string) {
  return label.split(" ").map((token, index) => {
    const prefix = index === 0 ? "" : " ";
    const match = CARD_CODE.exec(token);
    const suit = match ? SUIT_LETTER[match[2]] : undefined;
    if (!match || !suit) {
      return <span key={index}>{`${prefix}${token}`}</span>;
    }
    return (
      <span key={index} className={`meldfall-action-card ${suit.red ? "red" : "black"}`}>
        {`${prefix}${match[1]}${suit.glyph}`}
      </span>
    );
  });
}

function ActionGroup({
  title,
  choices,
  canAct,
  onPathSubmit,
  fallbackHint,
}: {
  title: string;
  choices: ActionChoice[];
  canAct: boolean;
  onPathSubmit?: (path: string[]) => void;
  fallbackHint?: (choice: ActionChoice) => string | null;
}) {
  if (choices.length === 0) {
    return null;
  }

  return (
    <section className="meldfall-action-group" aria-label={`${title} choices`}>
      <h3>{title}</h3>
      <div className="meldfall-action-grid">
        {choices.map((choice, index) => {
          const hint = choice.presentation?.helper_text ?? fallbackHint?.(choice) ?? null;
          return (
            <button
              type="button"
              className="meldfall-action"
              key={choice.segment}
              disabled={!canAct}
              aria-label={choice.accessibility_label}
              data-testid={`meldfall-action-${title.toLowerCase()}-${index}`}
              onClick={() => onPathSubmit?.([choice.segment])}
            >
              <strong>{renderActionLabel(choice.label)}</strong>
              {hint ? <small>{hint}</small> : null}
            </button>
          );
        })}
      </div>
    </section>
  );
}

// Plain-language explanation of the active seat's current step, derived only from the
// public phase and Rust-offered action kinds (no legality decided here, ML-UI-001).
// The turn runs draw -> table -> discard; in the table phase "Finish turn" advances to
// a mandatory discard, and "Go out" ends the round, neither of which the bare button
// labels make obvious.
function turnGuidanceText(phase: string, goOutAvailable: boolean, stockEmpty: boolean): string | null {
  if (phase === "draw") {
    // The stock pile is gone, so only a discard-pile pickup can start the turn — guiding
    // the player to a disabled stock button would be misleading.
    if (stockEmpty) {
      return "Stock is empty — pick up a usable card from the discard pile (highlighted) to start your turn.";
    }
    return "Start your turn: draw from the hidden stock, or pick up a card from the discard pile.";
  }
  if (phase === "table") {
    if (goOutAvailable) {
      return "Your hand is empty after tabling. Go out to end the round and settle every seat's held-card penalties now, or finish your turn.";
    }
    return "Optional: table new melds or lay off onto public melds. Then choose Finish turn to move on to your discard.";
  }
  if (phase === "discard") {
    return "Choose one card to discard. Discarding ends your turn.";
  }
  return null;
}

// Per-button clarification for the turn-control choices, whose Rust labels are terse.
function turnChoiceHint(choice: ActionChoice): string | null {
  if (choice.segment === "finish-turn") {
    return "Stop tabling and go to your discard.";
  }
  if (choice.segment === "go-out-without-discard") {
    return "End the round now; all seats settle held-card penalties.";
  }
  return null;
}

// Compact rank -> point value for the card codes carried in action labels (e.g. "4D",
// "10S", "AH"). Card values are constant in every scoring context (ML-SCORE-001), so a
// table play scores exactly the sum of its card values regardless of meld shape.
const SHORT_RANK_VALUE: Record<string, number> = {
  "2": 2,
  "3": 3,
  "4": 4,
  "5": 5,
  "6": 6,
  "7": 7,
  "8": 8,
  "9": 9,
  "10": 10,
  J: 10,
  Q: 10,
  K: 10,
  A: 15,
};

// Points a meld or lay-off button would score, summed from the card codes in its label.
// Surfaces the immediate tabled-score value at the decision point — the "table points
// now / shed penalty" trade-off the strategy guide centres on. Presentation-only readout
// of already-authorized card identities; Rust stays the scoring authority (ML-UI-001).
function tablePlayHint(choice: ActionChoice): string | null {
  let sum = 0;
  let cards = 0;
  for (const token of choice.label.split(" ")) {
    const match = CARD_CODE.exec(token);
    if (match) {
      sum += SHORT_RANK_VALUE[match[1]] ?? 0;
      cards += 1;
    }
  }
  if (cards === 0) {
    return null;
  }
  return `Scores +${sum} ${sum === 1 ? "point" : "points"}`;
}

const SUIT_GLYPH: Record<string, string> = {
  Spades: "♠",
  Hearts: "♥",
  Diamonds: "♦",
  Clubs: "♣",
};

const RANK_SHORT: Record<string, string> = {
  Two: "2",
  Three: "3",
  Four: "4",
  Five: "5",
  Six: "6",
  Seven: "7",
  Eight: "8",
  Nine: "9",
  Ten: "10",
  Jack: "J",
  Queen: "Q",
  King: "K",
  Ace: "A",
};

// Constant Meldfall Ledger card values under meldfall-ledger-rules-v1 (ML-SCORE-001):
// ace = 15; king, queen, jack, ten = 10; ranks 2-9 = pip value. These values are
// constant in every scoring context (a low ace in a run still scores 15), so this is
// a presentation-only readout of the viewer's already-authorized card identities.
// Rust remains the scoring authority and TypeScript decides no legality (ML-UI-001).
const RANK_VALUE: Record<string, number> = {
  Two: 2,
  Three: 3,
  Four: 4,
  Five: 5,
  Six: 6,
  Seven: 7,
  Eight: 8,
  Nine: 9,
  Ten: 10,
  Jack: 10,
  Queen: 10,
  King: 10,
  Ace: 15,
};

function cardValue(card: string): number {
  return RANK_VALUE[parseCard(card).rank] ?? 0;
}

// Display ordering for the viewer's own hand. Purely visual — actions are separate
// buttons keyed by Rust segment, so reordering the hand rail never changes which plays
// are legal. "By suit" groups runs together; "by rank" groups sets together, so the
// player can organize for whichever meld they are hunting instead of reading the raw
// deal/draw order. Ace sorts high.
type HandSort = "suit" | "rank";

const SUIT_ORDER: Record<string, number> = { Clubs: 0, Diamonds: 1, Hearts: 2, Spades: 3 };
const RANK_ORDER: Record<string, number> = {
  Two: 0,
  Three: 1,
  Four: 2,
  Five: 3,
  Six: 4,
  Seven: 5,
  Eight: 6,
  Nine: 7,
  Ten: 8,
  Jack: 9,
  Queen: 10,
  King: 11,
  Ace: 12,
};

function suitRank(card: { suit: string }): number {
  return SUIT_ORDER[card.suit] ?? 9;
}

function rankOrder(card: { rank: string }): number {
  return RANK_ORDER[card.rank] ?? 99;
}

function compareHandCards(a: string, b: string, sort: HandSort): number {
  const left = parseCard(a);
  const right = parseCard(b);
  if (sort === "rank") {
    return rankOrder(left) - rankOrder(right) || suitRank(left) - suitRank(right);
  }
  return suitRank(left) - suitRank(right) || rankOrder(left) - rankOrder(right);
}

function CardFace({ card }: { card: string }) {
  const parsed = parseCard(card);
  const glyph = SUIT_GLYPH[parsed.suit];
  const isRed = parsed.suit === "Hearts" || parsed.suit === "Diamonds";
  const rankShort = RANK_SHORT[parsed.rank] ?? parsed.rank;
  const value = RANK_VALUE[parsed.rank];
  return (
    <span className={`meldfall-face ${isRed ? "red" : "black"}`}>
      <span className="sr-only">
        {value === undefined
          ? `${parsed.rank} of ${parsed.suit}`
          : `${parsed.rank} of ${parsed.suit}, worth ${value} ${value === 1 ? "point" : "points"}`}
      </span>
      <strong className="meldfall-rank" aria-hidden="true">
        {rankShort}
      </strong>
      <span className="meldfall-suit" aria-hidden="true">
        {glyph ?? parsed.suit}
      </span>
      {value === undefined ? null : (
        <span className="meldfall-value" aria-hidden="true">
          {value}
        </span>
      )}
    </span>
  );
}

function groupChoices(choices: ActionChoice[]): GroupedChoices {
  return choices.reduce<GroupedChoices>(
    (groups, choice) => {
      if (choice.segment.startsWith("draw-")) groups.draw.push(choice);
      else if (choice.segment.startsWith("meld-new-") || choice.segment.startsWith("lay-off-")) groups.table.push(choice);
      else if (choice.segment.startsWith("discard-")) groups.discard.push(choice);
      else groups.turn.push(choice);
      return groups;
    },
    { draw: [], table: [], discard: [], turn: [] },
  );
}

function statusLabel(view: MeldfallLedgerPublicView): string {
  if (view.terminal) return terminalHeading(view.terminal.standings);
  if (view.phase === "round_settled") return "Round settled";
  return `${phaseLabel(view.phase)} phase`;
}

function roundEndLabel(roundEnd: string | null): string {
  if (!roundEnd) return "Round settled";
  const [reason, seatPart] = roundEnd.split(":");
  const seatMatch = /seat=(\d+)/.exec(seatPart ?? "");
  const seat = seatMatch ? seatLabel(`seat_${seatMatch[1]}` as MeldfallLedgerSeatId) : null;
  if (reason === "stock_exhausted") return "Stock exhausted";
  if ((reason === "go_out_without_discard" || reason === "go_out_by_final_discard") && seat) {
    return `${seat} went out`;
  }
  return "Round settled";
}

function actionStatus(view: MeldfallLedgerPublicView, pending: boolean): string {
  if (pending) return "Applying action";
  if (view.terminal) return "Match complete";
  return "No actions available";
}

function privateHeading(view: MeldfallLedgerPublicView): string {
  if (view.private_view_status === "seat") {
    // In-hand penalty if this round settled now: each held card subtracts its value
    // (ML-SCORE-003). Presentation aggregation of the viewer's own authorized cards;
    // opponent penalties stay hidden because only this seat's hand is visible.
    const penalty = view.own_hand.reduce((sum, card) => sum + cardValue(card), 0);
    return `${view.own_hand.length} cards · ${penalty} penalty if held`;
  }
  return `${view.hand_counts.join(" / ")} public counts`;
}

function terminalHeading(standings: MeldfallLedgerStandingView[]): string {
  const winner = winnerStanding(standings);
  return winner ? `${seatLabel(winner.seat)} wins` : "Match complete";
}

function winnerStanding(standings: MeldfallLedgerStandingView[]): MeldfallLedgerStandingView | null {
  return standings.find((standing) => standing.winner) ?? null;
}

function phaseLabel(phase: string): string {
  return titleCase(phase.replaceAll("_", " "));
}

function cardLabel(card: string): string {
  const parsed = parseCard(card);
  return `${parsed.rank} ${parsed.suit}`.trim();
}

function parseCard(card: string): { rank: string; suit: string } {
  const parts = card.replaceAll("-", "_").split("_").filter(Boolean);
  if (parts.length >= 2) {
    return { rank: titleCase(parts.slice(0, -1).join(" ")), suit: titleCase(parts.at(-1) ?? "") };
  }
  return { rank: card.toUpperCase(), suit: "Card" };
}

function titleCase(value: string): string {
  return value.replace(/\b[a-z]/g, (letter) => letter.toUpperCase());
}

function seatLabel(seat: MeldfallLedgerSeatId): string {
  const index = Number(seat.split("_")[1]);
  return Number.isFinite(index) ? `Seat ${index + 1}` : seat;
}
