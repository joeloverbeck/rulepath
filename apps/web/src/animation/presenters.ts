import { feedbackForEffect } from "../components/effectFeedback";
import type { SchedulerPresentation, SchedulerStep } from "./scheduler";

export type PresentationKind = "highlight" | "move" | "turn-banner" | "terminal-settle" | "redacted";

export type PresentationContext = {
  root?: ParentNode;
  overlay?: HTMLElement | null;
  reducedMotion?: boolean;
};

const REDACTED_EFFECT_TYPES = new Set([
  "commit_face_down",
  "own_commit_confirmed",
  "commitment_placed",
  "own_commit_accepted",
  "hand_dealt",
]);

export function genericPresentationKind(step: SchedulerStep): PresentationKind {
  if (isRedactedEffect(step)) {
    return "redacted";
  }

  const feedback = feedbackForEffect(step.entry);
  switch (feedback.tone) {
    case "movement":
      return "move";
    case "turn":
      return "turn-banner";
    case "terminal":
      return "terminal-settle";
    case "neutral":
      return "highlight";
  }
}

export function genericPresentation(step: SchedulerStep, context: PresentationContext = {}): SchedulerPresentation {
  const kind = genericPresentationKind(step);
  const root = context.root ?? document;
  const target = findPresentationTarget(root, step);
  const reducedMotion = context.reducedMotion ?? step.reducedMotion;

  if (!target) {
    return {};
  }

  switch (kind) {
    case "move":
      return { animations: [animateHighlight(target, reducedMotion)] };
    case "turn-banner":
      return { animations: [animateFade(target, reducedMotion)] };
    case "terminal-settle":
      return { animations: [animateHighlight(target, reducedMotion)] };
    case "redacted":
      return { animations: [animateRedacted(target, reducedMotion)] };
    case "highlight":
      return { animations: [animateHighlight(target, reducedMotion)] };
  }
}

export function animateHighlight(element: Element, reducedMotion = false): Animation {
  return element.animate(
    [
      { opacity: 1, transform: "scale(1)" },
      { opacity: reducedMotion ? 1 : 0.82, transform: reducedMotion ? "scale(1)" : "scale(1.015)" },
      { opacity: 1, transform: "scale(1)" },
    ],
    animationTiming(reducedMotion),
  );
}

export function animateFade(element: Element, reducedMotion = false): Animation {
  return element.animate([{ opacity: 0.72 }, { opacity: 1 }], animationTiming(reducedMotion));
}

export function animateRedacted(element: Element, reducedMotion = false): Animation {
  return element.animate(
    [
      { opacity: 0.76, transform: "translateY(0)" },
      { opacity: 1, transform: reducedMotion ? "translateY(0)" : "translateY(-2px)" },
      { opacity: 1, transform: "translateY(0)" },
    ],
    animationTiming(reducedMotion),
  );
}

export function animateFlip(element: Element, first: DOMRect, last: DOMRect, reducedMotion = false): Animation {
  const dx = first.left - last.left;
  const dy = first.top - last.top;
  return element.animate(
    [
      { transform: reducedMotion ? "translate(0, 0)" : `translate(${dx}px, ${dy}px)` },
      { transform: "translate(0, 0)" },
    ],
    animationTiming(reducedMotion),
  );
}

export function createGhostOverlay(source: HTMLElement, overlay: HTMLElement): HTMLElement {
  const ghost = source.cloneNode(true) as HTMLElement;
  ghost.classList.add("animation-ghost");
  ghost.setAttribute("aria-hidden", "true");
  ghost.removeAttribute("id");
  overlay.append(ghost);
  return ghost;
}

function isRedactedEffect(step: SchedulerStep): boolean {
  const payload = step.entry.effect.payload;
  return REDACTED_EFFECT_TYPES.has(payload.type) || payload.redacted === true;
}

function findPresentationTarget(root: ParentNode, step: SchedulerStep): Element | null {
  const type = cssEscape(step.entry.effect.payload.type);
  return root.querySelector(`[data-effect-kind="${type}"], [data-animation-surface]`);
}

function animationTiming(reducedMotion: boolean): KeyframeAnimationOptions {
  return {
    duration: reducedMotion ? 80 : 180,
    easing: "cubic-bezier(0.2, 0, 0, 1)",
    fill: "both",
  };
}

function cssEscape(value: string): string {
  if (typeof CSS !== "undefined" && CSS.escape) {
    return CSS.escape(value);
  }
  return value.replace(/["\\]/g, "\\$&");
}
