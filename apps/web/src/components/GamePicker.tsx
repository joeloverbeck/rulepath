import type { GameCatalogEntry } from "../wasm/client";
import { GameCatalogIcon } from "./GameCatalogIcon";

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
            <div
              className={game.game_id === selectedGameId ? "game-card selected" : "game-card"}
              data-game-id={game.game_id}
              role="listitem"
              key={game.game_id}
              onClick={(event) => {
                if ((event.target as HTMLElement).closest("button")) return;
                onSelect(game.game_id);
              }}
              aria-label={`Select ${game.display_name}`}
            >
              <button
                type="button"
                className={game.game_id === selectedGameId ? "game-option selected" : "game-option"}
                onClick={() => onSelect(game.game_id)}
                aria-pressed={game.game_id === selectedGameId}
              >
                <span className="game-card-accent" aria-hidden="true" />
                <span className="game-art" aria-hidden={game.game_id === "river_ledger" ? undefined : "true"}>
                  <GameCatalogIcon gameId={game.game_id} title={`${game.display_name} icon`} decorative={game.game_id !== "river_ledger"} />
                </span>
                <span className="game-card-copy">
                  <span className="game-card-eyebrow">{gameEyebrow(game)}</span>
                  <span className="game-card-title">{game.display_name}</span>
                  <small>{gameSummary(game)}</small>
                </span>
                {game.hidden_information || game.viewer_modes?.length ? (
                  <span className="game-flags">
                    <span>{seatCountSummary(game)}</span>
                    {game.hidden_information ? <span>Hidden info</span> : null}
                    {game.viewer_modes?.length ? <span>{game.viewer_modes.length} views</span> : null}
                  </span>
                ) : null}
                {game.game_id === selectedGameId ? (
                  <span className="game-selected-mark" aria-hidden="true">
                    Selected
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

function gameEyebrow(game: GameCatalogEntry): string {
  if (game.cooperative) return "Cooperative";
  if (game.hidden_information) return "Hidden information";
  if (game.viewer_modes && game.viewer_modes.length > 1) return "Multi-view";
  return "Classic table";
}

function gameSummary(game: GameCatalogEntry): string {
  const seatCopy = seatCountSummary(game);
  if (!game.variants?.length || game.variants.length === 1) {
    return `${game.variants?.[0]?.label ?? "Standard setup"}; ${seatCopy}`;
  }
  return `${game.variants.length} variants; ${seatCopy}`;
}

function seatCountSummary(game: GameCatalogEntry): string {
  const counts = game.supported_seats ?? [];
  if (counts.length === 1) return `${counts[0]} seats`;
  if (counts.length > 1) return `${counts.join(", ")} seats`;
  return "catalog seats";
}
