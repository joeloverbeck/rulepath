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
    case "placement_accepted":
      return {
        title: "Placement accepted",
        detail: `${payload.seat} chose ${payload.cell}.`,
        tone: "neutral",
      };
    case "disc_placed":
      return {
        title: "Disc placed",
        detail: `${payload.seat} placed on ${payload.cell}.`,
        tone: "movement",
      };
    case "discs_flipped":
      return {
        title: "Discs flipped",
        detail: `${payload.seat} flipped ${flipCount(payload.flips)} disc${flipCount(payload.flips) === 1 ? "" : "s"}.`,
        tone: "movement",
      };
    case "pass_taken":
      return {
        title: "Pass taken",
        detail: `${payload.seat} had no legal placement.`,
        tone: "turn",
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
        detail: `${payload.policy_id} selected ${payload.action_id ?? formatPath(payload.action_path)}.`,
        tone: "neutral",
      };
    case "move_committed":
      return {
        title: "Move committed",
        detail: `${payload.seat} moved ${payload.piece_id} from ${payload.start_cell} to ${payload.final_cell}.`,
        tone: "movement",
      };
    case "quiet_step":
      return {
        title: "Quiet step",
        detail: `${payload.piece_id} moved from ${payload.origin} to ${payload.landing}.`,
        tone: "movement",
      };
    case "capture_step":
      return {
        title: "Capture step",
        detail: `${payload.piece_id} landed on ${payload.landing} and captured ${payload.captured_piece_id}.`,
        tone: "movement",
      };
    case "promotion":
      return {
        title: "Promotion",
        detail: `${payload.piece_id} promoted on ${payload.cell}.`,
        tone: "movement",
      };
    case "forced_capture_available":
      return {
        title: "Forced capture",
        detail: String(payload.explanation ?? "A capture is available."),
        tone: "turn",
      };
    case "forced_continuation_required":
      return {
        title: "Forced continuation",
        detail: String(payload.explanation ?? "The capture path must continue."),
        tone: "turn",
      };
    case "commit_face_down":
      return {
        title: "Commitment placed",
        detail: `${payload.seat} committed a card face-down.`,
        tone: "neutral",
      };
    case "own_commit_confirmed":
      return {
        title: "Private commitment confirmed",
        detail: "Your selected card was committed face-down.",
        tone: "neutral",
      };
    case "cards_revealed":
      return {
        title: "Cards revealed",
        detail: "Rust revealed both committed cards.",
        tone: "movement",
      };
    case "round_scored":
      return {
        title: "Round scored",
        detail: payload.winner ? `${payload.winner} won the round.` : "The round was drawn.",
        tone: "turn",
      };
    case "refill_started":
      return {
        title: "Next round",
        detail: `${payload.next_lead_seat} leads the next round.`,
        tone: "turn",
      };
    case "terminal":
      return {
        title: "Duel complete",
        detail: payload.winner ? `${payload.winner} won the duel.` : "The duel ended in a draw.",
        tone: "terminal",
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

function formatPath(value: unknown): string {
  return Array.isArray(value) ? value.join(" > ") : "an action";
}

function flipCount(flips: unknown): number {
  return Array.isArray(flips) ? flips.length : 0;
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
