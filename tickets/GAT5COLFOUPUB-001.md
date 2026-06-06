# GAT5COLFOUPUB-001: Column Four rules research & IP source docs

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new docs `games/column_four/docs/RULES.md`, `games/column_four/docs/SOURCES.md` (no code surfaces)
**Deps**: None

## Problem

Gate 5 introduces the official game `column_four` (public name **Column Four**). Per `docs/OFFICIAL-GAME-CONTRACT.md` §3, rules research and original prose MUST precede implementation. Without an original, IP-clean rules summary and source/IP notes landing first, the Rust implementation has no authoritative rule reference and risks copied rulebook prose or trade dress from the commercially adjacent four-in-a-row family (spec §17).

## Assumption Reassessment (2026-06-06)

1. Sibling game `three_marks` ships `games/three_marks/docs/RULES.md` and `games/three_marks/docs/SOURCES.md` as original Rulepath prose; this ticket mirrors that shape for `column_four`. Verified both files exist under `games/three_marks/docs/`.
2. Spec `specs/gate-5-column-four-public-polish.md` §7 (rules model), §15 (`RULES.md`/`SOURCES.md` content), and §17 (IP/source posture) define required content. Identity: game id `column_four`, public name `Column Four`, variant `column_four_standard`, rules id `column_four-rules-v1`, 7 columns × 6 rows, cell ids `r1c1`..`r6c7` (spec §7 board/coordinates).
3. Cross-artifact boundary under audit: the doc-governed contract in `docs/OFFICIAL-GAME-CONTRACT.md` §4 (source notes) and §5 (original rules summary), plus `docs/IP-POLICY.md`. This ticket produces prose conforming to those docs, not new contract surface.
4. FOUNDATIONS §10 (IP conservatism) and §6 (official games are evidence-heavy) motivate this ticket. Restating before trusting the spec: public games MUST use original rules prose and original assets; no rulebook text, board art, token art, or trade dress may be copied; neutral naming where commercial trademark risk exists (Column Four, never Hasbro/Connect 4 branding in product surfaces — spec §17).

## Architecture Check

1. Front-loading rules prose gives every downstream ticket (002–018) one authoritative, IP-reviewed rule reference and prevents code/prose rule drift. Alternative (write docs last) is rejected by `docs/OFFICIAL-GAME-CONTRACT.md` §3.
2. No backwards-compatibility aliasing/shims — these are new files.
3. No code surfaces touched; `engine-core` stays free of mechanic nouns trivially and `game-stdlib` is untouched (no helper extraction).

## Verification Layers

1. Original-prose / IP-clean invariant -> manual review (IP-conservatism audit per `docs/IP-POLICY.md`; no copied prose/assets, neutral naming, no `Connect 4`/`Hasbro` in product framing).
2. Rule-content completeness invariant -> manual review against spec §7 (identity, board/coordinates, seats/turn order, legal column action, full-column illegality, gravity/landing, win H/V/both diagonals, draw, terminal, diagnostics, public/private model).
3. Source-note completeness invariant -> manual review against spec §17 (source categories, copyright/trademark/trade-dress notes, accessibility/reduced-motion references, no-copy statement, chosen/excluded variants).
4. Single-artifact-pair ticket: layers map to the two distinct surfaces (rules prose vs. source/IP notes); no code-proof surface applies because the ticket ships no code.

## What to Change

### 1. `games/column_four/docs/RULES.md`

Original Rulepath prose (use `templates/GAME-RULES.md` structure) covering, with stable rule IDs usable by later RULE-COVERAGE rows: identity (`column_four`, `Column Four`, `column_four_standard`, `column_four-rules-v1`); 7×6 board and the stable coordinate convention (columns `c1`..`c7` left→right, rows `r1`..`r6` bottom→top, cell ids `r1c1`..`r6c7`); two seats `seat_0`/`seat_1` and alternating turn order (`seat_0` starts); legal action = choose a non-full column on the active non-terminal turn; full-column illegality with a public diagnostic; gravity/landing (lowest empty row, Rust-determined); occupancy mutation; win on four contiguous same-seat pieces horizontal / vertical / both diagonals, with the deterministic winning-line tie-break rule when multiple lines complete; draw on full board with no line; win-precedence-over-draw; terminal no-actions; stale/non-active/invalid/unknown-column/full-column/terminal diagnostics; perfect-information public view and `not_applicable_perfect_information` private status; replay and bot notes; out-of-scope variants (no PopOut, misère, Five-in-a-Row, m/n/k, custom sizes, alternate gravity — spec §6); source/IP note.

### 2. `games/column_four/docs/SOURCES.md`

Record (per `templates/GAME-SOURCES.md` and spec §17.5): general public-knowledge basis for vertical four-in-a-row mechanics; copyright guidance (method vs. expressive text/art); trademark guidance; trade-dress caution (avoid blue-rack/red-yellow-disc presentation); accessibility target-size + keyboard references and reduced-motion reference; any bot-strategy source; explicit no-copied-prose/assets statement; neutral-naming rationale; chosen variant `column_four_standard` and excluded variants; consulted date 2026-06-06.

## Files to Touch

- `games/column_four/docs/RULES.md` (new)
- `games/column_four/docs/SOURCES.md` (new)

## Out of Scope

- Any Rust code, static data (`manifest.toml`/`variants.toml`), or tests (land in 002+).
- The remaining game docs (MECHANICS, RULE-COVERAGE, UI, AI, BENCHMARKS, ADMISSION, COMPETENT-PLAYER, BOT-STRATEGY-EVIDENCE-PACK, PUBLIC-RELEASE-CHECKLIST) — they cite implemented surfaces and land in 007/011/013/017.
- SVG/visual assets and palette decisions (land with the web renderer, GAT5COLFOUPUB-014).

## Acceptance Criteria

### Tests That Must Pass

1. `test -f games/column_four/docs/RULES.md && test -f games/column_four/docs/SOURCES.md` — both files exist.
2. `grep -iE "column_four_standard|column_four-rules-v1|r1c1|r6c7" games/column_four/docs/RULES.md` — identity, variant, rules version, and coordinate convention present.
3. Manual IP review: no sentence is a verbatim copy of any cited source; product naming is neutral (no `Connect 4`/`Hasbro` framing except as cited evidence).

### Invariants

1. Rules prose is original Rulepath wording and names only the standard 7×6 vertical four-in-a-row game scoped by spec §7 (no excluded variant described as in-scope).
2. SOURCES.md records consulted date 2026-06-06 and an explicit no-copy / trade-dress-avoidance posture.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep/test -f) plus the manual IP/rule-completeness review named in Assumption Reassessment.`

### Commands

1. `test -f games/column_four/docs/RULES.md && test -f games/column_four/docs/SOURCES.md`
2. `grep -niE "gravity|lowest empty row|four contiguous|draw|column_four_standard|r1c1|r6c7" games/column_four/docs/RULES.md`
3. A narrower (grep/manual) boundary is correct here: there is no compiled surface to exercise, and `tools/rule-coverage` validation of RULE-COVERAGE.md is deferred to GAT5COLFOUPUB-013.
