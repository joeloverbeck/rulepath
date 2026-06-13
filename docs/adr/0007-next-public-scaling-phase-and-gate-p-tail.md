# ADR: Next Public Scaling Phase and Gate P Tail Placement

Status: Proposed

Date: 2026-06-13

Decision owner: Rulepath maintainers

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/ARCHITECTURE.md`
- `docs/ENGINE-GAME-DATA-BOUNDARY.md`
- `docs/OFFICIAL-GAME-CONTRACT.md`
- `docs/MECHANIC-ATLAS.md`
- `docs/AI-BOTS.md`
- `docs/UI-INTERACTION.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `docs/ROADMAP.md`
- `docs/IP-POLICY.md`
- `docs/AGENT-DISCIPLINE.md`
- `specs/README.md`
- `specs/phase-0-next-phase-foundation-realignment.md`
- `reports/foundation-doc-realignment.md`
- `reports/public-game-ladder-and-implementation-order.md`

## Context

Gate 14 (`event_frontier`) is complete, and the public mechanic ladder through
Gate 14 has proven substantial public game complexity. It has not yet proven
3+ official seats, multi-seat hidden-information no-leak obligations,
multi-seat UI presentation, larger board/card surfaces, or larger benchmark
envelopes through public, IP-safe games.

`docs/ROADMAP.md` currently ends the public ladder at Gate 14 and carries Gate P
as a private monster-game appendix. Its header is law: a stage or gate may be
skipped or reordered only by accepted ADR. Adding a public scaling phase after
Gate 14 and moving Gate P behind that phase changes roadmap order, so it requires
an accepted ADR before the ROADMAP edit lands.

The next-phase research reports recommend a public scaling ladder before Gate P:
first N-seat infrastructure, then public games that prove 3+ seats and larger
surfaces, ending in an original medium-heavy public capstone. This supports
public playable Rulepath before private stress tests and keeps private licensed
pressure from silently shaping public architecture.

This ADR is intentionally a roadmap-admission decision. It does not authorize a
kernel change, schema migration, WASM exported-API change, new bot technique,
new static data format, or game implementation. Acceptance of this ADR is a
maintainer gate that must precede the ROADMAP edit tracked by the Phase 0 ticket
series.

## Decision

Rulepath SHOULD add a public scaling phase after Gate 14 once this ADR is
accepted and the ROADMAP is edited.

The public scaling phase MUST prove 3+ official seats, larger public surfaces,
N-player hidden-information safety, multi-seat UI presentation, larger benchmark
envelopes, and larger outcome explanations through public, IP-safe games before
any private monster-game red-team work can influence architecture.

Gate P MUST remain in the roadmap, but it MUST move to the very tail after the
public scaling phase. Gate P remains private, optional, isolated,
non-architectural, and non-public. It MUST NOT drive public architecture
retroactively unless a later public, foundation-consistent ADR and follow-up
public ladder task justify a change.

The initial public scaling ladder is the Gate 15+ seed tracked in
`specs/README.md`: Phase 0 foundation realignment, N-seat setup/catalog
metadata, N-seat simulator summaries, multi-seat shell framing, N-player no-leak
harness work, then public games beginning with River Ledger / Texas Hold'Em and
ending in an original medium-heavy public capstone before Gate P.

The ROADMAP edit, spec-index reconciliation, and downstream ticket work are
operational follow-ups. This ADR does not itself edit `docs/ROADMAP.md`.

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| Leave Gate P as the only non-done tail item and skip a public scaling phase | It is the smallest roadmap change and preserves the current document shape. | Rejected. It would send private stress testing at architecture before Rulepath proves 3+ seats, N-player hidden information, larger surfaces, and larger benchmarks through public games. |
| Let private Gate P drive the scaling architecture | Private work could expose complexity quickly. | Rejected. FOUNDATIONS prioritizes polished public playable Rulepath over private licensed stress tests, and IP policy forbids private content from shaping public architecture silently. |
| Add a public scaling phase before Gate P by accepted ADR | It follows ROADMAP header law while keeping complexity earned through public, reviewable games. | Accepted as the proposed decision. It preserves public-first ordering and gives later Gate 15+ specs an explicit authority path. |
| Create a separate `ROADMAP-NEXT-PHASE.md` as the only law for Gate 15+ | It could keep `docs/ROADMAP.md` shorter. | Rejected for now. The current roadmap remains manageable, and a single roadmap law reduces ambiguity. A subordinate file can be proposed later if size becomes a real maintenance problem. |

## Consequences

Positive consequences:

- Gate 15+ specs can be authored without ambiguity once this ADR is accepted and
  ROADMAP records the phase.
- Gate P cannot be pulled forward as an architecture driver.
- N-seat and larger-surface capability must be proven through public, IP-safe
  games first.
- `engine-core` and `game-stdlib` remain protected from private-pressure
  generalization.
- The spec index can keep a clear active-epoch tracker without pretending the
  ROADMAP changed before the ADR is accepted.

Negative or risky consequences:

- The public ladder grows before private stress testing, delaying any private
  red-team feedback.
- The Gate 15+ sequence may need later refinement as public implementation
  evidence accumulates.
- Documentation must be kept synchronized so ROADMAP law, `specs/README.md`, and
  ticket series do not diverge.

Operational requirements:

- After maintainer acceptance, update `docs/ROADMAP.md` to add the public scaling
  phase after Gate 14 and restate Gate P as the tail.
- Update `specs/README.md` after ROADMAP lands so interlock notes cite the
  accepted ADR and completed roadmap admission.
- Complete the Phase 0 foundation-doc and template realignment before authoring
  Gate 15+ game specs.
- Keep all Gate 15+ work subordinate to FOUNDATIONS, accepted ADRs, and the
  multi-seat contract produced by Phase 0.

## Determinism impact

No change. This ADR affects roadmap admission only. It does not affect RNG,
iteration order, clocks, floating point, parallelism, serialization order,
replay, or hashes.

Later Gate 15+ implementations must prove deterministic setup, legal action
generation, state transitions, view projection, replay, hashes, and benchmarks
under their own specs and tickets.

## Replay/hash impact

No command streams, state hashes, effect hashes, action-tree hashes,
public/private view hashes, trace format, or migration rules change. Existing
golden traces are preserved.

Later N-seat games may increase replay and visibility pressure, but any trace
schema, replay/hash, or migration change requires its own accepted ADR.

## Visibility impact

No visibility contract changes. This ADR does not expose, filter, store, log,
serialize, preview, animate, explain, or rank private information.

The public scaling phase must strengthen hidden-information proof by applying
the existing no-leak firewall to N-player contexts, including pairwise
viewer-private redaction and public-observer exports where applicable. Browser
payloads, DOM, local storage, logs, bot explanations, candidate rankings, and
replay exports remain forbidden leak paths.

## Data/Rust boundary impact

No new static data field, hand-authored format, expression, selector, behavior
ID, variant, or schema is introduced. Behavior remains in typed Rust.

The public scaling phase does not authorize YAML, a DSL, rule-like static data,
selectors, conditions, triggers, formulas, or procedural static content. Larger
surfaces may use typed content and parameters only; rule behavior stays in Rust.

## `engine-core` contamination risk

This ADR adds no game noun, mechanic noun, strategy, renderer concern,
networking concern, storage concern, or private content to `engine-core`.

N-seat and larger-surface pressure must first be handled by game crates, tooling,
web presentation, and earned `game-stdlib` helpers under the mechanic atlas.
`engine-core` already owns generic seat/viewer/replay contracts and must remain
a noun-free contract kernel.

## `game-stdlib` / primitive-pressure impact

No helper is introduced or promoted by this ADR.

The public scaling phase will increase pressure on cards, tricks, topology,
public resource accounting, reactions, teams, and outcome explanations. Each
repeated mechanic must follow the `MECHANIC-ATLAS.md` primitive-pressure process:
local first use, local second-use comparison, and third-use hard gate before any
narrow `game-stdlib` promotion or explicit deferral.

## UI impact

No UI files or contracts change. TypeScript remains presentation-only.

Later multi-seat UI work must render Rust/WASM-provided legal actions, turn
state, previews, effects, and viewer-safe public/private views. It must not infer
legality, hidden state, turn order, outcome allocation, or bot reasoning in
TypeScript.

## Bot impact

No bot code, public bot level, or bot policy class changes.

Later N-player imperfect-information bots must use the normal legal action API
and only the authorized view for their seat. Public v1/v2 still exclude MCTS,
ISMCTS, Monte Carlo rollout/search bots, ML, RL, and runtime LLM move selection.

## IP impact

This ADR lowers IP and product risk by keeping private monster-game work behind
public, IP-safe proofs. Gate P remains private, optional, isolated, and
non-public.

Public Gate 15+ games must use original prose, original or compatible assets,
neutral presentation where casino or trade-dress risk exists, and public-domain,
original, or permissioned source material only. Private licensed content must not
enter public files, public CI, public docs, public traces, public bundles, or
public WASM/JS.

## Benchmark impact

No benchmarks change. No threshold, harness, CI lane, or benchmark policy is
modified by this ADR.

Later Gate 15+ specs must define native and browser performance evidence
appropriate to their seat count, action fanout, view size, bot latency, replay
cost, and renderer surface.

## Migration notes

Existing docs to update:

- `docs/ROADMAP.md` after this ADR is accepted.
- `specs/README.md` after the ROADMAP update lands.
- Phase 0 foundation docs and templates as tracked by
  `specs/phase-0-next-phase-foundation-realignment.md`.

Existing games to back-port:

- None. This is documentation-only.

Existing traces to preserve or update:

- Preserve all traces. No behavior or format migration is authorized.

Existing data/schema versions to bump:

- None.

Existing public UI behavior to migrate:

- None.

## Review checklist

Before accepting this ADR, verify:

- the decision supports public playable Rulepath before engine research;
- Rust remains behavior authority;
- TypeScript does not decide legality;
- `engine-core` remains noun-free;
- `game-stdlib` remains earned and narrow;
- static data remains content/parameters, not behavior;
- replay determinism is preserved or migration is explicit;
- visibility boundaries remain safe;
- bots remain fair and explainable;
- benchmarks exist for hot paths when later gates implement them;
- IP/public-private boundaries are preserved;
- affected foundation docs and per-game docs are updated.
