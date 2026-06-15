import type { RiverLedgerCardView } from "../wasm/client";

export type RiverLedgerCardLike = Pick<
  RiverLedgerCardView,
  "card_id" | "rank" | "rank_value" | "suit" | "label" | "accessibility_label"
>;

type RiverLedgerCardProps = {
  card: RiverLedgerCardLike;
  tone: "board" | "private" | "showdown";
  className?: string;
};

const suitGlyphs: Record<string, string> = {
  clubs: "♣",
  diamonds: "♦",
  hearts: "♥",
  spades: "♠",
};

export function RiverLedgerCard({ card, tone, className = "" }: RiverLedgerCardProps) {
  const suit = card.suit.toLowerCase();
  const glyph = suitGlyphs[suit] ?? card.suit.slice(0, 1).toUpperCase();
  const classes = ["river-ledger-card", tone, `suit-${suit}`, className].filter(Boolean).join(" ");

  return (
    <div className={classes} aria-label={card.accessibility_label}>
      <strong>{card.label}</strong>
      <span className="river-ledger-card-suit">
        <b aria-hidden="true">{glyph}</b>
        <small>{card.suit}</small>
      </span>
      <span className="river-ledger-card-rank">{card.rank}</span>
    </div>
  );
}

export function riverLedgerCardGroupLabel(cards: readonly RiverLedgerCardLike[], fallback: string): string {
  if (cards.length === 0) {
    return fallback;
  }
  return `${fallback}: ${cards.map((card) => card.accessibility_label).join(", ")}`;
}
