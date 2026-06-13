import type { EffectEntry } from "../wasm/client";
import { segmentResolutionBursts, type ResolutionBurst } from "./bursts";

export type SchedulerStep = {
  burst: ResolutionBurst;
  entry: EffectEntry;
  durationMs: number;
  reducedMotion: boolean;
};

export type SchedulerPresentation = {
  animations?: Animation[];
  done?: Promise<void>;
};

export type SchedulerPresenter = (step: SchedulerStep) => SchedulerPresentation | Promise<SchedulerPresentation | void> | void;

export type SchedulerSettleHook = () => Promise<void> | void;

export type EffectAnimationSchedulerOptions = {
  presenter?: SchedulerPresenter;
  settle?: SchedulerSettleHook;
  reducedMotion?: boolean;
  rate?: number;
  defaultDurationMs?: number;
  reducedMotionDurationMs?: number;
  animationSource?: Pick<Document, "getAnimations">;
};

const DEFAULT_DURATION_MS = 180;
const REDUCED_MOTION_DURATION_MS = 80;

export class EffectAnimationScheduler {
  private readonly presenter: SchedulerPresenter;
  private readonly settle: SchedulerSettleHook;
  private readonly animationSource: Pick<Document, "getAnimations"> | null;
  private readonly defaultDurationMs: number;
  private readonly reducedMotionDurationMs: number;
  private queue: SchedulerStep[] = [];
  private running = false;
  private flushing = false;
  private rate: number;
  private reducedMotion: boolean;
  private inFlightAnimations: Animation[] = [];

  constructor(options: EffectAnimationSchedulerOptions = {}) {
    this.presenter = options.presenter ?? (() => undefined);
    this.settle = options.settle ?? (() => undefined);
    this.animationSource = options.animationSource ?? (typeof document === "undefined" ? null : document);
    this.defaultDurationMs = options.defaultDurationMs ?? DEFAULT_DURATION_MS;
    this.reducedMotionDurationMs = options.reducedMotionDurationMs ?? REDUCED_MOTION_DURATION_MS;
    this.rate = normalizeRate(options.rate ?? 1);
    this.reducedMotion = options.reducedMotion ?? false;
  }

  enqueueEffects(effects: EffectEntry[]): Promise<void> {
    return this.enqueueBursts(segmentResolutionBursts(effects));
  }

  enqueueBursts(bursts: ResolutionBurst[]): Promise<void> {
    for (const burst of bursts) {
      for (const entry of burst.visibleEntries) {
        this.queue.push({
          burst,
          entry,
          durationMs: this.durationFor(entry),
          reducedMotion: this.reducedMotion,
        });
      }
    }
    return this.drain();
  }

  setReducedMotion(reducedMotion: boolean): void {
    this.reducedMotion = reducedMotion;
  }

  setRate(rate: number): void {
    this.rate = normalizeRate(rate);
    for (const animation of this.currentAnimations()) {
      animation.playbackRate = this.rate;
    }
  }

  async flush(): Promise<void> {
    this.flushing = true;
    for (const animation of this.currentAnimations()) {
      animation.finish();
    }
    this.inFlightAnimations = [];
    this.queue = [];
    await this.runSettle();
    this.flushing = false;
  }

  get pendingSteps(): number {
    return this.queue.length;
  }

  private async drain(): Promise<void> {
    if (this.running) {
      return;
    }
    this.running = true;
    try {
      while (this.queue.length > 0 && !this.flushing) {
        const step = this.queue.shift();
        if (!step) {
          break;
        }
        await this.runStep(step);
      }
      if (!this.flushing) {
        await this.runSettle();
      }
    } finally {
      this.running = false;
    }
  }

  private async runStep(step: SchedulerStep): Promise<void> {
    const presentation = await this.presenter(step);
    const animations = presentation?.animations ?? [];
    this.inFlightAnimations = animations;
    for (const animation of animations) {
      animation.playbackRate = this.rate;
    }
    await presentation?.done;
    await dwell(this.scaledDuration(step.durationMs));
    this.inFlightAnimations = [];
  }

  private async runSettle(): Promise<void> {
    await this.settle();
  }

  private durationFor(_entry: EffectEntry): number {
    return this.reducedMotion ? this.reducedMotionDurationMs : this.defaultDurationMs;
  }

  private scaledDuration(durationMs: number): number {
    return Math.max(0, Math.round(durationMs / this.rate));
  }

  private currentAnimations(): Animation[] {
    return [...this.inFlightAnimations, ...(this.animationSource?.getAnimations() ?? [])];
  }
}

function normalizeRate(rate: number): number {
  return Number.isFinite(rate) && rate > 0 ? rate : 1;
}

function dwell(durationMs: number): Promise<void> {
  if (durationMs <= 0) {
    return Promise.resolve();
  }
  return new Promise((resolve) => {
    globalThis.setTimeout(resolve, durationMs);
  });
}
