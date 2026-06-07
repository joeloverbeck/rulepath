# ADR: Hidden-Info Replay-Export Taxonomy And Viewer-Aware Visibility Contract

Status: Accepted

Date: 2026-06-07

Decision owner: joeloverbeck

Related documents:

- [`../FOUNDATIONS.md`](../FOUNDATIONS.md)
- [`../ARCHITECTURE.md`](../ARCHITECTURE.md)
- [`../ENGINE-GAME-DATA-BOUNDARY.md`](../ENGINE-GAME-DATA-BOUNDARY.md)
- [`../WASM-CLIENT-BOUNDARY.md`](../WASM-CLIENT-BOUNDARY.md)
- [`../TRACE-SCHEMA-v1.md`](../TRACE-SCHEMA-v1.md)
- [`../TESTING-REPLAY-BENCHMARKING.md`](../TESTING-REPLAY-BENCHMARKING.md)
- [`../UI-INTERACTION.md`](../UI-INTERACTION.md)
- [`../OFFICIAL-GAME-CONTRACT.md`](../OFFICIAL-GAME-CONTRACT.md)
- [`../../specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md`](../../specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md)

## Context

Gate 8 introduces `high_card_duel`, Rulepath's first official chance and
hidden-information proof. Earlier public-perfect-information games can safely
export seed plus command stream because every command and replayed state is
already public to all viewers. That assumption is unsafe for hidden-information
games: seed material, raw private action paths, private card identities, and
internal deck order can reconstruct facts the browser viewer is not authorized
to know.

FOUNDATIONS §13 requires an ADR for changes to replay/hash semantics and
public/private visibility contracts. Gate 8 also hardens the WASM/browser
boundary so `get_view(match_id, viewer_seat)` honors the viewer seat for
hidden-information games instead of returning a public-equivalent projection.

## Decision

Rulepath will use two replay/export classes for hidden-information games.

1. **Internal full trace**
   - MUST remain the deterministic native-test and golden-trace authority.
   - MAY contain seed, full command stream, private action choices, internal
     checkpoints, and hashes needed by `replay-check`.
   - MUST be treated as test/dev evidence, not the default public browser
     export for hidden-information games.

2. **Viewer-scoped replay export**
   - MUST be the default browser export for hidden-information games.
   - MUST contain a public observer projection timeline by default.
   - MAY support an explicitly labelled seat-private export that includes only
     facts that seat was authorized to observe at each timeline step.
   - MUST NOT contain unrevealed deck order, private hands for unauthorized
     seats, pre-reveal hidden commitments, seed material that reconstructs
     hidden cards, raw private action paths containing card IDs, bot private
     candidates, or hidden-state-derived explanations.
   - MUST NOT auto-reveal unused deck tail or unplayed private cards at terminal.
   - Importing this export replays an observation timeline, not an omniscient
     hidden state.

Existing public-perfect-information games keep their current replay/export
semantics. This ADR adds a parallel hidden-information taxonomy; it does not
migrate or weaken existing Race to N, Three Marks, Column Four, Directional
Flip, or Draughts Lite replay contracts.

The WASM/API visibility contract is also viewer-aware:

- `get_view(match_id, viewer_seat)` MUST honor the requested viewer.
- Perfect-information games MAY return output-equivalent projections for all
  viewers.
- Hidden-information games MUST filter projections in Rust/WASM before any
  browser payload is produced.
- TypeScript/React MUST NOT receive hidden state and rely on rendering logic,
  CSS, dev panels, or feature flags to hide it.

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| Reuse seed + command stream as the public export for all games | Existing perfect-information games already use this shape. | Rejected. Hidden-information command paths and seed material can reconstruct private cards and deck order. |
| Keep only internal traces and disable public replay export for hidden-info games | Simple and leak-resistant. | Rejected. Gate 8 requires browser-safe replay/export/import proof; disabling the feature would dodge the product requirement. |
| Viewer-scoped projection export plus internal full trace | Preserves deterministic replay evidence while giving public/browser users a no-leak export. | Accepted. This keeps replay authority in Rust and separates omniscient test evidence from viewer-safe browser payloads. |
| Reveal all hidden information at terminal and then export normally | Common in some physical games and easy to debug. | Rejected. Gate 8 explicitly requires terminal public exports not to auto-reveal unused deck tail or private unplayed cards. |

## Consequences

Positive consequences:

- Hidden-information games can keep strong internal replay/hash proof without
  leaking hidden state through public exports.
- Browser exports become auditable by searching for known hidden tokens.
- Existing perfect-information replay behavior remains stable.

Negative or risky consequences:

- Hidden-information games now need two replay surfaces to test.
- Public replay imports cannot resume omniscient gameplay from hidden state;
  they replay observation timelines.
- Tooling and WASM surfaces must label the export class clearly.

Operational requirements:

- `high_card_duel` must test internal replay determinism and public export
  no-leak behavior.
- WASM export/import for hidden-information games must default to viewer-scoped
  export.
- No-leak tests must search public exports, DOM payloads, logs, command
  summaries, and dev surfaces for hidden card IDs and deck-tail identities.

## Determinism impact

Internal full traces remain deterministic and continue to carry the seed and
command evidence needed for replay/hash checks. Viewer-scoped exports are
deterministic observation timelines derived from Rust projections and effects;
they are not behavior-authority command streams.

Tests must prove:

- same internal seed plus commands reproduce the same hidden/internal surfaces;
- public/viewer export for the same replay is byte-stable;
- importing a public export reproduces the public projection timeline.

No wall-clock, nondeterministic iteration, floating point, thread ordering, or
browser randomness is introduced by this decision.

## Replay/hash impact

Internal state hashes, effect hashes, action-tree hashes, view hashes, and
golden traces remain the canonical replay-check surfaces. Hidden-information
games add a public/viewer export hash or stable serialization surface for the
redacted observation timeline.

Existing golden traces for perfect-information games are preserved. Gate 8
golden traces may include both internal full traces and redacted public
projection traces. Any intentional change to either surface still requires a
non-empty migration/update note in the trace evidence.

## Visibility impact

The decision is visibility-tightening. Public/browser payloads for
hidden-information games must not expose:

- private hands for unauthorized seats;
- hidden commitments before reveal;
- unrevealed deck order or future draw identities;
- seed material that reconstructs hidden cards;
- raw private action paths containing card IDs;
- private bot candidates, hidden-state-derived explanations, or candidate
  rankings;
- unused deck tail or unplayed private cards at terminal.

Observer exports are public by default. Seat-private exports, if implemented,
must be explicitly labelled and contain only that seat's authorized observations
at each step.

## Data/Rust boundary impact

This decision does not add static behavior data, a DSL, YAML behavior, or
procedural selectors. Replay/export artifacts are typed evidence and
observations only. Rust remains the behavior authority for setup, legality,
transitions, projection, effects, and replay derivation. Unknown fields and
behavior-looking fields remain rejected by existing trace/data validation rules
or by the game-local parser that consumes the artifact.

## `engine-core` contamination risk

No card, deck, hand, commitment, or replay-export game noun is added to
`engine-core`. The existing generic `Viewer`, `EffectEnvelope`, replay/hash, and
stable-serialization contracts are sufficient. Hidden-info game vocabulary stays
in `games/high_card_duel`.

## `game-stdlib` / primitive-pressure impact

No `game-stdlib` helper is promoted. This is a first hidden-information replay
export proof, not a repeated primitive. Future card/hidden-info games such as
`blackjack_lite`, `poker_lite`, or trick-taking games may create pressure for a
shared helper, but that requires mechanic-atlas review and later evidence.

## UI impact

The UI impact is boundary-hardening:

- TypeScript remains presentation-only.
- The browser receives only the viewer-authorized projection/export.
- Dev panels, replay viewers, local storage, logs, DOM attributes, CSS classes,
  and test IDs must not receive unauthorized hidden state.
- Perfect-information game UI behavior is unchanged.

## Bot impact

This ADR does not change bot levels or legal action APIs. It reinforces that
hidden-information bots and bot explanations must use only authorized bot/viewer
inputs. Public replay exports must not include private bot candidates or
hidden-state-derived rankings.

## IP impact

No new public naming, asset, font, or protected source material is introduced.
Replay/export artifacts must not ship private licensed content or hidden
private stress-test data into public browser bundles.

## Benchmark impact

No benchmark threshold changes are made. Hidden-information replay/export
serialization may receive game-local benchmarks in the Gate 8 benchmark ticket
if it becomes a hot path.

## Migration notes

Existing docs to update:

- `games/high_card_duel` docs and tests must cite this ADR where replay/export
  and WASM viewer behavior are implemented.

Existing games to back-port:

- None. Public-perfect-information games keep output-equivalent viewer behavior
  and their existing replay/export semantics.

Existing traces to preserve or update:

- Existing perfect-information traces are preserved.
- Gate 8 traces must label internal full trace versus public/viewer projection
  trace when both exist.

Existing data/schema versions to bump:

- None for existing games.
- Hidden-info replay/export artifacts may use a game-local export schema version
  when implemented.

Existing public UI behavior to migrate:

- None for existing games. Hidden-information UI must consume the viewer-aware
  projection/export contract from first implementation.

## Review checklist

Before accepting this ADR, verify:

- the decision supports public playable Rulepath before engine research;
- Rust remains behavior authority;
- TypeScript does not decide legality or hide unauthorized state client-side;
- `engine-core` remains noun-free;
- `game-stdlib` remains earned and narrow;
- static data remains content/parameters/evidence, not behavior;
- replay determinism is preserved for internal full traces;
- public/viewer exports are no-leak by default;
- bots remain fair and viewer-scoped;
- no benchmark thresholds are weakened;
- IP/public-private boundaries are preserved;
- implementing tickets 009 and 016 carry behavioral tests for the decision.
