# Veiled Draft Mechanics Inventory

Game ID: `secret_draft`

Roadmap stage/gate: Gate 9.1 simultaneous commitment/reveal proof

Rules version: `secret-draft-rules-v1`

Last updated: 2026-06-08

## Purpose

This inventory records Veiled Draft's game-local mechanic shapes and
primitive-pressure posture. It is evidence for
[../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md).

Veiled Draft is a deterministic two-seat hidden-commitment drafting game. Rust
owns setup, legal actions, hidden commitment storage, reveal timing, conflict
fallback, pool removal, scoring, terminal checks, tie-breaks, effects, replay,
visibility projection, and bot choice. TypeScript presents the Rust/WASM
projection only.

## Mechanic Inventory

| Category | Game-local description | Evidence | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | No board topology. The primary layout is two drafted collections around one visible pool. | [RULES.md](RULES.md), `SecretDraftBoard.tsx` | `local-only` | No board-space primitive pressure. |
| component/zone model | Visible draft pool, per-seat drafted collections, internal commitment slots, reveal history, and public scores. | [RULES.md](RULES.md), `state.rs`, `visibility.rs` | `local-only` | Commitment slots are internal before reveal. |
| action shape | Flat Rust action paths: `commit/<item-id>` for every visible item while the acting seat has not committed. | `actions.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md) | `local-only` | TypeScript never builds or filters legal choices. |
| turn/phase model | Six rounds; both uncommitted seats may act until each has one commitment; reveal resolution then advances or ends. | `rules.rs`, golden traces | `local-only first use` | Simultaneous commitment pressure is recorded in the atlas, not promoted. |
| randomness/chance | No random setup, pool order, reveal, fallback, scoring, or tie-break. Bot seeds affect bot choice only. | [SOURCES.md](SOURCES.md), replay traces | `local-only` | No RNG primitive pressure. |
| visibility/hidden information | Committed item ids are internal until synchronized reveal; pending booleans are public. | `visibility.rs`, `secret-draft.smoke.mjs` | `local-only first use` | Even the committing seat's browser view is redacted pre-reveal. |
| commitment/reveal | First commit emits item-free pending effects; second commit emits grouped reveal and award effects. | `effects.rs`, golden traces | `local-only first use` | No generic reveal helper is added. |
| drafting/pool removal | Two public items leave the visible pool per reveal: two distinct choices or contested item plus deterministic fallback. | `rules.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md) | `local-only first use` | Static data lists item facts only; rules live in Rust. |
| conflict fallback | Priority seat wins contested item; other seat receives lowest stable-order remaining item. | `rules.rs`, contested-pick trace | `local-only` | Deterministic and public after reveal. |
| scoring/outcome | Base value, complete sets, high-thread bonuses, terminal conflict-discipline bonus, and public tie-break ladder. | `rules.rs`, [RULES.md](RULES.md) | `local-only` | All score facts are public. |
| semantic effect shape | Commitment placed, pending seats, reveal batch, choices revealed, draft resolved, score changed, round advanced, terminal. | `effects.rs`, golden traces | `local-only` | Effects drive logs, replay, and UI feedback. |
| UI interaction pattern | Board shows visible pool buttons, pending seats, priority, scores, drafted collections, reveal history, replay, and reduced-motion path. | [UI.md](UI.md), `SecretDraftBoard.tsx` | `local-only` | Pending anchors use seat/round ids, not item ids. |
| bot policy pattern | Level 0 random legal plus Level 1 public heuristic: set completion, value, high-thread bonus, fallback safety. | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | `local-only` | No hidden-state sampling or public search. |
| benchmark/performance pressure | Legal action generation, validate/apply, reveal resolution, projection, replay export, Level 1 decision. | [BENCHMARKS.md](BENCHMARKS.md) | `local-only` | Smoke floors are documented. |

## Primitive Pressure Decision

Simultaneous commitment/reveal plus visible draft-pool removal remains
game-local for Gate 9.1. This is the first official local use, so it is not
evidence for a `game-stdlib` primitive by itself.

The repeated-shape pressure to watch later is hidden commitment with pending
booleans, synchronized reveal batches, redacted viewer-scoped export, and public
conflict resolution. A second official use should update the mechanic atlas
candidate row; a helper still requires the atlas/ADR process before promotion.

## Review Checklist

- `engine-core` remains noun-free.
- `game-stdlib` receives no commitment, reveal, draft pool, pending-seat,
  conflict, or scoring primitive in Gate 9.1.
- Browser controls and effect rows present Rust payloads only.
- Pre-reveal committed item IDs stay out of browser surfaces.
