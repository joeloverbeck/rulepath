import type { GameCatalogEntry } from "../wasm/client";

type GamePickerProps = {
  games: GameCatalogEntry[];
  selectedGameId: string;
  onSelect: (gameId: string) => void;
};

export function GamePicker({ games, selectedGameId, onSelect }: GamePickerProps) {
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
            <button
              type="button"
              className={game.game_id === selectedGameId ? "game-option selected" : "game-option"}
              key={game.game_id}
              onClick={() => onSelect(game.game_id)}
              aria-pressed={game.game_id === selectedGameId}
            >
              <span>{game.display_name}</span>
              <small>
                rules {game.rules_version} / schema {game.schema_version}
              </small>
            </button>
          ))
        )}
      </div>
    </section>
  );
}
