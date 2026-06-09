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

## Recommended lifecycle order

1. `GAME-SOURCES.md`
2. `GAME-RULES.md`
3. `GAME-RULE-COVERAGE.md`
4. `GAME-MECHANICS.md`
5. `GAME-IMPLEMENTATION-ADMISSION.md`
6. `AGENT-TASK.md` for bounded work
7. `GAME-HOW-TO-PLAY.md`
8. `COMPETENT-PLAYER.md` when strategy matters
9. `BOT-STRATEGY-EVIDENCE-PACK.md` before Level 2 bot coding
10. `GAME-AI.md`
11. `GAME-UI.md`
12. `GAME-BENCHMARKS.md`
13. `PRIMITIVE-PRESSURE-LEDGER.md` when repeated shape pressure exists
14. `PUBLIC-RELEASE-CHECKLIST.md` before public release

This order is requirements-first. Do not start serious implementation because a UI sketch is tempting. Public polish wins over speculative engine research, but public polish must sit on Rust rule authority, deterministic replay, coverage, and IP-safe content.

## Template index

| Template | Purpose | Required when | Main foundation authority |
|---|---|---|---|
| `GAME-SOURCES.md` | Source, variant, naming, asset, font, and IP notes. | Every official game before original rules prose. | `docs/IP-POLICY.md`, `docs/SOURCES.md`, `docs/OFFICIAL-GAME-CONTRACT.md` |
| `GAME-RULES.md` | Original Rulepath rules summary keyed by stable rule IDs. | Every official game before coverage and implementation. | `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/SOURCES.md`, `docs/IP-POLICY.md` |
| `GAME-RULE-COVERAGE.md` | Requirements traceability matrix from rule IDs to implementation, tests, traces, replay, UI smoke, bots, and benchmarks. | Every official game before and during implementation. | `docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/OFFICIAL-GAME-CONTRACT.md` |
| `GAME-MECHANICS.md` | Per-game mechanic inventory and primitive-pressure review. | Every official game before implementation admission and whenever mechanics change. | `docs/MECHANIC-ATLAS.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md` |
| `GAME-IMPLEMENTATION-ADMISSION.md` | Short gate receipt proving prerequisites are ready before serious coding. | Before serious implementation work starts for an official game. | `docs/FOUNDATIONS.md`, `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/ROADMAP.md` |
| `AGENT-TASK.md` | Universal bounded task packet for agents and humans. | Any bounded implementation, testing, docs, refactor, UI, bot, benchmark, or release-prep task. | `docs/AGENT-DISCIPLINE.md` |
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
- Shared law belongs in the foundation docs. Templates MAY include local acceptance checks; they MUST NOT paste the whole foundation checklist everywhere.
- Rule IDs are stable requirements. Rename or split them only with a migration note and coverage update.
- Rust is authoritative for legality, validation, effects, replay, visibility, serialization, and bots.
- TypeScript is presentation-only. It maps Rust-provided legal choices to controls and renders viewer-safe payloads.
- Static data is typed content, parameters, metadata, fixtures, traces, and reports only. It is not rule behavior.
- No YAML by default. No DSL at project start.
- Do not include proprietary rules text, card text, screenshots, scans, fonts, copied assets, or trade dress.
