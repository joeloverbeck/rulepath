# web-shell-replay-import-size-roundtrip — Replay import/export round-trip for full-length games

- **Filename:** `specs/web-shell-replay-import-size-roundtrip.md`
- **File operation:** Create as a new living spec. Do not overwrite any archived spec.
- **Spec ID:** `web-shell-replay-import-size-roundtrip`
- **Ticket prefix:** `WEBREPLAYRT`
- **Target type:** New implementation spec (non-gate, shared web-shell + WASM-importer correctness fix)
- **Roadmap stage:** Non-gate correctness fix for the shared replay import/export surface. Surfaced by the shipped Gate 20 game `starbridge_crossing`, but the defect lives in the shared component and its authoritative Rust importer guard, used by every game.
- **Roadmap build gate:** None. Independent of Gate 21. Blocks no other gate, but the documented replay round-trip is currently broken for any full-length game.
- **Status:** Done
- **Date:** 2026-06-28
- **Owner:** joeloverbeck
- **Provenance:** Authored against `main` @ `b652a45` (`joeloverbeck/rulepath`), the current `HEAD` when the defect was observed during a Starbridge Crossing playtest. Reassessment confirmed the import-size cap is enforced in **two** layers — a TypeScript pre-check and the authoritative Rust importer guard — both at `128 * 1024`. Re-verify repo-relative references if `main` advances before decomposition.
- **Authority order:** `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area law (`docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/WASM-CLIENT-BOUNDARY.md`, `docs/UI-INTERACTION.md`, `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`) → `docs/ROADMAP.md` → `docs/AGENT-DISCIPLINE.md` → accepted ADRs → `games/starbridge_crossing/docs/RULES.md` → this spec.
- **Subordination:** This spec may not redefine legality, serialization order, replay/hash semantics, visibility, or any foundation contract. Where this spec and upstream law disagree, upstream law wins.

---

## 1. Objective

Make the web shell able to **re-import any replay it can itself export** for a
legitimate complete match, restoring the documented replay export/import
round-trip for full-length games while preserving a sane defensive guard against
pathological local input.

Concretely:

1. A user who clicks **Export Current Run** on a finished match and then clicks
   **Import Replay** on that exact, unedited document MUST get a loaded replay in
   the Replay viewer — for every supported game and seat count, including the
   longest legitimate matches.
2. The defensive purpose of an import-size guard (avoid an unresponsive tab on an
   absurd paste) MUST be preserved, but its threshold MUST NOT reject the app's
   own valid maximum-length exports.
3. The export and import affordances MUST NOT be asymmetric in a way that lets
   the shell produce a document it silently refuses to consume.

**Authority placement.** The authoritative import-size guard lives in **Rust/WASM**
(`import_replay` in `crates/wasm-api`), which already parses, validates, and
fail-closes on oversize input. Rust/WASM remains the authority for replay parsing,
validation, determinism, and visibility. The correct fix therefore (a) raises the
authoritative Rust bound to a principled ceiling that admits the catalog's own
maximum-length exports, and (b) stops TypeScript from shadowing that decision with
a stricter pre-WASM reject. TypeScript owns only presentation and a non-blocking
local affordance, never an accept/reject decision Rust already owns. See §5 for
the selected approach.

---

## 2. Defect evidence

### 2.1 Observed behavior

During a `/playtest-game starbridge_crossing` iteration (preview build of `main`
@ `b652a45`):

- Started a **6-seat bot-vs-bot** Starbridge match and ran autoplay to the
  deterministic terminal (`turn_limit:2000`, the documented default `max_plies`
  per `SC-FINISH-005`).
- Clicked **Export Current Run**. The shell produced a valid JSON replay
  document of **561,667 characters** (`game_id=starbridge_crossing`, `seed`
  present, `commands.length === 2000`, no forbidden-term leaks). The export is
  pretty-printed (`JSON.stringify(document, null, 2)`), so the measured length
  reflects the indented form.
- Clicked **Import Replay** on that unedited document. The Replay viewer stayed
  on its empty placeholder ("Export or import a replay to inspect it here.") and
  a diagnostic appeared: **`replay_too_large` — "Replay document exceeds the
  local import size limit."**

The shell exported a replay it then refused to import. The export side shows no
warning that the document is un-importable.

### 2.2 Root cause

The 128 KB import cap is enforced in **two layers**, both at `128 * 1024`:

**(a) A TypeScript pre-check** in `apps/web/src/components/ReplayImportExport.tsx`:

```ts
const MAX_IMPORT_CHARS = 128 * 1024; // 131072

const importReplay = () => {
  setDiagnostic(null);
  if (documentText.length > MAX_IMPORT_CHARS) {
    setDiagnostic({
      code: "replay_too_large",
      message: "Replay document exceeds the local import size limit.",
    });
    return; // returns BEFORE calling WASM
  }
  ...
};
```

```
apps/web/src/components/ReplayImportExport.tsx:10  const MAX_IMPORT_CHARS = 128 * 1024;
apps/web/src/components/ReplayImportExport.tsx:25  if (documentText.length > MAX_IMPORT_CHARS) {
```

**(b) The authoritative Rust/WASM importer guard.** The diagnostic code
`replay_too_large` is **not** a TypeScript invention — the Rust importer enforces
the same bound and emits the same code:

```
crates/wasm-api/src/constants.rs:116  pub(crate) const MAX_REPLAY_IMPORT_BYTES: usize = 128 * 1024;
crates/wasm-api/src/lib.rs:3477       if doc.len() > MAX_REPLAY_IMPORT_BYTES { ... "replay_too_large" ... }
crates/wasm-api/src/tests.rs:2060     let oversized = "x".repeat(MAX_REPLAY_IMPORT_BYTES + 1); // asserts replay_too_large
```

The TypeScript symbol `MAX_IMPORT_CHARS` is local to the component, but it is a
**stricter shadow of the authoritative Rust guard**. The consequence is decisive:
even if the TypeScript pre-check were removed or raised, `import_replay` would
still reject the 561,667-byte Starbridge export (`561667 > 131072`) with the same
`replay_too_large` diagnostic. **The fix must raise the bound in Rust (the
authority); the TypeScript change alone cannot restore the round-trip.**

`ReplayImportExport` is a **shared web-shell component** used by every catalog
game (wired in `apps/web/src/main.tsx`), and `import_replay` is the shared
authoritative importer for every game. Earlier games' matches are short
(card/small-board), so their full exports fit under 128 KB and the round-trip
appears to work. Starbridge Crossing is the first official game whose normal
complete export (long race × up to 6 seats × verbose per-command records)
exceeds the cap, exposing the latent defect for the whole shell.

### 2.3 Why this is a contract problem, not a cosmetic one

- `SC-REPLAY-001` / `SC-VIS-004`: a Starbridge run is fully public and its
  command stream must reproduce state deterministically; the export is offered as
  a first-class affordance.
- `SC-UI-002`: the public UI must support "replay/import/export controls".
- FOUNDATIONS §9 / §11: V1/V2 must support "local replay import/export" and keep
  replay deterministic; the round-trip is part of the multiplayer-readiness path.
- FOUNDATIONS §2 / §11: validation authority belongs to Rust. The size guard is
  correctly the Rust importer's; the TypeScript pre-check is a redundant
  acceptance decision that the shell should not own.

A defensive limit that silently breaks the documented round-trip for a shipped
game — and that is duplicated across the TS/WASM boundary in a way that obscures
where the authority lives — is a shared-surface contract reconciliation, which is
why this is a spec (FOUNDATIONS §12: stop and reassess rather than generalize)
rather than an unprincipled bump of a magic number.

---

## 3. Scope

### In scope

- The authoritative import-size bound in Rust:
  `MAX_REPLAY_IMPORT_BYTES` (`crates/wasm-api/src/constants.rs:116`) and the
  `import_replay` guard that consumes it (`crates/wasm-api/src/lib.rs:3477`),
  plus its test coverage in `crates/wasm-api/src/tests.rs`.
- The replay import-size affordance and any closely-coupled UX in
  `apps/web/src/components/ReplayImportExport.tsx` (removing the stricter TS
  shadow so the shell defers to the Rust diagnostic, per §5).
- A regression test at the right layer(s) proving an export→import round-trip for
  a full-length, maximum-seat match.
- Documentation of the chosen import policy where the shell's behavior is
  described.

### Out of scope

- Replay/trace **schema** changes, serialization order, or hash semantics
  (governed by ADR 0009; not needed here — raising a size-bound validation
  constant is not a schema/serialization/hash change).
- Rust/WASM importer **parsing, dispatch, and validation semantics** beyond the
  single size-bound constant and its tests. The fix touches the threshold value,
  not how documents are parsed, dispatched, or validated.
- Game rules, legality, scoring, terminal detection.
- Compressing or restructuring the export format as a behavioral change.

### Not allowed

- Moving any legality/validation/determinism decision into TypeScript. (The fix
  moves the size decision the other way — fully into Rust, where §2 places it.)
- Removing the authoritative Rust importer guard, or raising it without a stated
  derivation rule (no bare magic number).
- Weakening, deleting, or skipping tests to get green.
- Introducing YAML/DSL or any data-driven-rules surface.

---

## 4. Deliverables

1. **Raise the authoritative Rust bound.** Set `MAX_REPLAY_IMPORT_BYTES`
   (`crates/wasm-api/src/constants.rs`) from a principled ceiling — comfortably
   above the largest legitimate self-export across the catalog (Starbridge 6-seat
   ≈ 549 KB today) with ample headroom for future games and format growth — and
   record the derivation rule as a code comment (not a bare constant). The
   `import_replay` guard (`lib.rs:3477`) continues to fail-closed above the new
   bound.
2. **Remove the TypeScript shadow.** In `ReplayImportExport.tsx`, delete the
   `MAX_IMPORT_CHARS` pre-check so the import path delegates to WASM, surfacing
   the authoritative Rust `replay_too_large` diagnostic when (and only when) the
   Rust bound is exceeded. Optionally retain a **non-blocking** "large replay"
   notice, but it MUST NOT reject a document the Rust importer would accept.
   While here, address the render-time parse cost (see §5): memoize or
   length-gate the on-render `replayCommandSummary` parse so a pathological paste
   does not re-parse the full document on every keystroke.
3. **Regression tests at the right layers:**
   - A Rust unit test in `crates/wasm-api/src/tests.rs` asserting that a
     full-length Starbridge 6-seat export round-trips through `import_replay`
     (complementing the existing oversize-rejection test at `tests.rs:2060`,
     which remains valid because it is keyed to `MAX_REPLAY_IMPORT_BYTES + 1`).
   - A UI smoke / component test (matching `apps/web/e2e/starbridge-crossing.smoke.mjs`
     patterns and/or a component unit test) that exports a **full-length,
     maximum-seat** match (Starbridge 6-seat run to the turn-limit terminal, or an
     equivalent fixture of comparable size), re-imports the unedited export, and
     asserts the Replay viewer loads it (cursor/standings render), with a no-leak
     assertion on the imported surface.
4. Documentation update describing the import policy and the round-trip
   guarantee (see §10), naming the authoritative Rust bound as the single source
   of truth.

---

## 5. Selected resolution (chosen at reassessment)

`/reassess-spec` (2026-06-28) selected a **hybrid of options 1 and 3 below**,
forced by the discovery that the authoritative size guard already lives in Rust
(§2.2):

> **Raise the authoritative bound in Rust, and have TypeScript defer to it.**
> Derive `MAX_REPLAY_IMPORT_BYTES` from a principled ceiling — at least an order
> of magnitude above the largest legitimate catalog self-export (≈549 KB today),
> e.g. on the order of a few MiB — chosen to admit all current and foreseeable
> legitimate exports while still bounding a pathological multi-hundred-MB paste,
> with the derivation rationale recorded in code and docs. Replace the TypeScript
> hard reject with delegation to the Rust importer (surfacing its
> `replay_too_large` diagnostic), optionally plus a soft non-blocking notice.

**Why this and not the alternatives.** The candidate directions considered were:

1. **Raise the threshold to a justified bound** — adopted, applied to the Rust
   constant (the authority), not the TS shadow.
2. **Bound by structure (command count/shape) in TypeScript** — *rejected.*
   Validating command count or document shape in TS edges toward TypeScript
   deciding document validity, which §3 Not-allowed forbids and §2 reserves for
   Rust. Rust already owns a fail-closed size guard, so a TS structural check is
   both redundant and a behavior-authority smell.
3. **Delegate size safety to the Rust importer + a soft UX guard** — adopted for
   the TypeScript side. This is now strongly supported rather than speculative:
   the Rust importer's guard is already fail-closed, blocking, and tested
   (`tests.rs:2060`), so the spec's own Not-allowed carve-out ("unless
   reassessment proves the Rust importer + browser already bound the risk
   acceptably and records that rationale") is satisfied.
4. **Asymmetry repair on the export side** — not needed once the authoritative
   bound sits above every legitimate export: with the bound raised, the shell can
   no longer produce a self-export it refuses to import, so no export-side warning
   is required.

**Cost-driver note (informs the "pathological paste" rationale).** The component
parses the full document on every render: `ReplayImportExport.tsx:15` calls
`replayCommandSummary(documentText)` unmemoized, running `JSON.parse(documentText)`
on each keystroke — gated by neither the TS pre-check nor the Rust guard. The
import-*click* guard therefore never protected against the render-time parse of a
huge paste. Removing the TS import guard does not worsen this; the real
responsiveness protection is to memoize or length-gate the on-render summary parse
(Deliverable 2). The Rust guard bounds the WASM-side cost; the soft TS notice (if
kept) bounds the UX surprise.

Selection criteria honored: preserves the round-trip for all shipped games; keeps
a real, authoritative defense against pathological paste (in Rust); introduces no
behavioral authority into TypeScript (it removes one); minimal blast radius (one
Rust constant + its test, one TS pre-check removal + a render-parse guard, docs).

---

## 6. Exit criteria

1. Export→import round-trip succeeds in the running web shell for a Starbridge
   6-seat full-length match (no `replay_too_large` on the shell's own export).
2. The same holds for at least one short game (regression: small replays still
   import) and the authoritative Rust guard still rejects a genuinely pathological
   oversize input (`import_replay` returns `replay_too_large` above the new bound).
3. New regression tests added and passing (Rust round-trip + UI/component); no
   existing test weakened or deleted; the existing oversize-rejection test at
   `tests.rs:2060` still passes against the raised bound.
4. `npm --prefix apps/web run build`, `npm --prefix apps/web run smoke:ui`, and
   the Starbridge e2e smoke pass.
5. Rust workspace gates green (`cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`) — now exercises the changed constant and its tests.
6. No replay/trace schema, serialization order, or hash change (ADR 0009 not
   engaged — the bound is a validation-threshold constant); confirmed by unchanged
   golden traces and `replay-check`.

---

## 7. Acceptance evidence

- Rust unit-test output showing a full-length Starbridge 6-seat export round-trips
  through `import_replay`, plus the retained oversize-rejection assertion
  (`tests.rs`).
- UI smoke / component test output showing the full-length round-trip in the shell.
- `smoke:ui` and Starbridge e2e smoke logs.
- `replay-check --game starbridge_crossing --all` unchanged (no schema/hash
  drift).
- No-leak assertion on the imported Replay viewer surface.
- Game-level Rust **rule** evidence: **not applicable** (no rule, trace, scoring,
  or fixture change). The Rust change is confined to the importer's size-bound
  constant and its `wasm-api` tests.

---

## 8. FOUNDATIONS & boundary alignment

- **§2 / §11 behavior authority:** the authoritative import-size decision is
  moved fully into Rust (`import_replay`); the TypeScript stricter shadow is
  removed. Rust keeps replay parsing, validation, determinism, and visibility.
- **§7 / UI protocol:** the shell continues to present Rust-owned views and
  diagnostics; it no longer makes an independent accept/reject size decision.
- **§9 local-first:** restores the promised local replay import/export round-trip.
- **§12 stop condition honored:** the defect was routed to a spec instead of an
  unprincipled magic-number change to a shared surface.
- **§13 ADR 0009 (replay/hash semantics):** not engaged — raising a validation
  threshold constant changes no schema, serialization order, or hash.
- **Hidden info:** Starbridge is all-public; the imported surface must still pass
  the standard no-leak scan (no `hidden_state`/`candidate_ranking`/etc.).

---

## 9. Forbidden changes

- No TypeScript legality/validation/determinism authority (the fix removes a
  TS-side acceptance decision, not adds one).
- No replay/trace schema, serialization order, or hash changes.
- No game-rule, scoring, or terminal-detection changes.
- No changes to `import_replay` parsing/dispatch/validation semantics beyond the
  size-bound constant and its tests.
- No test deletion/weakening.
- No new data format, selector, or DSL.

---

## 10. Documentation updates required

- This index status flip in `specs/README.md` (the Non-gate tracker row is
  already present; flip to `Done` at closeout).
- Document the replay import policy and the export/import round-trip guarantee in
  the web-shell docs (`apps/web/README.md` and/or `docs/WASM-CLIENT-BOUNDARY.md` /
  `docs/TESTING-REPLAY-BENCHMARKING.md`, whichever the reassessment of the doc
  home determines is canonical), naming `MAX_REPLAY_IMPORT_BYTES` in
  `crates/wasm-api` as the single authoritative bound and recording its derivation
  rule, so it is no longer an undocumented constant.
- If Starbridge UI docs reference replay controls, reconcile
  `games/starbridge_crossing/docs/UI.md` only as needed (no behavior change).

---

## 11. Sequencing

- **Predecessor:** Gate 20 (`starbridge_crossing`) is shipped; this repairs a
  shared-surface defect surfaced by it.
- **Successor:** independent of Gate 21; does not block the mechanic ladder, but
  should close before further reliance on full-length replay round-trips.
- **Admission:** may be decomposed immediately via `/spec-to-tickets`.

---

## 12. Assumptions (one-line-correctable)

1. **(Corrected at reassessment.)** The 128 KB cap is enforced in *two* layers —
   the TypeScript `MAX_IMPORT_CHARS` pre-check and the authoritative Rust
   `MAX_REPLAY_IMPORT_BYTES` guard. Raising the Rust bound is necessary and
   sufficient to pass the document; removing the TS pre-check alone is not.
2. Starbridge 6-seat at `max_plies=2000` (~549 KB, pretty-printed) is the current
   largest legitimate self-export; re-measure across the catalog at decomposition
   to set the bound's headroom.
3. The existing Rust oversize-rejection test (`tests.rs:2060`) is keyed to
   `MAX_REPLAY_IMPORT_BYTES + 1`, so it survives a bound increase; no other
   consumer depends on the literal `128 * 1024` value (verified: the only sites
   are `constants.rs:116`, `lib.rs:3477`, `tests.rs:2060`, and the now-removed TS
   shadow).

## Outcome

Completed: 2026-06-28

- Completed tickets: `archive/tickets/WEBSHEREP-001.md`,
  `archive/tickets/WEBSHEREP-002.md`, `archive/tickets/WEBSHEREP-003.md`, and
  `archive/tickets/WEBSHEREP-004.md`.
- Implementation summary: `MAX_REPLAY_IMPORT_BYTES` is now an 8 MiB
  Rust/WASM-authoritative import guard with derivation comments; generic
  Starbridge replay import uses the exported or command-inferred seat count so
  6-seat self-exports replay against the correct setup; `ReplayImportExport`
  removed the stricter TypeScript hard reject and memoizes/length-gates command
  summary parsing; the Starbridge browser smoke now covers a full-length 6-seat
  export/import round-trip with no-leak checks.
- Deviations: full-length Starbridge import exposed the pre-existing generic
  Starbridge replay seat-count fallback, repaired in the `WEBSHEREP-001` commit.
  The browser smoke proves the Replay viewer loads at `Cursor 0 /` rather than
  requiring exact denominator text; the full-length property is proven by the
  exported document size and 6-seat setup assertion.
- Verification evidence: `cargo fmt --all --check`; `cargo test -p wasm-api
  starbridge_full_length_export_imports_above_legacy_size_cap -- --nocapture`;
  `cargo test -p wasm-api`; `cargo run -p replay-check -- --game
  starbridge_crossing --all`; `npm --prefix apps/web run build`; `npm --prefix
  apps/web run smoke:ui`; `node apps/web/e2e/starbridge-crossing.smoke.mjs`;
  `npm --prefix apps/web run smoke:e2e`; `node scripts/check-doc-links.mjs`;
  `node scripts/check-catalog-docs.mjs`.
- Archive truthing: active ticket glob emptied before final spec archive;
  archived tickets carry `**Status**: COMPLETED` and `## Outcome`; docs and
  `specs/README.md` now name the Rust/WASM bound as authoritative and remove the
  stale UI-side 128 KiB cap claim.
- Unrelated worktree changes left untouched: `.claude/skills/brainstorm/SKILL.md`,
  `.claude/skills/skill-audit/SKILL.md`, and `.claude/skills/playtest-game/`.
