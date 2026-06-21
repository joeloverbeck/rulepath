import type { SeatDisplayLabel } from "./wasm/client";

export type SeatLabelSources = {
  activeSeatLabels?: SeatDisplayLabel[] | null;
  catalogSeatLabels?: SeatDisplayLabel[] | null;
  catalogUiSeatLabels?: SeatDisplayLabel[] | null;
  warn?: (message: string) => void;
  dev?: boolean;
};

export function resolveSeatLabel(seat: string, sources: SeatLabelSources = {}): string {
  const supplied = findSeatLabel(seat, sources.activeSeatLabels)
    ?? findSeatLabel(seat, sources.catalogSeatLabels)
    ?? findSeatLabel(seat, sources.catalogUiSeatLabels);
  if (supplied) {
    return supplied;
  }
  if (sources.dev ?? isDev()) {
    sources.warn?.(`Missing Rust-supplied label for ${seat}; using defensive fallback.`);
  }
  return fallbackSeatLabel(seat);
}

export function resolveSeatLabels(seats: string[], sources: SeatLabelSources = {}): SeatDisplayLabel[] {
  return seats.map((seat) => ({ seat, label: resolveSeatLabel(seat, sources) }));
}

function findSeatLabel(seat: string, labels: SeatDisplayLabel[] | null | undefined): string | null {
  const label = labels?.find((entry) => entry.seat === seat)?.label.trim();
  return label ? label : null;
}

function fallbackSeatLabel(seat: string): string {
  const match = /^seat_(\d+)$/.exec(seat);
  if (!match) {
    return seat;
  }
  return `Seat ${Number(match[1]) + 1}`;
}

function isDev(): boolean {
  return Boolean(import.meta.env?.DEV);
}
