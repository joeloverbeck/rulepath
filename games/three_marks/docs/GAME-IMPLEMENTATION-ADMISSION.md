# Three Marks Game Implementation Admission

Game ID: `three_marks`

Rules version: `three_marks-rules-v1`

Last updated: 2026-06-06

## Admission summary

Three Marks is admissible as the Gate 4 official board-smoke game once the capstone verifies the CLI, UI, replay, docs, boundary, and CI evidence. This document records the implementation posture before the Gate 4 final status flip.

## Checklist

| Area | Evidence | Status | Notes |
|---|---|---|---|
| rules/source docs | `RULES.md`, `SOURCES.md` | complete | original Rulepath prose and public-domain/classic-family context |
| Rust setup/rules | `src/setup.rs`, `src/actions.rs`, `src/rules.rs` | complete | no board nouns in `engine-core` |
| semantic effects | `src/effects.rs` | complete | viewer-safe public effects |
| visibility | `src/visibility.rs`, visibility tests | complete | perfect information; private view not applicable |
| bots | `src/bots.rs`, bot tests | complete | Level 0 and Level 1 only |
| replay/traces | `src/replay_support.rs`, golden traces | complete | state/effect/action-tree/view/replay hashes |
| WASM/API | `crates/wasm-api`, `smoke:wasm` | complete | static registry includes `three_marks` |
| web UI | `ThreeMarksBoard.tsx`, `three-marks.smoke.mjs` | complete | board-first, accessible, reduced-motion-safe |
| benchmarks | `benches/three_marks.rs`, `thresholds.json` | complete with follow-up | random-playout target miss remains visible |
| docs | this doc set and atlas update | complete | no extraction |
| native CLI tools | GAT4THRMARBOA-014 | pending at doc landing | depends on this coverage doc |
| CI capstone | GAT4THRMARBOA-016 | pending | wires Three Marks gates |

## Deferrals

- No `game-stdlib` board/line helper extraction.
- No MCTS/ML/RL bots.
- No hidden-information/private-view game behavior.
- No alternate variants, generalized board sizes, movement/sliding phases, or configurable rule data.

## Boundary statement

Rust owns behavior. TypeScript presents Rust/WASM output. Static data remains typed metadata/fixtures/traces/reports, not rule behavior. Replay and public-view payloads remain viewer-safe and deterministic.
