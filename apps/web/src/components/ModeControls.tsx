import type { BotDecisionSummary, SetupPlayMode } from "../state/shellReducer";
import type { PublicView, SeatId } from "../wasm/client";

type ModeControlsProps = {
  playMode: SetupPlayMode;
  view: PublicView | null;
  gameId: string;
  gameName: string;
  autoplayRunning: boolean;
  lastBotDecision: BotDecisionSummary | null;
  pending: boolean;
  onRulesOpen: (gameId: string) => void;
  onBotStep: () => void;
  onAutoplayStart: () => void;
  onAutoplayPause: () => void;
};

export function ModeControls({
  playMode,
  view,
  gameId,
  gameName,
  autoplayRunning,
  lastBotDecision,
  pending,
  onRulesOpen,
  onBotStep,
  onAutoplayStart,
  onAutoplayPause,
}: ModeControlsProps) {
  const terminal = isTerminalView(view);
  const activeSeat = view?.active_seat ?? null;
  const botActive = activeSeat ? isBotSeat(playMode, activeSeat) : false;
  const canRunBot = Boolean(view && botActive && !terminal && !pending);
  const canAutoplay = playMode === "bot_vs_bot" && Boolean(view && !terminal);
  const status = activeSeat ? `${activeActorLabel(view, activeSeat, playMode)} to act` : "No active player";

  return (
    <section className="mode-controls" aria-label="Play mode controls">
      <div>
        <p className="eyebrow">Mode</p>
        <h2>{modeLabel(playMode)}</h2>
        <p>{status}</p>
      </div>

      <div className="mode-actions">
        <button
          type="button"
          className="secondary rules-trigger"
          onClick={() => onRulesOpen(gameId)}
          disabled={!gameId}
          aria-label={`How to play ${gameName}`}
        >
          Rules
        </button>

        {playMode === "human_vs_bot" ? (
          <button type="button" onClick={onBotStep} disabled={!canRunBot}>
            Run Bot Turn
          </button>
        ) : null}

        {playMode === "bot_vs_bot" ? (
          <>
            <button type="button" onClick={onBotStep} disabled={!canRunBot}>
              Step Bot
            </button>
            {autoplayRunning ? (
              <button type="button" onClick={onAutoplayPause}>
                Pause
              </button>
            ) : (
              <button type="button" onClick={onAutoplayStart} disabled={!canAutoplay || pending}>
                Start Autoplay
              </button>
            )}
          </>
        ) : null}
      </div>

      {gameId === "event_frontier" && lastBotDecision ? (
        <details className="bot-note bot-why" data-testid="bot-explanation">
          <summary>Bot why</summary>
          <strong>{lastBotDecision.rationale}</strong>
          <span>{policyLabel(lastBotDecision)}</span>
        </details>
      ) : null}
    </section>
  );
}

function policyLabel(decision: BotDecisionSummary): string {
  const version = decision.policyVersion === null ? "" : ` v${decision.policyVersion}`;
  const level = decision.policyId.includes("level1") ? "Level 1" : "Rust";
  return `${level} bot policy${version}`;
}

function isBotSeat(playMode: SetupPlayMode, seat: SeatId): boolean {
  if (playMode === "bot_vs_bot") {
    return true;
  }
  if (playMode === "human_vs_bot") {
    return seat === "seat_1";
  }
  return false;
}

function modeLabel(playMode: SetupPlayMode): string {
  switch (playMode) {
    case "human_vs_bot":
      return "Human vs bot";
    case "hotseat":
      return "Hotseat";
    case "bot_vs_bot":
      return "Bot vs bot";
  }
}

function activeActorLabel(view: PublicView | null, seat: SeatId, playMode: SetupPlayMode): string {
  if (view && "game_id" in view && view.game_id === "event_frontier") {
    const label = view.ui.seat_labels.find((entry) => entry.seat === seat)?.label;
    return `${label ?? playerLabel(seat)}${roleSuffix(playMode, seat)}`;
  }
  return playerLabel(seat);
}

function playerLabel(seat: SeatId): string {
  return seat === "seat_0" ? "Player 1" : "Player 2";
}

function roleSuffix(playMode: SetupPlayMode, seat: SeatId): string {
  if (playMode === "bot_vs_bot") {
    return " (bot)";
  }
  if (playMode === "hotseat") {
    return seat === "seat_0" ? " (you)" : " (local)";
  }
  return seat === "seat_0" ? " (you)" : " (bot)";
}

function isTerminalView(view: PublicView | null): boolean {
  if (!view) {
    return false;
  }
  if ("winner" in view) {
    return view.winner !== null;
  }
  if ("game_id" in view && view.game_id === "plain_tricks") {
    return view.terminal.kind !== "non_terminal";
  }
  if ("game_id" in view && view.game_id === "masked_claims") {
    return view.terminal.kind !== "non_terminal";
  }
  if ("game_id" in view && view.game_id === "flood_watch") {
    return view.terminal.kind !== "non_terminal";
  }
  if ("game_id" in view && view.game_id === "frontier_control") {
    return view.terminal.kind !== "non_terminal";
  }
  if ("game_id" in view && view.game_id === "event_frontier") {
    return view.terminal.kind !== "non_terminal";
  }
  if ("terminal" in view) {
    return view.terminal.terminal;
  }
  return view.terminal_kind !== "non_terminal";
}
