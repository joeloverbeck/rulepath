import type { ActionChoice, ActionTree, ApiError, FeatureReport, PublicView } from "../wasm/client";
import type { SetupPlayMode } from "../state/shellReducer";

type DevPanelProps = {
  open: boolean;
  featureReport: FeatureReport | null;
  selectedGameName: string | null;
  matchId: string | null;
  seed: number;
  playMode: SetupPlayMode;
  view: PublicView | null;
  actionTree: ActionTree | null;
  pendingActionPath: string[];
  effectCursor: number;
  effectCount: number;
  pendingOperation: string | null;
  replayId: string | null;
  replayCursor: number | null;
  diagnostic: ApiError | null;
  canSubmitStale: boolean;
  onToggle: () => void;
  onSubmitStale: () => void;
};

export function DevPanel({
  open,
  featureReport,
  selectedGameName,
  matchId,
  seed,
  playMode,
  view,
  actionTree,
  pendingActionPath,
  effectCursor,
  effectCount,
  pendingOperation,
  replayId,
  replayCursor,
  diagnostic,
  canSubmitStale,
  onToggle,
  onSubmitStale,
}: DevPanelProps) {
  const redactsActionPaths = Boolean(view && "game_id" in view && view.game_id === "high_card_duel");
  return (
    <section className="dev-panel" aria-labelledby="dev-panel-heading">
      <button type="button" className="dev-toggle" onClick={onToggle} aria-expanded={open}>
        <span id="dev-panel-heading">Developer panel</span>
        <span>{open ? "Hide" : "Show"}</span>
      </button>

      {open ? (
        <div className="dev-panel-body">
          <dl className="dev-grid">
            <div>
              <dt>API</dt>
              <dd>{featureReport?.api_version ?? "Unavailable"}</dd>
            </div>
            <div>
              <dt>Features</dt>
              <dd>{featureReport?.features.join(", ") ?? "Unavailable"}</dd>
            </div>
            <div>
              <dt>Operations</dt>
              <dd>{featureReport?.operations.length ?? 0}</dd>
            </div>
            <div>
              <dt>Game</dt>
              <dd>{selectedGameName ?? "None"}</dd>
            </div>
            <div>
              <dt>Match</dt>
              <dd>{matchId ?? "None"}</dd>
            </div>
            <div>
              <dt>Seed</dt>
              <dd>{seed}</dd>
            </div>
            <div>
              <dt>Mode</dt>
              <dd>{playMode}</dd>
            </div>
            <div>
              <dt>Actor</dt>
              <dd>{view?.active_seat ?? "None"}</dd>
            </div>
            <div>
              <dt>Freshness</dt>
              <dd>{view?.freshness_token ?? "None"}</dd>
            </div>
            <div>
              <dt>Action choices</dt>
              <dd>{actionTree?.choices.length ?? 0}</dd>
            </div>
            <div>
              <dt>Pending path</dt>
              <dd>{pendingActionPath.length > 0 ? formatActionPath(pendingActionPath) : "None"}</dd>
            </div>
            <div>
              <dt>Effect cursor</dt>
              <dd>{effectCursor}</dd>
            </div>
            <div>
              <dt>Effect entries</dt>
              <dd>{effectCount}</dd>
            </div>
            <div>
              <dt>Pending</dt>
              <dd>{pendingOperation ?? "None"}</dd>
            </div>
            <div>
              <dt>Replay</dt>
              <dd>{replayId ? `${replayId} @ ${replayCursor ?? 0}` : "None"}</dd>
            </div>
          </dl>

          {diagnostic ? (
            <div className="diagnostic" role="status">
              <strong>{diagnostic.code}</strong>
              <span>{diagnostic.message}</span>
            </div>
          ) : null}

          <div className="dev-actions">
            <button type="button" onClick={onSubmitStale} disabled={!canSubmitStale} data-testid="stale-action">
              Submit Stale Action
            </button>
          </div>

          {actionTree?.choices.length && redactsActionPaths ? (
            <div className="dev-action-paths" aria-label="Action tree paths">
              <span>Action paths</span>
              <p>Redacted for hidden-information viewer safety.</p>
            </div>
          ) : actionTree?.choices.length ? (
            <div className="dev-action-paths" aria-label="Action tree paths">
              <span>Action paths</span>
              <ol>
                {actionPaths(actionTree).map((path) => (
                  <li key={path.join("\u0000")}>
                    <code>{formatActionPath(path)}</code>
                  </li>
                ))}
              </ol>
            </div>
          ) : null}
        </div>
      ) : null}
    </section>
  );
}

function actionPaths(actionTree: ActionTree): string[][] {
  return actionTree.choices.flatMap((choice) => choicePaths(choice, []));
}

function choicePaths(choice: ActionChoice, prefix: string[]): string[][] {
  const path = [...prefix, choice.segment];
  if (!choice.next?.choices?.length) {
    return [path];
  }
  return choice.next.choices.flatMap((next) => choicePaths(next, path));
}

function formatActionPath(path: string[]): string {
  return path.join(" > ");
}
