import type { SetupPlayMode } from "../state/shellReducer";
import type { GameCatalogEntry, SeatDisplayLabel } from "../wasm/client";

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
  return (
    <section className="region setup-region" aria-labelledby="setup-heading">
      <div className="region-heading">
        <p className="eyebrow">Setup</p>
        <h2 id="setup-heading">Match setup</h2>
      </div>

      <div className="setup-grid">
        <div className="setup-summary">
          <span>Selected</span>
          <strong>{selectedGame?.display_name ?? "No game selected"}</strong>
          <small>
            {selectedGame
              ? gameMetadata(selectedGame, selectedVariant?.label ?? null)
              : "Load the Rust catalog to continue"}
          </small>
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
          </label>
        ) : null}
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

      <div className="seat-roles" aria-label="Seat roles">
        {setupSeatRoles(selectedGame, playMode).map((role) => (
          <div key={role.seat}>
            <span>{role.label}</span>
            <strong>{role.actor}</strong>
          </div>
        ))}
      </div>

      <button type="button" className="primary setup-start" onClick={onStart} disabled={!canStart}>
        Start Match
      </button>
    </section>
  );
}

function gameMetadata(game: GameCatalogEntry, selectedVariantLabel: string | null): string {
  if (game.variants && game.variants.length > 1) {
    return selectedVariantLabel ? `Variant: ${selectedVariantLabel}` : `${game.variants.length} variants available`;
  }
  return "Standard setup";
}

function modeDetail(playMode: SetupPlayMode, game: GameCatalogEntry | null): string {
  const labels = setupLabels(game);
  if (labels.length >= 2) {
    switch (playMode) {
      case "human_vs_bot":
        return `${labels[0].label} is you; ${labels[1].label} uses Rust's legal bot.`;
      case "hotseat":
        return `${labels[0].label} and ${labels[1].label} are local on this device.`;
      case "bot_vs_bot":
        return `${labels[0].label} and ${labels[1].label} are driven by Rust bot turns.`;
    }
  }
  switch (playMode) {
    case "human_vs_bot":
      return "First player is local; second player uses Rust's legal bot.";
    case "hotseat":
      return "Both players are local on this device.";
    case "bot_vs_bot":
      return "Both players are driven by Rust bot turns.";
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
  const labels = game?.ui?.seat_labels ?? [];
  return labels.length
    ? labels
    : [
        { seat: "player_1", label: "Player 1" },
        { seat: "player_2", label: "Player 2" },
      ];
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
