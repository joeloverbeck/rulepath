import type { SchedulerPresentation, SchedulerStep } from "./scheduler";
import { genericPresentation, type PresentationContext } from "./presenters";

export type PresentationBuilder = (step: SchedulerStep, context: PresentationContext) => SchedulerPresentation | Promise<SchedulerPresentation>;

export type AnimationRegistry = {
  register(gameId: string, effectType: string, builder: PresentationBuilder): void;
  resolve(gameId: string, step: SchedulerStep, context?: PresentationContext): SchedulerPresentation | Promise<SchedulerPresentation>;
  has(gameId: string, effectType: string): boolean;
};

export function createAnimationRegistry(): AnimationRegistry {
  const registrations = new Map<string, PresentationBuilder>();

  return {
    register(gameId, effectType, builder) {
      registrations.set(registryKey(gameId, effectType), builder);
    },
    resolve(gameId, step, context = {}) {
      const builder = registrations.get(registryKey(gameId, step.entry.effect.payload.type));
      return builder ? builder(step, context) : genericPresentation(step, context);
    },
    has(gameId, effectType) {
      return registrations.has(registryKey(gameId, effectType));
    },
  };
}

export const animationRegistry = createAnimationRegistry();

function registryKey(gameId: string, effectType: string): string {
  return `${gameId}:${effectType}`;
}
