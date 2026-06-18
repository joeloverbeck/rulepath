import type { GameCatalogEntry, PublicView, SeatDisplayLabel } from "../wasm/client";

export type SeatFrameViewerMode = { kind: "observer" } | { kind: "seat"; seat: string };

type SeatFrameProps = {
  game: GameCatalogEntry | null;
  view: PublicView | null;
  viewerMode: SeatFrameViewerMode;
  onViewerModeChange?: (viewerMode: SeatFrameViewerMode) => void;
};

type SeatProjection = {
  active_seat?: string | null;
  active_seat_labels?: SeatDisplayLabel[];
  active_seats?: string[];
  pending_seats?: string[];
  pending_responders?: string[];
};

export function SeatFrame({ game, view, viewerMode, onViewerModeChange }: SeatFrameProps) {
  const seats = activeSeatLabels(view, game);
  const activeSeats = projectedSeatSet(view, "active");
  const pendingSeats = projectedSeatSet(view, "pending");
  const selectedSeat = viewerMode.kind === "seat" ? viewerMode.seat : null;
  const viewerDisabled = !onViewerModeChange;

  return (
    <section className="seat-frame" aria-label="Seats">
      <fieldset className="seat-frame-viewers" aria-label="Viewer">
        <legend>Viewer</legend>
        <label className={viewerMode.kind === "observer" ? "selected" : ""}>
          <input
            type="radio"
            name="seat-frame-viewer"
            value="observer"
            checked={viewerMode.kind === "observer"}
            disabled={viewerDisabled}
            onChange={() => onViewerModeChange?.({ kind: "observer" })}
          />
          <span>Observer</span>
        </label>
        {seats.map((seat) => (
          <label
            key={seat.seat}
            className={selectedSeat === seat.seat ? "selected" : ""}
          >
            <input
              type="radio"
              name="seat-frame-viewer"
              value={seat.seat}
              checked={selectedSeat === seat.seat}
              disabled={viewerDisabled}
              onChange={() => onViewerModeChange?.({ kind: "seat", seat: seat.seat })}
            />
            <span>{seat.label}</span>
          </label>
        ))}
      </fieldset>

      <ol className="seat-frame-rail">
        {seats.map((seat) => {
          const active = activeSeats.has(seat.seat);
          const pending = pendingSeats.has(seat.seat);
          return (
            <li
              key={seat.seat}
              className={[active ? "active" : "", pending ? "pending" : "", selectedSeat === seat.seat ? "viewing" : ""]
                .filter(Boolean)
                .join(" ")}
              data-seat={seat.seat}
            >
              <span>{seat.label}</span>
              <small>{seatStatus(active, pending, selectedSeat === seat.seat)}</small>
            </li>
          );
        })}
      </ol>
    </section>
  );
}

function activeSeatLabels(view: PublicView | null, game: GameCatalogEntry | null): SeatDisplayLabel[] {
  if (view) {
    const projection = view as SeatProjection;
    if (Array.isArray(projection.active_seat_labels) && projection.active_seat_labels.length > 0) {
      return projection.active_seat_labels;
    }
  }
  return catalogSeatLabels(game);
}

function catalogSeatLabels(game: GameCatalogEntry | null): SeatDisplayLabel[] {
  const labels = game?.seat_labels ?? game?.ui?.seat_labels ?? [];
  return labels.length
    ? labels
    : [
        { seat: "seat_0", label: "Seat 0" },
        { seat: "seat_1", label: "Seat 1" },
      ];
}

function projectedSeatSet(view: PublicView | null, kind: "active" | "pending"): Set<string> {
  if (!view) {
    return new Set();
  }
  const projection = view as SeatProjection;
  if (kind === "active") {
    if (Array.isArray(projection.active_seats)) {
      return new Set(projection.active_seats);
    }
    return projection.active_seat ? new Set([projection.active_seat]) : new Set();
  }
  return new Set([...(projection.pending_seats ?? []), ...(projection.pending_responders ?? [])]);
}

function seatStatus(active: boolean, pending: boolean, viewing: boolean): string {
  if (active && viewing) return "Active view";
  if (active) return "Active";
  if (pending) return "Pending";
  if (viewing) return "Viewing";
  return "Waiting";
}
