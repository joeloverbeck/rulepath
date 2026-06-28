import { useMemo, useRef, useState, type KeyboardEvent } from "react";
import type {
  ActionChoice,
  ActionTree,
  EffectEntry,
  StarbridgeCrossingPublicView,
  StarbridgeCrossingSeatView,
} from "../wasm/client";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type StarbridgeCrossingBoardProps = {
  view: StarbridgeCrossingPublicView;
  actionTree: ActionTree | null;
  pendingPath: string[];
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  onPendingPathChange?: (path: string[]) => void;
  onPendingPathClear?: () => void;
  onPathSubmit?: (path: string[]) => void;
};

type BoardPoint = { x: number; y: number };

type SpaceAction = {
  choice: ActionChoice;
  path: string[];
  kind: "peg" | "step" | "jump";
  over: string | null;
};

const SEAT_COLORS = ["#365fbf", "#b84242", "#28745f", "#8a4bb3", "#b56b21", "#56731f"];
const SEAT_SYMBOLS = ["A", "B", "C", "D", "E", "F"];

export function StarbridgeCrossingBoard({
  view,
  actionTree,
  pendingPath,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  onPendingPathChange,
  onPendingPathClear,
  onPathSubmit,
}: StarbridgeCrossingBoardProps) {
  const terminal = view.terminal !== null;
  const canPlay = !pending && !terminal;
  const sortedSpaces = useMemo(() => [...view.spaces].sort((left, right) => naturalSpaceOrder(left.space, right.space)), [view.spaces]);
  const bounds = useMemo(() => boardBounds(sortedSpaces.map((space) => space.ui.anchor)), [sortedSpaces]);
  const points = useMemo(
    () => new Map(sortedSpaces.map((space) => [space.space, normalizePoint(space.ui.anchor, bounds)] as const)),
    [bounds, sortedSpaces],
  );
  const current = useMemo(() => currentChoices(actionTree, pendingPath), [actionTree, pendingPath]);
  const pegSpaces = useMemo(
    () =>
      new Map(
        view.spaces
          .filter((space) => space.occupant)
          .map((space) => [space.occupant?.peg ?? "", space.space] as const),
      ),
    [view.spaces],
  );
  const spaceActions = useMemo(() => actionsBySpace(actionTree, pendingPath, pegSpaces), [actionTree, pendingPath, pegSpaces]);
  const previewOvers = useMemo(
    () => new Set([...spaceActions.values()].map((action) => action.over).filter((space): space is string => Boolean(space))),
    [spaceActions],
  );
  const legalSpaces = useMemo(() => new Set(spaceActions.keys()), [spaceActions]);
  const recent = useMemo(() => recentSpaces(effects), [effects]);
  const seatNames = useMemo(() => seatNameMap(view.seats), [view.seats]);
  const nameForSeat = (seat: string | null) => resolveSeatName(seat, seatNames);
  const [focusedSpace, setFocusedSpace] = useState(() => sortedSpaces[0]?.space ?? "");
  const refs = useRef(new Map<string, SVGElement>());
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const controls = current.choices.filter((choice) => !spaceSegment(choice));
  const outcomeExplanation =
    terminal && view.terminal_rationale
      ? outcomeSurfaceData({
          gameId: "starbridge_crossing",
          heading: "Starbridge Crossing result",
          rationale: view.terminal_rationale,
          resultKind: "terminal",
          decisiveCause: view.terminal ?? "terminal",
          templateKey: "starbridge_crossing.finish_order_complete",
          finalStanding: [],
          ruleIds: [],
        })
      : null;

  const chooseSpace = (spaceId: string) => {
    if (!canPlay) {
      return;
    }
    const action = spaceActions.get(spaceId);
    if (!action) {
      return;
    }
    if (action.choice.next?.choices?.length) {
      onPendingPathChange?.(action.path);
    } else {
      onPathSubmit?.(action.path);
      onPendingPathClear?.();
    }
  };

  const chooseControl = (choice: ActionChoice) => {
    if (!canPlay) {
      return;
    }
    const path = [...current.path, choice.segment];
    if (choice.next?.choices?.length) {
      onPendingPathChange?.(path);
    } else {
      onPathSubmit?.(path);
      onPendingPathClear?.();
    }
  };

  const moveFocus = (delta: number) => {
    const index = Math.max(0, sortedSpaces.findIndex((space) => space.space === focusedSpace));
    const next = sortedSpaces[(index + delta + sortedSpaces.length) % sortedSpaces.length];
    if (!next) {
      return;
    }
    setFocusedSpace(next.space);
    refs.current.get(next.space)?.focus();
  };

  return (
    <section
      className={`starbridge-board${terminal ? " terminal" : ""}${reducedMotion ? " reduced" : ""}`}
      aria-labelledby="starbridge-heading"
      onKeyDown={(event: KeyboardEvent<HTMLElement>) => {
        if (event.key === "Escape") {
          event.preventDefault();
          onPendingPathClear?.();
        }
      }}
    >
      <div className="starbridge-header">
        <div>
          <p className="eyebrow">Starbridge Crossing</p>
          <h2 id="starbridge-heading">{terminal ? terminalLabel(view) : `${nameForSeat(view.active_seat)} to move`}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {terminal ? "Complete" : `Ply ${view.ply_count}`}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {liveSummary(view, pendingPath, legalSpaces.size, feedback, seatNames)}
      </p>

      {outcomeExplanation ? (
        <p className="sr-only" aria-live="polite">
          {outcomeAnnouncementText(outcomeExplanation)}
        </p>
      ) : null}

      <div className="starbridge-status" aria-label="Match status">
        <div>
          <span>Active</span>
          <strong>{nameForSeat(view.active_seat)}</strong>
        </div>
        <div>
          <span>Legal targets</span>
          <strong>{legalSpaces.size}</strong>
        </div>
        <div>
          <span>Path</span>
          <strong>{pendingPath.length > 0 ? pendingPath.join(" > ") : "ready"}</strong>
        </div>
      </div>

      <div className="starbridge-stage" data-testid="starbridge-board">
        <svg
          className="starbridge-svg"
          viewBox="0 0 1000 900"
          role="grid"
          aria-label="Starbridge Crossing 121-space board"
          aria-activedescendant={focusedSpace ? `starbridge-space-${focusedSpace}` : undefined}
        >
          <g className="starbridge-zone-lines" aria-hidden="true">
            {sortedSpaces.map((space) => {
              const point = points.get(space.space) ?? { x: 0, y: 0 };
              return <circle key={`zone-${space.space}`} cx={point.x} cy={point.y} r="15" className={zoneClass(space.zone)} />;
            })}
          </g>
          <g>
            {sortedSpaces.map((space) => {
              const point = points.get(space.space) ?? { x: 0, y: 0 };
              const action = spaceActions.get(space.space) ?? null;
              const ownerIndex = space.occupant?.owner_seat_index ?? null;
              const recentOrigin = recent.origins.has(space.space);
              const recentLanding = recent.landings.has(space.space);
              const recentOver = recent.overs.has(space.space) || previewOvers.has(space.space);
              return (
                <g
                  key={space.space}
                  id={`starbridge-space-${space.space}`}
                  ref={(node) => {
                    if (node) {
                      refs.current.set(space.space, node);
                    } else {
                      refs.current.delete(space.space);
                    }
                  }}
                  role="gridcell"
                  tabIndex={focusedSpace === space.space ? 0 : -1}
                  aria-label={spaceLabel(space, action, seatNames)}
                  aria-disabled={!action || !canPlay}
                  data-starbridge-space={space.space}
                  className={`starbridge-space${action ? " legal" : ""}${focusedSpace === space.space ? " focused" : ""}${
                    recentOrigin ? " recent-origin" : ""
                  }${recentLanding ? " recent-landing" : ""}${recentOver ? " recent-over" : ""}`}
                  onFocus={() => setFocusedSpace(space.space)}
                  onClick={() => chooseSpace(space.space)}
                  onKeyDown={(event) => {
                    if (event.key === "Enter" || event.key === " ") {
                      event.preventDefault();
                      chooseSpace(space.space);
                    } else if (event.key === "ArrowRight" || event.key === "ArrowDown") {
                      event.preventDefault();
                      moveFocus(1);
                    } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
                      event.preventDefault();
                      moveFocus(-1);
                    }
                  }}
                >
                  <circle cx={point.x} cy={point.y} r={action ? 19 : 15} className="starbridge-space-dot" />
                  {action ? <circle cx={point.x} cy={point.y} r="25" className={`starbridge-legal-ring ${action.kind}`} /> : null}
                  {space.occupant ? (
                    <>
                      <circle
                        cx={point.x}
                        cy={point.y}
                        r="12"
                        className="starbridge-peg"
                        style={{ fill: SEAT_COLORS[ownerIndex ?? 0] ?? "#33433d" }}
                      />
                      <text x={point.x} y={point.y + 4} className="starbridge-peg-label" aria-hidden="true">
                        {SEAT_SYMBOLS[ownerIndex ?? 0] ?? String(ownerIndex)}
                      </text>
                    </>
                  ) : null}
                  <text x={point.x} y={point.y + 34} className="starbridge-space-label" aria-hidden="true">
                    {space.space}
                  </text>
                </g>
              );
            })}
          </g>
        </svg>
      </div>

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}

      <div className="starbridge-controls">
        <div className="starbridge-path">
          <span>Selected path</span>
          <strong>{pendingPath.length > 0 ? pendingPath.join(" > ") : "No path selected"}</strong>
        </div>
        <div className="starbridge-control-buttons" aria-label="Path controls">
          {controls.map((choice) => (
            <button key={choice.segment} type="button" disabled={!canPlay} onClick={() => chooseControl(choice)}>
              {choice.label}
            </button>
          ))}
          {pendingPath.length > 0 ? (
            <button type="button" disabled={pending} onClick={() => onPendingPathClear?.()}>
              Clear
            </button>
          ) : null}
        </div>
      </div>

      <div className="starbridge-legend" aria-label="Seat legend">
        {view.seats.map((seat) => (
          <div key={seat.seat_id}>
            <span style={{ background: SEAT_COLORS[seat.seat_index] ?? "#33433d" }} aria-hidden="true">
              {SEAT_SYMBOLS[seat.seat_index] ?? seat.seat_index}
            </span>
            <strong>{nameForSeat(seat.seat_id)}</strong>
            <small>
              to {formatPoint(seat.target)}
              {seat.finish_rank ? `, rank ${seat.finish_rank}` : ""}
            </small>
          </div>
        ))}
      </div>
    </section>
  );
}

function currentChoices(actionTree: ActionTree | null, pendingPath: string[]) {
  let choices = actionTree?.choices ?? [];
  const path: string[] = [];
  for (const segment of pendingPath) {
    const choice = choices.find((candidate) => candidate.segment === segment);
    if (!choice) {
      return { choices: [], path, valid: false };
    }
    path.push(segment);
    choices = choice.next?.choices ?? [];
  }
  return { choices, path, valid: true };
}

function actionsBySpace(actionTree: ActionTree | null, pendingPath: string[], pegSpaces: Map<string, string>) {
  const actions = new Map<string, SpaceAction>();
  const current = currentChoices(actionTree, pendingPath);
  addSpaceActions(actions, current.choices, current.path, pegSpaces);
  if (actions.size === 0 && pendingPath.length === 0) {
    const move = current.choices.find((choice) => choice.segment === "move");
    if (move?.next?.choices) {
      addSpaceActions(actions, move.next.choices, ["move"], pegSpaces);
    }
  }
  return actions;
}

function addSpaceActions(actions: Map<string, SpaceAction>, choices: ActionChoice[], prefix: string[], pegSpaces: Map<string, string>) {
  for (const choice of choices) {
    const tags = new Set(choice.tags ?? []);
    if (tags.has("peg")) {
      const peg = metadataValue(choice, "peg") ?? choice.segment;
      const space = pegSpaces.get(peg);
      if (space) {
        actions.set(space, { choice, path: [...prefix, choice.segment], kind: "peg", over: null });
      }
    } else if (tags.has("step")) {
      for (const destination of choice.next?.choices ?? []) {
        const space = metadataValue(destination, "destination") ?? destination.segment;
        actions.set(space, { choice: destination, path: [...prefix, choice.segment, destination.segment], kind: "step", over: null });
      }
    } else if (tags.has("jump")) {
      for (const landing of choice.next?.choices ?? []) {
        const space = metadataValue(landing, "landing") ?? landing.segment;
        actions.set(space, {
          choice: landing,
          path: [...prefix, choice.segment, landing.segment],
          kind: "jump",
          over: metadataValue(landing, "over"),
        });
      }
    } else if (tags.has("jump_landing")) {
      const space = metadataValue(choice, "landing") ?? choice.segment;
      actions.set(space, {
        choice,
        path: [...prefix, choice.segment],
        kind: "jump",
        over: metadataValue(choice, "over"),
      });
    }
  }
}

function metadataValue(choice: ActionChoice, key: string) {
  return choice.metadata?.find((entry) => entry.key === key)?.value ?? null;
}

function spaceSegment(choice: ActionChoice) {
  const tags = new Set(choice.tags ?? []);
  return tags.has("peg") || tags.has("step") || tags.has("jump") || tags.has("jump_landing") || tags.has("step_destination");
}

function boardBounds(points: BoardPoint[]) {
  const xs = points.map((point) => point.x);
  const ys = points.map((point) => point.y);
  return {
    minX: Math.min(...xs),
    maxX: Math.max(...xs),
    minY: Math.min(...ys),
    maxY: Math.max(...ys),
  };
}

function normalizePoint(point: BoardPoint, bounds: ReturnType<typeof boardBounds>) {
  const width = Math.max(1, bounds.maxX - bounds.minX);
  const height = Math.max(1, bounds.maxY - bounds.minY);
  return {
    x: 80 + ((point.x - bounds.minX) / width) * 840,
    y: 70 + ((point.y - bounds.minY) / height) * 760,
  };
}

function recentSpaces(effects: EffectEntry[]) {
  const origins = new Set<string>();
  const landings = new Set<string>();
  const overs = new Set<string>();
  for (const entry of effects.slice(-3)) {
    const payload = entry.effect.payload;
    const kind = String(payload.type ?? payload.kind ?? "");
    if (kind === "step") {
      addString(origins, payload.from);
      addString(landings, payload.to);
    } else if (kind === "jump_chain") {
      addString(origins, payload.from);
      for (const hop of Array.isArray(payload.hops) ? payload.hops : []) {
        if (isRecord(hop)) {
          addString(overs, hop.over);
          addString(landings, hop.to);
        }
      }
    }
  }
  return { origins, landings, overs };
}

function addString(set: Set<string>, value: unknown) {
  if (typeof value === "string") {
    set.add(value);
  }
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function spaceLabel(
  space: StarbridgeCrossingPublicView["spaces"][number],
  action: SpaceAction | null,
  names: Map<string, string>,
) {
  const occupant = space.occupant ? `${space.occupant.peg} owned by ${resolveSeatName(space.occupant.owner_seat, names)}` : "empty";
  const legal = action ? ` Legal ${action.kind}${action.over ? ` over ${action.over}` : ""}.` : "";
  return `${space.space}, ${space.ui.coordinate_label}, ${space.ui.zone_label}, ${occupant}.${legal}`;
}

function liveSummary(
  view: StarbridgeCrossingPublicView,
  pendingPath: string[],
  legalCount: number,
  feedback: ReturnType<typeof feedbackForEffect> | null,
  names: Map<string, string>,
) {
  if (feedback) {
    return `${feedback.title}. ${feedback.detail}`;
  }
  if (view.terminal) {
    return terminalLabel(view);
  }
  return `${resolveSeatName(view.active_seat, names)} active. ${legalCount} legal board target${legalCount === 1 ? "" : "s"}. Path ${
    pendingPath.length > 0 ? pendingPath.join(" ") : "ready"
  }.`;
}

function terminalLabel(view: StarbridgeCrossingPublicView) {
  if (view.terminal) {
    return view.terminal.replaceAll("_", " ");
  }
  return "In progress";
}

function seatNameMap(seats: StarbridgeCrossingSeatView[]): Map<string, string> {
  return new Map(seats.map((seat) => [seat.seat_id, formatPoint(seat.home)]));
}

function resolveSeatName(seat: string | null, names: Map<string, string>): string {
  if (!seat) {
    return "No active seat";
  }
  const name = names.get(seat);
  if (name) {
    return name;
  }
  const suffix = Number(seat.replace("seat_", ""));
  return Number.isFinite(suffix) ? `Seat ${suffix + 1}` : seat;
}

function formatPoint(point: string): string {
  return point
    .split(/[_\s]+/)
    .filter(Boolean)
    .map((word) => `${word.charAt(0).toUpperCase()}${word.slice(1)}`)
    .join(" ");
}

function zoneClass(zone: string) {
  if (zone.includes("home")) {
    return "home";
  }
  if (zone.includes("target")) {
    return "target";
  }
  return "neutral";
}

function naturalSpaceOrder(left: string, right: string) {
  return spaceNumber(left) - spaceNumber(right);
}

function spaceNumber(space: string) {
  const match = space.match(/\d+/);
  return match ? Number(match[0]) : 0;
}
