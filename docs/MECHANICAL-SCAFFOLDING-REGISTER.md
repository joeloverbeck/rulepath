# Mechanical Scaffolding Register

Status: governed by accepted
[ADR 0008](adr/0008-mechanical-scaffolding-governance.md).

This register records decisions for Rulepath mechanical scaffolding:
behavior-free typed infrastructure that supports generic contracts without
deciding game rules. It is parallel to, and does not replace, the behavioral
mechanic atlas and primitive-pressure ledger.

Mechanical scaffolding may only cover repeated plumbing around already-lawful
generic contracts: effect envelopes, seat IDs, actor/viewer IDs, action trees,
command envelopes, visibility scopes, replay/hash bytes, serialization
boundaries, benchmark/evidence records, and dev-only evidence harnesses.

It must not encode legality, scoring, reveal policy, turn policy, strategy,
hidden-state semantics, renderer policy, game-local state meaning, or private
licensed content.

## Entry Schema

Each register entry must include these fields.

| Field | Required content |
|---|---|
| Entry id | Stable id, date, status, and owner. |
| Candidate | Short name of the proposed scaffolding shape. |
| semantic.risk | `low`, `medium`, or `high`, with rationale for why the shape is behavior-free or why it is rejected. |
| Proposed home | `engine-core`, `game-stdlib`, `game-test-support`, `wasm-api`, local-only, or rejected. |
| Production-vs-test home | Whether production crates may depend on it, or whether it is dev/test-only. |
| Exact duplicate sites | File paths, symbols, and games/tools that currently repeat the shape. |
| Behavior exclusions | What game mechanics, policies, and hidden-state meanings the candidate explicitly does not own. |
| Affected hashes | State, effect, action-tree, public-view, seat-private-view, export, domain, or none. |
| Visibility impact | Public, viewer-scoped, seat-private, internal-dev, private-source, or none. |
| Determinism impact | Ordering, serialization, RNG, stable bytes, or none. |
| Migration set | Every official game, crate, tool, or doc that must migrate, or `none`. |
| Acceptance evidence | Tests, examples, no-leak checks, replay/hash checks, benchmarks, and docs required before adoption. |
| Rejection rationale | Required when the decision is local-only, deferred, or rejected. |
| Next review trigger | Second-use review, pre-third-copy hard decision, named gate, or no further review. |

## Decision States

| State | Meaning |
|---|---|
| `candidate` | Repetition is observed, but no reuse decision has landed. |
| `local-only` | Keep all known sites local with rationale. |
| `promoted` | A narrow behavior-free helper is adopted in the named home and all migration obligations are closed. |
| `promotion-debt-open` | A helper is adopted, but one or more matching sites still require migration or accepted exception. |
| `deferred` | Revisit at a named trigger; no helper exists yet. |
| `rejected` | The shape is not scaffolding or is not worth extracting. |

## Non-Promotion List

These shapes stay behavioral. They are not mechanical scaffolding merely because
multiple games use similar words or data paths.

| Shape | Register stance |
|---|---|
| Deal schedule, shuffle/deal policy, redeal policy | stays behavioral; game-local unless the mechanic atlas separately promotes a narrow helper. |
| Reveal timing, hidden commitment reveal, staged public reveal | stays behavioral; visibility and effect policy remain game-owned. |
| Projection and redaction policy for game state | stays behavioral; scaffolding may carry generic visibility scopes only, not decide what facts are visible. |
| Betting, bidding, contribution, raise, call, or fold policy | stays behavioral; economic and action legality policy remain game-owned. |
| Pot construction, side-pot allocation, remainder order | stays behavioral; allocation semantics stay game-owned. |
| Trick lifecycle, led-suit policy, trump policy, winner-leads policy | stays behavioral except for helpers already promoted by the mechanic atlas with explicit scope. |
| Teams, partnerships, alliances, shared victory, teammate visibility | stays behavioral; seat identity scaffolding must not encode team policy. |
| Graph, topology, adjacency, movement, reachability, connectivity | stays behavioral unless the mechanic atlas records a narrow promoted primitive. |
| Resource accounting, market costs, shared ledgers, scoring ledgers | stays behavioral; accounting semantics stay game-owned. |
| Reaction windows, interrupts, pending responder policy | stays behavioral; response legality and resolution stay game-owned. |
| Scoring, terminal outcome, ranking, tiebreakers, victory rationale | stays behavioral; scaffolding may transport typed evidence only. |

If a proposed entry touches one of these shapes, the default decision is
`rejected` for this register and rerouted to the mechanic atlas or a separate
ADR. A future exception must cite accepted authority and explain why the helper
is behavior-free despite the listed risk.

## Current Entries

No mechanical-scaffolding helper is promoted by this pre-Gate-18 documentation
pass. The successor Part C code-extraction unit must add entries here before it
extracts any shared helper.

| Entry id | Candidate | Status | Proposed home | semantic.risk | Next review trigger |
|---|---|---|---|---|---|
| _None_ | _No promoted scaffolding entries yet._ | _Not applicable_ | _Not applicable_ | _Not applicable_ | Part C candidate review |

## Review Checklist

Before accepting a register entry, verify:

- the candidate is behavior-free;
- the API uses allowed generic vocabulary or correctly game-layer typed inputs;
- behavior exclusions name the mechanics the helper does not own;
- affected hashes and visibility impact are explicit;
- hidden information cannot leak through payloads, DOM, logs, bot explanations,
  candidate rankings, replay exports, traces, fixtures, or tests;
- deterministic ordering and stable bytes are proven where relevant;
- migration set is complete, explicitly deferred, or rejected with rationale;
- `engine-core` remains free of mechanic nouns;
- `game-stdlib` remains earned and narrow;
- no YAML, DSL, selector, condition, trigger, formula, or rule behavior enters
  static data.
