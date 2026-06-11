# EFFMAP-002: Cross-game effect-feedback regression guard (no `undefined`/`null`/`[object Object]`/`NaN`)

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new test/harness under `apps/web/scripts/` (+ a `package.json` script). No Rust, schema, trace, or product-behavior change.
**Deps**: EFFMAP-001 (so the masked_claims fix is in place when the guard runs; the guard is authored to pass only after EFFMAP-001).

## Problem

The `undefined is now active` defect (EFFMAP-001) was possible because two structural hazards have no regression coverage:

1. Effect `type` discriminators are a **flat global namespace** consumed by one TS `switch` (`apps/web/src/components/effectFeedback.ts`), yet each game defines its effect vocabulary independently in Rust — nothing guarantees a discriminator maps to a single payload shape across games.
2. The TS `EffectEntry.payload` type is a **loose open bag** (`{ type: string; [key: string]: unknown }`, `apps/web/src/wasm/client.ts:835`), so reading a field a given variant does not emit yields `undefined` at runtime with **no compile-time error**.

There is currently no test that renders effects through `feedbackForEffect` and checks the output. This ticket adds a cross-game guard that renders the real effects every game emits and fails if any rendered `title`/`detail` contains a sentinel of un-projected data (`undefined`, `null`, `[object Object]`, `NaN`). It catches the EFFMAP-001 class for all games and covers the currently-fragile `round_scored` field-presence branch (`effectFeedback.ts:168`).

## Assumption Reassessment (2026-06-11)

1. `apps/web` has **no unit-test framework** — `apps/web/package.json` has no `test` script and no vitest/jest dependency; the only automated coverage is node `assert`-based smoke harnesses (`apps/web/scripts/smoke-ui.mjs`, `smoke-load-wasm.mjs`) run via `npm run smoke:ui` / `smoke:wasm`. The guard MUST follow this existing convention (node + `assert`, no new test framework) to avoid scope creep.
2. `smoke-ui.mjs` already starts each game in wasm and collects real semantic effects via `rulepath_get_effects` (e.g. `:101`, and per-game effect assertions at `:217,:282,:377`). It does NOT currently import `feedbackForEffect`. The guard's job is to route those collected effects through the render mapper.
3. Shared boundary under audit: the Rust→browser effect-envelope JSON contract and its single TS consumer `feedbackForEffect` (`apps/web/src/components/effectFeedback.ts`). The render mapper is the surface; the guard is a no-leak-of-`undefined` presentation invariant, not a behavior change.
4. FOUNDATIONS principle restated: §11 "Tests, traces, simulations, benchmarks, docs, and source notes cover the change" and the official-game contract's UI-smoke obligation (`docs/OFFICIAL-GAME-CONTRACT.md` "UI smoke tests cover start, legal action display, one human action, one bot action where applicable, effects, replay stepping…"). This guard strengthens the "effects" leg of that smoke obligation.
5. No hidden-information surface is added: the guard only asserts on already-public effect payloads each game emits to an observer/seat viewer; it introduces no new payload, DOM, or storage surface (§11 no-leak firewall unaffected).
6. Schema/consumer note: the guard depends on the browser-projection effect-envelope shape per game. It must enumerate effects the games actually emit (driven from wasm), not a hand-guessed list, so it stays in sync as games evolve. Where driving a particular effect through wasm is impractical in a smoke pass (rare branches), a small hand-authored fixture table keyed by the real `wasm-api` `type` strings is an acceptable supplement — but the table is a supplement to, not a replacement for, the wasm-driven effects.
7. This ticket renames/removes nothing; it adds a harness + a `package.json` script. Blast radius is additive.
8. Adjacent contradiction handling: the loose `EffectEntry.payload` typing (`client.ts:835`) is the deeper structural cause; fully fixing it (typed discriminated union) is deliberately OUT OF SCOPE here and recorded as possible future hardening — this ticket buys the regression safety net at smoke-test cost without the union rewrite.

## Architecture Check

1. A runtime render-and-assert guard over real per-game effects is the highest-value, lowest-new-infrastructure option: it catches divergent-shape collisions and missing-field projections for ALL games (current and future) without typing every effect payload. It is strictly cheaper than the typed-discriminated-union alternative and directly exercises the real failure path (`feedbackForEffect` output).
2. No backwards-compatibility shim introduced; the guard is net-new.
3. `engine-core`/`game-stdlib` untouched; no mechanic-atlas pressure (presentation test only).

## Verification Layers

1. Every effect each game emits renders a `title`/`detail` free of `undefined`/`null`/`[object Object]`/`NaN` -> new node guard rendering wasm-collected effects through `feedbackForEffect`.
2. The guard actually exercises `masked_claims` turn-advance and score-change (the EFFMAP-001 cases) and `round_scored` for plain_tricks + high_card_duel -> per-game coverage assertion in the guard (fail if a targeted game/effect was never observed).
3. Guard is wired into the web CI gate -> `package.json` script + invocation in the documented web command set (`CLAUDE.md` web gate / `docs/` smoke command list).

## What to Change

### 1. New guard harness `apps/web/scripts/smoke-effect-feedback.mjs`

- Reuse the `smoke-ui.mjs` pattern: load the built wasm, start each catalog game, drive enough actions (including a bot action where applicable) to surface its representative effects, and collect them via `rulepath_get_effects`.
- Render each collected effect through `feedbackForEffect`. Because `effectFeedback.ts` is TypeScript and the harness is node, transpile it on the fly with the esbuild instance already shipped as a Vite dependency (e.g. `esbuild.transform`/`build` on `src/components/effectFeedback.ts`), or import it from the production build output if the build exposes it. (Bounded implementation choice — pick the simpler of the two at implementation time; do not add a new test framework.)
- For each rendered `{ title, detail }`, `assert` it contains none of `undefined`, `null`, `[object Object]`, `NaN`.
- Assert coverage: fail if `masked_claims` `claim_turn_advanced` + `claim_score_changed`, and `round_scored` for both `plain_tricks` and `high_card_duel`, were not observed in the run (guards against the test silently skipping the regression cases). Supplement with a small fixture table only for effects not reachable in a smoke pass.

### 2. Wire it into the web gate

- Add `"smoke:effects": "npm run build && node scripts/smoke-effect-feedback.mjs"` to `apps/web/package.json`.
- Add the command to the web verification set in `CLAUDE.md` (Web gate) and any `docs/` smoke-command listing so CI runs it.

## Files to Touch

- `apps/web/scripts/smoke-effect-feedback.mjs` (new)
- `apps/web/package.json` (modify — add `smoke:effects` script)
- `CLAUDE.md` (modify — add the command to the Web CI gate list)

## Out of Scope

- Replacing the loose `EffectEntry.payload` bag with a typed discriminated union (deferred future hardening; note in the harness header).
- Any Rust, schema, trace, hash, or game-behavior change.
- Rendering animation/visual correctness (this guard only asserts the textual feedback projection has no un-projected sentinels).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:effects` passes with EFFMAP-001 applied, and (proof of efficacy) FAILS if EFFMAP-001 is reverted (the masked_claims turn-advance renders `undefined`).
2. The guard reports coverage of the named regression cases (masked_claims `claim_turn_advanced`/`claim_score_changed`; `round_scored` for plain_tricks and high_card_duel) and fails if any was not observed.
3. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:effects`.

### Invariants

1. No game's rendered effect feedback contains `undefined`/`null`/`[object Object]`/`NaN`.
2. The guard derives its effect set from real wasm-emitted effects (not a hand-guessed list), so new/changed game effects are covered as the catalog evolves.

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-effect-feedback.mjs` — renders wasm-collected effects for every catalog game through `feedbackForEffect` and asserts no un-projected sentinels; the regression harness for the EFFMAP-001 class.
2. `apps/web/package.json` — `smoke:effects` script wiring.

### Commands

1. `npm --prefix apps/web run smoke:effects`
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:effects`
3. Revert-EFFMAP-001 spot check (manual, once): confirm the guard fails on the original `undefined is now active` to prove it actually catches the class.

## Outcome

Completed: 2026-06-11

What changed:

- Added `apps/web/scripts/smoke-effect-feedback.mjs`, a Node/WASM smoke that bundles the real `feedbackForEffect` mapper, drives representative WASM-emitted effects for every catalog game, and rejects rendered `title`/`detail` text containing `undefined`, `null`, `[object Object]`, or `NaN`.
- The guard asserts the named regression coverage: masked_claims `claim_turn_advanced` and `claim_score_changed`, plus `round_scored` for both plain_tricks and high_card_duel.
- Added the `smoke:effects` package script and documented it in `CLAUDE.md`, `AGENTS.md`, and `apps/web/README.md`.

Deviations from original plan:

- The harness uses esbuild to bundle the TypeScript feedback module at runtime rather than importing from Vite output; this keeps the guard close to the source mapper and avoids adding a new test framework.
- No supplemental hand-authored effect table was needed. The named regression effects are all reached through real WASM actions.

Verification:

- `npm --prefix apps/web run smoke:effects` — passed; checked 66 emitted effects and observed all named regression cases.
- Temporary revert spot check — passed by failing as intended with `masked_claims turn_advanced rendered unprojected data: {"title":"Turn advanced","detail":"undefined is now active.","tone":"turn"}`.
- `npm --prefix apps/web run smoke:wasm` — passed.
- `npm --prefix apps/web run smoke:ui` — passed.
- `node scripts/check-doc-links.mjs` — passed.
