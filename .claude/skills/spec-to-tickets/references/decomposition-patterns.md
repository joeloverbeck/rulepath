# Decomposition Patterns (Step 3)

Analyze the spec and identify discrete work units:

- Each ticket is a **reviewable diff** — small enough for comfortable manual review.
- Map **dependencies** between tickets and name them in each ticket's `Deps`.
- Determine **priority ordering** from the dependency graph and criticality.
- Ensure **every spec deliverable is covered** — no silent skipping. If a deliverable seems wrong, flag it with the 1-problem / 3-options / 1-recommendation format rather than omitting it.
- Use the spec's **Out of Scope** / **Forbidden changes** sections to populate each ticket's Out of Scope — these are pre-validated non-goals.

## Deliverable-coverage categories

Most deliverables get their own ticket. These categories are exempt from per-deliverable ticket production — document them in the Step 4 summary and Step 6 coverage mapping instead:

- **No-change deliverable** — the spec explicitly states no change is needed ("No new crate"). Note it if non-obvious; produce no ticket.
- **Cross-spec consumer-side deliverable** — the work is consumed by a sibling gate's tickets rather than producing a discrete output here. The change is needed, but it happens in another spec's decomposition. Note the ownership as `cross-spec: <sibling-gate>` in Step 4 and `not ticketed (cross-spec: …)` in Step 6.
- **Distributed deliverable** — one conceptual deliverable encoded as a contract distributed across multiple tickets (e.g. a per-game applicability rule carried as a predicate on each affected module, with no single owner). Note it as `distributed across <ticket-list>`; produce no separate "container" ticket.
- **Informational-pointer deliverable** — a section whose body only points to sibling deliverables ("handled in §X above and §Y below"). The actual work is in the pointed-to siblings, each ticketed. Note the routing; produce no ticket.
- **Already-landed deliverable** — the intent was satisfied by an edit landed during a prior in-session skill invocation (e.g. `/reassess-spec`'s user-authorized in-session edit). Produce no ticket; record `not ticketed (already landed during <skill> session <date>)` in Step 6.
- **Inferred deliverable** — named or implied by §Objective / §Acceptance evidence / §Exit criteria but absent from §Deliverables. This DOES get a ticket; document the inference in the ticket's Assumption Reassessment with a citation to the motivating section, and flag it in Step 4 Notes as `inferred from <section>`. When the motivating reference is ambiguous or would expand scope, route through a Step 2 Issue instead.

## Sizing rules

- **Merge** — when multiple deliverables share the same file set and cannot be implemented independently, merge into one ticket. Note merged deliverables in Step 4 Notes.
- **Split** — when a single deliverable is too large for one reviewable diff (it spans, say, a contract surface + a schema + a validation rule + a bot emitter), split by logical sub-concern (e.g. write path vs. read/verify path). Note the split in Step 4 and show `D<n>→00x+00y` in Step 6.
- **Candidate-AGENT-TASK granularity is a floor, not a 1:1 mapping** — when the spec's Work-breakdown table labels each item as a candidate AGENT-TASK (e.g. a `Becomes AGENT-TASK: yes` column), treat that as the author's intended *granularity floor* — the coarsest acceptable grouping — not a binding one-ticket-per-WB-item rule. Honor it as the default, but apply the Split rule when a labeled item exceeds one reviewable diff (the highest-risk surface — `engine-core` kernel growth, a full `games/*` crate — usually does). Surface every divergence from the spec's item count in the Step 4 table with `D<n>→00x+00y`, and offer the coarser-grained alternative ticket count so the user can opt back down to the spec's granularity.
- **Same file / same new directory** — when all deliverables modify the same file or land under one new directory (a new crate, a new `games/*` dir), decompose by logical section/module, not by file boundary. Each ticket targeting a different section is a valid reviewable diff. Note the shared file once in Step 4.
- **Intra-batch create-then-modify chains** — when ticket B's Files to Touch contains a `(modify)` entry for a file ticket A creates `(new)` in the same batch, declare `Deps: A` on B explicitly. The file-creation is a load-bearing structural dependency (B fails at implementation time if A hasn't landed). This is distinct from §Shared-file overlaps (Step 6 item 5), which covers parallel adds by mutually-independent tickets to a shared file — whether pre-existing or created in-batch by a common foundation ticket they each `Deps:` on.
- **Greenfield foundation batch** — when the spec is the repo's first code (no pre-existing code tree, e.g. the Gate 0 repository-skeleton spec), every ticket's Files to Touch is legitimately all-`(new)`; do not treat that as a suspicious under-statement. The structural ordering is carried by intra-batch create-then-modify chains (above), and the deliverables' consumers are the sequenced later gates, not present code — mirroring the greenfield note in `reassess-spec`'s `references/codebase-validation.md`.

## Ticket shapes

### Capstone integration ticket

A single trailing ticket whose scope IS the spec's §Exit criteria / §Acceptance evidence, exercising every prior implementation ticket end-to-end. It introduces no new production logic; it exercises the pipeline the earlier tickets composed.

- Its acceptance criteria enumerate the spec's exit-criteria / acceptance-evidence bullets as test sub-cases.
- Re-enumerate expected counts from a fixture at test start rather than hardcoding (hardcoded counts go stale).
- **Files to Touch** — `_TEMPLATE.md` makes the section mandatory, but a capstone introduces no production logic, so it is either the new e2e/smoke/golden-trace test the ticket adds, or `None — verification-only` when the gate is exercised purely by running existing scripts (simulation CLI, replay-check) plus a manual runbook. Do NOT list the upstream tickets' files — the capstone exercises them, it does not modify them.
- **Manual-runbook variant** — when an exit-criterion cannot run in the project's test infra — either it requires invoking an LLM-driven skill (not runnable from test code), **or** it is a UI/manual smoke with no browser-automation harness in the project (the WASM web shell) — structure the ticket as a hybrid: a *manual runbook* (in the What to Change section) the implementer follows by hand — for an LLM skill: fixture copy strategy, skill invocations against the copy, expected post-skill state with verification commands; for a UI smoke: the click-path with each step's expected observable result — plus an automated-test subsection for any test-runnable portion (golden-trace assertions, replay/hash checks, no-leak negative tests, bot-legality checks). The acceptance criteria distinguish CI-runnable assertions from implementer-checklist runbook steps.
- **`Deps`** — prefer a single `Deps: <transitive-head-ticket>` when the upstream DAG is a linear chain (the head's own `Deps` reach every prior ticket). When the upstream DAG has parallel branches (e.g. a docs ticket running parallel to implementation tickets), enumerate the **leaf set** — every leaf whose transitive `Deps` collectively cover the full chain.

### Cross-cutting docs ticket

A single trailing ticket whose Files to Touch are markdown docs (READMEs, architectural docs, skill reference files, the `specs/README.md` index status flip) that must land atomically once all upstream implementation tickets ship.

- One Files-to-Touch entry per docs surface the deliverable names.
- Acceptance criteria as grep-proofs against the post-implementation tree (exact-string matches for new/renamed symbols, count assertions for status-line updates).
- Out of Scope names the implementation surfaces this ticket does NOT touch, so reviewers know docs-only is the contract.
- **`Deps`** — list every implementation ticket whose surface the docs reference (docs reference each surface independently, so transitive-head is insufficient here). **Exception** — when the docs ticket only annotates *aggregate completion* (a `specs/README.md` status flip / "verified by Gate N" note) rather than citing individual symbols, and a capstone ticket already gates every referenced surface, `Deps: <capstone>` is sufficient and more legible than re-enumerating the surface set. Enumerate per-surface only when the docs cite individual symbols/APIs that could go stale independently.
- **When NOT to use** — if each docs change can co-locate with its implementing ticket without a staleness window, per-ticket docs are cleaner. Use this shape when ≥1 docs surface requires all upstream surfaces to exist coherently before it can land (status-line counts, cross-tool tables, multi-surface skill guidance).
