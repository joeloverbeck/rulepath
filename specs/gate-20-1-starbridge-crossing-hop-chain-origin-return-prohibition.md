# Gate 20.1 — Starbridge Crossing hop-chain origin-return prohibition

## 1. Header

| Field | Value |
|---|---|
| Spec ID | `gate-20-1-starbridge-crossing-hop-chain-origin-return-prohibition` |
| Stage / unit | Public scaling phase — Gate 20 correctness follow-on (post-`Done`) |
| Gate | Gate 20.1 (correctness fix on shipped Gate 20 `starbridge_crossing`) |
| Status | Planned (spec written; not yet executed) |
| Date | 2026-06-27 |
| Owner | TBD |
| Authority order | `docs/FOUNDATIONS.md` → `docs/adr/0009-*` (replay/fixture/hash taxonomy v2) → `games/starbridge_crossing/docs/RULES.md` → this spec |

This follows the established post-gate correctness-spec pattern used for
Gate 19.1 / 19.2 and the River Ledger correctness specs: a shipped official
game has a legality defect that requires a rule-text clarification and a
governed determinism migration, neither of which is a bounded ticket-only
change.

## 2. Objective

Make a Starbridge Crossing hop chain unable to land on the moving peg's **own
origin space** (its space at the start of the turn). This closes a defect in the
shipped Gate 20 game where a peg can hop out and back to where it started,
producing a committable **net-zero no-op turn** — a back-door voluntary pass
that contradicts the game's documented turn model.

## 3. Scope

**In scope**

- Rust legality: hop-chain enumeration (`games/starbridge_crossing/src/actions.rs`)
  and validation (`games/starbridge_crossing/src/rules.rs`) must treat the
  moving peg's origin space as a forbidden landing for the entire chain.
- New rule `SC-MOVE-010` in `games/starbridge_crossing/docs/RULES.md` stating
  that a hop chain may not land on the moving peg's own origin space (a committed
  turn must change board occupancy unless Rust issues `pass_blocked`),
  cross-referencing `SC-TURN-002` / `SC-MOVE-009`, with a Rule-ID Migration Note.
  `SC-MOVE-007` (the action-tree-finiteness / no-revisit-landing rule) and its
  `SOURCES.md` `SC-AMB-004` anchor are left unchanged — origin-return is a
  distinct turn-model concern, not a finiteness concern.
- `games/starbridge_crossing/docs/RULE-COVERAGE.md` row + evidence for the new
  `SC-MOVE-010` (cross-referencing `SC-TURN-002` / `SC-MOVE-009`, whose intent
  this defect violated).
- TDD: a failing-first test asserting a return-to-origin chain is rejected by
  `validate_jump_command` and is absent from the enumerated action tree, plus a
  regression that legitimate multi-hop chains (including direction changes and
  stop-midway) are unchanged.
- Governed determinism migration (ADR 0009): regenerate only the affected
  action-tree-hash / golden-trace / replay / benchmark artifacts whose states
  offered an origin-return continuation, with an explicit per-artifact authority
  note. No blanket golden regeneration.

**Out of scope**

- Any other movement, finish, terminal, visibility, bot-algorithm, or UI
  behavior change.
- New variants, seat counts, or piece counts.
- Web-shell behavior beyond what naturally follows from the Rust action tree no
  longer exposing the origin-return continuation (TypeScript presents Rust
  output only; no TS legality change).

**Not allowed**

- TypeScript deciding adjacency, jump midpoints, repeated-landing legality, or
  any path legality (`SC-UI-001`).
- Introducing a strategic/voluntary pass (`SC-MOVE-009`).
- Blanket golden-trace/hash regeneration outside ADR 0009 per-artifact authority.
- Any `engine-core` topology/path/jump noun.

## 4. Deliverables

- Updated `games/starbridge_crossing/src/rules.rs::legal_jump_landings` with a
  single internal `if landing == origin { continue; }` guard (the function
  already computes `origin`) so the origin space cannot be a landing anywhere in
  the chain. This one chokepoint covers both the action-tree enumeration
  (`actions.rs::jump_landing_choices`) and validation (`validate_jump_command`).
- Updated `games/starbridge_crossing/docs/RULES.md` (new `SC-MOVE-010` +
  Rule-ID Migration Note) and `RULE-COVERAGE.md`.
- New/updated tests in `games/starbridge_crossing/tests/rules.rs`, plus a
  `tests/property.rs` assertion that every committed non-`pass_blocked` turn
  changes board occupancy (A2's invariant is property-shaped).
- Regenerated, authority-annotated evidence artifacts (golden traces, replay
  fixtures, benchmark baselines) strictly limited to those that changed, under
  ADR 0009.

## 5. Work breakdown (candidate AGENT-TASKs)

1. **STACROSORIG-001** — RED test: add `hop_chain_cannot_return_to_origin_space`
   to `tests/rules.rs` reproducing `origin → A → origin` and asserting rejection
   (`validate_jump_command` error) and action-tree absence. Confirm it fails on
   `main`.
2. **STACROSORIG-002** — GREEN fix: add an explicit `landing == origin` guard in
   `legal_jump_landings` (the single chokepoint for both enumeration and
   validation; it already computes `origin`), keeping `occupancy_during_chain`
   semantics for genuine mid-chain spaces. Add the `tests/property.rs` assertion
   that every committed non-`pass_blocked` turn changes board occupancy (A2).
   Verify legitimate chains unchanged.
3. **STACROSORIG-003** — Rules update: add new `SC-MOVE-010` rule text +
   Rule-ID Migration Note; `RULE-COVERAGE.md` evidence row + trace; cross-reference
   `SC-TURN-002` / `SC-MOVE-009`. Leave `SC-MOVE-007` and `SOURCES.md`
   `SC-AMB-004` unchanged.
4. **STACROSORIG-004** — Governed migration: run `fixture-check` / `replay-check`
   / `rule-coverage`, regenerate only changed artifacts with per-artifact ADR
   0009 authority notes; refresh `GAME-EVIDENCE.md`; capture CI receipts.

Dependency order: 001 → 002 → 003 → 004.

## 6. Exit criteria

- `validate_jump_command` rejects any chain landing on the moving peg's origin
  space; the enumerated action tree never offers it.
- All legitimate hop-chain behavior (`SC-MOVE-003`..`SC-MOVE-006`) is preserved,
  proven by unchanged direction-change / stop-midway tests.
- No turn can leave the board occupancy identical to its pre-turn state except a
  Rust-issued `pass_blocked` (`SC-MOVE-009`).
- `RULES.md` carries the new `SC-MOVE-010` and `RULE-COVERAGE.md` its evidence
  row with a migration note; `SC-MOVE-007` is unchanged;
  `node scripts/check-doc-links.mjs` passes.
- CI gate 0 (`fmt`, `clippy -D warnings`, `build`, `test`) and gate 1
  (`simulate`, `replay-check --all`, `fixture-check`, `rule-coverage`,
  `boundary-check.sh`, web smokes) pass with only ADR-0009-authorized artifact
  diffs.

## 7. Acceptance evidence

- Failing-first unit test (STACROSORIG-001) transcript, then green.
- `cargo test -p starbridge_crossing` and workspace test pass.
- `cargo run -p replay-check -- --game starbridge_crossing --all` and
  `cargo run -p fixture-check -- --game starbridge_crossing` pass with annotated
  diffs only.
- `cargo run -p rule-coverage -- --game starbridge_crossing` shows `SC-MOVE-010`
  covered by the new test/trace.
- Web e2e (`node apps/web/e2e/starbridge-crossing.smoke.mjs`) still green; manual
  Puppeteer reproduction (the `origin → A → origin` no-op) no longer offered.

## 8. FOUNDATIONS & boundary alignment

- **Determinism / explicit migration** — legality change alters the action tree
  and dependent hashes; migration is explicit and ADR-0009-governed, not silent
  (FOUNDATIONS §11; "No blanket golden regeneration").
- **Stop-and-reassess (§12)** — the defect surfaced during play; rather than
  unilaterally regenerating fixtures, this spec scopes the governed change.
- **Behavior authority** — Rust owns hop legality; TypeScript unaffected
  (`SC-UI-001`).
- **engine-core noun-freedom** — fix stays inside `games/starbridge_crossing`.

## 9. Forbidden changes

- No new pass option, variant, or piece/seat count.
- No change to `occupancy_during_chain` semantics for non-origin spaces.
- No TypeScript legality.
- No regeneration of artifacts whose state never offered an origin-return node.

## 10. Documentation updates required

- `games/starbridge_crossing/docs/RULES.md` — new `SC-MOVE-010` + migration note.
- `games/starbridge_crossing/docs/RULE-COVERAGE.md` — evidence rows.
- `games/starbridge_crossing/docs/GAME-EVIDENCE.md` — fix receipt.
- `specs/README.md` — add the Gate 20.1 tracker row; flip to `Done` at closeout.
- Web-shell catalog docs (`apps/web/README.md`): `not applicable` — no catalog,
  renderer-list, or smoke-list membership change (game already listed).

## 11. Sequencing

- Predecessor: Gate 20 (`Done`).
- Successor: does not block Gate 21; it is an independent correctness follow-on
  on the shipped Gate 20 game and may be executed at any time before public
  release (the Gate 20 closeout already notes IP/public-release review pending).

## 12. Assumptions (one-line-correctable)

- A1: The engine's intent is that a turn must change board occupancy unless Rust
  issues `pass_blocked` (grounded in `SC-TURN-002` + `SC-MOVE-009`).
- A2: Returning the moving peg to its exact origin space is the only no-op-turn
  vector (steps require an empty adjacent destination, so a step is never a
  no-op; non-origin hop landings always net-displace the peg).
- A3: Affected evidence artifacts are a small subset (only states that offered an
  origin-return continuation), so the ADR-0009 migration is bounded.
