import type { CardFaceView } from "../wasm/client";

type DeckFlowPanelProps = {
  label: string;
  currentLabel: string;
  nextLabel: string;
  discardLabel: string;
  faceDownLabel: string;
  faceDownSummary: string;
  current: CardFaceView | null;
  next: CardFaceView | null;
  discard: CardFaceView[];
  faceDownCount?: number | null;
};

export function DeckFlowPanel({
  label,
  currentLabel,
  nextLabel,
  discardLabel,
  faceDownLabel,
  faceDownSummary,
  current,
  next,
  discard,
  faceDownCount,
}: DeckFlowPanelProps) {
  const hasFaceDownCount = typeof faceDownCount === "number";

  return (
    <section className="deck-flow-panel" aria-label={label} data-testid="deck-flow-panel">
      <div className="deck-flow-heading">
        <span>{label}</span>
        <strong>
          {discard.length} {discardLabel.toLowerCase()}
        </strong>
      </div>
      <div className="deck-flow-slots">
        <CardSlot label={currentLabel} card={current} testId="deck-current-card" />
        <CardSlot label={nextLabel} card={next} testId="deck-next-card" />
        <section className="deck-flow-slot deck-flow-face-down" aria-label={faceDownLabel} data-testid="deck-face-down">
          <div className="deck-flow-slot-kicker">
            <span className="deck-flow-icon" aria-hidden="true">
              ?
            </span>
            <span>{faceDownLabel}</span>
            {hasFaceDownCount ? (
              <strong className="deck-flow-count" data-testid="deck-face-down-count">
                {faceDownCount}
              </strong>
            ) : null}
          </div>
          <p>{faceDownSummary}</p>
        </section>
      </div>
      <details className="deck-flow-discard" data-testid="deck-discard">
        <summary>
          <span>{discardLabel}</span>
          <strong>{discard.length}</strong>
        </summary>
        {discard.length ? (
          <ol>
            {discard.map((card) => (
              <li key={card.id} data-testid="deck-discard-card">
                <MiniCard card={card} />
              </li>
            ))}
          </ol>
        ) : (
          <p className="muted">None</p>
        )}
      </details>
    </section>
  );
}

function CardSlot({ label, card, testId }: { label: string; card: CardFaceView | null; testId: string }) {
  return (
    <section className="deck-flow-slot" aria-label={card?.accessibility_label ?? label} data-testid={testId}>
      <div className="deck-flow-slot-kicker">
        <span className="deck-flow-icon" aria-hidden="true">
          {card ? "!" : "-"}
        </span>
        <span>{label}</span>
      </div>
      {card ? <MiniCard card={card} /> : <p className="muted">None</p>}
    </section>
  );
}

function MiniCard({ card }: { card: CardFaceView }) {
  return (
    <article className="deck-flow-card" aria-label={card.accessibility_label} data-card-family={card.family}>
      <span>{card.family}</span>
      <strong>{card.label}</strong>
      <p>{card.summary}</p>
      <details className="deck-flow-card-details">
        <summary>Details</summary>
        <p>{card.summary}</p>
      </details>
    </article>
  );
}
