import type { SetupPlayMode } from "../state/shellReducer";
import { selectVariantDescription, type GameCatalogEntry, type SeatDisplayLabel } from "../wasm/client";
import { GameCatalogIcon } from "./GameCatalogIcon";

type MatchSetupProps = {
  selectedGame: GameCatalogEntry | null;
  seed: number;
  playMode: SetupPlayMode;
  variantId: string | null;
  canStart: boolean;
  onSeedChange: (seed: number) => void;
  onPlayModeChange: (mode: SetupPlayMode) => void;
  onVariantChange: (variantId: string) => void;
  onRulesOpen: (gameId: string) => void;
  onStart: () => void;
};

const PLAY_MODES: { value: SetupPlayMode; label: string }[] = [
  {
    value: "human_vs_bot",
    label: "Human vs bot",
  },
  {
    value: "hotseat",
    label: "Hotseat",
  },
  {
    value: "bot_vs_bot",
    label: "Bot vs bot",
  },
];

export function MatchSetup({
  selectedGame,
  seed,
  playMode,
  variantId,
  canStart,
  onSeedChange,
  onPlayModeChange,
  onVariantChange,
  onRulesOpen,
  onStart,
}: MatchSetupProps) {
  const variants = selectedGame?.variants ?? [];
  const selectedVariant = variants.find((variant) => variant.id === variantId) ?? variants[0] ?? null;
  const variantDescription = selectVariantDescription(selectedVariant);
  const seatCounts = supportedSeatCounts(selectedGame);
  const defaultSeatCount = selectedGame?.default_seats ?? seatCounts[0] ?? "";
  return (
    <section className="region setup-region" aria-labelledby="setup-heading">
      <div className="region-heading">
        <p className="eyebrow">Setup</p>
        <h2 id="setup-heading">Match setup</h2>
      </div>

      <div className="setup-hero" data-game-id={selectedGame?.game_id}>
        <div className="setup-hero-art" aria-hidden="true">
          {selectedGame ? (
            <GameCatalogIcon gameId={selectedGame.game_id} title={`${selectedGame.display_name} icon`} />
          ) : null}
        </div>
        <div className="setup-hero-copy">
          <span>Selected game</span>
          <strong>{selectedGame?.display_name ?? "No game selected"}</strong>
          <small>{selectedGame ? gameMetadata(selectedGame, selectedVariant?.label ?? null) : "Load the Rust catalog to continue"}</small>
          {selectedGame?.ui?.faction_labels?.length ? (
            <div className="faction-chips" aria-label="Factions">
              {selectedGame.ui.faction_labels.map((faction) => (
                <span key={faction.faction}>{faction.label}</span>
              ))}
            </div>
          ) : null}
        </div>
        <div className="setup-hero-actions setup-summary">
          <button
            type="button"
            className="secondary rules-trigger"
            onClick={() => selectedGame && onRulesOpen(selectedGame.game_id)}
            disabled={!selectedGame}
            aria-label={selectedGame ? `How to play ${selectedGame.display_name}` : "How to play selected game"}
          >
            How to Play / Rules
          </button>
        </div>
      </div>

      <div className="setup-grid">
        <label className="field">
          <span>Seed</span>
          <input
            type="number"
            min="1"
            step="1"
            value={seed}
            onChange={(event) => onSeedChange(Number(event.currentTarget.value) || 1)}
          />
        </label>

        {variants.length > 1 ? (
          <label className="field">
            <span>Variant</span>
            <select value={selectedVariant?.id ?? ""} onChange={(event) => onVariantChange(event.currentTarget.value)}>
              {variants.map((variant) => (
                <option value={variant.id} key={variant.id}>
                  {variant.label}
                </option>
              ))}
            </select>
            {variantDescription ? <small className="variant-description">{variantDescription}</small> : null}
          </label>
        ) : null}

        <label className="field">
          <span>Seats</span>
          <select value={defaultSeatCount} disabled aria-label="Supported seats from Rust catalog">
            {seatCounts.length ? (
              seatCounts.map((count) => (
                <option value={count} key={count}>
                  {count}
                </option>
              ))
            ) : (
              <option value="">Catalog pending</option>
            )}
          </select>
          <small>{seatCountDetail(selectedGame)}</small>
        </label>
      </div>

      <fieldset className="mode-picker">
        <legend>Mode</legend>
        {PLAY_MODES.map((mode) => (
          <label key={mode.value} className={mode.value === playMode ? "mode-option selected" : "mode-option"}>
            <input
              type="radio"
              name="play-mode"
              value={mode.value}
              checked={mode.value === playMode}
              onChange={() => onPlayModeChange(mode.value)}
            />
            <span>{mode.label}</span>
            <small>{modeDetail(mode.value, selectedGame)}</small>
          </label>
        ))}
      </fieldset>

      <section className="players-roles" aria-label="Players and roles">
        <div className="players-roles-heading">
          <span>Players & roles</span>
          <small>{selectedGame?.seat_labels?.length ? "From the Rust catalog" : "Fallback labels"}</small>
        </div>
        <div className="seat-roles">
          {setupSeatRoles(selectedGame, playMode).map((role) => (
            <div key={role.seat}>
              <span>{role.label}</span>
              <strong>{role.actor}</strong>
            </div>
          ))}
        </div>
      </section>

      <button type="button" className="primary setup-start" onClick={onStart} disabled={!canStart}>
        Start Match
      </button>
    </section>
  );
}

function gameMetadata(game: GameCatalogEntry, selectedVariantLabel: string | null): string {
  const seatCopy = seatCountSummary(game);
  if (game.variants && game.variants.length > 1) {
    const variantCopy = selectedVariantLabel ? `Variant: ${selectedVariantLabel}` : `${game.variants.length} variants available`;
    return `${variantCopy}; ${seatCopy}`;
  }
  return `Standard setup; ${seatCopy}`;
}

function modeDetail(playMode: SetupPlayMode, game: GameCatalogEntry | null): string {
  const labels = setupLabels(game);
  if (labels.length >= 2) {
    switch (playMode) {
      case "human_vs_bot":
        if (labels.length > 2) {
          return `${labels[0].label} is you; all other seats are automated.`;
        }
        return `${labels[0].label} is you; ${labels[1].label} is an automated opponent.`;
      case "hotseat":
        if (labels.length > 2) {
          return `${labels.map((label) => label.label).join(", ")} are local on this device.`;
        }
        return `${labels[0].label} and ${labels[1].label} are local on this device.`;
      case "bot_vs_bot":
        if (labels.length > 2) {
          return `All ${labels.length} seats are automated locally.`;
        }
        return `${labels[0].label} and ${labels[1].label} are automated locally.`;
    }
  }
  switch (playMode) {
    case "human_vs_bot":
      return "First player is local; second player is automated.";
    case "hotseat":
      return "Both players are local on this device.";
    case "bot_vs_bot":
      return "Both players are automated locally.";
  }
}

function setupSeatRoles(game: GameCatalogEntry | null, playMode: SetupPlayMode): Array<{ seat: string; label: string; actor: string }> {
  return setupLabels(game).map((entry, index) => ({
    seat: entry.seat,
    label: entry.label,
    actor: actorLabel(playMode, index),
  }));
}

function setupLabels(game: GameCatalogEntry | null): SeatDisplayLabel[] {
  const labels = game?.seat_labels ?? game?.ui?.seat_labels ?? [];
  return labels.length
    ? labels
    : [
        { seat: "player_1", label: "Player 1" },
        { seat: "player_2", label: "Player 2" },
      ];
}

function supportedSeatCounts(game: GameCatalogEntry | null): number[] {
  return game?.supported_seats ?? [];
}

function seatCountSummary(game: GameCatalogEntry): string {
  const counts = supportedSeatCounts(game);
  if (counts.length === 1) {
    return `${counts[0]} seats`;
  }
  if (counts.length > 1) {
    return `${counts.join(", ")} seats`;
  }
  return "seat count from Rust catalog";
}

function seatCountDetail(game: GameCatalogEntry | null): string {
  if (!game) {
    return "Loaded from the Rust catalog.";
  }
  const counts = supportedSeatCounts(game);
  if (counts.length) {
    const defaultCopy = typeof game.default_seats === "number" ? `; default ${game.default_seats}` : "";
    return `Supported count${counts.length === 1 ? "" : "s"}: ${counts.join(", ")}${defaultCopy}.`;
  }
  return "Supported counts unavailable from the catalog.";
}

function actorLabel(playMode: SetupPlayMode, index: number): string {
  if (playMode === "bot_vs_bot") {
    return "bot";
  }
  if (playMode === "hotseat") {
    return index === 0 ? "you (local)" : "local";
  }
  return index === 0 ? "you (local)" : "bot";
}
