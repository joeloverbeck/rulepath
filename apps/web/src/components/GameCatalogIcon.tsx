type GameCatalogIconProps = {
  gameId: string;
  title?: string;
  decorative?: boolean;
  className?: string;
};

type IconProps = {
  titleId?: string;
};

type IconComponent = (props: IconProps) => ReactElement;

const ICONS: Record<string, IconComponent> = {
  "race_to_n": RaceToNIcon,
  "three_marks": ThreeMarksIcon,
  "column_four": ColumnFourIcon,
  "directional_flip": DirectionalFlipIcon,
  "draughts_lite": DraughtsLiteIcon,
  "high_card_duel": HighCardDuelIcon,
  "token_bazaar": TokenBazaarIcon,
  "secret_draft": SecretDraftIcon,
  "poker_lite": PokerLiteIcon,
  "river_ledger": RiverLedgerIcon,
  "plain_tricks": PlainTricksIcon,
  "briar_circuit": BriarCircuitIcon,
  "vow_tide": VowTideIcon,
  "meldfall_ledger": MeldfallLedgerIcon,
  "masked_claims": MaskedClaimsIcon,
  "flood_watch": FloodWatchIcon,
  "frontier_control": FrontierControlIcon,
  "event_frontier": EventFrontierIcon,
};

export function GameCatalogIcon({ gameId, title, decorative = true, className }: GameCatalogIconProps) {
  const Icon = ICONS[gameId] ?? FallbackIcon;
  const titleId = decorative ? undefined : `catalog-icon-${gameId}`;
  return (
    <svg
      className={className}
      data-icon-game={gameId}
      viewBox="0 0 24 24"
      fill="none"
      aria-hidden={decorative ? "true" : undefined}
      role={decorative ? undefined : "img"}
      aria-labelledby={decorative ? undefined : titleId}
      focusable="false"
    >
      {decorative ? null : <title id={titleId}>{title ?? "Game identity"}</title>}
      <Icon titleId={titleId} />
    </svg>
  );
}

function AccentCircle({ cx, cy, r }: { cx: number; cy: number; r: number }) {
  return <circle cx={cx} cy={cy} r={r} fill="var(--game-accent-2)" opacity="0.88" />;
}

function RaceToNIcon() {
  return (
    <>
      <path d="M4 18h4v-4h4v-4h4V6h4" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      <path d="M4 20h16" stroke="var(--game-card-art-line)" strokeWidth="1.75" strokeLinecap="round" opacity="0.75" />
      <AccentCircle cx={18} cy={6} r={2} />
    </>
  );
}

function ThreeMarksIcon() {
  return (
    <>
      <path d="M5 5h14v14H5z" stroke="currentColor" strokeWidth="1.75" />
      <path d="M9.5 5v14M14.5 5v14M5 9.5h14M5 14.5h14" stroke="var(--game-card-art-line)" strokeWidth="1.25" opacity="0.65" />
      <path d="M8 8l2.5 2.5M10.5 8L8 10.5M13.5 13.5l2.5 2.5M16 13.5L13.5 16" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" />
      <AccentCircle cx={16} cy={8} r={1.6} />
    </>
  );
}

function ColumnFourIcon() {
  return (
    <>
      <path d="M7 4h10v16H7z" stroke="currentColor" strokeWidth="1.75" strokeLinejoin="round" />
      {[6.5, 10.2, 13.8, 17.5].map((cy, index) => (
        <circle key={cy} cx="12" cy={cy} r="1.75" fill={index % 2 === 0 ? "var(--game-accent-2)" : "currentColor"} opacity={index % 2 === 0 ? 0.88 : 0.72} />
      ))}
      <path d="M5 20h14" stroke="var(--game-card-art-line)" strokeWidth="1.75" strokeLinecap="round" opacity="0.75" />
    </>
  );
}

function DirectionalFlipIcon() {
  return (
    <>
      <path d="M6 12a6 6 0 0 1 10-4.5" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <path d="M18 12a6 6 0 0 1-10 4.5" stroke="var(--game-card-art-line)" strokeWidth="2" strokeLinecap="round" />
      <path d="M15.5 4.5H19v3.5" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      <path d="M8.5 19.5H5v-3.5" stroke="var(--game-card-art-line)" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      <AccentCircle cx={12} cy={12} r={2.1} />
    </>
  );
}

function DraughtsLiteIcon() {
  return (
    <>
      <path d="M5 6h14M5 12h14M5 18h14M6 5v14M12 5v14M18 5v14" stroke="var(--game-card-art-line)" strokeWidth="1.15" opacity="0.6" />
      <circle cx="8" cy="16" r="2.4" fill="currentColor" opacity="0.78" />
      <circle cx="16" cy="8" r="2.4" fill="var(--game-accent-2)" opacity="0.88" />
      <path d="M8 16l8-8" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" />
    </>
  );
}

function HighCardDuelIcon() {
  return (
    <>
      <rect x="5" y="7" width="6" height="10" rx="1.4" stroke="currentColor" strokeWidth="1.75" />
      <rect x="13" y="7" width="6" height="10" rx="1.4" stroke="var(--game-card-art-line)" strokeWidth="1.75" />
      <path d="M8 14l1.4-4h.2L11 14M8.5 12.5h2" stroke="currentColor" strokeWidth="1.15" strokeLinecap="round" strokeLinejoin="round" />
      <AccentCircle cx={16} cy={12} r={1.7} />
    </>
  );
}

function TokenBazaarIcon() {
  return (
    <>
      <path d="M5 9h14M7 5h10l2 4-7 10L5 9z" stroke="currentColor" strokeWidth="1.75" strokeLinejoin="round" />
      <path d="M9 9l3 10M15 9l-3 10" stroke="var(--game-card-art-line)" strokeWidth="1.35" opacity="0.7" />
      <AccentCircle cx={12} cy={9} r={1.6} />
    </>
  );
}

function SecretDraftIcon() {
  return (
    <>
      <path d="M6 8l6-3 6 3v7l-6 4-6-4z" stroke="currentColor" strokeWidth="1.75" strokeLinejoin="round" />
      <path d="M8.5 10.5h7M8.5 13h7" stroke="var(--game-card-art-line)" strokeWidth="1.5" strokeLinecap="round" opacity="0.75" />
      <path d="M12 5v14" stroke="currentColor" strokeWidth="1.25" opacity="0.35" />
      <AccentCircle cx={12} cy={16} r={1.6} />
    </>
  );
}

function PokerLiteIcon() {
  return (
    <>
      <rect x="5" y="8" width="5.8" height="8.5" rx="1.2" stroke="currentColor" strokeWidth="1.6" />
      <rect x="13.2" y="8" width="5.8" height="8.5" rx="1.2" stroke="var(--game-card-art-line)" strokeWidth="1.6" />
      <path d="M8 5h8M7 19h10" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" />
      <AccentCircle cx={12} cy={12.2} r={2.1} />
    </>
  );
}

function RiverLedgerIcon() {
  return (
    <>
      <path d="M5.4 8.7l4.4-2.2 2.8 8.4-4.4 2.2z" stroke="currentColor" strokeWidth="1.45" strokeLinejoin="round" />
      <path d="M10.1 6.5h5.2v9.2h-5.2z" stroke="var(--game-card-art-line)" strokeWidth="1.45" strokeLinejoin="round" />
      <path d="M14.2 7l4.4 2.2-2.8 8.2-4.4-2.1z" stroke="currentColor" strokeWidth="1.45" strokeLinejoin="round" />
      <path d="M5.2 18.7c3.4-1.3 5.8-1.2 8.2.1 1.7.9 3.3.9 5.4-.1" stroke="var(--game-card-art-line)" strokeWidth="1.6" strokeLinecap="round" />
      <path d="M7.2 19.4v-2.1M10.1 19.7v-2.4M13 20v-2.5M15.9 20v-2.2" stroke="currentColor" strokeWidth="1.05" strokeLinecap="round" opacity="0.82" />
      <AccentCircle cx={12.4} cy={11.2} r={1.45} />
    </>
  );
}

function PlainTricksIcon() {
  return (
    <>
      <path d="M6 7h5v10H6zM13 7h5v10h-5z" stroke="currentColor" strokeWidth="1.65" strokeLinejoin="round" />
      <path d="M8.5 11h7M15.5 11l-2-2M15.5 11l-2 2" stroke="var(--game-card-art-line)" strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round" />
      <AccentCircle cx={12} cy={17} r={1.5} />
    </>
  );
}

function BriarCircuitIcon() {
  return (
    <>
      <path d="M12 4l6 4v8l-6 4-6-4V8z" stroke="currentColor" strokeWidth="1.65" strokeLinejoin="round" />
      <path d="M8.2 9.3h7.6M8.2 12h7.6M8.2 14.7h7.6" stroke="var(--game-card-art-line)" strokeWidth="1.35" strokeLinecap="round" />
      <path d="M12 4v16M6 8l12 8M18 8L6 16" stroke="currentColor" strokeWidth="1.05" opacity="0.42" />
      <AccentCircle cx={12} cy={12} r={1.7} />
    </>
  );
}

function VowTideIcon() {
  return (
    <>
      <path d="M5 16c2.2-2 4.5-2 7 0s4.8 2 7 0" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" />
      <path d="M7 6h5.2v8H7zM12.8 6H18v8h-5.2z" stroke="currentColor" strokeWidth="1.45" strokeLinejoin="round" />
      <path d="M9.6 9.8h5M15 9.8l-1.5-1.5M15 9.8l-1.5 1.5" stroke="var(--game-card-art-line)" strokeWidth="1.45" strokeLinecap="round" strokeLinejoin="round" />
      <path d="M6 19c3-1.1 5-1.1 7 0 1.6.9 3.2.9 5 0" stroke="var(--game-card-art-line)" strokeWidth="1.45" strokeLinecap="round" />
      <AccentCircle cx={12} cy={15.5} r={1.55} />
    </>
  );
}

function MeldfallLedgerIcon() {
  return (
    <>
      <rect x="5.4" y="5.5" width="5.2" height="8.4" rx="1.1" stroke="currentColor" strokeWidth="1.45" />
      <rect x="11.2" y="5.5" width="5.2" height="8.4" rx="1.1" stroke="var(--game-card-art-line)" strokeWidth="1.45" />
      <rect x="7.8" y="15.5" width="8.4" height="3.2" rx="1" stroke="currentColor" strokeWidth="1.45" />
      <path d="M6.8 18.8h10.4M8 8.2h1.8M12.6 8.2h2" stroke="var(--game-card-art-line)" strokeWidth="1.35" strokeLinecap="round" />
      <path d="M12 13.9v1.6" stroke="currentColor" strokeWidth="1.35" strokeLinecap="round" />
      <AccentCircle cx={17.3} cy={15.7} r={1.55} />
    </>
  );
}

function MaskedClaimsIcon() {
  return (
    <>
      <path d="M6 9c2-2.7 10-2.7 12 0v4c-2.2 2.8-9.8 2.8-12 0z" stroke="currentColor" strokeWidth="1.75" strokeLinejoin="round" />
      <path d="M9 12h2M13 12h2" stroke="var(--game-card-art-line)" strokeWidth="1.8" strokeLinecap="round" />
      <path d="M10 16l2 2 2-2" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
      <AccentCircle cx={12} cy={8} r={1.35} />
    </>
  );
}

function FloodWatchIcon() {
  return (
    <>
      <path d="M5 14c2.2-2 4.5-2 7 0s4.8 2 7 0M5 18c2.2-2 4.5-2 7 0s4.8 2 7 0" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" />
      <path d="M8 10l4-6 4 6a4 4 0 0 1-8 0z" stroke="var(--game-card-art-line)" strokeWidth="1.65" strokeLinejoin="round" />
      <AccentCircle cx={12} cy={10} r={1.55} />
    </>
  );
}

function FrontierControlIcon() {
  return (
    <>
      <path d="M6 6h5l2 3h5v9H6z" stroke="currentColor" strokeWidth="1.75" strokeLinejoin="round" />
      <path d="M8 15h8M10 12h4M9 9h2" stroke="var(--game-card-art-line)" strokeWidth="1.45" strokeLinecap="round" opacity="0.8" />
      <AccentCircle cx={17} cy={7} r={1.7} />
    </>
  );
}

function EventFrontierIcon() {
  return (
    <>
      <path d="M5 16l5-10 4 7 2-4 3 7z" stroke="currentColor" strokeWidth="1.75" strokeLinejoin="round" />
      <path d="M6 19h12" stroke="var(--game-card-art-line)" strokeWidth="1.75" strokeLinecap="round" opacity="0.75" />
      <path d="M15 5h4M17 3v4" stroke="var(--game-card-art-line)" strokeWidth="1.7" strokeLinecap="round" />
      <AccentCircle cx={10} cy={16} r={1.55} />
    </>
  );
}

function FallbackIcon() {
  return (
    <>
      <path d="M6 6h12v12H6z" stroke="currentColor" strokeWidth="1.75" strokeLinejoin="round" />
      <path d="M8.5 12h7M12 8.5v7" stroke="var(--game-card-art-line)" strokeWidth="1.6" strokeLinecap="round" />
    </>
  );
}
import type { ReactElement } from "react";
