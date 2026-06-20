import type { BotDecisionSummary, SetupPlayMode } from "../state/shellReducer";
import type { PublicView, SeatDisplayLabel, ViewerSeatId } from "../wasm/client";

type ModeControlsProps = {
  playMode: SetupPlayMode;
  view: PublicView | null;
  gameId: string;
  gameName: string;
  autoplayRunning: boolean;
  orchestrationPaused: boolean;
  orchestrationActive: boolean;
  orchestrationRate: number;
  lastBotDecision: BotDecisionSummary | null;
  pending: boolean;
  onRulesOpen: (gameId: string) => void;
  onBotStep: () => void;
  onSkip: () => void;
  onOrchestrationPause: () => void;
  onOrchestrationResume: () => void;
  onOrchestrationRateChange: (rate: number) => void;
  onAutoplayStart: () => void;
  onAutoplayPause: () => void;
};

export function ModeControls({
  playMode,
  view,
  gameId,
  gameName,
  autoplayRunning,
  orchestrationPaused,
  orchestrationActive,
  orchestrationRate,
  lastBotDecision,
  pending,
  onRulesOpen,
  onBotStep,
  onSkip,
  onOrchestrationPause,
  onOrchestrationResume,
  onOrchestrationRateChange,
  onAutoplayStart,
  onAutoplayPause,
}: ModeControlsProps) {
  const terminal = isTerminalView(view);
  const activeSeat = view?.active_seat ?? null;
  const botActive = activeSeat ? isBotSeat(playMode, activeSeat) : false;
  const canRunBot = Boolean(view && botActive && !terminal && !pending);
  const canAutoplay = playMode === "bot_vs_bot" && Boolean(view && !terminal);
  const humanOrchestrationDisabled = terminal || !orchestrationActive;
  const skipLabel = humanOrchestrationDisabled ? "Skip: nothing to skip right now" : "Skip current bot or animation advance";
  const pauseLabel = orchestrationPaused
    ? humanOrchestrationDisabled
      ? "Resume: nothing is paused right now"
      : "Resume bot or animation advance"
    : humanOrchestrationDisabled
      ? "Pause: nothing to pause right now"
      : "Pause bot or animation advance";
  const status =
    playMode === "human_vs_bot" && botActive && activeSeat && !terminal
      ? `${activeActorLabel(view, activeSeat, playMode)} turn in progress`
      : activeSeat
        ? `${activeActorLabel(view, activeSeat, playMode)} to act`
        : "No active player";

  return (
    <section className="mode-controls" aria-label="Play mode controls">
      <div className="mode-controls-header">
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
            <>
              <button type="button" onClick={onSkip} disabled={humanOrchestrationDisabled} aria-label={skipLabel} title={skipLabel}>
                Skip
              </button>
              {orchestrationPaused ? (
                <button
                  type="button"
                  onClick={onOrchestrationResume}
                  disabled={humanOrchestrationDisabled}
                  aria-label={pauseLabel}
                  title={pauseLabel}
                >
                  Resume
                </button>
              ) : (
                <button
                  type="button"
                  onClick={onOrchestrationPause}
                  disabled={humanOrchestrationDisabled}
                  aria-label={pauseLabel}
                  title={pauseLabel}
                >
                  Pause
                </button>
              )}
            </>
          ) : null}

          {playMode === "bot_vs_bot" ? (
            <>
              <button type="button" onClick={onBotStep} disabled={!canRunBot}>
                Step Bot
              </button>
              <label className="speed-field">
                <span>Speed</span>
                <select value={orchestrationRate} onChange={(event) => onOrchestrationRateChange(Number(event.currentTarget.value))}>
                  <option value={0.5}>0.5x</option>
                  <option value={1}>1x</option>
                  <option value={2}>2x</option>
                </select>
              </label>
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
      </div>

      {gameId === "event_frontier" && lastBotDecision && !lastBotDecision.publicExplanation ? (
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

function isBotSeat(playMode: SetupPlayMode, seat: ViewerSeatId): boolean {
  if (playMode === "bot_vs_bot") {
    return true;
  }
  if (playMode === "human_vs_bot") {
    return seat !== "seat_0";
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

function activeActorLabel(view: PublicView | null, seat: ViewerSeatId, playMode: SetupPlayMode): string {
  const seatLabels = seatLabelsForView(view);
  if (seatLabels.length > 0) {
    const label = seatLabels.find((entry) => entry.seat === seat)?.label;
    return `${label ?? playerLabel(seat)}${roleSuffix(playMode, seat)}`;
  }
  return playerLabel(seat);
}

function seatLabelsForView(view: PublicView | null): SeatDisplayLabel[] {
  const ui = view && "ui" in view ? view.ui : null;
  if (ui && "seat_labels" in ui && Array.isArray(ui.seat_labels)) {
    return ui.seat_labels;
  }
  return [];
}

function playerLabel(seat: ViewerSeatId): string {
  return `Player ${Number(seat.replace("seat_", "")) + 1}`;
}

function roleSuffix(playMode: SetupPlayMode, seat: ViewerSeatId): string {
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
  if ("game_id" in view && view.game_id === "briar_circuit") {
    return view.phase === "terminal";
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
