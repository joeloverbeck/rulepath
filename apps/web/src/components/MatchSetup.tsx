import type { SetupPlayMode } from "../state/shellReducer";
import type { GameCatalogEntry } from "../wasm/client";

type MatchSetupProps = {
  selectedGame: GameCatalogEntry | null;
  seed: number;
  playMode: SetupPlayMode;
  canStart: boolean;
  onSeedChange: (seed: number) => void;
  onPlayModeChange: (mode: SetupPlayMode) => void;
  onStart: () => void;
};

const PLAY_MODES: { value: SetupPlayMode; label: string; detail: string }[] = [
  {
    value: "human_vs_bot",
    label: "Human vs bot",
    detail: "Seat 0 is local; Seat 1 uses Rust's legal bot.",
  },
  {
    value: "hotseat",
    label: "Hotseat",
    detail: "Both seats are local on this device.",
  },
  {
    value: "bot_vs_bot",
    label: "Bot vs bot",
    detail: "Both seats are driven by Rust bot turns.",
  },
];

export function MatchSetup({
  selectedGame,
  seed,
  playMode,
  canStart,
  onSeedChange,
  onPlayModeChange,
  onStart,
}: MatchSetupProps) {
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
              ? `rules ${selectedGame.rules_version} / schema ${selectedGame.schema_version}`
              : "Load the Rust catalog to continue"}
          </small>
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
            <small>{mode.detail}</small>
          </label>
        ))}
      </fieldset>

      <div className="seat-roles" aria-label="Seat roles">
        <div>
          <span>Seat 0</span>
          <strong>{playMode === "bot_vs_bot" ? "Bot" : "Local"}</strong>
        </div>
        <div>
          <span>Seat 1</span>
          <strong>{playMode === "hotseat" ? "Local" : "Bot"}</strong>
        </div>
      </div>

      <button type="button" className="primary setup-start" onClick={onStart} disabled={!canStart}>
        Start Match
      </button>
    </section>
  );
}
