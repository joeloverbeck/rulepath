# Codebase Validation (Step 3)

Validate every reference from Step 2 against the current codebase.

**Parallel grep-batch guard**: when validating via direct `grep` / `find` / `test` calls batched in one parallel turn, guard each with `|| true`. A non-zero exit from one call (grep finding nothing, `test` on a missing path) cancels its siblings in the batch. Zero-match results are expected and valid during validation. Prefer several small independent Bash calls over one long `{ …; } | head` compound — a sub-command that dies mid-block silently loses every later command's output; if a compound block returns fewer results than commands, re-run the missing checks individually rather than trusting the partial output.

Substep applicability by Pre-Process classification:

| Substep | (a) new | (b) extension | (c) refactor | (d) retroactive |
|---|---|---|---|---|
| 3.0 Cross-file scope | ✓ | ✓ | ✓ | skip |
| 3.1 File paths | ✓ | ✓ | ✓ | ✓ (rigorous) |
| 3.2 Types / schema / contract fields | ✓ | ✓ | ✓ | ✓ (rigorous) |
| 3.3 Functions / exports / commands | ✓ | ✓ | ✓ | ✓ (rigorous) |
| 3.4 Dependencies | ✓ | ✓ | ✓ | ✓ (rigorous) |
| 3.5 Skill-structure | ✓ | if SKILL.md changes structurally | if content moves between SKILLs | skip |
| 3.6 Downstream consumers | ✓ | ✓ | skip | skip |
| 3.7 Upstream spec / gate refs | ✓ | ✓ | ✓ | skip |
| 3.8 FOUNDATIONS-contract fidelity | ✓ | if behavior-authority/boundary/acceptance-invariant/visibility/replay semantics touched | skip | skip |
| 3.9 New-deliverable consumer verification | ✓ | ✓ | skip | skip |
| 3.10 Source-document completeness | ✓ | ✓ | skip | ✓ (rigorous — verify landing) |
| 3.11 Spec structural completeness | ✓ | ✓ | skip | skip |

For specs with >10 references, consider parallel Explore agents organized by theme (max 3). Spot-check agent claims with direct Grep/Read before including in findings — agent results are leads, not facts; trust a direct tool result over an agent claim.

**Greenfield / foundation specs** (the spec's deliverables ARE the repo's first code — no pre-existing code tree, e.g. the Gate 0 repository-skeleton spec): the substep table still applies, but the consumer-grep substeps have nothing to grep. Handle them explicitly rather than skipping silently:

- **3.6 Downstream Consumers**: record `N/A — greenfield, no code tree` instead of an empty grep. There are no existing call sites because there is no code yet.
- **3.9 New-Deliverable Consumer Verification**: the deliverables' consumers are **planned**, not present — every later gate named in `docs/ROADMAP.md` or the `specs/README.md` index (and the spec's own Sequencing section). That satisfies the "explicitly planned" branch; do NOT fire the zero-consumer HIGH Issue for a foundation spec whose consumers are unbuilt-but-sequenced future gates. Record the planned consumers (the index's gate list) for the audit trail.

This is distinct from `SKILL.md`'s "no greenfield approach proposals" guardrail, which is about not proposing alternative designs — here "greenfield" means the *repo* (or the relevant crate tree) is empty.

## 3.0 Cross-File Scope Establishment

For patterns referenced across multiple files (type imports, schema-field usage, command/tool invocations), run a cross-file count grep first to establish full scope before per-file analysis. Compare the spec's claimed locations against the actual count — this catches files the spec missed and prevents incomplete deliverables.

## 3.1 File Paths

Glob/Grep to confirm each path exists. If moved, renamed, or deleted, record the actual location. Distinguish existing paths (must exist now) from proposed paths (exist after implementation); proposed paths still need their parent directory to exist (or to be created by a named deliverable), must not collide with an existing file, and must follow conventions (kebab-case filenames, `<gate>-<slug>.md` for specs under `specs/`, `<PREFIX>-NNN.md` ticket files under `tickets/` (from `tickets/_TEMPLATE.md`), crate dirs under `crates/`, game dirs under `games/`, tools under `tools/`, per `docs/ARCHITECTURE.md`).

**Name-collision check for proposed paths**: when a spec proposes a NEW file or crate, list the parent directory for SIMILAR filenames (substring match on the distinctive token, not exact-path). A proposed file whose parent already holds a near-name sibling occupying the same conceptual slot is a HIGH Issue — the spec should MODIFY the existing file instead. An exact-path existence check passes this silently.

**Generated-artifact check for edited paths**: for each existing file a spec proposes *editing*, check whether it is a generated/derived artifact rather than a hand-authored source. Grep the path across `scripts/`, build configs, and any codegen sources (e.g. `diff` the candidate against a suspected source; search for a copy/generate step that writes it). If the file is generated, a spec that hand-edits it is a HIGH Issue — the deliverable must instead edit the canonical source and re-run the generator, and the parity/check guard (e.g. a `scripts/check-*.mjs` that asserts source↔generated equality) becomes an exit-criteria row. An exact-path existence check passes this silently because the generated file is real and present. (Rulepath has a recurring generated-asset pattern: `apps/web/public/rules/<game_id>.md` is copied from `games/<game_id>/docs/HOW-TO-PLAY.md` via `scripts/copy-player-rules.mjs`, guarded by `scripts/check-player-rules.mjs`.)

## 3.2 Types, Schema, and Contract Fields

Grep for each type, trait, struct, or schema/contract field. Confirm existence and current shape. Check:

- **Field existence and naming** — flag fields the spec assumes but that don't exist or have different names/types. The schema/contract-authoritative sources for Rulepath are `docs/ARCHITECTURE.md` and `docs/ENGINE-GAME-DATA-BOUNDARY.md` (action-tree, command-envelope, effect-envelope, diagnostic, public/private view, checkpoint, hash, serialization-boundary vocabulary) and `docs/OFFICIAL-GAME-CONTRACT.md` (per-game deliverable set). When the spec defines, serializes, or validates one of these structures, verify field names against the boundary/architecture docs and the actual crate code, not against FOUNDATIONS prose (prose describes *intent*; the boundary docs and code carry the *actual* contract surface, and the two can drift).
- **Type accuracy** — verify assumed types match actual types.
- **Field-choice drift** — when a contract offers multiple semantically-distinct fields and the spec's algorithm picks one, verify it picked the semantically-correct one (both fields exist, so a name-drift check passes silently); a wrong choice with correctness consequences is HIGH or CRITICAL.
- **Enum / table exhaustiveness** — if the spec includes a lookup keyed by a string enum (e.g. diagnostic code → handler, evidence category → check), verify it covers all current values.
- **Schema fidelity** — if the spec proposes a JSON/TOML schema or a static-data manifest entry, verify against `docs/ENGINE-GAME-DATA-BOUNDARY.md` and any existing schema/manifest. Confirm the schema rejects unknown fields and refuses behavior-looking fields (§5/§11).
- **Data-availability / projection** — when a deliverable *consumes* a value (a label, flag, or field rendered/read by a downstream surface), verify the value is present on the **specific surface the consumer reads** — the catalog entry, the WASM bridge JSON, the public/private view projection — not merely that it exists *somewhere* upstream (a `*.toml` manifest, a Rust struct). A value that lives in an upstream manifest but is never carried through the projection the consumer reads is unavailable to that consumer: the spec needs an added projection deliverable (Rust-side), or the consumer (TypeScript) would have to invent the value — a §2 behavior-authority risk (TS inventing presentation data). A field-existence grep passes silently because the value exists upstream; the gap is the missing projection hop. HIGH when a deliverable's only data source is the un-projected upstream.

## 3.3 Functions, Exports, and Commands

Grep for each function, trait method, CLI tool, or command; confirm signature, location, crate, and export status. Line-number references are informational — verify they point to the claimed content; if drifted, correct them or replace with grep-stable symbol names. Check:

- **Signature differences** from what the spec assumes; **parameter sufficiency** at every call site.
- **Crate placement** — verify the symbol lives in the crate the spec claims (`engine-core` vs `game-stdlib` vs `games/*` vs `ai-core` vs `wasm-api` vs `tools/*`). A symbol the spec places in `engine-core` that is actually (or must be) a game-specific type is a §3 boundary finding.
- **Reuse opportunities** — for each new function the spec proposes, grep for an existing one serving the same purpose; a duplicate is an Issue (prefer reuse) or Improvement (note the alternative).
- **Code-example fidelity** — Before/After snippets and Rust signatures must match the actual control-flow structure and types.
- **Pseudocode dependency completeness** — each call/constructor in spec pseudocode must either exist or be defined as a deliverable elsewhere in the spec; neither = an incomplete deliverable Issue.
- **Surface-convention fidelity** — proposed CLI tools, flags, or APIs should match existing Rulepath conventions (`tools/*` naming, the legal-action API surface, `docs/ARCHITECTURE.md`); flag deviations.

## 3.4 Dependencies (gates / specs / skills)

- **Spec / gate sequencing**: verify the spec's `authority order` list resolves to existing `docs/*` files, and that its Sequencing section's predecessor/successor gates resolve to the `specs/README.md` index. For a predecessor gate the spec claims is `Done`, verify the index status actually reads `Done`; a spec admitted before its predecessor gate passes exit criteria is a sequencing Issue (the index admission rule forbids it). For a sibling-gate reference that resolves to no spec file yet, that is expected for not-yet-specced gates — flag only if the spec treats an unwritten sibling as if it exists.
  - **Parenthetical scope-claim verification**: for a sequencing entry with a parenthetical scope claim (e.g. `Gate 1 (race_to_n)`), verify the parenthetical against the ROADMAP gate name and the index row. A misattribution propagates misleading provenance into decomposition — HIGH Issue; the file/row existence check passes silently.
  - **Named-list drift**: for a predecessor/successor gate, scan the sibling spec's named lists (deliverable tables, exit-criteria rows, evidence categories) for entries the target references. A target naming a deliverable/value the sibling's enumeration omits is a HIGH Issue (cross-spec contract drift).
- **Skill / template dependencies**: verify `.claude/skills/<name>/` directories and `templates/*.md` referenced in the spec exist. For a spec whose Work breakdown decomposes into `templates/AGENT-TASK.md` packets, verify that template exists and that the spec's claimed AGENT-TASK section set matches it.

## 3.5 Skill-Structure Validation

For deliverables that propose SKILL.md changes, applicability is gated on the SHAPE of the change. Content-only edits (rewording, prose updates) need no 3.5 validation — report N/A. Structural edits (frontmatter, HARD-GATE block, Step/Phase definitions, Output declarations) require verifying:

- Frontmatter declares `name`, `description`, `user-invocable`, `arguments`; the description names triggers, produces, and mutates.
- A Prerequisites / required-reads block is present.
- The Final Rule (or equivalent) is a single enforceable sentence.
- If the skill mutates files, a `<HARD-GATE>` block and a Write/Commit step are present.

Match the conventions of the repo's existing skills (`brainstorm`, `skill-audit`, this skill) rather than inventing new structure.

## 3.6 Downstream Consumers

For types, traits, functions, schemas, or contracts the spec modifies, grep all call sites and usage points across the crate tree (`crates/*`, `games/*`, `apps/*`, `tools/*`), `.claude/skills/*`, `templates/*`, and `docs/*`. Record blast radius.

For **new, retired, or changed** string-enum values (a new/retired diagnostic code, effect kind, visibility scope, or dispatch token), grep each affected value across all consumer sites — new values need a new arm at every dispatch site; retired values need every consumer updated (or retired alongside); changed values need both. Surface the consumer count explicitly. (A spec that ONLY retires values is the common case the literal "new enum" framing would steer past — the broadened scope closes that gap.)

For **schema/contract extensions** (action tree, command/effect envelope, public/private view, golden trace, checkpoint, serialized save, static-data manifest entry), confirm consumers of that schema are updated, or the extension is additive-only (new optional field with a default) — mirrors `tickets/README.md` pre-implementation check 10.

**Audit the spec's own completeness-sweep / gate command**: when a spec ships a self-verification `grep`/`find`/`test`/`cargo` "completeness sweep, re-run as a gate" (common in removal/rename specs), validate the gate's coverage against where consumers actually live. A gate that omits a consumer crate/directory or uses a pattern that misses a syntax variant is an under-scoped gate — the spec's own verification passes green while real consumers drift. HIGH Issue; recommend widening the gate's paths/pattern and adding the missed consumers to the Deliverables / Files-to-touch list.

## 3.7 Upstream Spec / Gate References

Grep specs in `specs/` and the `specs/README.md` index for references to this spec's deliverables or gate; note affected specs. Use matches to refresh the Sequencing section and any "gate X has not landed yet" claims with accurate status.

**Forward-compat with successor gates**: for specs that define schemas/contracts/tools AND whose Sequencing names successor gates, read each successor gate's spec (when written) for extensions to the current spec's surfaces (new conditionally-required fields, new diagnostic codes, new enum values). If a successor gate proposes additions the current spec's design would silently reject (strict shape validation, closed enums, no unknown-field tolerance beyond the §11 reject-unknown rule), flag a forward-compat Improvement at MEDIUM. Skip when there are no successor gates specced yet or they don't extend the current spec's surfaces.

## 3.8 FOUNDATIONS-Contract Fidelity

For deliverables that touch FOUNDATIONS-governed semantics — behavior authority (§2), the `engine-core` kernel boundary (§3), `game-stdlib` promotion (§4), static-data discipline (§5), the universal acceptance invariants (§11):

- **No principle weakening**: read the relevant FOUNDATIONS sections. For each principle the deliverable touches, verify the proposal enforces it at least as strictly as the constitution requires. A proposal that weakens a principle is a CRITICAL Issue.
- **Behavior-authority preservation**: verify no deliverable moves setup, legal-action generation, validation, state transitions, scoring, RNG, semantic effects, view projection, replay/hash, serialization, or bot decisions out of Rust, and that TypeScript stays presentation-only (§2). Letting TS decide legality is CRITICAL.
- **Kernel-boundary preservation**: verify no deliverable introduces a mechanic/domain noun into `engine-core` (§3). A game-specific type in the kernel is a boundary-failure CRITICAL Issue.
- **Visibility / no-leak preservation**: for deliverables affecting public/private view projection, previews, effect logs, bot explanations, candidate rankings, UI test IDs, or replay exports, verify no path lets hidden information reach a viewer the deterministic views forbid (§11 no-leak firewall, §12). Missing firewall is CRITICAL.
- **Determinism preservation**: for deliverables touching replay, hash, serialization order, RNG, or traces, verify identical inputs+versions produce identical output and no nondeterministic input enters canonical forms (§2, §11). Violations are HIGH; a change to replay/hash *semantics* additionally trips the §13 ADR trigger.
- **Validation discipline**: for deliverables proposing validation, verify it stays deterministic, fail-closed, and blocking, distinguishes warnings from blockers, rejects unknown fields by default, blocks behavior-looking fields, and names what failing means (§5, §11). Unaddressed second-order effects are Improvement findings at minimum.

## 3.9 New-Deliverable Consumer Verification

For each proposed new deliverable (new crate, new CLI tool, new validator, new public type, new contract field, new reference-file section), verify at least one identifiable consumer exists or is explicitly planned. Grep for references to it by name across `crates/*`, `games/*`, `apps/*`, `tools/*`, `.claude/skills/*`, `specs/*`, `tickets/*`, and `docs/*`, and inspect the spec's own Objective/Scope/Work-breakdown for a concrete consumer-side workflow.

**Outcome**:
- **≥1 consumer found**: deliverable justified — record the consumers in Step 6 for audit-trail visibility.
- **Zero consumers AND no pending consumer named**: HIGH Issue → present at Step 6 as a Question with three options: (a) drop per YAGNI; (b) keep with explicit rationale naming a near-term consumer; (c) defer to a separate consumer-driven spec/gate. Defer the decision to the user — do not silently drop at Step 3. (Greenfield/foundation specs are **not** zero-consumer cases — their consumers are the sequenced future gates; see the Greenfield note above the substep sections.)

**Structurally-wired deliverables** (e.g. a tool registered in a workspace `Cargo.toml` members list, a bot registered in a game's bot registry) have a structural consumer model — registration *is* the wiring. Confirm the registration site rather than name-grepping for callers; flagging "zero consumers" there is a false positive.

## 3.10 Source-Document Completeness Check

For specs citing an external source document (a `docs/ROADMAP.md` gate, a brainstorm output under `docs/plans/`, a report) in their Objective / Scope / Work breakdown, the claims were enumerated and tagged at Step 2. Here:

1. **Verify** each enumerated claim is adjudicated by the spec — **Accepted** in Scope/Deliverables/Work-breakdown with a per-claim mapping (this covers accept-with-divergence: the spec adopts the intent but deliberately diverges; record the divergence in the mapping), **Rejected** in Out-of-scope/Not-allowed with a rationale, or **Deferred** with a named follow-up gate.
2. **Surface unadjudicated claims** as MEDIUM Improvement findings — name the claim, cite the source line, recommend the spec add an adjudication. For ROADMAP gates specifically, every "Exit" and "Not allowed" line of the gate must be mapped: an unmapped ROADMAP exit criterion is a HIGH Issue (the spec's Exit-criteria section must mirror it row-for-row, per `specs/README.md`), and an unmapped "Not allowed" line is a HIGH Issue (it must carry into the spec's Not-allowed / Forbidden-changes sections).
3. **Surface source-internal inconsistency** — when the source contradicts itself AND the spec follows one part, surface as a MEDIUM Improvement (LOW if the spec already handles it) and recommend the spec record which variant it follows and why, so decomposition doesn't re-adopt the rejected variant.

For classification (d), apply a stronger variant: verify each "accepted" claim actually landed in the codebase. An accepted-but-unredeemed claim with no delivering citation is a HIGH Issue (silent completion-claim risk).

**Skip** when: (c) refactor classification, or no external source document is cited (self-originating specs are scoped by 3.0–3.9 alone). A precedent sibling-spec the spec only *mirrors* is not a source document — validate it under 3.4/3.7 — and standards/UX citation footnotes are provenance, not adjudicable claims; when only these appear, 3.10 is N/A.

## 3.11 Spec Structural Completeness Check

For specs introducing new work (classes (a) and (b)), verify the spec carries the sections AGENT-TASK decomposition needs, per the canonical spec format in `specs/README.md`:

- **§Deliverables and §Work breakdown** — each deliverable names a concrete target (file/crate path, function signature, tool name) the implementer can grep against, and each Work-breakdown item is a bounded candidate AGENT-TASK with dependency order. A Scope that reads as feature description without named targets is incomplete on this axis.
- **§Exit criteria + §Acceptance evidence** — exit criteria mapped row-for-row to the ROADMAP gate's exit list, and acceptance evidence naming re-runnable confirmation (tests, traces, replay/hash checks, benchmarks, simulations, boundary reviews) per `docs/TESTING-REPLAY-BENCHMARKING.md`. Use explicit `not applicable` rows over silent omissions. For a game gate, the evidence set must cover the `docs/OFFICIAL-GAME-CONTRACT.md` deliverables (rule coverage, golden traces, replay, visibility/no-leak, bot legality, benchmarks). A game spec missing these is a HIGH Issue.
- **§Forbidden changes + §Assumptions** — gate-specific prohibitions (carrying the ROADMAP "Not allowed" list) and one-line-correctable assumptions surfaced explicitly rather than discovered at task-time.

**Severity**: a code/gate spec missing §Deliverables, §Work-breakdown, §Exit-criteria, or §Acceptance-evidence entirely is a HIGH Issue (decomposition can't proceed). Missing §Forbidden-changes when the gate's ROADMAP entry has a "Not allowed" list is a HIGH Issue (the prohibition must carry). Missing §Assumptions is a MEDIUM Improvement — downgrade to LOW when the spec surfaces its open questions under another section. A docs-only or process spec missing these is LOW or N/A depending on whether its content depends on implementer follow-up.

**Skip** for (c) refactor and (d) retroactive (the latter's structural completeness lives in its Acceptance-evidence / Outcome content).

## Conditional Deliverable Validation

For specs with conditional deliverables ("If root cause X is confirmed, do Y"), validate: (1) **diagnostic sufficiency** — the investigation can distinguish the hypotheses; (2) **fix correctness** — each proposed fix references correct types/functions/crate paths regardless of which is selected; (3) **soundness** — each fix respects FOUNDATIONS even though conditional (a conditional violation is still a spec defect).

**Staleness-gated deliverables — record the resolved set**: a distinct conditional shape is the deliverable gated on a staleness/feature premise ("touch file F **only if** its status summary is stale", "update the manifest label **only if** confirmed semantically stale"). Common in doc-realignment / maintenance specs that list many "likely affected files." Step 3 resolves each gate to *touch* or *don't-touch* — and when the spec is substantially accurate, most resolve to **don't-touch** (the file is already correct). Do not let that resolution evaporate: the don't-touch set is load-bearing output, because without it the implementer re-investigates every conditional from scratch. Record the resolved set explicitly at Step 7 — an Out-of-scope "validated already-correct, do not re-edit" block, a "validated current state" note, or resolved §Assumptions rows (one per gate, citing the evidence) — so the resolution survives into decomposition. A gate that resolves to don't-touch is **not** a dropped deliverable; it is a confirmed no-op worth one Assumptions row. Surface this as an Addition (record-the-resolved-set) at Step 6 when the spec leaves it implicit.
