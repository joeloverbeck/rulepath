import type { EffectEntry } from "../wasm/client";

export type BurstMarkerKind = "human_action" | "human_choice" | "bot_action" | "automated_phase" | "initial";

export type ResolutionBurst = {
  id: string;
  label: string;
  markerKind: BurstMarkerKind;
  marker: EffectEntry | null;
  entries: EffectEntry[];
  visibleEntries: EffectEntry[];
};

const DECISION_MARKERS = new Set([
  "action_started",
  "choice_taken",
  "bot_chose_action",
  "bot_chose_action_public",
  "environment_phase_began",
]);

export function segmentResolutionBursts(effects: EffectEntry[]): ResolutionBurst[] {
  const bursts: ResolutionBurst[] = [];
  let current = createBurst(null, 0);

  for (const entry of effects) {
    if (isDecisionMarker(entry)) {
      pushIfNotEmpty(bursts, current);
      current = createBurst(entry, bursts.length);
      continue;
    }
    current.entries.push(entry);
    current.visibleEntries.push(entry);
  }

  pushIfNotEmpty(bursts, current);
  return bursts;
}

export function latestResolutionBurst(effects: EffectEntry[]): ResolutionBurst | null {
  return segmentResolutionBursts(effects).at(-1) ?? null;
}

export function isDecisionMarker(entry: EffectEntry): boolean {
  return DECISION_MARKERS.has(entry.effect.payload.type);
}

function pushIfNotEmpty(bursts: ResolutionBurst[], burst: ResolutionBurst): void {
  if (burst.marker || burst.visibleEntries.length > 0) {
    bursts.push(burst);
  }
}

function createBurst(marker: EffectEntry | null, index: number): ResolutionBurst {
  const markerKind = marker ? markerKindFor(marker) : "initial";
  const markerCursor = marker ? String(marker.cursor) : String(index);
  return {
    id: `burst-${markerCursor}-${markerKind}`,
    label: labelFor(marker, markerKind, index),
    markerKind,
    marker,
    entries: marker ? [marker] : [],
    visibleEntries: [],
  };
}

function markerKindFor(entry: EffectEntry): BurstMarkerKind {
  switch (entry.effect.payload.type) {
    case "action_started":
      return "human_action";
    case "choice_taken":
      return "human_choice";
    case "bot_chose_action":
    case "bot_chose_action_public":
      return "bot_action";
    case "environment_phase_began":
      return "automated_phase";
    default:
      return "initial";
  }
}

function labelFor(marker: EffectEntry | null, markerKind: BurstMarkerKind, index: number): string {
  if (!marker) {
    return "Initial effects";
  }

  const payload = marker.effect.payload;
  switch (markerKind) {
    case "human_action":
      return `Human action${typeof payload.actor === "string" ? ` by ${payload.actor}` : ""}`;
    case "human_choice":
      return "Player choice";
    case "bot_action":
      return `Bot turn${typeof payload.policy_id === "string" ? ` (${payload.policy_id})` : ""}`;
    case "automated_phase":
      return "Automated phase";
    case "initial":
      return `Resolution ${index + 1}`;
  }
}
