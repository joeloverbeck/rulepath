import { useEffect, useState } from "react";
import type { EffectEntry } from "../wasm/client";

const STORAGE_KEY = "rulepath.reducedMotion";

export type ReducedMotionOverride = "system" | "reduce" | "motion";

export type EffectFeedback = {
  title: string;
  detail: string;
  tone: "neutral" | "movement" | "turn" | "terminal";
};

export function feedbackForEffect(entry: EffectEntry): EffectFeedback {
  const payload = entry.effect.payload;
  switch (payload.type) {
    case "action_started":
      return {
        title: "Action started",
        detail: `${payload.actor} chose a committed action.`,
        tone: "neutral",
      };
    case "counter_advanced":
      return {
        title: "Counter advanced",
        detail: `${payload.actor} moved from ${payload.from} to ${payload.to}.`,
        tone: "movement",
      };
    case "turn_changed":
      return {
        title: "Turn changed",
        detail: `${payload.next_actor} is now active.`,
        tone: "turn",
      };
    case "game_ended":
      return {
        title: "Game ended",
        detail: payload.winner ? `${payload.winner} won the match.` : "The match ended.",
        tone: "terminal",
      };
    case "mark_placed":
      return {
        title: "Mark placed",
        detail: `${payload.seat} placed on ${payload.cell}.`,
        tone: "movement",
      };
    case "drop_accepted":
      return {
        title: "Drop accepted",
        detail: `${payload.seat} chose ${payload.column}.`,
        tone: "neutral",
      };
    case "piece_landed":
      return {
        title: "Piece landed",
        detail: `${payload.seat} landed on ${payload.cell}.`,
        tone: "movement",
      };
    case "active_player_changed":
      return {
        title: "Turn changed",
        detail: `${payload.active_seat} is now active.`,
        tone: "turn",
      };
    case "line_completed":
      return {
        title: "Line completed",
        detail: `${payload.winning_seat} completed a line.`,
        tone: "terminal",
      };
    case "win_detected":
      return {
        title: "Win detected",
        detail: `${payload.winning_seat} completed a line.`,
        tone: "terminal",
      };
    case "draw_reached":
    case "draw_detected":
      return {
        title: "Draw reached",
        detail: "The board is full without a winner.",
        tone: "terminal",
      };
    case "bot_chose_action":
      return {
        title: "Bot chose action",
        detail: `${payload.policy_id} selected ${payload.action_id}.`,
        tone: "neutral",
      };
    case "placement_rejected":
      return {
        title: "Placement rejected",
        detail: String(payload.label ?? payload.reason ?? "Rejected"),
        tone: "neutral",
      };
    case "action_completed":
      return {
        title: "Action completed",
        detail: `${payload.actor} finished the action.`,
        tone: "neutral",
      };
    default:
      return {
        title: "Effect",
        detail: payload.type,
        tone: "neutral",
      };
  }
}

export function summarizeEffect(entry: EffectEntry): string {
  const feedback = feedbackForEffect(entry);
  return `${entry.cursor}: ${feedback.title} - ${feedback.detail}`;
}

export function useReducedMotionPreference() {
  const [systemReduced, setSystemReduced] = useState(false);
  const [override, setOverrideState] = useState<ReducedMotionOverride>(() => readOverride());

  useEffect(() => {
    const query = window.matchMedia("(prefers-reduced-motion: reduce)");
    const update = () => setSystemReduced(query.matches);
    update();
    query.addEventListener("change", update);
    return () => query.removeEventListener("change", update);
  }, []);

  const setOverride = (next: ReducedMotionOverride) => {
    setOverrideState(next);
    if (next === "system") {
      window.localStorage.removeItem(STORAGE_KEY);
    } else {
      window.localStorage.setItem(STORAGE_KEY, next);
    }
  };

  return {
    reducedMotion: override === "system" ? systemReduced : override === "reduce",
    override,
    setOverride,
  };
}

function readOverride(): ReducedMotionOverride {
  if (typeof window === "undefined") {
    return "system";
  }
  const stored = window.localStorage.getItem(STORAGE_KEY);
  return stored === "reduce" || stored === "motion" ? stored : "system";
}
