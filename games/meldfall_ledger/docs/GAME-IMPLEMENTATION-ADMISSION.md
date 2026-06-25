# Meldfall Ledger Implementation Admission

Game ID: `meldfall_ledger`

Public display name: `Meldfall Ledger`

Implemented variant: `classic_500_single_deck_v1`

Roadmap stage/gate: Public scaling phase / Gate 19 Five Hundred Rummy proof

Public role: hidden-info meld-tableau proof / variable-seat card-zone surface /
second forward-v1 game

Prepared by: `Codex`

Date: 2026-06-26

Evidence receipt: `GAME-EVIDENCE.md` in a later ticket

## Purpose

This is the pre-code admission receipt for Meldfall Ledger. It answers whether
implementation work may begin under the current Rulepath foundation docs and
the ADR 0008 `forward-v1` mechanical-scaffolding obligation.

Admission does not waive later rule coverage, tests, traces, benchmarks, UI
polish, bot evidence, no-leak proof, forward-v1 machine receipt, release
checklist, or capstone gates. The post-build `ci/scaffolding-audits.json`
receipt and central register/atlas reconciliation are assigned to
GAT19MELLEDFIV-022.

## Authority References

| Authority | Admission use |
|---|---|
| `docs/FOUNDATIONS.md` | Rust remains behavior authority; `engine-core` stays generic; hidden information does not leak; new official games complete the reuse-first scaffolding audit before serious implementation. |
| `docs/ARCHITECTURE.md` | Rust/WASM boundary, action/view/effect/replay model, and deterministic command-log posture. |
| `docs/ENGINE-GAME-DATA-BOUNDARY.md` | Card, meld, discard, tableau, lay-off, scoring, and zone nouns stay game-local. |
| `docs/OFFICIAL-GAME-CONTRACT.md` | Requirements-first official-game workflow, docs, tests, traces, bots, benchmarks, and UI evidence. |
| `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` | 2-6 seat declaration, public observer, pairwise no-leak matrix, larger hand/tableau surface, and Rust-owned action fanout. |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | `MSC-8C-001` through `MSC-8C-010` audit targets and forward per-game maintenance cadence. |
| `docs/adr/0004-hidden-info-replay-export-taxonomy.md` | Viewer-scoped public and seat-private export posture. |
| `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md` | Public scaling phase and Gate 19 roadmap placement. |
| `docs/adr/0008-mechanical-scaffolding-governance.md` | Reuse-first audit, register-new-on-first-use, and queue-or-dispose prior-game refactor obligation. |
| `docs/adr/0009-replay-fixture-hash-taxonomy.md` | Replay/fixture/hash taxonomy and no migration without explicit authority. |
| `specs/gate-19-meldfall-ledger-five-hundred-rummy.md` | Gate-local scope, variant pin, first-use primitive posture, and Appendix C audit matrix. |

## Source, Scope, And Rule Readiness

| Admission surface | Status | Evidence | Notes/blockers |
|---|---|---|---|
| source/IP notes are ready | ready | `SOURCES.md` | Source-use statement, variant reconciliation, neutral-name rationale, excluded variants, and pending human review are recorded. |
| original rules with stable rule IDs are ready | ready | `RULES.md` | Stable `ML-*` IDs and downstream scoring/terminal tokens are present. |
| implemented variant and out-of-scope variants are explicit | ready | `RULES.md`, `SOURCES.md` | Only `classic_500_single_deck_v1` is admitted. |
| supported seat counts and stable labels are explicit | ready | `RULES.md` | 2-6 seats; default 4; stable `seat_0` through `seat_5`. |
| rule coverage strategy is identified | pending later artifact | `RULE-COVERAGE.md` will land in GAT19MELLEDFIV-018 | This does not block the pre-code admission record. |

## Novel Mechanics And Pressure

| Surface | Admission result | Evidence | Blocks implementation? |
|---|---|---|---:|
| mechanic inventory complete enough to start | ready by rules/source scope | `RULES.md`, `SOURCES.md`, Gate 19 spec Appendix D | no |
| behavioral primitive-pressure decision | local-only first use | Gate 19 spec §8.2 / Appendix D; `PRIMITIVE-PRESSURE-LEDGER.md` lands later | no |
| mechanical-scaffolding reuse-first audit complete | ready | this document | no |
| matching registered/promoted scaffolding will be reused, or accepted exceptions are linked | ready | this document's `MSC-8C-001`...`MSC-8C-010` audit | no |
| newly anticipated behavior-free scaffolding has a planned register disposition | `no-new-scaffolding` | this document; machine receipt deferred to GAT19MELLEDFIV-022 | no |
| expected prior-game matching sites and follow-on/no-follow-on disposition are identified | no follow-on unit expected | no new behavior-free shape; `MSC-8C-010` keeps behavior local | no |
| ADR needed for boundary-changing work | no | No kernel, DSL, YAML, trace/hash, visibility, or architecture exception is admitted. | no |

## Boundary Risks

| Boundary | Admission result | Evidence/notes |
|---|---|---|
| `engine-core` remains generic and noun-free | pass | Card, deck, hand, suit, rank, meld, set, run, stock, discard, pile, tableau, lay-off, and scoring nouns stay in `games/meldfall_ledger`. |
| no static-data behavior language is introduced | pass | Data may carry identity, typed constants, presentation metadata, fixtures, traces, and docs only; no formulas, selectors, triggers, scripts, or behavior-looking fields. |
| Rust remains authority for legality, validation, effects, views, and bots | pass | `RULES.md` requires Rust-owned setup, draw, pickup commitment, meld, lay-off, scoring, projection, replay, and bot behavior. |
| hidden-information risk has a named proof plan | pass | Gate 19 spec §7.3 defines public observer plus all six seat-private viewer matrix; implementation tickets 012, 013, 020, and 021 own proofs. |
| private licensed content is excluded from public paths | pass/constrained | `SOURCES.md` records no copied source prose/assets and pending human IP/public-release review. |
| trick-taking helper pressure is excluded | pass | `game_stdlib::trick_taking` and `game-stdlib::trick_taking` are not used; no tricks, trump, bids, contracts, teams, or partnership scoring exist in Meldfall Ledger. |

## Forward-v1 Mechanical-Scaffolding Reuse-First Audit

Gate 19 is the second `forward-v1` user after Blackglass Pact. This audit
reviews the accepted scaffolding baseline before serious implementation. It
does not authorize behavior extraction.

| Surface | Register target | Disposition | Behavior exclusion | Prior-game follow-on? | Compatibility expectation |
|---|---|---|---|---|---|
| semantic effect envelopes | `MSC-8C-001` | reuse expected | Constructors may carry caller-supplied public/seat-private effect payloads; they do not decide reveal policy, effect meaning, scoring, animation, or redaction. | none expected | Effect order, payload, scope, and hashes remain game-owned and stable once pinned. |
| canonical seat grammar and import boundary | `MSC-8C-002` | reuse expected | Seat helpers may parse/format stable `seat_<n>` identifiers only; they do not decide dealer, active seat, turn order, labels, teams, or visibility. | none expected | Canonical seat strings only; no alias-output migration without ADR 0009 authority. |
| seat-count validation and ring-index arithmetic | `MSC-8C-003` | reuse expected with local policy | Count/ring helpers may support structural 2-6 validation and clockwise wrapping; deal sizes, dealer/start-seat policy, draw order, and settlement remain Meldfall-owned. | none expected | Setup diagnostics and replay bytes remain game-local evidence. |
| action-tree encoding/hash v1 | `MSC-8C-004` | reuse expected | Action-tree framing may transport Rust-owned choices; it must not generate meld legality, lay-off targets, discard-pickup commitments, scoring, or turn phases. | none expected | Action-tree bytes are parallel evidence unless a later ticket names migration authority. |
| stable-byte writer v1 | `MSC-8C-005` | not-present unless an authorized evidence surface needs it | Stable byte writing frames caller-supplied bytes only; it may not decide state meaning, serialization order beyond explicit fields, visibility, or hash authority. | none expected | No broad state/effect/view/replay authority flip in this gate. |
| dev-only game test-support crate | `MSC-8C-006` | reuse expected as dev/test support only | Test helpers may assert no-leak/profile shapes; production crates, WASM, tools, and browser bundles must not gain normal/build dependencies on dev-only support. | none expected | No runtime hash/visibility impact. |
| pairwise no-leak assertion geometry | `MSC-8C-007` | reuse expected | Matrix geometry may enumerate source seat x viewer x surface; it must not decide which Meldfall facts are public or private. | none expected | Deterministic six-seat matrix enumeration; no persistent canary artifacts in public exports. |
| evidence-profile drivers | `MSC-8C-008` | reuse expected where profiles apply | Drivers may validate profile metadata and shape; setup, commands, projection, import/export, scoring, and domain checks remain game/tool code. | none expected | Fixture/export/profile authority named per artifact under ADR 0009. |
| bounded-index sampling | `MSC-8C-009` | reuse expected only for accepted RNG primitives with parity evidence | Bounded sampling does not become shuffle/deal policy; Meldfall owns deck construction, deal partitioning, and any versioned RNG consumption. | none expected | No RNG/hash migration without explicit evidence and authority. |
| behavioral-policy bundle on the non-promotion list | `MSC-8C-010` | apply as rejected/local-only | Meld validation, public tableau, discard-pickup rules, lay-off, scoring, terminal outcome, bot policy, visibility, and UI policy are behavior, not scaffolding. | accepted no-unit disposition unless implementation invents a pure scaffolding match | No shared behavior extraction; revisit behavior via mechanic atlas, not the scaffolding register. |

Admission disposition: `no-new-scaffolding`.

Prior-game retrofit disposition: no follow-on unit expected at admission,
because Gate 19 does not introduce a new behavior-free scaffolding shape that
earlier games must characterize or migrate to. River Ledger, Vow Tide, and
Blackglass Pact are pattern exemplars only; they do not share rummy
meld/tableau/discard-pickup behavior. If implementation later invents a pure
selected-card-zone metadata shape or another behavior-free exact duplicate, the
post-build closeout must register it and either queue a bounded prior-game unit
or record an accepted no-unit disposition with evidence.

## Lawful Shared Homes Review

| Home | Gate 19 admission result |
|---|---|
| `engine-core` | Allowed only for existing generic contracts: game id, seat id, viewer, action tree, command envelope, visibility scope, effects, replay, hash, and serialization boundary. No rummy nouns or behavior enter. |
| `game-stdlib::seat` | Allowed for behavior-free seat-count and ring helpers when the implementation proves they fit; dealer, deal, draw, turn, pickup, and scoring policy stay game-local. |
| `game_stdlib::trick_taking` / `game-stdlib::trick_taking` | Explicitly excluded. Meldfall Ledger has no tricks, led suit, trump, bids, contracts, nils, bags, or teams. |
| `game-test-support` | Allowed as dev/test-only proof support for no-leak and evidence profiles. Production code must not depend on it. |
| `wasm-api` | Allowed as a safe bridge for Rust-owned catalog/setup/view/action/export payloads. It must not decide legality or visibility. |
| static data | Allowed for typed parameters, IDs, fixture metadata, presentation labels, and docs. It must not encode selectors, formulas, conditions, triggers, or rule behavior. |

## First-Use Primitive Decisions

| Primitive-pressure shape | Admission decision | Rationale | Later evidence owner |
|---|---|---|---|
| meld validation: sets, runs, ace low/high/no-wrap | first official use, local-only | This is rummy behavior and belongs in `games/meldfall_ledger`. | GAT19MELLEDFIV-006 / GAT19MELLEDFIV-022 |
| public meld tableau and tabled-card score-credit model | first official use, local-only | Public tableau state and credit semantics are game behavior. | GAT19MELLEDFIV-007 / GAT19MELLEDFIV-022 |
| draw/discard zones with public discard-tail pickup and hidden stock | first official use, local-only | Immediate-use commitment and discard-tail pickup are rule behavior. | GAT19MELLEDFIV-009 / GAT19MELLEDFIV-022 |
| laying off onto any player's meld | first official use, local-only | Legality depends on meld shape and score-credit ownership. | GAT19MELLEDFIV-008 / GAT19MELLEDFIV-022 |
| multi-round cumulative scoring to 500 with hand penalties and tie continuation | first official use, local-only | Scoring, terminal eligibility, and tiebreak policy are behavior. | GAT19MELLEDFIV-011 / GAT19MELLEDFIV-022 |
| deterministic shuffle plus private hand plus redacted exports | reviewed, no new hard gate | Existing hidden-info/replay law covers the proof; no shared shuffle helper is admitted. | GAT19MELLEDFIV-013 / GAT19MELLEDFIV-022 |

No `game-stdlib` rummy helper is admitted. No `engine-core` rummy/card/tableau
noun is admitted. No foundation amendment is expected.

## Required Evidence Profile

| Evidence area | Required before coding? | Required before release/closeout? | Owner/link |
|---|---:|---:|---|
| original rules/source notes | yes | yes | `RULES.md`, `SOURCES.md` |
| pre-implementation scaffolding audit receipt | yes | yes | this document |
| conformance receipt | strategy only | yes | `GAME-EVIDENCE.md` in GAT19MELLEDFIV-023 |
| named rule tests and coverage | strategy only | yes | `RULE-COVERAGE.md` in GAT19MELLEDFIV-018 |
| replay/hash and serialization proof | no | yes | GAT19MELLEDFIV-016 and `GAME-EVIDENCE.md` |
| no-leak proof | plan yes | yes | GAT19MELLEDFIV-012, 013, 020, and 021 |
| UI evidence | no | yes | GAT19MELLEDFIV-020 and 021 |
| bot evidence | no | yes | GAT19MELLEDFIV-014 and 015 |
| benchmark evidence | no | yes | GAT19MELLEDFIV-017 |
| post-implementation register freshness and prior-game refactor receipt | no | yes | GAT19MELLEDFIV-022 |
| CI scaffolding-audit record | no | yes | `ci/scaffolding-audits.json` in GAT19MELLEDFIV-022 |

## Admission Decision

Decision: admitted for crate skeleton and implementation after this ticket is
archived and committed.

Decision rationale:

- GAT19MELLEDFIV-001 completed the source and rules contract with original
  prose, stable `ML-*` rule IDs, and downstream scoring/terminal tokens.
- The `MSC-8C-001` through `MSC-8C-010` forward-v1 reuse-first audit is
  complete in this document.
- All rummy/meld/tableau/discard-pickup/scoring/bot/visibility work is
  classified as behavior and kept local under `MSC-8C-010` plus the future
  primitive-pressure ledger.
- No active FOUNDATIONS §12 stop condition is present in the admitted scope.

Explicit constraints:

- Reuse only behavior-free scaffolding whose accepted register entry fits the
  implementation surface.
- Do not broaden any helper for meld validation, lay-off legality, discard
  pickup, scoring, visibility policy, bot strategy, or UI behavior.
- Do not use `game_stdlib::trick_taking` or `game-stdlib::trick_taking`.
- Keep all production behavior out of static data and TypeScript.
- Do not introduce trace/hash/fixture/export/RNG migration without explicit
  authority and evidence under ADR 0009.
- Human IP/public-release review remains pending before public release.

## Blocking Issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| none for implementation admission | not applicable | not applicable | no |
| human IP/public-release review pending | complete review before public release | Rulepath maintainers | no for coding; yes before release |
| forward-v1 machine receipt absent | add `meldfall_ledger` row to `ci/scaffolding-audits.json` and pass checker | GAT19MELLEDFIV-022 | no for coding; yes before gate closeout |
| rule coverage / evidence / release docs absent | fill later official-game docs and command receipts | later Gate 19 tickets | no for coding; yes before gate closeout |

## Sign-off

Prepared by: `Codex`

Reviewed by: pending maintainer review

Date: 2026-06-26
