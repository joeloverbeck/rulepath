import { resolveSeatLabel, resolveSeatLabels } from "./seatLabels";
import type { SeatDisplayLabel } from "./wasm/client";

function assertEqual(actual: unknown, expected: unknown, message: string): void {
  if (actual !== expected) {
    throw new Error(`${message}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`);
  }
}

function assertDeepEqual(actual: unknown, expected: unknown, message: string): void {
  const actualJson = JSON.stringify(actual);
  const expectedJson = JSON.stringify(expected);
  if (actualJson !== expectedJson) {
    throw new Error(`${message}: expected ${expectedJson}, got ${actualJson}`);
  }
}

const activeSeatLabels: SeatDisplayLabel[] = [
  { seat: "seat_0", label: "View Seat 1" },
  { seat: "seat_1", label: "View Seat 2" },
];
const catalogSeatLabels: SeatDisplayLabel[] = [
  { seat: "seat_0", label: "Catalog Seat 1" },
  { seat: "seat_1", label: "Catalog Seat 2" },
];
const catalogUiSeatLabels: SeatDisplayLabel[] = [
  { seat: "seat_0", label: "Ui Seat 1" },
  { seat: "seat_1", label: "Ui Seat 2" },
];

assertEqual(
  resolveSeatLabel("seat_0", { activeSeatLabels, catalogSeatLabels, catalogUiSeatLabels }),
  "View Seat 1",
  "view-projected labels take precedence"
);

assertEqual(
  resolveSeatLabel("seat_1", { catalogSeatLabels, catalogUiSeatLabels }),
  "Catalog Seat 2",
  "catalog labels are the second source"
);

assertEqual(
  resolveSeatLabel("seat_0", { catalogUiSeatLabels }),
  "Ui Seat 1",
  "catalog ui labels are the third source"
);

assertEqual(resolveSeatLabel("seat_3"), "Seat 4", "fallback is one-based for seat ids");
assertEqual(resolveSeatLabel("observer"), "observer", "fallback preserves non-seat ids");

const warnings: string[] = [];
assertEqual(
  resolveSeatLabel("seat_4", { dev: true, warn: (message) => warnings.push(message) }),
  "Seat 5",
  "dev fallback still returns the one-based label"
);
assertEqual(warnings.length, 1, "dev fallback emits one warning");
assertEqual(
  warnings[0],
  "Missing Rust-supplied label for seat_4; using defensive fallback.",
  "dev fallback warning names the missing seat"
);

assertDeepEqual(
  resolveSeatLabels(["seat_0", "seat_1"], { catalogSeatLabels }),
  [
    { seat: "seat_0", label: "Catalog Seat 1" },
    { seat: "seat_1", label: "Catalog Seat 2" },
  ],
  "list resolver returns seat-label entries"
);
