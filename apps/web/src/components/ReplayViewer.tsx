import { feedbackForEffect } from "./effectFeedback";
import type { ReplaySessionState } from "../state/shellReducer";
import type {
  ColumnFourPublicView,
  DirectionalFlipPublicView,
  DraughtsLitePublicView,
  EffectEntry,
  EventFrontierPublicView,
  FloodWatchPublicView,
  FrontierControlPublicView,
  GameCatalogEntry,
  MaskedClaimsPublicView,
  PublicView,
  SecretDraftPublicView,
  ThreeMarksPublicView,
  TokenBazaarPublicView,
} from "../wasm/client";
import { ColumnFourBoard } from "./ColumnFourBoard";
import { DirectionalFlipBoard } from "./DirectionalFlipBoard";
import { DraughtsLiteBoard } from "./DraughtsLiteBoard";
import { EventFrontierBoard } from "./EventFrontierBoard";
import { FrontierControlBoard } from "./FrontierControlBoard";
import { SeatFrame } from "./SeatFrame";
import { ThreeMarksBoard } from "./ThreeMarksBoard";

type ReplayViewerProps = {
  game: GameCatalogEntry | null;
  replay: ReplaySessionState | null;
  reducedMotion: boolean;
  onStep: () => void;
  onReset: () => void;
};

export function ReplayViewer({ game, replay, reducedMotion, onStep, onReset }: ReplayViewerProps) {
  const step = replay?.step ?? null;
  const effects = step?.effects ?? [];
  const publicEffects = step?.public_effects ?? [];
  const latestReplayEffect = effects.at(-1);
  const latestEntry: EffectEntry | null = latestReplayEffect && step ? { cursor: step.cursor, effect: latestReplayEffect } : null;
  const threeMarksView: ThreeMarksPublicView | null = step && isThreeMarksView(step.view) ? step.view : null;
  const columnFourView: ColumnFourPublicView | null = step && isColumnFourView(step.view) ? step.view : null;
  const directionalFlipView: DirectionalFlipPublicView | null =
    step && isDirectionalFlipView(step.view) ? step.view : null;
  const draughtsLiteView: DraughtsLitePublicView | null = step && isDraughtsLiteView(step.view) ? step.view : null;
  const secretDraftView: SecretDraftPublicView | null = step && isSecretDraftView(step.view) ? step.view : null;
  const tokenBazaarView: TokenBazaarPublicView | null = step && isTokenBazaarView(step.view) ? step.view : null;
  const replayEffectEntries: EffectEntry[] = step
    ? effects.map((effect, index) => ({ cursor: step.cursor + index, effect }))
    : [];
  const frontierControlView: FrontierControlPublicView | null =
    step && isFrontierControlView(step.view) ? step.view : null;
  const eventFrontierView: EventFrontierPublicView | null =
    step && isEventFrontierView(step.view) ? step.view : null;

  return (
    <section className="replay-viewer" aria-labelledby="replay-viewer-heading">
      <div className="region-heading">
        <p className="eyebrow">Viewer</p>
        <h2 id="replay-viewer-heading">Replay viewer</h2>
      </div>

      {step ? (
        <>
          <div className="replay-progress">
            <span>
              Cursor {step.cursor} / {step.command_count ?? step.total_steps ?? 0}
            </span>
            <progress value={step.cursor} max={Math.max(step.command_count ?? step.total_steps ?? 0, 1)} />
          </div>

          <div className="replay-snapshot">
            {snapshotItems(step.view, step.done).map((item) => (
              <div key={item.label}>
                <span>{item.label}</span>
                <strong>{item.value}</strong>
              </div>
            ))}
          </div>

          <SeatFrame game={game} view={step.view} viewerMode={{ kind: "observer" }} />

          {threeMarksView ? (
            <div className="replay-board">
              <ThreeMarksBoard
                view={threeMarksView}
                latestEffect={latestEntry}
                reducedMotion={reducedMotion}
                pending={false}
                interactive={false}
              />
              {replay ? <PlacementSequence replay={replay} /> : null}
            </div>
          ) : columnFourView ? (
            <div className="replay-board">
              <ColumnFourBoard
                view={columnFourView}
                latestEffect={latestEntry}
                effects={replayEffectEntries}
                reducedMotion={reducedMotion}
                pending={false}
                interactive={false}
              />
              {replay ? <PlacementSequence replay={replay} /> : null}
            </div>
          ) : directionalFlipView ? (
            <div className="replay-board">
              <DirectionalFlipBoard
                view={directionalFlipView}
                latestEffect={latestEntry}
                effects={replayEffectEntries}
                reducedMotion={reducedMotion}
                pending={false}
                interactive={false}
              />
              {replay ? <PlacementSequence replay={replay} /> : null}
            </div>
          ) : draughtsLiteView ? (
            <div className="replay-board">
              <DraughtsLiteBoard
                view={draughtsLiteView}
                actionTree={null}
                pendingPath={[]}
                latestEffect={latestEntry}
                effects={replayEffectEntries}
                reducedMotion={reducedMotion}
                pending={false}
                interactive={false}
              />
              {replay ? <PlacementSequence replay={replay} /> : null}
            </div>
          ) : (tokenBazaarView || secretDraftView) && replay ? (
            <div className="replay-board">
              <PlacementSequence replay={replay} />
            </div>
          ) : frontierControlView ? (
            <div className="replay-board">
              <FrontierControlBoard
                view={frontierControlView}
                actionTree={null}
                latestEffect={latestEntry}
                effects={replayEffectEntries}
                reducedMotion={reducedMotion}
                pending={false}
                interactive={false}
              />
              {replay ? <PlacementSequence replay={replay} /> : null}
            </div>
          ) : eventFrontierView ? (
            <div className="replay-board">
              <EventFrontierBoard
                view={eventFrontierView}
                actionTree={null}
                latestEffect={latestEntry}
                effects={replayEffectEntries}
                reducedMotion={reducedMotion}
                pending={false}
                interactive={false}
              />
              <div className="replay-snapshot" aria-label="Event Frontier replay markers">
                <div>
                  <span>Epoch</span>
                  <strong>{eventFrontierView.epoch}</strong>
                </div>
                <div>
                  <span>Reckoning</span>
                  <strong>{eventFrontierView.reckoning_count}</strong>
                </div>
              </div>
              {replay ? <PlacementSequence replay={replay} /> : null}
            </div>
          ) : null}

          <ol className="replay-effects">
            {step.public_export && publicEffects.length > 0 ? (
              <>
                {step.redacted_command_summary ? (
                  <li className={reducedMotion ? "reduced" : ""}>
                    <strong>Command</strong>
                    <span>{formatPublicCommandSummary(step.redacted_command_summary)}</span>
                  </li>
                ) : null}
                {publicEffects.map((effect, index) => {
                  const observation = formatPublicObservation(effect);
                  return (
                    <li key={`${step.cursor}-public-${index}`} className={reducedMotion ? "reduced" : ""}>
                      <strong>{observation.title}</strong>
                      <span>{observation.detail}</span>
                    </li>
                  );
                })}
              </>
            ) : effects.length === 0 ? (
              <li>No replay effects at this cursor.</li>
            ) : (
              effects.map((effect, index) => {
                const entry: EffectEntry = { cursor: index + 1, effect };
                const feedback = feedbackForEffect(entry);
                return (
                  <li key={`${step.cursor}-${index}`} className={reducedMotion ? "reduced" : ""}>
                    <strong>{feedback.title}</strong>
                    <span>{feedback.detail}</span>
                  </li>
                );
              })
            )}
          </ol>
        </>
      ) : (
        <p className="muted">Export or import a replay to inspect it here.</p>
      )}

      <div className="replay-actions">
        <button type="button" onClick={onReset} disabled={!step || step.cursor === 0}>
          Reset
        </button>
        <button type="button" onClick={onStep} disabled={!step || step.done}>
          Step
        </button>
      </div>
    </section>
  );
}

function formatPublicObservation(value: string): { title: string; detail: string } {
  const [eventToken, ...detailParts] = value.split(":");
  const title = eventToken.replace(/^hcd_/, "").replaceAll("_", " ");
  const detail = detailParts.join(":");
  return {
    title: title.length > 0 ? title : "Observation",
    detail: formatPublicObservationDetail(detail),
  };
}

function formatPublicCommandSummary(value: string): string {
  return value.replaceAll("_", " ").replace(":", " ");
}

function formatPublicObservationDetail(detail: string): string {
  if (detail.length === 0) {
    return "Public observation";
  }
  return detail
    .split(";")
    .filter((part) => part.length > 0)
    .map((part) => {
      const [rawKey, ...rawValue] = part.split("=");
      const key = rawKey.replaceAll("_", " ");
      const value = rawValue.join("=");
      if (value.length === 0) {
        return key;
      }
      return `${key} ${value.replace(/hcd:r[0-9a-z:]+/gi, "revealed card")}`;
    })
    .join(", ");
}

function PlacementSequence({ replay }: { replay: ReplaySessionState }) {
  const commands = replay.document && "commands" in replay.document ? replay.document.commands : [];
  if (commands.length === 0) {
    return null;
  }

  return (
    <ol className="placement-sequence" aria-label="Replay command sequence">
      {commands.map((command) => {
        const isCurrent = command.index < replay.cursor;
        return (
          <li key={command.index} className={isCurrent ? "current" : ""}>
            <span>{command.index + 1}</span>
            <strong>{command.actor_seat}</strong>
            <code>{formatActionPath(command.action_path)}</code>
          </li>
        );
      })}
    </ol>
  );
}

function isThreeMarksView(view: PublicView | null): view is ThreeMarksPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "three_marks");
}

function isColumnFourView(view: PublicView | null): view is ColumnFourPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "column_four");
}

function isDirectionalFlipView(view: PublicView | null): view is DirectionalFlipPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "directional_flip");
}

function isDraughtsLiteView(view: PublicView | null): view is DraughtsLitePublicView {
  return Boolean(view && "game_id" in view && view.game_id === "draughts_lite");
}

function isTokenBazaarView(view: PublicView | null): view is TokenBazaarPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "token_bazaar");
}

function isSecretDraftView(view: PublicView | null): view is SecretDraftPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "secret_draft");
}

function isMaskedClaimsView(view: PublicView | null): view is MaskedClaimsPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "masked_claims");
}

function isFloodWatchView(view: PublicView | null): view is FloodWatchPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "flood_watch");
}

function isFrontierControlView(view: PublicView | null): view is FrontierControlPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "frontier_control");
}

function isEventFrontierView(view: PublicView | null): view is EventFrontierPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "event_frontier");
}

function formatActionPath(path: string[]): string {
  return path.join(" > ");
}

function cardFaceLabel(value: unknown): string {
  if (value && typeof value === "object" && "label" in value && typeof value.label === "string") {
    return value.label;
  }
  return typeof value === "string" ? value : "None";
}

function snapshotItems(view: PublicView | null, done: boolean | undefined): { label: string; value: string }[] {
  if (!view) {
    return [
      { label: "Replay", value: "Public observer timeline" },
      { label: "View", value: "Redacted" },
      { label: "Status", value: done ? "Complete" : "In progress" },
    ];
  }
  if ("counter" in view) {
    return [
      { label: "Counter", value: `${view.counter} / ${view.target}` },
      { label: "Turn", value: view.winner ? `${view.winner} won` : view.active_seat },
      { label: "Status", value: done ? "Complete" : "In progress" },
    ];
  }

  if (view.game_id === "high_card_duel") {
    return [
      { label: "Round", value: `${view.round_number} / ${view.round_limit}` },
      { label: "Turn", value: view.terminal_kind === "win" ? `${view.winning_seat} won` : view.active_seat ?? "terminal" },
      { label: "Status", value: view.phase },
    ];
  }

  if (view.game_id === "token_bazaar") {
    return [
      { label: "Score", value: `${view.scores.seat_0}-${view.scores.seat_1}` },
      {
        label: "Turn",
        value: view.terminal.terminal
          ? view.terminal.draw
            ? "draw"
            : `${view.terminal.winner} won`
          : view.active_seat ?? "terminal",
      },
      { label: "Market", value: `${view.market_slots.filter((slot) => !slot.is_empty).length} visible` },
    ];
  }

  if (view.game_id === "secret_draft") {
    return [
      { label: "Round", value: `${view.round_number} / ${view.round_limit}` },
      {
        label: "Turn",
        value: view.terminal.terminal
          ? view.terminal.draw
            ? "draw"
            : `${view.terminal.winner} won`
          : view.active_seat ?? "pending reveal",
      },
      { label: "Score", value: `${view.scores.seat_0}-${view.scores.seat_1}` },
    ];
  }

  if (view.game_id === "poker_lite") {
    return [
      { label: "Round", value: `${view.round.round_index + 1} / 2` },
      {
        label: "Turn",
        value: view.terminal.terminal
          ? view.terminal.draw
            ? "split"
            : `${view.terminal.winner} won`
          : view.active_seat ?? "resolving",
      },
      { label: "Shared pool", value: String(view.shared_pool) },
    ];
  }

  if (view.game_id === "plain_tricks") {
    return [
      { label: "Round", value: `${view.round_index + 1} / 2` },
      {
        label: "Turn",
        value:
          view.terminal.kind !== "non_terminal"
            ? view.terminal.draw
              ? "split"
              : `${view.terminal.winner} won`
            : view.active_seat ?? "resolving",
      },
      { label: "Tricks", value: `${view.total_trick_counts.seat_0}-${view.total_trick_counts.seat_1}` },
    ];
  }

  if (isMaskedClaimsView(view)) {
    return [
      { label: "Turn", value: `${view.turn_index + 1} / 8` },
      {
        label: "Turn holder",
        value:
          view.terminal.kind !== "non_terminal"
            ? view.terminal.draw
              ? "draw"
              : `${view.terminal.winner} won`
            : view.active_seat ?? "resolving",
      },
      { label: "Score", value: `${view.scores.seat_0}-${view.scores.seat_1}` },
    ];
  }

  if (isFloodWatchView(view)) {
    return [
      { label: "Turn", value: String(view.turn_number) },
      {
        label: "Storm status",
        value: view.terminal.kind === "non_terminal" ? `${view.undrawn_count} undrawn` : view.terminal.summary.public_summary,
      },
      { label: "Forecast", value: cardFaceLabel(view.forecast) },
    ];
  }

  if (isFrontierControlView(view)) {
    return [
      { label: "Round", value: String(view.round_number) },
      {
        label: "Turn",
        value: view.terminal.kind === "non_terminal" ? view.active_seat ?? "terminal" : view.terminal.summary,
      },
      { label: "Score", value: `${view.scores.garrison}-${view.scores.prospectors}` },
    ];
  }

  if (isEventFrontierView(view)) {
    return [
      { label: "Epoch", value: String(view.epoch) },
      {
        label: "Turn",
        value: view.terminal.kind === "non_terminal" ? view.active_seat ?? "resolving" : `${view.terminal.winner} won`,
      },
      { label: "Score", value: `${view.scores.charter}-${view.scores.freeholders}` },
    ];
  }

  return [
    { label: "Board", value: `${view.board_rows} x ${view.board_columns}` },
    { label: "Turn", value: view.terminal_kind === "win" ? `${view.winning_seat} won` : view.active_seat ?? "terminal" },
    { label: "Status", value: view.status_label },
  ];
}
