import type { GameCatalogEntry } from "../wasm/client";

type GamePickerProps = {
  games: GameCatalogEntry[];
  selectedGameId: string;
  onSelect: (gameId: string) => void;
  onRulesOpen: (gameId: string) => void;
};

export function GamePicker({ games, selectedGameId, onSelect, onRulesOpen }: GamePickerProps) {
  return (
    <section className="region" aria-labelledby="game-picker-heading">
      <div className="region-heading">
        <p className="eyebrow">Game</p>
        <h2 id="game-picker-heading">Choose a game</h2>
      </div>
      <div className="game-list" role="list">
        {games.length === 0 ? (
          <p className="muted">Waiting for the Rust catalog...</p>
        ) : (
          games.map((game) => (
            <div className="game-card" role="listitem" key={game.game_id}>
              <button
                type="button"
                className={game.game_id === selectedGameId ? "game-option selected" : "game-option"}
                onClick={() => onSelect(game.game_id)}
                aria-pressed={game.game_id === selectedGameId}
              >
                <span>{game.display_name}</span>
                <small>
                  rules {game.rules_version} / schema {game.schema_version}
                  {game.variants?.length ? ` / ${game.variants.join(", ")}` : ""}
                </small>
                {game.hidden_information || game.viewer_modes?.length ? (
                  <span className="game-flags">
                    {game.hidden_information ? <span>Hidden info</span> : null}
                    {game.viewer_modes?.length ? <span>{game.viewer_modes.length} views</span> : null}
                  </span>
                ) : null}
              </button>
              <button
                type="button"
                className="secondary rules-trigger"
                onClick={() => onRulesOpen(game.game_id)}
                aria-label={`How to play ${game.display_name}`}
              >
                How to Play
              </button>
            </div>
          ))
        )}
      </div>
    </section>
  );
}
