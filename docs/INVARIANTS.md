# Rulepath Shared Invariants

Status: cross-cutting law. Single source of truth for protocols and checklist items that every other Rulepath document depends on.

This document exists so the same protocol or checklist is written once and referenced everywhere. Other docs link here instead of restating these items. It does not introduce new policy; it consolidates policy already stated across the foundation set. Substantive change to anything here still requires the normal process — and where it touches a constitutional principle, an accepted ADR (`docs/FOUNDATIONS.md`: "Supersede only by accepted ADR").

## 1. Failing-test protocol

When tests fail, humans and agents MUST:

1. determine whether the failing tests are still valid;
2. determine whether the issue is in the system under test or the test suite;
3. fix the issue;
4. add or update regression coverage;
5. report what changed.

Tests MUST NOT be deleted, weakened, renamed away, or rewritten merely to get green output.

## 2. Kernel-change protocol

Any `engine-core` change MUST answer:

1. Which already implemented official games require this?
2. Why can the change not live inside `games/*`?
3. Why can the change not live inside `game-stdlib` after earned pressure?
4. Does it introduce any game noun, mechanic noun, strategy, renderer concern, network concern, or storage concern?
5. Does it preserve deterministic replay, visibility boundaries, serialization compatibility, and hashes?
6. Does it require ADR?

Default answer: do not change `engine-core`.

## 3. Universal acceptance invariants

These invariants apply to every substantial change, every official game, and every accepted ADR. Individual documents add their own area-specific checklist items on top of these; they should reference this list rather than repeat it.

- Rust owns behavior: setup, legal action generation, validation, transitions, scoring, terminal detection, deterministic randomness, semantic effects, visibility, replay/hash, serialization, and bot choices.
- TypeScript does not decide legality.
- `engine-core` remains noun-free.
- `game-stdlib` changes are earned through mechanic-atlas / primitive-pressure-ledger pressure; a third repeated mechanic shape is resolved before proceeding.
- Static data is typed content/parameters only: it deserializes into typed structures, rejects unknown fields, blocks behavior-looking fields, and encodes no rule behavior.
- Semantic effects drive animation; the renderer settles to the latest viewer-safe public view.
- Replay, hashes, and trace format are deterministic, or are explicitly migrated with notes.
- Hidden information is safe: public/private views are viewer-safe and no hidden state leaks through views, action trees, previews, effect logs, diagnostics, UI payloads, DOM attributes, local storage, bot explanations, candidate rankings, or replay exports.
- Bots use the normal legal action API and only their allowed view; they are deterministic, fair, and explainable.
- The WASM API is batched and does not cross the JS/WASM boundary inside rule hot loops.
- Public UI is play-first, not debug-first; dev inspectors are safe and secondary.
- Tests, traces, simulations, and benchmarks cover the change.
- Public files contain no licensed/private content; IP and public/private boundaries are preserved.
- Documentation matches implementation.
- An ADR exists for architecture-changing decisions.
- Agent output is bounded, reviewable, and delivered as complete files or coherent complete sections, not diffs.

## 4. Stop conditions

The repository stop conditions — the triggers that require halting and reassessing — are defined once in `docs/FOUNDATIONS.md` §12. Treat that list as canonical; do not duplicate it here.
