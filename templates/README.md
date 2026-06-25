# Rulepath Template Replacement Set

This folder contains the replacement Rulepath template set.

The templates implement the corrected foundation documents. They are operational forms for applying repository law; they are not independent law and MUST NOT redefine the foundation contracts.

Foundation authority, in normal reading order:

- `docs/README.md`
- `docs/FOUNDATIONS.md`
- `docs/ARCHITECTURE.md`
- `docs/ENGINE-GAME-DATA-BOUNDARY.md`
- `docs/OFFICIAL-GAME-CONTRACT.md`
- `docs/MECHANIC-ATLAS.md`
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
- `docs/AI-BOTS.md`
- `docs/UI-INTERACTION.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `docs/ROADMAP.md`
- `docs/IP-POLICY.md`
- `docs/AGENT-DISCIPLINE.md`
- `docs/SOURCES.md`

Do not add an ADR template. ADRs live in `docs/adr/` under the foundation document set.

## Universal completion rule

Every official game MUST complete the relevant templates in full. Tiny games, including `race_to_n`, do not get a lighter mode. They MAY use explicit `not applicable` rows, but silent omissions are not allowed.

A filled template may be stored under the game directory using the repository's per-game naming convention. Keep the template intent intact even if the filled file is named `RULES.md`, `SOURCES.md`, or similar.

## Completion profiles

Every official game MUST maintain `GAME-EVIDENCE.md` as the status and
artifact-link receipt for its current completion profile. A completion profile
selects which evidence surfaces are applicable for a game stage; it does not
waive any invariant in `docs/FOUNDATIONS.md` §11 or any stop condition in §12.

Use these profile labels unless a spec defines a narrower gate-specific label:

| Completion profile | Use when | Required not applicable rationale |
|---|---|---|
| `full` | The game is intended to satisfy the complete official-game contract. | Any unsupported variant, bot, benchmark, visibility, or release surface. |
| `minimal-perfect-information` | The game is perfect-information and has no seat-private state. | Hidden-information, pairwise no-leak, and seat-private export rows must explain why they are not applicable. |
| `hidden-information` | Any game state, action, effect, diagnostic, bot evidence, or replay/export surface is viewer-scoped or seat-private. | Public observer, each seat, and every private-data surface must have evidence or a blocker. |
| `n-seat` | The game supports more than two seats, teams, partnerships, coalitions, asymmetric roles, or multiple viewer classes. | Every omitted seat-range, viewer matrix, outcome, UI, replay/export, and no-leak row needs a reason. |
| `release-candidate` | A game is being prepared for public linking, hosting, tagging, screenshotting, or demo video capture. | Any pending human/legal review, public artifact inspection, or release checklist row must name the owner/blocker. |
| `intentionally-deferred` | A bounded spec or gate explicitly defers completion of a surface. | Link the deferring ticket/spec and state what must happen before public release or the next gate. |

Profile rows use explicit `not applicable: <rationale>` text. Do not leave
blank rows, and do not use a profile to bypass Rust authority, viewer safety,
deterministic replay/hash expectations, IP conservatism, bot limits, or
official-game evidence requirements.

## N-seat adoption note

Every game with more than two seats MUST fill the seat-range, turn-order, view matrix, pairwise no-leak matrix, outcome matrix, and surface-scale fields across the relevant templates. This includes `GAME-RULES.md`, `GAME-RULE-COVERAGE.md`, `GAME-MECHANICS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `GAME-UI.md`, `GAME-BENCHMARKS.md`, `GAME-AI.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `COMPETENT-PLAYER.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, and `AGENT-TASK.md`.

Two-seat games SHOULD still state their seat model explicitly and may use clear `not applicable` rows for N-seat-only obligations.

## Recommended lifecycle order

1. `GAME-SOURCES.md`
2. `GAME-RULES.md`
3. `GAME-RULE-COVERAGE.md`
4. `GAME-MECHANICS.md`
5. `GAME-IMPLEMENTATION-ADMISSION.md`
6. `AGENT-TASK.md` for bounded work
7. `GAME-EVIDENCE.md` initialized with the current completion profile
8. `GAME-HOW-TO-PLAY.md`
9. `COMPETENT-PLAYER.md` when strategy matters
10. `BOT-STRATEGY-EVIDENCE-PACK.md` before Level 2 bot coding
11. `GAME-AI.md`
12. `GAME-UI.md`
13. `GAME-BENCHMARKS.md`
14. `PRIMITIVE-PRESSURE-LEDGER.md` when repeated shape pressure exists
15. `PUBLIC-RELEASE-CHECKLIST.md` before public release

This order is requirements-first. Do not start serious implementation because a UI sketch is tempting. Public polish wins over speculative engine research, but public polish must sit on Rust rule authority, deterministic replay, coverage, and IP-safe content.

For every new official game, `GAME-MECHANICS.md` contains the pre-implementation
mechanical-scaffolding reuse-first audit before implementation admission.
`GAME-EVIDENCE.md` is initialized with that audit result and later receives the
post-implementation register-freshness and prior-game-refactor closeout. No
separate domain template is required: the register owns shared-scaffolding
decisions, while the evidence receipt owns per-game status and links.

## Template index

| Template | Purpose | Required when | Main foundation authority |
|---|---|---|---|
| `GAME-SOURCES.md` | Source, variant, naming, asset, font, and IP notes. | Every official game before original rules prose. | `docs/IP-POLICY.md`, `docs/SOURCES.md`, `docs/OFFICIAL-GAME-CONTRACT.md` |
| `GAME-RULES.md` | Original Rulepath rules summary keyed by stable rule IDs. | Every official game before coverage and implementation. | `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/SOURCES.md`, `docs/IP-POLICY.md` |
| `GAME-RULE-COVERAGE.md` | Requirements traceability matrix from rule IDs to implementation, tests, traces, replay, UI smoke, bots, and benchmarks. | Every official game before and during implementation. | `docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/OFFICIAL-GAME-CONTRACT.md` |
| `GAME-MECHANICS.md` | Per-game mechanic inventory, behavioral primitive-pressure review, and mandatory mechanical-scaffolding reuse-first audit. | Every official game before implementation admission and whenever mechanics or scaffolding change. | `docs/MECHANIC-ATLAS.md`, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md` |
| `GAME-IMPLEMENTATION-ADMISSION.md` | Short gate receipt proving rules, behavioral pressure, scaffolding reuse-first audit, and boundary prerequisites are ready before serious coding. | Before serious implementation work starts for an official game. | `docs/FOUNDATIONS.md`, `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/ROADMAP.md` |
| `AGENT-TASK.md` | Universal bounded task packet, including mandatory new-game scaffolding reuse/track fields and the deeper scaffold-refactor profile when migration work is authorized. | Any bounded implementation, testing, docs, refactor, UI, bot, benchmark, or release-prep task. | `docs/AGENT-DISCIPLINE.md`, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` |
| `GAME-EVIDENCE.md` | Machine-friendly status and artifact-link receipt, including pre-code scaffolding audit, post-build register freshness, and prior-game refactor disposition. | Every official game; initialize at implementation admission and update whenever evidence or scaffolding disposition changes. | `docs/FOUNDATIONS.md`, `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/EVIDENCE-FIXTURE-CONTRACT.md` |
| `GAME-HOW-TO-PLAY.md` | Player-facing rules prose rendered by the shared web How to Play / Rules surface. Distinct from formal `GAME-RULES.md` and strategy-oriented `COMPETENT-PLAYER.md`. | Every official catalog game. | `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/UI-INTERACTION.md`, `docs/IP-POLICY.md` |
| `COMPETENT-PLAYER.md` | Human/LLM-authored strategy analysis checked against rules. | When a game needs a competent bot or strategy-sensitive public UX. | `docs/AI-BOTS.md`, `docs/OFFICIAL-GAME-CONTRACT.md` |
| `BOT-STRATEGY-EVIDENCE-PACK.md` | Formal Level 2 authored-policy design input. | Before coding any Level 2 bot. | `docs/AI-BOTS.md`, `docs/TESTING-REPLAY-BENCHMARKING.md` |
| `GAME-AI.md` | Per-game bot registry and status document. | Every official game; update as bot levels mature. | `docs/AI-BOTS.md` |
| `GAME-UI.md` | Product-facing UI plan: legal controls, previews, effects, accessibility, hidden-info safety, replay, bot explanations. | Every web-exposed official game. | `docs/UI-INTERACTION.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md` |
| `GAME-BENCHMARKS.md` | Native and WASM/browser benchmark report and regression thresholds. | Every official game; update before release and performance-sensitive changes. | `docs/TESTING-REPLAY-BENCHMARKING.md` |
| `PRIMITIVE-PRESSURE-LEDGER.md` | Evidence ledger for repeated mechanic shapes and promotion/defer decisions. | At second-use review and mandatory before a third official game repeats a shape. | `docs/MECHANIC-ATLAS.md`, `docs/ARCHITECTURE.md` |
| `PUBLIC-RELEASE-CHECKLIST.md` | Final public/web exposure gate. | Before a game is linked, built, hosted, or otherwise shipped publicly. | `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/IP-POLICY.md`, `docs/UI-INTERACTION.md`, `docs/AI-BOTS.md`, `docs/TESTING-REPLAY-BENCHMARKING.md` |

## Usage rules

- Prefer explicit `not applicable` with rationale over blank sections.
- Keep `GAME-EVIDENCE.md` as status and artifact links only. Put domain prose,
  rules text, strategy analysis, UI copy, and behavior details in their owning
  templates.
- Completion profiles require explicit not applicable reasons and never waive
  `docs/FOUNDATIONS.md` §11 invariants or §12 stop conditions.
- Shared law belongs in the foundation docs. Templates MAY include local acceptance checks; they MUST NOT paste the whole foundation checklist everywhere.
- Rule IDs are stable requirements. Rename or split them only with a migration note and coverage update.
- Rust is authoritative for legality, validation, effects, replay, visibility, serialization, and bots.
- TypeScript is presentation-only. It maps Rust-provided legal choices to controls and renders viewer-safe payloads.
- Static data is typed content, parameters, metadata, fixtures, traces, and reports only. It is not rule behavior.
- No YAML by default. No DSL at project start.
- Do not include proprietary rules text, card text, screenshots, scans, fonts, copied assets, or trade dress.
