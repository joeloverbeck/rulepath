# river-ledger-showdown-and-seat-presentation-fixes — Canonical showdown results and active-seat presentation hardening

- **Filename:** `specs/river-ledger-showdown-and-seat-presentation-fixes.md`
- **File operation:** Create this as a new living spec; do not overwrite either archived River Ledger spec.
- **Spec ID:** `river-ledger-showdown-and-seat-presentation-fixes`
- **Ticket prefix:** `RIVLEDFIX`
- **Target type:** New implementation spec
- **Roadmap stage:** Non-gate correctness fix and public-presentation hardening for the shipped Gate 15 game `river_ledger`, plus a shared-shell multi-seat repair
- **Roadmap build gate:** None; this work blocks further River Ledger presentation expansion until closed
- **Status:** Done
- **Date:** 2026-06-18
- **Owner:** joeloverbeck
- **Provenance:** Authored against `main` @ `c4910a2` (`joeloverbeck/rulepath`), which was the current `HEAD` at reassessment on 2026-06-18. Evidence references below are repo-relative; re-verify them if `main` advances before decomposition.
- **Baseline:** Gate 15, RIVLEDSHO, and RIVLEDSHOWUX are complete. This spec repairs regressions and coverage gaps inside that shipped baseline; it does not re-propose their delivered surfaces.
- **Authority order:** `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area law (`docs/OFFICIAL-GAME-CONTRACT.md`, `docs/UI-INTERACTION.md`, `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`) → `docs/ROADMAP.md` → `docs/IP-POLICY.md` → `docs/AGENT-DISCIPLINE.md` → `docs/WASM-CLIENT-BOUNDARY.md` → accepted ADRs → `games/river_ledger/docs/RULES.md` → this spec.
- **Subordination:** This spec may not redefine evaluator strength, pot accounting, seat legality, visibility, replay, trace, or foundation contracts. Where this spec and upstream law disagree, upstream law wins.

---

## 1. Objective

Repair four concrete defects without rebuilding the already-shipped River Ledger showcase work:

1. eliminate every possibility that a showdown surface names different winners, whether the mismatch comes from inconsistent seat labels, duplicate winner derivation, or misuse of payout order as semantic winner order;
2. make every match-scoped seat surface show exactly the active match's seats rather than the game's maximum capability;
3. make the shared observer/seat viewpoint selector work for every Rust-declared active seat, not only `seat_0` and `seat_1`; and
4. contain and center the existing River Ledger card rank and suit presentation without redesigning the card component.

The completed fix must preserve the platform boundary:

- Rust owns seat existence, seat labels, viewer-safe projections, evaluated hand identity, the canonical winner set, allocation, showdown narration, serialization, and no-leak behavior.
- TypeScript/React owns component selection, layout, focus, control semantics, and visual containment. It may select and render Rust-projected facts; it may not derive winners, parse seat IDs into public labels, invent seats, or authorize a viewer.

---

## 2. Delta framing and current evidence

### 2.1 Shipped baseline

The following work is complete and is the baseline for this spec:

- **RIVLEDSHO** (`archive/specs/river-ledger-showdown-legibility-and-table-presentation.md`) shipped Rust-authored showdown explanation fields, the neutral `RiverLedgerCard`, the hand-ranking reference, and the first legible terminal presentation.
- **RIVLEDSHOWUX** (`archive/specs/river-ledger-showcase-ux.md`) shipped the V2 result banner, decisive comparison, ranked standings, card-usage marks, table recomposition, live-region result announcement, River-scoped visual tokens, and viewer-safe bot “Why?”.
- RIVLEDSHOWUX also repaired raw `seat_N` copy at several Rust authoring sites, but it did **not** close the distinction between an internal zero-based seat ID, the public display label, catalog capability labels, and the active match's seat inventory.

This spec does **not** add another showdown surface, another card component, another table redesign, or another bot-explanation layer. It makes the shipped surfaces internally coherent.

### 2.2 Corrected diagnosis of the observed unique-winner contradiction

The observed output was, in one terminal Outcome block:

> “Seat 0 wins — The strongest revealed five-card hand receives the ledger.”
>
> “Seat 1 wins with Two pair, Queens and Fives. … Two pair outranks One pair. Closest challenger: Seat 3.”

The initial hypothesis in the research brief—evaluation-order winners versus button-order allocation winners—is a real defect for tied winners, but it cannot by itself explain this exact **single-winner** contradiction. A one-element vector cannot change winner identity when reordered.

The direct mechanism at the target commit is a public-label split:

- `games/river_ledger/src/ui.rs` defines `seat_public_label(seat)` as `Seat {index + 1}`, so internal `seat_0` is publicly narrated as **Seat 1**.
- The same file's `seat_labels(count)` emits `seat_0 → Seat 0`, `seat_1 → Seat 1`, and so on.
- `apps/web/src/components/RiverLedgerBoard.tsx` locally derives labels by stripping `seat_` from the ID, so internal `seat_0` is rendered as **Seat 0**.
- The generic Outcome heading is built in TypeScript from `view.terminal.winners`, while the V2 showdown headline is authored in Rust with `seat_public_label`.

The prior seat-label pass therefore removed raw IDs while leaving two incompatible public numbering systems. This is a correctness defect in player-facing identity, not merely copy polish.

### 2.3 Deterministic seed-hunt reproduction

An algorithm-faithful seed hunt was performed from the exact-commit implementation of:

- `SeededRng` in `crates/engine-core/src/rng.rs`;
- canonical deck order and card labels in `games/river_ledger/src/cards.rs`;
- unbiased Fisher–Yates shuffle and deal order in `games/river_ledger/src/setup.rs`; and
- best-five-of-seven evaluation and tie-break ordering in `games/river_ledger/src/evaluator.rs`.

The first locked regression case for this spec is:

| Field | Reproduction |
|---|---|
| Seed | `10018` |
| Seat count | `4` |
| Button | default `seat_0` |
| Board | `5D 3S QD 5C 9S` |
| `seat_0` | `QS 2S` → Two pair, Queens and Fives, Nine kicker — unique winner |
| `seat_1` | `10D AC` → Pair of Fives |
| `seat_2` | `AD JS` → Pair of Fives — closest challenger |
| `seat_3` | `4C 2H` → Pair of Fives |

At the target commit, that state composes the observed wording:

- TypeScript heading from internal winner ID: **“Seat 0 wins”**;
- Rust V2 headline from `seat_public_label(seat_0)`: **“Seat 1 wins with Two pair, Queens and Fives.”**;
- Rust decisive basis: **“Two pair outranks One pair.”**;
- Rust closest-challenger label for internal `seat_2`: **“Seat 3.”**

The implementation tickets must turn this seed into a native regression fixture/test and a browser assertion. The test must verify the winner identity and every public label, not merely snapshot the final prose.

### 2.4 Independent split-order defect

A second seed demonstrates the separate semantic-order problem:

| Field | Reproduction |
|---|---|
| Seed | `31` |
| Seat count | `4` |
| Button | `seat_2` |
| Board | `JC 10S 5S AD 9S` |
| `seat_0` | `3H 6D` → Ace-high |
| `seat_1` | `3S KD` → Ace-King-high — tied winner |
| `seat_2` | `KC 7H` → Ace-King-high — tied winner |
| `seat_3` | `KH 7C` → Ace-King-high — tied winner |
| Evaluation/stable-seat order | `[seat_1, seat_2, seat_3]` |
| Button/remainder order | `[seat_2, seat_3, seat_1]` |

At the target commit:

- `winning_seats()` returns winners in evaluation/ledger order;
- `allocate_single_pot()` calls `winners_in_button_order()` and stores that order in both `PotAllocation.winners` and `PotAllocation.remainder_order`;
- `resolve_showdown()` authors some narration from the original winner vector but stores `TerminalOutcome::Showdown.winners = allocation.winners`;
- `explain_showdown()` and `showdown_presentation_v2()` use `allocation.winners`;
- `primary_winner()` takes `winners.first()`.

Button order is legitimate for assigning integer remainder units under `RL-POT-REMAINDER-001`. It is not a legitimate second definition of the semantic winner set or of the “primary” narrated winner. The fix must represent these two concepts separately.

### 2.5 Seat-count and viewpoint evidence

At the target commit:

- `games/river_ledger/src/ui.rs::ui_metadata()` emits `seat_labels(STANDARD_MAX_SEATS)`, producing six capability labels for every catalog entry and view metadata instance.
- `apps/web/src/components/SeatFrame.tsx::catalogSeatLabels()` uses that full catalog list for both the viewpoint row and the right-hand seat rail.
- `apps/web/src/components/MatchSetup.tsx::modeDetail()` and `setupSeatRoles()` use all catalog labels even when the selected setup count is four.
- `apps/web/src/main.tsx::changeSeatFrameViewerMode()` forwards only `seat_0` or `seat_1`; clicks for `seat_2` through `seat_5` are silently ignored.

This explains all three six-seat overcount surfaces and the dead higher-seat buttons. The previous seat-label work fixed vocabulary but did not establish a capability-versus-active-seat data contract or generalize the shared callback.

### 2.6 Card evidence

The existing component is correct in concept and remains the component of record:

- `RiverLedgerCard.tsx` renders rank text plus a suit glyph and the full suit word.
- `.river-ledger-card` has a compact fixed visual width and padding.
- `.river-ledger-card-suit` centers an inline-flex row, but the text child has no maximum inline size or overflow strategy.
- The longest full word, `diamonds`, can extend past the private card's right edge; the rank and suit group do not read as optically centered.

The repair is a CSS/layout containment task. It must not change card identity, public labels, data shape, or visual language.

---

## 3. Scope

### 3.1 In scope

- one canonical Rust-owned showdown result assembly path;
- explicit separation of semantic winner order from split-remainder order;
- one Rust-authored public label for every River Ledger seat ID across catalog, live view, status, terminal outcome, accessibility text, and showdown narration;
- active-match seat labels/count for the seat rail and viewpoint selector;
- selected setup count for setup-mode role copy;
- generic shared-shell seat-viewer mapping for every Rust-projected active seat;
- invalid/stale viewer selection normalization that fails closed without leaking data;
- River Ledger and cross-catalog browser regression coverage;
- minimal `RiverLedgerCard` suit containment and centering;
- deterministic trace, replay, serialization, rule-coverage, and no-leak reconciliation where output intentionally changes.

### 3.2 Out of scope

- new hand categories or evaluator behavior;
- all-in, side pots, no-limit, pot-limit, stacks, or Gate 15.1 accounting;
- changing `RL-POT-REMAINDER-001` button-order remainder policy;
- a new outcome panel or showdown V3;
- a card redesign, abbreviated rank system, proprietary card art, casino styling, or a new suit vocabulary;
- player names/accounts, hosted multiplayer, permissions, or authentication;
- generic card, pot, seat, betting, or evaluator nouns in `engine-core`;
- a generalized game DSL, YAML behavior, or unearned `game-stdlib` promotion.

### 3.3 Not allowed

- TypeScript winner, hand-strength, split, seat-count legality, or redaction logic;
- deriving a public label by parsing `seat_N` in React;
- using `STANDARD_MAX_SEATS` as a match-scoped seat inventory;
- authorizing a viewer because its string matches a hardcoded list;
- exposing another seat's private cards while testing viewpoint changes;
- deleting, weakening, or broadening snapshots to hide changed output;
- changing trace schema/version or replay compatibility policy without the required ADR;
- silently changing public numbering from one convention to another without migrating every affected surface and test together.

---

## 4. Foundation and boundary alignment

| Authority | Requirement engaged | Spec stance |
|---|---|---|
| `FOUNDATIONS.md` §2 | Rust owns behavior, view projection, terminal detection, and serialization | Winner identity, seat existence, labels, and redaction remain Rust-authored. React only renders and manages focus/layout. |
| `FOUNDATIONS.md` §7 | Public UI is a central product surface | Contradictory winner identity, phantom seats, dead controls, and overflowing card text are release-blocking public defects. |
| `FOUNDATIONS.md` §11 | Deterministic, leak-safe viewer projections | Winner ordering and labels become deterministic; every viewpoint test remains pairwise no-leak. |
| `FOUNDATIONS.md` §12 | Stop on TS legality, leaks, or debug-first UX | The implementation must stop rather than add TS seat inference or expose full state to make selection easy. |
| `ARCHITECTURE.md` | Rust/WASM behavior, TS presentation | No ownership movement is authorized. |
| `ENGINE-GAME-DATA-BOUNDARY.md` | Mechanic nouns stay game-local | Showdown and allocation work stays in `games/river_ledger`; shared shell work is presentation only. |
| `UI-INTERACTION.md` §§3, 5, 10B, 10C, 16 | Rust supplies viewer-safe seat/outcome facts; selector and outcome are presentation | Active seat labels and winner facts cross the existing Rust/WASM seam. TS neither parses IDs into labels nor computes terminal meaning. |
| `MULTI-SEAT-AND-SURFACE-CONTRACT.md` §§2, 5, 6, 11, 12 | Stable labels, authoritative seat order, pairwise no-leak, Rust-owned outcome facts | Stable seat order defines canonical semantic winner order. Button order remains a separate allocation fact. Active viewer choices are drawn from the Rust-projected match inventory. |
| `IP-POLICY.md` | Original, neutral, non-casino public presentation | Full neutral suit words remain; no commercial trade dress or casino vocabulary is introduced. |
| `WASM-CLIENT-BOUNDARY.md` | Deterministic typed JSON seam | Any active-seat-label or terminal payload adjustment is explicit, typed, deterministic, and reflected in client types/tests. |
| `AGENT-DISCIPLINE.md` §4 | Never weaken tests to get green | Existing failures are classified first; intentional trace changes are reviewed and regenerated through normal tooling. |
| `games/river_ledger/docs/RULES.md` | `RL-SCORE-*`, `RL-SHOW-*`, `RL-POT-REMAINDER-001`, `RL-VIS-*`, `RL-UI-*` | The spec repairs conformance; it does not change the rules oracle. |

### ADR determination

No ADR is required for the intended implementation. The work restores already-declared ownership, label, determinism, outcome, and no-leak contracts. An ADR becomes mandatory only if implementation proposes to:

- change trace schema/version or replay compatibility policy;
- move game-local seat/showdown vocabulary into `engine-core` or `game-stdlib`;
- let TypeScript determine active seats or winner identity; or
- change `RL-POT-REMAINDER-001` rather than separating it from semantic winner order.

---

## 5. External research grounding

External sources sharpen the implementation and acceptance criteria; they do not override Rulepath's rules contract.

1. **Winner identity and odd-unit order are separate concepts.** Poker Tournament Directors Association Rule 20 specifies an odd-chip order relative to the button after the winning hands are known. That supports keeping the winner set semantic and using button order only for indivisible remainder distribution, exactly as `RL-POT-REMAINDER-001` requires. [Poker TDA Rules, Rule 20](https://www.pokertda.com/view-poker-tda-rules/)
2. **A viewpoint chooser is a one-of-many selection control.** The WAI-ARIA Authoring Practices radio-group pattern defines a group in which no more than one option is checked, with keyboard movement and an exposed checked state. The shared selector should use native radio inputs or equivalent `radiogroup`/`radio` semantics rather than a row of visually toggle-like buttons with ambiguous state. [WAI-ARIA APG Radio Group Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/radio/)
3. **Dense selector rows still need usable targets and focus.** WCAG 2.2's Target Size (Minimum) guidance uses a 24-by-24 CSS-pixel baseline or sufficient spacing, and the APG example makes focus encompass both control and label. This informs the acceptance check for three-to-six seat selectors. [WCAG 2.2 — Target Size (Minimum)](https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html) · [APG Roving-Tabindex Radio Example](https://www.w3.org/WAI/ARIA/apg/patterns/radio/examples/radio/)
4. **Compact content must reflow without disappearing.** W3C technique C31 targets layouts that avoid horizontal scrolling at 320 CSS pixels, while WCAG 1.4.4 requires text to remain available at 200% resize. These checks apply to the seat selector and compact cards; clipping away the suit word is not an acceptable “fix.” [W3C C31](https://www.w3.org/WAI/WCAG22/Techniques/css/C31) · [WCAG 2.2 — Resize Text](https://www.w3.org/WAI/WCAG22/Understanding/resize-text.html)
5. **Use the CSS box model and two-axis alignment deliberately.** `box-sizing: border-box` keeps declared dimensions inclusive of padding/border, and `place-items` centers grid children in both axes. Those primitives support a surgical card containment fix without changing the component's visual identity. [MDN `box-sizing`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/box-sizing) · [MDN `place-items`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/place-items)

---

## 6. Committed decisions and invariants

### D1. Internal seat IDs and public labels are different data

- `seat_0` through `seat_5` remain stable internal/replay IDs.
- The public label is authored once in Rust and projected with the seat ID.
- React must use the projected label. It must not strip prefixes, add one, or otherwise transform the ID into display copy.
- This spec keeps the currently intended public one-based labels—`Seat 1` through `Seat 6`—because `seat_public_label()` and the shipped prose already use that convention. The catalog label generator must be corrected to match it.
- A single migration changes every affected visible, accessibility, trace, and browser assertion together. Mixed zero-/one-based output is never an accepted intermediate state.

### D2. A resolved showdown is assembled once in Rust

This value is the single canonical showdown result and the authoritative winner source for the terminal outcome, awarded ledger, banner, decisive reason, standings, explanations, serialization, and replay evidence.

`resolve_showdown()` must produce or consume one internal resolved-showdown value containing, at minimum:

- ordered evaluations;
- canonical semantic winner IDs;
- per-winner allocation amounts;
- explicit remainder order/recipients;
- canonical Rust-authored winner labels;
- decisive comparison and comparison basis;
- explanations and V2 presentation.

The exact private type name is implementation detail; `ResolvedShowdown` is the recommended name. Downstream fields must be derived from that value rather than recomputing winner meaning from partly transformed structures.

### D3. Canonical winner order and remainder order are separate

- **Canonical semantic winner order:** stable active-seat/evaluation order, matching the authoritative `state.seats`/ledger order.
- **Remainder order:** button order among the canonical winner set, used only to decide which tied winner receives each indivisible extra unit.
- `TerminalOutcome::Showdown.winners`, banner copy, decisive reason, standings winner flags, explanation result labels, and serialized winner arrays use canonical semantic order.
- Allocation amounts may be calculated with the remainder order, but serialized `allocations` should be emitted in canonical winner order for a stable, human-readable outcome contract.
- No public copy describes a tied result as though the first remainder recipient were the sole or primary winner.

### D4. Catalog capability and active-match inventory are separate

- Catalog metadata may continue to advertise all supported River Ledger seats/counts.
- A match-scoped projection must expose exactly the seats participating in that match, in authoritative Rust order, with their Rust-authored public labels.
- The seat rail and viewpoint selector consume active-match labels when a match exists.
- Setup copy consumes the selected supported count and the corresponding Rust-authored labels; it does not consume `max_seats` as though it were the current choice.

### D5. Viewer selection is generic but not permissive

- The shell accepts any seat ID present in the current Rust-projected active-seat list.
- It does not hardcode `seat_0`/`seat_1`, parse numeric suffixes, or accept arbitrary strings.
- Observer remains a distinct explicit option.
- A selected seat that disappears after match replacement, replay reset/import, or seat-count change is normalized to observer (or another explicitly documented safe mode) before any viewer request is made.
- An unknown/stale seat selection never causes a broader projection or fallback to omniscient data.

### D6. Hidden information remains projected before the browser

Changing the selected viewer changes only which already-authorized Rust projection is requested. It must not:

- fetch full state and hide it with CSS;
- preload every seat's private cards into the DOM;
- retain another seat's private payload in a shared cache, log, local storage, accessible name, test ID, animation payload, or replay export;
- expose private card identity while generating setup/seat labels.

### D7. Keep full suit words, contained and centered

**Recommendation and default:** retain the full neutral suit word (`clubs`, `diamonds`, `hearts`, `spades`) beside/below the glyph, because the text is readable, original, color-independent redundancy and avoids turning a presentation bug into information loss.

The implementation may tune layout, padding, line-height, and a bounded responsive font token, but it may not abbreviate, truncate, ellipsize, or switch to glyph-only presentation unless a later accepted change explicitly overrides this decision.

---

## 7. Workstream 1 — Showdown winner and seat-identity coherence

### 7.1 Root cause

There are two interacting but distinct defects.

#### A. Direct unique-winner contradiction

- Rust narration uses `seat_public_label(seat)`, which renders `Seat {index + 1}`.
- catalog metadata and `RiverLedgerBoard.tsx::seatLabel()` use zero-based visible numbers.
- the shared Outcome heading and V2 result banner therefore render the same internal winner with two different public identities.

This is the exact cause reproduced by seed `10018`.

#### B. Split-result semantic-order defect

- `winning_seats()` returns the canonical winner set in evaluation/seat order;
- `allocate_single_pot()` reorders those winners for remainder distribution and stores the result as `PotAllocation.winners`;
- some outcome fields use the pre-allocation winner order, while others use `allocation.winners`;
- `primary_winner()` reads the first element of whichever vector it receives.

This cannot change a singleton winner, but it can make tied-winner narration, selection, list order, accessibility copy, and allocation order depend on a payout-specific order. Seed `31`, button `seat_2`, is the locked regression case.

### 7.2 Prescribed fix

#### Rust/game-local work

1. Correct `seat_labels(count)` in `games/river_ledger/src/ui.rs` so it delegates to the same public-label function used by narration. Add a test that every generated `(seat_id, label)` pair matches `seat_public_label(RiverLedgerSeat::from_index(index))` for all six supported indices.
2. Refactor `games/river_ledger/src/pot.rs` so button order is represented only as `remainder_order` (and, if useful, explicit `remainder_recipients`). Do not overwrite semantic winner order.
3. Refactor `games/river_ledger/src/showdown.rs::resolve_showdown()` to construct one canonical result and pass it to all explanation/presentation builders.
4. Preserve `winning_seats()` output as canonical semantic winner order. Name it accordingly at call sites (`canonical_winners`), so code review can distinguish it from payout order.
5. Produce `PotShare` entries in canonical winner order while calculating each amount from the remainder-recipient set/order. Pot conservation and public explanation remain unchanged.
6. Remove or narrow `primary_winner()` for tied results. A split headline names all canonical winners. A shared equal-hand description may use the common evaluation but must not imply that one tied seat is the true winner.
7. Add construction-time/debug assertions:
   - canonical winners are nonempty and unique;
   - each canonical winner has exactly one positive allocation;
   - no nonwinner has a positive allocation;
   - allocations sum to `pot_total`;
   - `remainder_order` contains exactly the canonical winner set, with no duplicates;
   - every winner flag in V2 standings matches the canonical winner set;
   - single-winner banner identity matches the sole `TerminalOutcome.winners` ID;
   - split banner names every winner and no loser.
8. Keep all hand naming, decisive comparison, comparison basis, closest challenger selection, and reveal authorization in Rust.

#### Rust/WASM projection work

1. Ensure the terminal JSON carries canonical winner IDs and Rust-authored labels through the existing deterministic view projection.
2. Prefer the existing V2 `result_banner.headline` for the showdown's visible result statement. Where a generic outcome surface still needs a seat label, resolve it from projected Rust seat-label metadata rather than deriving it from an ID.
3. Do not introduce a second browser-only “winner label” field if the existing projected label map can serve the purpose cleanly. If a new typed field is necessary, add it in Rust, serialize it explicitly, and update `apps/web/src/wasm/client.ts` in the same ticket.

#### TypeScript presentation work

1. Delete River Ledger's `seat.replace("seat_", "")` public-label path.
2. Build a label lookup from Rust-projected `SeatDisplayLabel` rows and use it for:
   - status heading;
   - generic Outcome heading/final standings;
   - private-view heading;
   - seat ledger headings;
   - active-seat copy;
   - accessibility names.
3. For a showdown, do not synthesize a second winner sentence when the Rust V2 result banner is available. The outer Outcome heading may remain “Outcome” while the live/result announcement uses the Rust-authored banner.
4. A missing label is a contract failure: fail to a neutral non-identity string in production and surface a dev assertion. Do not reveal the raw ID as fallback public copy.

### 7.3 Acceptance criteria

1. Seed `10018`, four seats, produces one coherent unique-winner result:
   - internal winner set is `[seat_0]`;
   - every player-facing surface names that seat **Seat 1**;
   - no visible or accessible string says “Seat 0 wins”;
   - the decisive reason names **Seat 3** as the closest challenger because it represents internal `seat_2` under the one-based public label contract;
   - allocations award the full ledger to `seat_0`.
2. Seed `31`, four seats, button `seat_2`, produces:
   - canonical winner IDs `[seat_1, seat_2, seat_3]`;
   - remainder order `[seat_2, seat_3, seat_1]`;
   - a split banner that names all three co-winners in canonical order;
   - allocation amounts reflecting button-order remainders without changing semantic winner order;
   - no sole-winner or “primary winner” wording.
3. Unique, even-split, and remainder-split outcomes satisfy winner/allocation conservation assertions.
4. `TerminalOutcome.winners`, V2 banner, decisive reason, standings, accessibility announcement, and awarded shares agree on winner identity for every tested showdown.
5. Foldout behavior remains distinct and does not reveal folded hole cards.
6. No TypeScript function computes or ranks a hand, chooses a winner, or parses a seat ID into public copy.
7. No trace-schema version change is introduced.

### 7.4 Test, golden-trace, replay, and no-leak obligations

#### Native tests

- Add a focused `ui.rs` label-consistency unit test over indices `0..STANDARD_MAX_SEATS`.
- Add showdown tests for seed `10018` and seed `31`/button `seat_2`.
- Add `pot.rs` tests proving:
  - canonical input winner order is preserved;
  - remainder recipients follow button order;
  - shares serialize in canonical order;
  - total allocation is conserved.
- Add a property test over generated winner subsets, seat counts `3..=6`, button positions, and pot totals verifying winner-set equality, conservation, uniqueness, deterministic ordering, and remainder policy.
- Assert V2 standings winner flags and `allocation_label` values against the canonical result.

#### Golden traces

Add or update deterministic evidence through normal trace tooling:

- new `games/river_ledger/tests/golden_traces/showdown-seat-label-consistency.trace.json` based on seed `10018`, or an equivalently named deterministic fixture whose terminal payload captures the contradiction regression;
- new `games/river_ledger/tests/golden_traces/split-winner-order-vs-remainder.trace.json` based on seed `31` and button `seat_2`;
- update existing `high-card-showdown`, `pair-beats-high-card`, `split-pot-even`, and `split-pot-remainder-button-order` traces only where the canonical label/order migration intentionally changes serialized output;
- review every changed hash and field. Do not bulk-accept unrelated trace churn.

#### Browser/e2e

- Extend `apps/web/e2e/river-ledger.smoke.mjs` to assert one consistent winner label across the status region, Outcome announcement, V2 banner, decisive reason, and standings.
- Assert the live-region announcement contains the same winner set and no raw `seat_N` token.
- Assert split copy names all co-winners and distinguishes equal share from remainder receipt without presenting an odd-chip recipient as the sole winner.

#### No-leak

- Re-run observer plus every seat-private view hash for both regression fixtures.
- The unique-winner and split traces may reveal only showdown-eligible cards authorized by `RL-VIS-SHOWDOWN-001`.
- Folded/non-revealed private cards remain absent from payloads, DOM, accessibility text, logs, test IDs, storage, effect logs, and public replay export.

---

## 8. Workstream 2 — Active seat count across viewpoint row, setup copy, and seat rail

### 8.1 Root cause

`ui_metadata()` conflates two valid but different concepts:

- **capability metadata:** River Ledger supports up to six seats;
- **match inventory:** this particular match contains three, four, five, or six seats.

The shared shell consumes the capability list as the active list:

- `SeatFrame.tsx` renders all catalog labels in the viewpoint row and seat rail;
- `MatchSetup.tsx` renders all catalog labels in play-mode prose and role rows;
- a four-seat match therefore displays two phantom seats as “WAITING,” and setup copy claims six local/automated seats.

### 8.2 Prescribed fix

#### Rust/game projection

1. Keep River Ledger catalog capability metadata explicit:
   - supported counts `3, 4, 5, 6`;
   - minimum, default, and maximum;
   - a stable label for every supported seat ID.
2. Add or correct a **match-scoped active seat-label projection** whose rows are exactly the active state's seats in authoritative order. The recommended implementation is a River Ledger `ui_metadata_for_seat_count(count)`/`active_seat_labels(state)` path used by `visibility.rs`, while the catalog continues to use capability metadata.
3. Validate the count in Rust. The browser must never obtain an active-seat list for an unsupported count.
4. The active label rows must contain only public ID/label information; they must not include private cards, roles not already public, or hidden viewer authorization.
5. No match-scoped active-seat-label field exists on the shared WASM view envelope today — the seam carries only catalog `seat_labels` and the current `active_seat` actor ID — so a new Rust-owned typed active-seat-label field is **required**, not conditional. Add the smallest typed field needed and document it in `WASM-CLIENT-BOUNDARY.md` if that document's contract surface changes. The active list MUST be authored in Rust; TypeScript MUST NOT derive it by slicing catalog labels to the selected count (that would let TS decide the active-seat inventory, violating §2 behavior authority).

#### Shared TypeScript presentation

1. Replace `catalogSeatLabels(game)` with a resolver that distinguishes context:
   - **in match/replay:** use the Rust-projected active match label rows;
   - **before match:** use the selected supported seat count and Rust catalog labels in authoritative order;
   - **fixed two-seat fallback:** allowed only when the Rust catalog declares exactly one supported count of two and no match-scoped list is available.
2. The resolver must not invent an ID, parse `seat_N`, or silently pad to `max_seats`.
3. The seat rail and viewpoint row must consume the same resolved active list so they cannot drift.
4. `MatchSetup.modeDetail()` and `setupSeatRoles()` must receive `selectedSeatCount` (or the resolved selected active label rows) rather than reading all catalog labels.
5. If selected count and catalog labels disagree, fail closed to count-only generic copy and raise a dev assertion; do not show phantom seats.

### 8.3 Acceptance criteria

For River Ledger setup and live matches:

| Selected/active count | Viewpoint options | Setup role rows/copy | Seat rail |
|---:|---:|---:|---:|
| 3 | Observer + 3 seats | exactly 3 seats | exactly 3 seats |
| 4 | Observer + 4 seats | exactly 4 seats | exactly 4 seats |
| 5 | Observer + 5 seats | exactly 5 seats | exactly 5 seats |
| 6 | Observer + 6 seats | exactly 6 seats | exactly 6 seats |

Additional criteria:

1. A four-seat hotseat setup says exactly Seats 1–4 are local; it does not mention Seats 5–6.
2. A four-seat bot-vs-bot setup says all **4** seats are automated.
3. A four-seat match has no phantom “WAITING” rows for `seat_4` or `seat_5`.
4. Catalog/setup may still communicate that River Ledger supports up to six seats; capability copy is not removed, only kept out of match-scoped surfaces.
5. Fixed two-seat games continue to show exactly Observer + two seats and exactly two seat-rail entries.
6. Active seat order equals the Rust state/view order and remains deterministic across replay export/import.
7. No active-seat projection contains hidden information.

### 8.4 Test, trace, and no-leak obligations

- Rust unit/serialization tests for active label rows at counts `3, 4, 5, 6`.
- Setup validation tests continue to reject every unsupported count.
- TypeScript/component tests for setup copy and resolved labels at all four River counts.
- River Ledger e2e starts 3-, 4-, 5-, and 6-seat matches and counts selector options and rail rows.
- Replay import/reset test verifies the active list is restored from the replayed Rust projection, not stale setup state.
- Public observer and every seat-private payload contain identical active public label rows but different authorized private data only where rules allow.
- Existing setup traces (`setup-3p` through `setup-6p`) are updated only if the projected public UI metadata intentionally changes; viewer hashes are reviewed individually.

---

## 9. Workstream 3 — General seat-to-viewpoint mapping in the shared shell

### 9.1 Root cause

`apps/web/src/main.tsx::changeSeatFrameViewerMode()` contains an explicit two-seat allowlist:

```ts
if (viewerMode.seat === "seat_0" || viewerMode.seat === "seat_1") {
  changeViewerMode({ kind: "seat", seat: viewerMode.seat });
}
```

`SeatFrame` renders buttons for Seats 2–5 from catalog metadata, but the callback silently discards them. The fixed two-seat catalog hid this design defect until River Ledger became the first 3–6-seat game.

### 9.2 Prescribed fix

#### Shared-shell mapping

1. Replace the hardcoded allowlist with a generic type guard against the current Rust-projected active seat IDs.
2. When the requested ID is active, forward `{ kind: "seat", seat: requestedId }` to the existing WASM viewer API.
3. When it is not active, make no seat-private request. Normalize to observer or preserve the last valid projection according to one documented policy; observer is recommended because it is unambiguously safe.
4. Keep action authority separate from viewing authority. Selecting another viewpoint does not make that seat a legal human actor in human-vs-bot or hotseat orchestration.
5. Remove/narrow two-seat helpers only where they are incorrectly used for viewer selection. Two-seat play-mode orchestration may remain game-specific where that is its actual contract, but it may not gate the generic viewer selector.
6. Normalize selected viewer state whenever match ID, game, active seat set, replay document, or replay cursor changes and the selected seat is no longer present.
7. No API type should require casting an arbitrary `string` to a seat union without validating it against Rust-projected IDs.

#### Control semantics and accessibility

1. Present Observer + active seat choices as one named one-of-many control:
   - prefer native radio inputs in a `fieldset`/`legend`; or
   - use `role="radiogroup"`, `role="radio"`, `aria-checked`, roving `tabIndex`, Space, and arrow-key behavior matching the APG pattern.
2. The selected viewer is programmatically and visually evident without color alone.
3. Focus remains visible; each option meets the 24 CSS-pixel target baseline or the spacing exception.
4. Three-to-six seat rows wrap/reflow without horizontal page overflow and without losing the selected state.

### 9.3 Acceptance criteria

1. In a six-seat River Ledger match, Observer and Seats 1–6 all respond to pointer and keyboard selection.
2. Selecting internal `seat_2` requests and renders only `seat_2`'s authorized projection; the visible label is Seat 3.
3. Selecting `seat_5` requests and renders only `seat_5`'s authorized projection; the visible label is Seat 6.
4. Observer selection removes all private hole-card identities that are not public.
5. Repeated Observer → Seat A → Seat B → Observer transitions do not leave Seat A's private cards in DOM, accessible text, logs, storage, animation payloads, or replay export.
6. An unknown or stale seat ID cannot be forwarded to WASM and cannot broaden the projection.
7. All fixed two-seat games retain their existing Observer/Seat 1/Seat 2 behavior through the same generic path.
8. Viewer selection never changes legal actor, bot scheduling, command seat, or terminal outcome.

### 9.4 Test and no-leak obligations

- Unit/component tests for active-ID validation, observer mapping, stale selection, and selected-state semantics.
- Keyboard e2e for Tab entry, arrow movement, Space selection, and visible focus.
- River Ledger six-seat e2e selects every seat and verifies the matching private-view owner label.
- Pairwise no-leak loop for River Ledger: for each source seat A and distinct viewer B, B never receives A's private cards before authorized showdown reveal.
- Shared hidden-information regression at minimum for `high_card_duel`, `secret_draft`, `poker_lite`, `plain_tricks`, `masked_claims`, `flood_watch`, and `event_frontier`.
- `a11y-noleak.smoke.mjs` and per-game smoke suites must inspect hidden DOM text, `aria-label`, `data-testid`, logs, and replay exports after switching viewpoints.

---

## 10. Workstream 4 — Private-view card suit containment and centering

### 10.1 Root cause

`RiverLedgerCard` already renders the correct neutral content, but its compact box and descendants do not form a strict containment contract:

- the declared width is combined with padding under the current box model;
- rank and suit are centered independently rather than as a deliberate two-row composition;
- the suit text has no bounded inline size or tested longest-label treatment;
- `diamonds` can overflow the private card by roughly 17 pixels in the observed layout.

### 10.2 Prescribed fix

Keep the component markup and public vocabulary unless a minimal wrapper is required for alignment. The recommended CSS treatment is:

1. set the card to `box-sizing: border-box` so width includes padding and border;
2. use a two-row grid or equivalent contained flex column and center with `place-items: center`/`align-items: center`;
3. give rank and suit children `min-width: 0` and `max-inline-size: 100%`;
4. make the suit group occupy the available inline width and center its glyph/text;
5. use a bounded responsive suit-word font token and line-height that keeps all four full English suit words inside the existing card at supported sizes;
6. retain the full word in visible text and the existing accessibility label;
7. use overflow containment only as a safety net—normal supported rendering must not clip, ellipsize, or split a suit word;
8. apply the same containment to `board`, `private`, and `showdown` tones so one card context does not regress another.

No new art, suit icon set, card ratio, corner-index system, shadow language, or casino treatment is authorized.

### 10.3 Acceptance criteria

1. `clubs`, `diamonds`, `hearts`, and `spades` remain fully visible and centered in every River Ledger card tone.
2. Rank, glyph, and suit word remain within the card's border box at normal browser font settings, 200% text resize, supported narrow viewport, and reduced-motion mode.
3. Bounding-box checks show no child extending beyond the card's inline edge.
4. Rank and suit composition is visually centered; the longest word does not pull the group to one side.
5. The full suit word remains in the DOM and accessibility name; there is no glyph-only or color-only dependency.
6. Existing neutral appearance, tones, card IDs, data payloads, and public terminology remain unchanged.

### 10.4 Test and accessibility obligations

- Add a deterministic component fixture containing all four suits and representative one- and two-character ranks.
- Extend River Ledger browser smoke to measure card and child bounding rectangles in private, board, and showdown contexts.
- Test at 200% browser text zoom and a 320-CSS-pixel page width; no suit text may disappear.
- Run accessibility/no-leak checks to confirm the layout change does not expose hidden cards in observer mode.
- No golden trace should change for a CSS-only repair. A trace change indicates accidental data/copy churn and must be investigated.

---

## 11. Platform-wide impact list

At the target commit, River Ledger is the only variable-seat public game. It is directly broken by capability-as-active-seat rendering. The fourteen earlier games are fixed two-seat games; they are shared-shell consumers whose current success depends on the same hardcoded two-seat assumption that broke River Ledger.

| Game | Declared shape at target commit | Required work/proof |
|---|---|---|
| `river_ledger` | Variable 3–6 seats; hidden information | Direct active-count fix, generic Seat 3–6 selection, full pairwise no-leak, showdown regressions |
| `race_to_n` | Fixed two seats | Shared-shell regression: exactly two seat options/rows, unchanged play behavior |
| `three_marks` | Fixed two seats | Shared-shell regression |
| `column_four` | Fixed two seats | Shared-shell regression |
| `directional_flip` | Fixed two seats | Shared-shell regression |
| `draughts_lite` | Fixed two seats | Shared-shell regression |
| `high_card_duel` | Fixed two seats; hidden information | Shared-shell regression plus viewpoint no-leak |
| `token_bazaar` | Fixed two seats | Shared-shell regression |
| `secret_draft` | Fixed two seats; hidden commitments | Shared-shell regression plus viewpoint no-leak |
| `poker_lite` | Fixed two seats; hidden cards/showdown | Shared-shell regression plus viewpoint no-leak/outcome label check |
| `plain_tricks` | Fixed two seats; hidden hands | Shared-shell regression plus viewpoint no-leak |
| `masked_claims` | Fixed two seats; hidden claims/tiles | Shared-shell regression plus viewpoint no-leak |
| `flood_watch` | Fixed two seats; viewer-sensitive card/role state | Shared-shell regression plus viewpoint no-leak |
| `frontier_control` | Fixed two seats | Shared-shell regression |
| `event_frontier` | Fixed two seats; hidden deck/future information | Shared-shell regression plus viewpoint no-leak |

### Platform rule

There is no per-game exception for the generic viewer callback. Every game uses the same “requested ID must be in the Rust-projected active seat set” rule. A fixed two-seat game may continue to project exactly two labels; it may not rely on a shell hardcode.

The implementation does **not** need to retrofit variable-seat support into the fourteen fixed games. It needs to prove that the shared repair preserves their declared two-seat contracts and hidden-information boundaries.

---

## 12. Expected deliverables and files

The implementation is expected to touch only the smallest coherent subset of these files. Tickets must name exact files after reassessment.

### Rust — River Ledger

- `games/river_ledger/src/showdown.rs`
- `games/river_ledger/src/pot.rs`
- `games/river_ledger/src/ui.rs`
- `games/river_ledger/src/state.rs` only if an internal/public carrier must change
- `games/river_ledger/src/visibility.rs`
- `games/river_ledger/src/replay_support.rs` only if deterministic serialized output changes
- `games/river_ledger/tests/rules.rs`
- `games/river_ledger/tests/property.rs`
- `games/river_ledger/tests/replay.rs`
- `games/river_ledger/tests/serialization.rs`
- `games/river_ledger/tests/visibility.rs`
- targeted golden traces under `games/river_ledger/tests/golden_traces/`

### Rust/WASM seam

- `crates/wasm-api/src/lib.rs` — required for the match-scoped active-seat-label projection (no such field exists today); also any terminal payload shape change
- `apps/web/src/wasm/client.ts` matching the Rust JSON shape

### Shared web shell and River renderer

- `apps/web/src/main.tsx`
- `apps/web/src/components/SeatFrame.tsx`
- `apps/web/src/components/MatchSetup.tsx`
- `apps/web/src/components/RiverLedgerBoard.tsx`
- `apps/web/src/components/RiverLedgerCard.tsx` only if a minimal wrapper/class is required
- `apps/web/src/styles.css`
- `apps/web/e2e/river-ledger.smoke.mjs`
- `apps/web/e2e/shell.smoke.mjs`
- `apps/web/e2e/a11y-noleak.smoke.mjs`
- affected fixed-two-seat e2e scripts only where an explicit selector regression is added

### Documentation and closeout

- `games/river_ledger/docs/UI.md`
- `games/river_ledger/docs/RULE-COVERAGE.md`
- `games/river_ledger/docs/RULES.md` only for clarification, not rule changes
- `specs/README.md` to add this non-gate spec as `Planned`, then mark it `Done` at closeout
- this spec's Outcome section

---

## 13. Ticket-sized decomposition

Every ticket must use the shape in `tickets/_TEMPLATE.md`: Problem, Assumption Reassessment, Architecture Check, Verification Layers, What to Change, Files to Touch, Out of Scope, Acceptance Criteria, Invariants, New/Modified Tests, and exact Commands. Every ticket is one reviewable diff and must leave its scoped tests green.

### RIVLEDFIX-001 — Preserve canonical winners; separate remainder order

- **Priority:** P0
- **Engine changes:** River Ledger only; no `engine-core`
- **Scope:** Refactor `pot.rs`/`showdown.rs` so canonical winners remain stable-seat ordered and button order is remainder-only. Add seed `31` regression and allocation properties.
- **Acceptance:** semantic winner arrays, narration inputs, and standings use canonical order; remainder allocation still follows `RL-POT-REMAINDER-001`; conservation holds.
- **Gate:** Gate 0 plus targeted River Ledger native Gate 1.

### RIVLEDFIX-002 — Unify Rust public seat labels and close seed-10018 contradiction

- **Priority:** P0
- **Engine changes:** River Ledger UI metadata/projection only
- **Scope:** Make catalog and narration labels delegate to one Rust function; remove River board ID parsing; add seed `10018` native/browser regression.
- **Acceptance:** internal `seat_0` is always publicly Seat 1; every unique-winner surface agrees; no raw IDs or zero-based public winner copy.
- **Deps:** RIVLEDFIX-001
- **Gate:** Gate 0, targeted River tests, web build, River e2e.

### RIVLEDFIX-003 — Canonical resolved-showdown assembly and invariant checks

- **Priority:** P0
- **Engine changes:** River Ledger only
- **Scope:** Consolidate `resolve_showdown()` derivation into one internal result; route terminal outcome, V2 banner, decisive reason, explanations, standings, and allocations through it; add invariant assertions.
- **Acceptance:** no downstream builder chooses a winner source independently; ties name all co-winners.
- **Deps:** RIVLEDFIX-001, RIVLEDFIX-002
- **Gate:** Gate 0 plus River native Gate 1.

### RIVLEDFIX-004 — Trace, replay, serialization, and rule-coverage reconciliation

- **Priority:** P0
- **Engine changes:** None beyond deterministic evidence
- **Scope:** Add two new golden traces; regenerate only intentional label/order changes; review hashes; update `RULE-COVERAGE.md` rows for `RL-SCORE-SHOWDOWN`, `RL-SCORE-SPLIT`, `RL-POT-REMAINDER-001`, `RL-UI-SHOWDOWN-001`, and visibility rows.
- **Acceptance:** replay-check and serialization tests pass; no trace schema/version change; no unexplained churn.
- **Deps:** RIVLEDFIX-003
- **Gate:** Gate 0 and complete River Gate 1 replay/fixture/rule-coverage commands.

### RIVLEDFIX-005 — Project active match seat labels through Rust/WASM

- **Priority:** P1
- **Engine changes:** River Ledger view projection; WASM seam change required (new Rust-owned active-seat-label field; no such field exists today)
- **Scope:** Separate catalog capability labels from exact active-match labels for 3/4/5/6 seats; type the seam. The active list is Rust-authored, not a client-side slice of catalog labels.
- **Acceptance:** match projection contains exactly active seats in Rust order; no private data in label rows.
- **Deps:** RIVLEDFIX-002
- **Gate:** Gate 0, River serialization/visibility, WASM smoke/build.

### RIVLEDFIX-006 — Make SeatFrame active-seat scoped and viewer mapping generic

- **Priority:** P1
- **Engine changes:** None
- **Scope:** Consume active labels, remove the `seat_0 || seat_1` viewer guard, validate requested IDs against active projection, normalize stale selection, implement accessible one-of-many semantics.
- **Acceptance:** every active River seat and Observer works; unknown seats fail closed; fixed two-seat games remain correct.
- **Deps:** RIVLEDFIX-005
- **Gate:** Web build, shell smoke, a11y/no-leak smoke, River e2e, full fixed-two-seat selector regression.

### RIVLEDFIX-007 — Make setup prose and role rows use selected supported count

- **Priority:** P1
- **Engine changes:** None
- **Scope:** Thread selected seat count/resolved setup labels through `modeDetail()` and `setupSeatRoles()`; remove max-capability use from match-intent copy.
- **Acceptance:** 3/4/5/6 copy and role rows are exact; unsupported/mismatched data fails safely.
- **Deps:** RIVLEDFIX-005
- **Gate:** Web build, shell/setup smoke, River e2e.

### RIVLEDFIX-008 — Cross-catalog viewer and no-leak regression matrix

- **Priority:** P1
- **Engine changes:** None expected
- **Scope:** Add shared-shell coverage for all fourteen fixed two-seat games and targeted hidden-information viewpoint transitions.
- **Acceptance:** generic mapping replaces hardcoded success; no private-state residue across viewer changes.
- **Deps:** RIVLEDFIX-006
- **Gate:** Full `smoke:e2e`, `a11y-noleak`, presentation-copy guard, workspace tests.

### RIVLEDFIX-009 — Contain and center River Ledger card suit text

- **Priority:** P2
- **Engine changes:** None
- **Scope:** Surgical component/CSS containment; full suit words retained; bounding-box and resize checks.
- **Acceptance:** no overflow/clipping in private, board, or showdown cards; no redesign or data churn.
- **Deps:** None; may run after P0 fixes to avoid mixed screenshots
- **Gate:** Web build, River e2e, a11y/no-leak smoke.

### RIVLEDFIX-010 — Documentation and closeout

- **Priority:** P1
- **Engine changes:** None
- **Scope:** Reconcile `UI.md`, `RULE-COVERAGE.md`, `specs/README.md`, exact command evidence, and this Outcome section; archive tickets through the normal workflow when complete.
- **Acceptance:** every exit criterion has evidence; all CI gates pass; no stale “Planned” status remains.
- **Deps:** RIVLEDFIX-004 through RIVLEDFIX-009
- **Gate:** Full Gate 0, full Gate 1, existing Gate 2 threshold lane, docs checks.

---

## 14. Exit criteria

The spec is complete only when every row is satisfied.

| # | Exit criterion | Required evidence |
|---:|---|---|
| 1 | Unique-winner identity is coherent | Seed `10018` native regression and browser assertion show one winner/label across terminal, banner, reason, standings, allocation, and accessibility output |
| 2 | Split semantic order is not payout order | Seed `31` test shows canonical winners `[seat_1, seat_2, seat_3]` and remainder order `[seat_2, seat_3, seat_1]` with correct shares |
| 3 | One Rust winner source feeds all showdown surfaces | Code review plus invariant tests; no independent downstream winner derivation |
| 4 | Public seat labels are single-source | `ui.rs` all-seat test; no River React ID-to-label parsing; presentation-copy guard passes |
| 5 | Active seat count drives match surfaces | 3/4/5/6 River setup/live e2e counts viewpoint options, role copy, and rail rows |
| 6 | Every active viewpoint works | Six-seat pointer/keyboard e2e selects Observer and Seats 1–6 |
| 7 | Pairwise no-leak survives viewpoint switching | River all-pairs visibility tests plus hidden-information shared-shell smoke |
| 8 | Fixed two-seat games do not regress | Full catalog e2e shows exactly two active seats and functional selector behavior |
| 9 | Card text is contained and centered | Bounding-rectangle, 200% resize, 320px-width, and visual review evidence for all suits/tones |
| 10 | Determinism is reconciled | Replay, serialization, view hashes, golden traces, and trace ordering pass with reviewed intentional migrations only |
| 11 | No ownership violation or unearned abstraction | Boundary check and architecture review pass; no mechanic nouns enter `engine-core`; no TS winner/seat legality |
| 12 | Docs and status are closed | `UI.md`, `RULE-COVERAGE.md`, `specs/README.md`, tickets, and Outcome agree |

---

## 15. Acceptance evidence and CI gates

### 15.1 Gate 0 — required for every implementation ticket

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
cargo test --workspace
bash scripts/boundary-check.sh
```

A web-only ticket may run targeted commands during development, but the merged ticket must not knowingly break Gate 0. The capstone runs the full set.

### 15.2 River Ledger native Gate 1

```bash
cargo test -p river_ledger
cargo run -p simulate -- --game river_ledger --games 1000 --seat-count 6 --action-cap 48
cargo run -p replay-check -- --game river_ledger --all
cargo run -p fixture-check -- --game river_ledger
cargo run -p rule-coverage -- --game river_ledger
```

Add targeted commands for the two locked regression tests to each P0 ticket so reviewers can reproduce them directly.

### 15.3 Web/WASM Gate 1

```bash
npm --prefix apps/web ci
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run build
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:effects
npm --prefix apps/web run smoke:e2e
```

The full `smoke:e2e` run is required because the viewpoint callback is shared across the catalog.

### 15.4 Documentation and presentation guards

```bash
node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
node scripts/check-player-rules.mjs
node scripts/check-presentation-copy.mjs
node scripts/check-outcome-explanations.mjs
```

### 15.5 Gate 2

The intended fix does not alter evaluator complexity or simulation strategy, so new benchmark thresholds are not authorized. Existing Gate 2 must remain green:

```bash
cargo bench -p river_ledger
```

If implementation changes evaluator/search work rather than only result assembly, stop and reassess scope before accepting any threshold change.

### 15.6 Manual review matrix

- River Ledger at 3, 4, 5, and 6 seats;
- observer plus every seat-private viewpoint;
- unique winner, even split, and remainder split;
- hotseat, human-vs-bot, bot-vs-bot, replay import/step/reset;
- normal and reduced motion;
- keyboard-only selector use;
- 320 CSS-pixel layout and 200% text resize;
- all four suits in private, board, and showdown card tones;
- DOM/accessibility/log/storage scan after repeated viewer switching.

---

## 16. Forbidden implementation shortcuts

1. Do not “fix” seed `10018` by changing evaluator output; the evaluator result is correct.
2. Do not choose zero-based public labels merely because TypeScript currently emits them; public labels are Rust-authored and must be migrated consistently.
3. Do not sort winners in React.
4. Do not use allocation order as the semantic winner order.
5. Do not remove button-order remainder allocation.
6. Do not make `SeatFrame` show six options and merely disable inactive ones; inactive seats do not belong in a match-scoped selector.
7. Do not load every seat projection and toggle visibility in CSS.
8. Do not cast arbitrary strings to `ViewerSeatId` without active-list validation.
9. Do not keep stale private payloads in component state after a viewer change.
10. Do not shorten `diamonds` to hide an avoidable layout defect.
11. Do not update every golden trace wholesale or accept unrelated hash churn.
12. Do not add shared mechanic abstractions merely because both River Ledger and Poker Lite have cards/showdowns.

---

## 17. Documentation updates required

### `games/river_ledger/docs/UI.md`

Clarify:

- internal seat IDs versus one-based public labels;
- canonical semantic winner order versus button-order remainder distribution;
- one resolved-showdown source for banner, reason, standings, and allocation;
- active-match seat-label projection and match-scoped surface rule;
- full suit-word containment requirement.

### `games/river_ledger/docs/RULE-COVERAGE.md`

Map the new tests/traces to:

- `RL-SCORE-SHOWDOWN`;
- `RL-SCORE-SPLIT`;
- `RL-POT-REMAINDER-001`;
- `RL-SHOW-WINNER-001`;
- `RL-SHOW-SPLIT-001`;
- `RL-VIS-SHOWDOWN-001`;
- `RL-REPLAY-SERIAL-001`;
- `RL-UI-SEATS-001`;
- `RL-UI-SHOWDOWN-001`;
- `RL-UI-NOLEAK-001`.

### `games/river_ledger/docs/RULES.md`

No rule change is expected. A narrow clarification may state that button order selects remainder recipients but does not redefine or rank tied winners. Keep all stable `RL-*` IDs.

### `specs/README.md`

Add this non-gate fix spec as `Planned` after the two completed River Ledger UX specs. Mark it `Done` only after this spec's exit criteria pass and its Outcome section records evidence.

---

## 18. Sequencing

1. **P0 correctness first:** RIVLEDFIX-001 through RIVLEDFIX-004. Do not merge presentation-only suppression of contradictory copy before the canonical Rust result and label contract are fixed.
2. **Active-seat data before shell behavior:** RIVLEDFIX-005 precedes the generic selector and setup-copy tickets.
3. **Shared shell then catalog proof:** RIVLEDFIX-006 precedes RIVLEDFIX-008.
4. **Card containment can proceed independently** but should be reviewed after P0 output stabilizes so screenshots do not mix semantic and cosmetic changes.
5. **Closeout last:** RIVLEDFIX-010 records exact evidence and status.

No successor River Ledger showcase work should add further outcome or seat presentation fields until this spec is complete; otherwise it would build on contradictory identity and active-seat contracts.

---

## 19. Assumptions

1. Public River Ledger labels remain one-based (`Seat 1`–`Seat 6`); internal IDs remain zero-based (`seat_0`–`seat_5`).
2. Stable state/evaluation seat order is the canonical semantic order for tied winners.
3. `RL-POT-REMAINDER-001` continues to assign extra indivisible units in button order among tied winners.
4. The current V2 showdown payload remains the public presentation version; internal assembly may change without creating V3.
5. Catalog capability labels may include the maximum supported inventory, but match-scoped surfaces may not.
6. Full suit words remain the desired visible presentation; CSS must make them fit.
7. No existing accepted ADR explicitly supersedes the foundation or area-law sections engaged here.

Each assumption is one-line-correctable during ticket reassessment. Any correction that changes authority, trace schema, public numbering, or remainder rules requires spec reassessment before implementation.

---

## 20. Outcome

**Status:** Done.

Completed: 2026-06-18

Merged ticket range:

- `RIVLEDSHOSEA-001` through `RIVLEDSHOSEA-010`
- Ticket commits:
  - `8fe727a` — `RIVLEDSHOSEA-001`
  - `898b3f9` — `RIVLEDSHOSEA-002`
  - `5455fe5` — `RIVLEDSHOSEA-003`
  - `4bea70d` — `RIVLEDSHOSEA-004`
  - `6f440ad` — `RIVLEDSHOSEA-005`
  - `31a525f` — `RIVLEDSHOSEA-006`
  - `c11e230` — `RIVLEDSHOSEA-007`
  - `fcb238b` — `RIVLEDSHOSEA-008`
  - `576dcb4` — `RIVLEDSHOSEA-009`
  - Closeout commit: this commit (`Complete RIVLEDSHOSEA-010 docs closeout`)

Locked regression evidence:

- Seed `10018` unique-winner contradiction is covered by native/browser evidence in `RIVLEDSHOSEA-002` and reconciled golden trace evidence in `RIVLEDSHOSEA-004`. The final public surfaces agree that internal `seat_0` is displayed as `Seat 1`, and the River Ledger e2e smoke asserts the generic heading, V2 banner, live announcement, standings, and accessibility surface use the Rust-authored label consistently.
- Seed `31` split-order defect is covered by the native split/remainder tests and `split-winner-order-vs-remainder.trace.json`. The canonical semantic winner order is `["seat_1", "seat_2", "seat_3"]`; the remainder-recipient order remains `["seat_2", "seat_3", "seat_1"]`.

Golden trace and serialization reconciliation:

- Added `games/river_ledger/tests/golden_traces/showdown-seat-label-consistency.trace.json`.
- Added `games/river_ledger/tests/golden_traces/split-winner-order-vs-remainder.trace.json`.
- Migrated split/remainder trace output intentionally where the serialized `winners` and `allocations` now use canonical semantic order while `remainder_order` preserves button order.
- No trace schema/version migration was required. Final `cargo run -p replay-check -- --game river_ledger --all` accepted every River Ledger golden trace.

Active-seat and viewer evidence:

- Rust/WASM now projects `active_seat_labels` for River Ledger views.
- `SeatFrame` consumes active-match labels for the viewer radio group and seat rail; setup role copy consumes the selected supported seat count.
- River Ledger e2e covers 3/4/5/6 setup/live seat surfaces, six-seat pointer and keyboard selection, stale viewer normalization, and observer plus every active seat.
- Cross-catalog shell e2e covers every fixed two-seat catalog game through the generic active-ID selector path.

No-leak evidence:

- River Ledger pairwise pre-showdown browser switching verifies each seat sees only its own private cards, observer sees none, and every distinct source/viewer pair rejects source private card IDs across DOM text, attributes, `data-testid`, storage, and console.
- `a11y-noleak.smoke.mjs` and the full per-game hidden-information smoke suite cover the cross-catalog selector/no-leak matrix for `high_card_duel`, `secret_draft`, `poker_lite`, `plain_tricks`, `masked_claims`, `flood_watch`, and `event_frontier`.

Card containment evidence:

- River Ledger CSS now bounds rank, glyph, and full suit word inside the card grid for private, board, and showdown tones.
- `node apps/web/e2e/river-ledger.smoke.mjs` checks visible real cards plus an injected all-suit fixture (`clubs`, `diamonds`, `hearts`, `spades`) at normal size and 200% text / 320px viewport.
- `RiverLedgerCard.tsx` markup and card data shape did not change.

Documentation/status updates:

- `games/river_ledger/docs/UI.md` now documents Rust-authored public labels, active-match seat-label projection, canonical semantic winner order versus remainder order, pairwise viewer no-leak, and suit-word containment.
- `games/river_ledger/docs/RULE-COVERAGE.md` maps the final seed, trace, selector, no-leak, and card evidence to the relevant `RL-*` rows.
- `games/river_ledger/docs/RULES.md` clarifies that `RL-POT-REMAINDER-001` assigns odd units only and does not redefine or rank tied winners.
- `specs/README.md` records this non-gate fix series as `Done`.

Command evidence:

- Gate 0 capstone passed:
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo build --workspace`
  - `cargo test --workspace`
  - `bash scripts/boundary-check.sh`
- River Ledger Gate 1 capstone passed:
  - `cargo test -p river_ledger`
  - `cargo run -p simulate -- --game river_ledger --games 1000 --seat-count 6 --action-cap 48` (`games_run=1000`, six-seat order, `split_games=72`)
  - `cargo run -p replay-check -- --game river_ledger --all` (`replay-check: all traces passed`)
  - `cargo run -p fixture-check -- --game river_ledger`
  - `cargo run -p rule-coverage -- --game river_ledger`
- Web/doc capstone passed:
  - `npm --prefix apps/web ci` (install passed; npm reported one low-severity audit item)
  - `npm --prefix apps/web run smoke:wasm`
  - `npm --prefix apps/web run build`
  - `npm --prefix apps/web run smoke:ui`
  - `npm --prefix apps/web run smoke:effects`
  - `npm --prefix apps/web run smoke:e2e`
  - `node scripts/check-doc-links.mjs`
  - `node scripts/check-catalog-docs.mjs`
  - `node scripts/check-player-rules.mjs`
  - `node scripts/check-presentation-copy.mjs`
  - `node scripts/check-outcome-explanations.mjs`
- Gate 2 capstone passed:
  - `cargo bench -p river_ledger`; benchmark JSON emitted for setup, legal actions, apply call, all-viewer projection, public export/import, evaluator showdown batch, and level2 full playout.

Accepted deviations:

- `RIVLEDSHOSEA-008` centralized the fixed two-seat selector regression in `shell.smoke.mjs` rather than duplicating the same selector assertion in every per-game script. Full `smoke:e2e` still runs every per-game smoke.
- `RIVLEDSHOSEA-009` kept the card containment repair CSS-only; no wrapper was needed in `RiverLedgerCard.tsx`.
