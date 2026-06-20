# GAT16BRICIRTRI-004: Crate skeleton, workspace wiring, typed card model, and phase state

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new crate `games/briar_circuit` (`src/{lib,ids,cards,state}.rs`, `data/{manifest,variants}.toml`); root `Cargo.toml` workspace member
**Deps**: 001, 003

## Problem

Gate 16 needs a new coexisting official game crate. This ticket creates the `games/briar_circuit` skeleton: the typed game-local card model (`Suit`, `Rank`, `Card`, stable `CardId`, canonical deterministic ordering), the explicit phase/state model (`Passing`, `PlayingTrick`, `ScoringHand`, `Terminal` plus substate), the fixed-four-seat setup diagnostic, and the workspace/data wiring every later module ticket builds on. All trick-taking vocabulary stays game-local Rust — `engine-core` gains no mechanic noun.

## Assumption Reassessment (2026-06-20)

1. `games/plain_tricks/` and `games/river_ledger/` are the convention exemplars: river_ledger has a dedicated `src/cards.rs` (precedent for a typed card model), and both follow the `src/{lib,ids,state,...}.rs` + `data/{manifest,variants}.toml` layout. No `games/briar_circuit/` exists yet; root `Cargo.toml` lists 15 game members, none `briar_circuit`.
2. `specs/gate-16-briar-circuit-trick-taking.md` §4.1 (target tree), §4.2 (Setup/Card model/Phase-state rows), and Appendix B.1 (state phases) fix this content; §4.1 permits a module-split correction during reassessment without broadening scope.
3. Cross-crate boundary under audit: `engine-core` supplies generic `SeatId`/`Seed`/`DeterministicRng`/serialization contracts (`crates/engine-core/src/lib.rs`, verified noun-free); `Suit`/`Rank`/`Card`/`CardId`/phase types are game-local in `games/briar_circuit` and MUST NOT leak into the kernel.
4. FOUNDATIONS §3 (`engine-core` is a contract kernel) is the principle under audit: card/suit/rank/trick/pass/hearts/moon nouns are introduced here as game-local types only. A game-specific type in `engine-core` would be a boundary-failure; `bash scripts/boundary-check.sh` must stay green.
5. This ticket builds the substrate for two deferred enforcement surfaces: deterministic serialization/replay-hash (the canonical card/seat ordering authored here, enforced by GAT16BRICIRTRI-010) and the no-leak visibility firewall (the private-hand state shape, enforced by GAT16BRICIRTRI-009). The card model and state introduce no nondeterministic input (no wall-clock/hash-map iteration in canonical forms) and no field that forces a later leak; private hands live in seat-keyed substate projected owner-only downstream.

## Architecture Check

1. A dedicated `cards.rs` + explicit phase enum (over inlining cards into `state.rs`) mirrors the river_ledger precedent and gives serialization, action trees, tests, and explanations one canonical card ordering to reference.
2. No backwards-compatibility aliasing/shims — greenfield crate; all files `(new)` except the additive root `Cargo.toml` member line.
3. `engine-core` stays free of mechanic nouns (§3); no `game-stdlib` change (§4) — the second-use keep-local decision (GAT16BRICIRTRI-003) already settled that trick logic stays game-local.

## Verification Layers

1. 52 unique cards with deterministic canonical `CardId` ordering -> unit + property test (`tests/property.rs`) over the full deck.
2. Fixed-four-seat setup accepts 4 and rejects all other counts with a stable diagnostic -> `tests/rules.rs` positive/negative cases.
3. `engine-core` gains no mechanic noun -> `bash scripts/boundary-check.sh` grep-proof.
4. Crate compiles and is wired into the workspace -> `cargo build -p briar_circuit`.

## What to Change

### 1. Crate skeleton and workspace wiring

Create `games/briar_circuit/Cargo.toml`, the `src/` module stubs (`lib.rs`, `ids.rs`, `cards.rs`, `state.rs`, plus empty-but-declared `actions.rs`, `rules.rs`, `setup.rs`, `scoring.rs`, `effects.rs`, `visibility.rs`, `replay_support.rs`, `bots.rs`, `ui.rs`, `variants.rs` so later tickets fill them), and add `games/briar_circuit` to the root `Cargo.toml` members list.

### 2. Typed card model (`src/cards.rs`, `src/ids.rs`)

Game-local `Suit`, `Rank`, `Card`, stable `CardId`, and canonical deterministic ordering used for serialization, action trees, tests, and explanations.

### 3. Phase/state model (`src/state.rs`)

Explicit `Passing`, `PlayingTrick`, `ScoringHand`, `Terminal` states plus deterministic substate (pending pass seats, current trick, captured cards/points, dealer, hand number, hearts-broken, cumulative scores, active seat) per Appendix B.1, with seat-keyed private hands held for owner-only projection.

### 4. `data/manifest.toml`, `data/variants.toml`

Typed identity, display metadata, version anchors (`briar-circuit-data-v1`), official seat metadata (fixed 4), and the `briar_circuit_standard` variant — no behavior-looking fields.

## Files to Touch

- `games/briar_circuit/Cargo.toml` (new)
- `games/briar_circuit/src/lib.rs` (new)
- `games/briar_circuit/src/ids.rs` (new)
- `games/briar_circuit/src/cards.rs` (new)
- `games/briar_circuit/src/state.rs` (new)
- `games/briar_circuit/src/{actions,rules,setup,scoring,effects,visibility,replay_support,bots,ui,variants}.rs` (new — stubs)
- `games/briar_circuit/data/manifest.toml` (new)
- `games/briar_circuit/data/variants.toml` (new)
- `games/briar_circuit/tests/{rules,property,serialization}.rs` (new)
- `Cargo.toml` (modify — add workspace member)

## Out of Scope

- Deterministic shuffle/deal and dealer/pass-cycle rotation (GAT16BRICIRTRI-005).
- Pass and play action generation/validation (GAT16BRICIRTRI-006/007).
- WASM/CI registration (GAT16BRICIRTRI-012/013) — this ticket opens an expected `check-ci-games` red window until the `ci/games.json` row lands in GAT16BRICIRTRI-012.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit --test property` — 52 unique cards, deterministic ordering.
2. `cargo test -p briar_circuit --test rules` — fixed-four-seat accept/reject diagnostic.
3. `cargo build -p briar_circuit && bash scripts/boundary-check.sh` — crate compiles, kernel stays noun-free.

### Invariants

1. `engine-core` contains no `card`/`suit`/`rank`/`trick`/`pass` noun (§3); all are game-local types.
2. Card and seat ordering are deterministic and canonical (§2/§11) — no hash-map iteration or wall-clock in canonical forms.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/property.rs` — 52-unique-card and deterministic-ordering properties.
2. `games/briar_circuit/tests/rules.rs` — seat-count accept/reject diagnostics.
3. `games/briar_circuit/tests/serialization.rs` — canonical card/seat serialization order (round-trip).

### Commands

1. `cargo test -p briar_circuit`
2. `cargo build --workspace && bash scripts/boundary-check.sh`
3. A workspace build is the correct boundary because the deliverable's risk is kernel-boundary leakage and workspace wiring, both proven by a full build + boundary check.
