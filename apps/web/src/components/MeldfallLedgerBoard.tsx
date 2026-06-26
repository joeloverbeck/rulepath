import { useEffect, useMemo, useState } from "react";
import type {
  ActionChoice,
  ActionTree,
  EffectEntry,
  MeldfallLedgerMeldGroupView,
  MeldfallLedgerPublicView,
  MeldfallLedgerSeatId,
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

// Summary of the most recently settled round, captured from the public `round_score`
// effect. The shell only keeps the last 12 effects, and bots auto-deal the next round,
// so the settlement feedback scrolls out of the status line before a human can read it.
// Holding it in board state keeps last round's public deltas/cumulative scores visible
// (ML-VIS-006 exposes these) until the next round settles. Presentation only.
type RoundSettlement = {
  cursor: number;
  roundNumber: number;
  deltas: number[];
  cumulative: number[];
};

function effectKind(payload: Record<string, unknown>): string {
  return String(payload.type ?? payload.kind ?? "");
}

function numberList(value: unknown): number[] {
  return Array.isArray(value) ? value.map((entry) => Number(entry)).filter((entry) => Number.isFinite(entry)) : [];
}

function parseRoundSettlement(entry: EffectEntry): RoundSettlement | null {
  const payload = entry.effect.payload;
  if (effectKind(payload) !== "round_score") return null;
  const deltas = numberList(payload.deltas);
  const cumulative = numberList(payload.cumulative_scores);
  if (deltas.length === 0 && cumulative.length === 0) return null;
  const roundIndex = typeof payload.round_index === "number" ? payload.round_index : null;
  return {
    cursor: entry.cursor,
    roundNumber: roundIndex === null ? 0 : roundIndex + 1,
    deltas,
    cumulative,
  };
}

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

  // Persist the latest settled-round summary across renders. A new match restarts the
  // effect cursor low, so when the buffer's newest cursor drops below what we stored we
  // discard the stale summary instead of carrying it into the next match.
  const [settlement, setSettlement] = useState<RoundSettlement | null>(null);
  useEffect(() => {
    const newestCursor = effects.reduce((max, entry) => Math.max(max, entry.cursor), -1);
    if (settlement && newestCursor >= 0 && newestCursor < settlement.cursor) {
      setSettlement(null);
      return;
    }
    for (let index = effects.length - 1; index >= 0; index -= 1) {
      const parsed = parseRoundSettlement(effects[index]);
      if (parsed) {
        if (!settlement || parsed.cursor > settlement.cursor) setSettlement(parsed);
        break;
      }
    }
  }, [effects, settlement]);
  const seats = SEATS.slice(0, view.hand_counts.length);
  const canAct = Boolean(interactive && !pending && !view.terminal && choices.length > 0);
  // A discard pickup is offered only when its chosen card has an immediate legal use
  // this turn; Rust omits the choice otherwise (ML-TURN-004). When this seat is making
  // a live draw decision we can explain why a disabled discard is not pickable.
  const drawDecision = canAct && !view.terminal && view.phase === "draw";
  const roundSettled = !view.terminal && view.phase === "round_settled";
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

      {settlement && !view.terminal ? <SettlementSummary settlement={settlement} seats={seats} /> : null}

      <div className="meldfall-table-shell" aria-label="Meldfall Ledger table">
        <section className="meldfall-seat-rail" aria-label="Seat score ledger">
          {seats.map((seat, index) => (
            <SeatLedger key={seat} view={view} seat={seat} index={index} />
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
                className="meldfall-stock"
                disabled={!canAct || !drawStockChoice}
                aria-label={drawStockChoice?.accessibility_label ?? `${view.stock_count} hidden stock cards`}
                data-testid="meldfall-stock"
                onClick={() => drawStockChoice && onPathSubmit?.([drawStockChoice.segment])}
              >
                <span>Stock</span>
                <strong>{view.stock_count}</strong>
                <small>{drawStockChoice ? "Draw from stock" : "Hidden order"}</small>
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
            <ActionGroup title="Draw" choices={groupedChoices.draw} canAct={canAct} onPathSubmit={onPathSubmit} />
            <ActionGroup title="Table" choices={groupedChoices.table} canAct={canAct} onPathSubmit={onPathSubmit} />
            <ActionGroup title="Discard" choices={groupedChoices.discard} canAct={canAct} onPathSubmit={onPathSubmit} />
            <ActionGroup title="Turn" choices={groupedChoices.turn} canAct={canAct} onPathSubmit={onPathSubmit} />
          </section>

          <div className="meldfall-latest" role="status" data-animation-target="meldfall-status">
            <span>{outcomeExplanation ? "Outcome" : feedback?.title ?? "Waiting"}</span>
            <strong>
              {outcomeExplanation
                ? outcomeAnnouncementText(outcomeExplanation)
                : feedback?.detail ?? "Visible state changes will update here."}
            </strong>
          </div>
        </div>
      </div>

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function SettlementSummary({
  settlement,
  seats,
}: {
  settlement: RoundSettlement;
  seats: MeldfallLedgerSeatId[];
}) {
  const topScore = settlement.cumulative.length ? Math.max(...settlement.cumulative) : null;
  return (
    <section className="meldfall-settlement" aria-label={`Round ${settlement.roundNumber} settlement`}>
      <div className="meldfall-section-heading">
        <span>Last round settled</span>
        <strong>Round {settlement.roundNumber} · tabled points minus cards held</strong>
      </div>
      <div className="meldfall-settlement-grid">
        {seats.map((seat, index) => {
          const delta = settlement.deltas[index] ?? 0;
          const total = settlement.cumulative[index] ?? 0;
          const leads = topScore !== null && total === topScore;
          return (
            <article className={`meldfall-settlement-seat${leads ? " leads" : ""}`} key={seat}>
              <header>
                <strong>{seatLabel(seat)}</strong>
                {leads ? <span className="meldfall-settlement-tag">Leads</span> : null}
              </header>
              <p className={`meldfall-settlement-delta ${delta >= 0 ? "gain" : "loss"}`}>
                {delta >= 0 ? `+${delta}` : `${delta}`}
                <span className="sr-only"> round delta</span>
              </p>
              <p className="meldfall-settlement-total">
                {total} <small>/ {MATCH_TARGET}</small>
              </p>
            </article>
          );
        })}
      </div>
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

function SeatLedger({ view, seat, index }: { view: MeldfallLedgerPublicView; seat: MeldfallLedgerSeatId; index: number }) {
  const handCount = view.hand_counts[index] ?? 0;
  // Public go-out threat: a seat one or two cards from empty can end the round soon,
  // settling everyone's in-hand penalties. Hand counts are public (ML-VIS-001), so
  // flagging the threat surfaces a legal proximity signal the strategy guide calls out.
  const nearGoOut = !view.terminal && handCount > 0 && handCount <= 2;
  return (
    <article
      className={`meldfall-seat${view.active_seat === seat ? " active" : ""}${view.dealer === seat ? " dealer" : ""}${
        nearGoOut ? " near-goout" : ""
      }`}
    >
      <header>
        <strong>{seatLabel(seat)}</strong>
        <span>{view.active_seat === seat ? "Turn" : view.dealer === seat ? "Dealer" : "Seat"}</span>
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
}: {
  title: string;
  choices: ActionChoice[];
  canAct: boolean;
  onPathSubmit?: (path: string[]) => void;
}) {
  if (choices.length === 0) {
    return null;
  }

  return (
    <section className="meldfall-action-group" aria-label={`${title} choices`}>
      <h3>{title}</h3>
      <div className="meldfall-action-grid">
        {choices.map((choice, index) => (
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
            {choice.presentation?.helper_text ? <small>{choice.presentation.helper_text}</small> : null}
          </button>
        ))}
      </div>
    </section>
  );
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
