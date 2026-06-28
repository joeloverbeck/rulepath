# web-shell-replay-import-size-roundtrip — Replay import/export round-trip for full-length games

- **Filename:** `specs/web-shell-replay-import-size-roundtrip.md`
- **File operation:** Create as a new living spec. Do not overwrite any archived spec.
- **Spec ID:** `web-shell-replay-import-size-roundtrip`
- **Ticket prefix:** `WEBREPLAYRT`
- **Target type:** New implementation spec (non-gate, shared web-shell correctness fix)
- **Roadmap stage:** Non-gate correctness fix for the shared web-shell replay import/export surface. Surfaced by the shipped Gate 20 game `starbridge_crossing`, but the defect lives in the shared component used by every game.
- **Roadmap build gate:** None. Independent of Gate 21. Blocks no other gate, but the documented replay round-trip is currently broken for any full-length game.
- **Status:** Planned (authored by `/playtest-game`; pending `/reassess-spec`)
- **Date:** 2026-06-28
- **Owner:** joeloverbeck
- **Provenance:** Authored against `main` @ `b652a45` (`joeloverbeck/rulepath`), the current `HEAD` when the defect was observed during a Starbridge Crossing playtest. Re-verify repo-relative references if `main` advances before decomposition.
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
2. The defensive purpose of an import-size guard (avoid freezing the tab on an
   absurd paste) MUST be preserved, but its threshold MUST NOT reject the app's
   own valid maximum-length exports.
3. The export and import affordances MUST NOT be asymmetric in a way that lets
   the shell produce a document it silently refuses to consume.

The fix is **presentation-layer only**. Rust/WASM remains the authority for
replay parsing, validation, determinism, and visibility. TypeScript owns only the
local affordance policy (the pre-WASM size guard and any UX around it).

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
  present, `commands.length === 2000`, no forbidden-term leaks).
- Clicked **Import Replay** on that unedited document. The Replay viewer stayed
  on its empty placeholder ("Export or import a replay to inspect it here.") and
  a diagnostic appeared: **`replay_too_large` — "Replay document exceeds the
  local import size limit."**

The shell exported a replay it then refused to import. The export side shows no
warning that the document is un-importable.

### 2.2 Root cause

`apps/web/src/components/ReplayImportExport.tsx`:

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

The cap is a **pure-TypeScript pre-check** that fires before the Rust/WASM
importer (`rulepath_import_replay`) is ever invoked. It is an undocumented magic
constant referenced nowhere in tests, docs, or other specs:

```
apps/web/src/components/ReplayImportExport.tsx:10  const MAX_IMPORT_CHARS = 128 * 1024;
apps/web/src/components/ReplayImportExport.tsx:25  if (documentText.length > MAX_IMPORT_CHARS) {
```

`ReplayImportExport` is a **shared web-shell component** used by every catalog
game (wired in `apps/web/src/main.tsx`). Earlier games' matches are short
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

A defensive UI limit that silently breaks the documented round-trip for a
shipped game is a shared-surface contract reconciliation, which is why this is a
spec (FOUNDATIONS §12: stop and reassess rather than generalize) rather than an
unprincipled bump of a magic number.

---

## 3. Scope

### In scope

- The replay import-size guard and any closely-coupled UX in
  `apps/web/src/components/ReplayImportExport.tsx`.
- A regression test (UI smoke / unit) proving an export→import round-trip for a
  full-length, maximum-seat match.
- Documentation of the chosen import policy where the shell's behavior is
  described.

### Out of scope

- Replay/trace **schema** changes, serialization order, or hash semantics
  (governed by ADR 0009; not needed here).
- Rust/WASM importer validation logic.
- Game rules, legality, scoring, terminal detection.
- Compressing or restructuring the export format as a behavioral change (may be
  considered as one design option in §5, but is not assumed).

### Not allowed

- Moving any legality/validation/determinism decision into TypeScript.
- Removing the guard entirely with no replacement protection against a
  pathological paste (unless reassessment proves the Rust importer + browser
  already bound the risk acceptably and records that rationale).
- Weakening, deleting, or skipping tests to get green.
- Introducing YAML/DSL or any data-driven-rules surface.

---

## 4. Deliverables

1. A corrected import policy in `ReplayImportExport.tsx` that admits the shell's
   own valid maximum-length exports while retaining a defensible upper bound.
2. A regression test at the right layer (UI smoke preferred, matching
   `apps/web/e2e/starbridge-crossing.smoke.mjs` patterns, and/or a component
   unit test) that:
   - exports a **full-length, maximum-seat** match (Starbridge 6-seat run to the
     turn-limit terminal, or an equivalent fixture of comparable size), and
   - re-imports the unedited export and asserts the Replay viewer loads it
     (cursor/standings render), with a no-leak assertion on the imported surface.
3. Documentation update describing the import policy and the round-trip
   guarantee (see §10).

---

## 5. Design options to resolve at reassessment

`/reassess-spec` MUST choose and justify one approach. Candidate directions:

1. **Raise the threshold to a justified bound.** Set the cap from a principled
   ceiling — e.g. comfortably above the largest legitimate self-export across the
   catalog (Starbridge 6-seat ≈ 549 KB today) with headroom for future games and
   format growth. Requires a stated rule for deriving the number, not a bare
   magic constant.
2. **Bound by structure, not raw character count.** Validate command count
   and/or shape before delegating to the Rust importer, so the guard tracks the
   actual cost driver rather than a string length that scales with formatting.
3. **Delegate size safety to the Rust importer + a soft UX guard.** Let WASM be
   the authority (it already parses/validates) and replace the hard reject with a
   non-blocking "large replay" notice. Must record why tab-freeze risk is
   acceptable.
4. **Asymmetry repair on the export side.** If any hard import cap is kept, the
   export affordance must not silently produce un-importable documents (warn, or
   guarantee export ≤ import bound).

Selection criteria: preserves the round-trip for all shipped games; keeps a
real defense against pathological paste; introduces no behavioral authority into
TypeScript; minimal blast radius on the shared component.

---

## 6. Exit criteria

1. Export→import round-trip succeeds in the running web shell for a Starbridge
   6-seat full-length match (no `replay_too_large` on the shell's own export).
2. The same holds for at least one short game (regression: small replays still
   import) and the chosen guard still rejects a genuinely pathological oversize
   input if a guard is retained.
3. New regression test added and passing; no existing test weakened or deleted.
4. `npm --prefix apps/web run build`, `npm --prefix apps/web run smoke:ui`, and
   the Starbridge e2e smoke pass.
5. Rust workspace gates remain green (`cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`) — expected unaffected, verified anyway.
6. No replay/trace schema, serialization order, or hash change (ADR 0009 not
   engaged); confirmed by unchanged golden traces and `replay-check`.

---

## 7. Acceptance evidence

- UI smoke / component test output showing the full-length round-trip.
- `smoke:ui` and Starbridge e2e smoke logs.
- `replay-check --game starbridge_crossing --all` unchanged (no schema/hash
  drift).
- No-leak assertion on the imported Replay viewer surface.
- Game-level Rust rule evidence: **not applicable** (presentation-only fix; no
  rule, trace, or fixture change).

---

## 8. FOUNDATIONS & boundary alignment

- **§2 / §11 behavior authority:** Rust/WASM keeps replay parsing, validation,
  determinism, and visibility. The change is a local affordance policy only.
- **§7 / UI protocol:** the shell continues to present Rust-owned views; it does
  not invent legality. The guard is an input-safety affordance, not a rule.
- **§9 local-first:** restores the promised local replay import/export round-trip.
- **§12 stop condition honored:** the defect was routed to a spec instead of an
  unprincipled magic-number change to a shared surface.
- **Hidden info:** Starbridge is all-public; the imported surface must still pass
  the standard no-leak scan (no `hidden_state`/`candidate_ranking`/etc.).

---

## 9. Forbidden changes

- No TypeScript legality/validation/determinism authority.
- No replay/trace schema, serialization order, or hash changes.
- No game-rule, scoring, or terminal-detection changes.
- No test deletion/weakening.
- No new data format, selector, or DSL.

---

## 10. Documentation updates required

- This index status flip in `specs/README.md` (add the Non-gate tracker row;
  flip to `Done` at closeout).
- Document the replay import policy and the export/import round-trip guarantee in
  the web-shell docs (`apps/web/README.md` and/or `docs/WASM-CLIENT-BOUNDARY.md` /
  `docs/TESTING-REPLAY-BENCHMARKING.md`, whichever the reassessment determines is
  the canonical home), so the chosen bound is no longer an undocumented constant.
- If Starbridge UI docs reference replay controls, reconcile
  `games/starbridge_crossing/docs/UI.md` only as needed (no behavior change).

---

## 11. Sequencing

- **Predecessor:** Gate 20 (`starbridge_crossing`) is shipped; this repairs a
  shared-surface defect surfaced by it.
- **Successor:** independent of Gate 21; does not block the mechanic ladder, but
  should close before further reliance on full-length replay round-trips.
- **Admission:** may be decomposed immediately via `/reassess-spec` →
  `/spec-to-tickets`.

---

## 12. Assumptions (one-line-correctable)

1. The 128 KB cap is the only barrier to importing the shell's own full-length
   exports (the Rust importer accepts the document once it is passed through).
2. Starbridge 6-seat at `max_plies=2000` (~549 KB) is the current largest
   legitimate self-export; re-measure across the catalog at reassessment.
3. No consumer depends on `replay_too_large` firing at exactly 128 KB.
