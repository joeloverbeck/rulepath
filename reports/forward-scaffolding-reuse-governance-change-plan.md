# Rulepath Forward Scaffolding-Reuse Governance Change Plan

**Advisory deliverable:** intermediate change-plan document. It does not replace repository law, and it is not a finished implementation spec or a ticket-ready decomposition.  
**Target repository:** `joeloverbeck/rulepath`  
**Target commit:** `5ed1664de53eed9d51615786344905e3c05619d4` (`5ed1664`)  
**Freshness claim:** user-supplied target commit only; this plan does **not** independently verify that the commit is the current `main`.  
**Manifest role:** user-supplied path inventory only.  
**Repository-evidence rule:** repository-state claims below use only manifest-listed files fetched through verified exact-commit URLs.  
**External-evidence rule:** external sources are used only to pressure-test the governance design; they do not establish Rulepath repository state.  
**Historical-baseline note:** historical evidence files name earlier baselines for their own work: the prior plan names `db0c50b95f84df12b349710033c77db2bf7326b3`; the archived 8M spec also names `db0c50b`; the R2, R3, and R4 characterizations name `51a5c12636696d974b9491cc49bcff5590fca64b`, `b0be7a4157f8`, and `9c5b4c8730fc917af88aefdfae7e641c258e94d5`. Those strings are historical file content, not acquisition targets; this plan uses `5ed1664de53eed9d51615786344905e3c05619d4` for repository-state claims throughout.  
**Recommended governance unit:** `8F — Pre-Gate-18 forward scaffolding-reuse governance`, proposed spec slug `pre-gate-18-forward-scaffolding-reuse-governance.md`.  
**Recommended ADR mechanism:** append-only dated extension of accepted ADR 0008, with a short successor ADR only if the maintainers enforce immutable accepted ADR text.

---

## 1. Executive summary

The maintainer's three concerns are sound:

1. every new official game must perform a **reuse-first audit** before it reimplements behavior-free plumbing;
2. every new behavior-free scaffolding shape invented by a game must be **registered**, even when it remains local on first use; and
3. when a new game's scaffolding makes already-shipped games matching duplicates, a bounded **follow-on refactoring unit must be queued automatically**, rather than left as an optional cleanup note.

This is a sharp forward-governance delta, not a cold-start reuse proposal. Rulepath already has the important retroactive machinery:

| Implemented baseline at `5ed1664` | Current role | This plan's stance |
|---|---|---|
| Accepted ADR 0008 | Defines the mechanical-scaffolding lane, allowed homes, exclusions, second-use review, and pre-third-copy hard decision. | Keep it. Extend it with a standing per-new-game lifecycle; do not re-propose the lane. |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | Defines the entry schema, decision states, Non-Promotion List, and MSC-8C-001…010 with R1–R4 receipts. | Keep the register and historical receipts. Add a forward maintenance cadence and automatic prior-game-refactor rule. |
| `docs/MECHANIC-ATLAS.md` §4 | Holds the behavioral third-use hard gate. | Preserve its wording and force. The scaffolding lane remains parallel and narrower. |
| `templates/GAME-IMPLEMENTATION-ADMISSION.md` | Has a conditional pre-code “mechanical-scaffolding decision, if needed” row. | Replace conditional treatment with a mandatory reuse-first audit. |
| `templates/GAME-EVIDENCE.md` | Has a pre-implementation register-decision receipt row. | Add post-implementation register-freshness and prior-game-refactor receipts. |
| Unit 8C and 8C-R1…R4 | Extracted accepted helpers and retrofitted the 17-game corpus in bounded waves. | Treat them as the shipped precedent. Generalize their one-off wave-seeding pattern into standing law. |
| `game-test-support`, `engine-core`, `game-stdlib`, and `wasm-api` shared seams | Provide the lawful homes and already-promoted scaffolding that a new game should reuse. | Make auditing these homes a normal admission obligation. |

The current gap is not that Rulepath lacks a scaffolding doctrine. The gap is that a new game can still move through the official workflow without a mandatory, mechanically checked sequence that says:

```text
reuse-first audit
  -> adopt existing lawful scaffolding or record an accepted exception
  -> implement only the remaining game-local behavior and necessary local scaffolding
  -> register every newly invented behavior-free shape
  -> identify prior official games that now match
  -> queue a named retrofit unit or record an accepted no-retrofit disposition
  -> close the game evidence receipt and CI audit record
```

This plan closes that gap through six coordinated moves:

1. add the obligation to `FOUNDATIONS.md` invariants and stop conditions;
2. define forward conformance in the architecture and engine/game/data boundary docs;
3. wire the lifecycle into the official-game workflow, atlas/register seam, agent law, roadmap, and spec tracker;
4. make the admission, mechanics, evidence, and task templates execute the obligation;
5. add a Gate 1 mechanical check that validates per-game audit receipts, known promoted-shape fingerprints, register decisions, accepted exceptions, and queued retrofit units; and
6. land all of the above in a dedicated blocking pre-Gate-18 governance unit, making Spades the first game admitted under the standing rule.

The recommendation does **not** change game behavior, promote a new helper, add a game, alter any trace or fixture bytes, add YAML or a DSL, put mechanic nouns in `engine-core`, or weaken determinism, visibility, no-leak, or the behavioral third-use gate.

---

## 2. Method and evidence base

### 2.1 Provenance and evidence lanes

Acquisition used the uploaded manifest only as an exact path inventory. Every repository URL was constructed from this base:

```text
https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/
```

No clone, branch-name fetch, default-branch lookup, repository metadata lookup, GitHub code search, target-repository snippet search, or repository-scoped connector argument was used. Seventy-three repository files were requested and seventy-three were successfully verified. The complete append-only URL ledger appears in Appendix A.

The analysis kept three evidence lanes separate:

- **Target-repository evidence:** exact-commit contents of manifest-listed files.
- **User-supplied control material:** the manifest and research brief, used for inventory, scope, settled intentions, and authority—not as substitutes for fetched repository contents.
- **External research:** official or primary sources used only to sharpen the process design.

References to other repositories inside validly fetched files were treated as ordinary file content and never as fetch-provenance contamination.

### 2.2 Repository material used

The authority spine, workflow docs, ADRs, live spec tracker, and requested templates were read in authority order. The prior plan, the shipped 8M spec, Unit 8C, the R1–R4 characterization material, and the representative R4 spec/report were used as historical implementation evidence. The existing CI scripts and Gate 1 workflow were inspected for integration style. The shared scaffolding homes and Vow Tide exemplar were inspected to ground the audit and lint signals in actual promoted surfaces.

The most important current-state observations are:

- `OFFICIAL-GAME-CONTRACT.md` §3 includes a one-time behavioral primitive-pressure comparison but no standing scaffolding reuse-first step or closeout step.
- ADR 0008's decision rule starts at second exact duplication and hard-gates a third copy, but it does not explicitly require every new game to produce an audit receipt or register a novel first-use scaffolding shape.
- the register is historically rich but operationally reactive: Unit 8C entries and R1–R4 receipts land when owning tickets prove evidence, and “Next review trigger” fields do not themselves create tracker work;
- `AGENT-DISCIPLINE.md` §8A governs tasks already labeled `scaffold-refactor`; it does not make the audit mandatory for ordinary new-game implementation tasks;
- `GAME-MECHANICS.md` routes behavioral pressure to the atlas and primitive-pressure ledger but omits the mechanical-scaffolding register from its required repo-update section;
- `GAME-EVIDENCE.md` has a register-decision row but no post-build “new scaffolding / prior-game retrofit” closeout receipt;
- Gate 1 already has an appropriate one-time `repo-checks` lane, but no scaffolding-governance check; and
- the active tracker has completed 8M, 8C, and 8C-R1…R4, then proceeds directly to unwritten Gate 18.

### 2.3 External calibration

The external material supports restraint, not a wider architecture program:

- the Rule of Three and “use before reuse” are useful evidence thresholds, but they do not justify ignoring a known supported helper before the third copy;[^ext-rule-three]
- test code sometimes benefits from deliberate local repetition for clarity, so the new gate must allow `local-only` and accepted exceptions rather than treating all textual similarity as a defect;[^ext-damp]
- an effective golden path is explicit, supported, and updated when the real path changes, which supports wiring the obligation into the official workflow rather than leaving it in a specialist register;[^ext-golden-path]
- architecture decisions should retain their motivating context and consequences, which supports an append-only dated ADR 0008 extension rather than rewriting its historical decision as though it had always contained this lifecycle.[^ext-adr]

These sources do not change Rulepath's accepted lane, thresholds, homes, or exclusions.

---

## 3. The exact forward-governance gap

### 3.1 Gap 1 — reuse-first audit

**Current state:** the official workflow requires a behavioral primitive-pressure comparison. Admission asks for a scaffolding decision only “if needed.” A task becomes subject to §8A only after somebody has already classified it as a scaffold refactor.

**Failure mode:** a new game can locally rebuild an effect-envelope wrapper, seat grammar, action-tree encoder, stable-byte frame, no-leak matrix, or evidence-profile driver without first proving why the registered/promoted surface does not fit.

**Required close:** every new game must complete an audit before serious implementation. The audit is mandatory even when the result is “no relevant scaffolding surface.” A not-applicable result requires a rationale; silence is not a result.

### 3.2 Gap 2 — register-new

**Current state:** ADR 0008 guarantees review at second exact duplication and before a third copy. The register records Unit 8C candidates and retrofit evidence. `GAME-MECHANICS.md` does not require a register update.

**Failure mode:** a useful first-use behavior-free shape can remain invisible until another game independently recreates it. At that point reviewers must reconstruct provenance and intent from code rather than starting from a recorded boundary and next-review trigger.

**Required close:** when a new official game invents a behavior-free scaffolding shape, it adds a lightweight register entry before game closeout. First use does **not** authorize promotion. The normal first-use state is `candidate`, `local-only`, or `rejected`, with the second-use or other named next-review trigger.

### 3.3 Gap 3 — auto-schedule-refactor

**Current state:** Unit 8C explicitly seeded four bounded C-11 waves; the R1–R4 specs executed them; the register records per-row next-review triggers. That scheduling rule exists in historical one-off spec text, not as a standing lifecycle.

**Failure mode:** a new game can expose matching prior-game copies, record “review later,” and still close without a tracker unit. The trigger has no owner, order, or admission consequence.

**Required close:** the same closeout that records the new game's scaffolding decision must either:

- add a named bounded follow-on unit to `specs/README.md`; or
- record an accepted `local-only`, `deferred`, or `rejected` disposition with rationale, owner, evidence, and next-review trigger.

“Automatically” means the tracker row is a mandatory artifact of the game closeout—not that a script fabricates a refactoring spec. The script verifies the row or accepted no-unit disposition.

### 3.4 Relationship to existing second-use and third-copy law

The forward rule strengthens observability without changing the accepted thresholds:

| Situation | Required forward action | Advancement effect |
|---|---|---|
| No relevant scaffolding touched | Record `no-new-scaffolding` with rationale and evidence link. | No refactor unit. |
| Existing promoted/registered helper fits | Reuse it and link the register entry and call sites. | No new entry unless the entry's migration/trigger data changes. |
| Existing helper does not fit | Record an accepted exception or local-only rationale before implementing a parallel shape. | Continue only if the rationale clears the register boundary. |
| New first-use behavior-free shape | Add a `candidate`, `local-only`, or `rejected` entry with exclusions and next-review trigger. | No extraction required merely because it is first use. |
| New game creates a second exact semantic copy | Make the ADR 0008 decision and queue a bounded refactor unit unless accepted local-only/deferred/rejected. | Current game may close only after the decision and tracker/no-unit receipt exist. |
| A third copy would be introduced | Resolve reuse/promotion/defer/reject/ADR before the copy is admitted. | Existing ADR 0008 hard decision remains blocking. |
| A helper is promoted with prior matching sites | Name the full migration set and queue closure work or accepted exceptions. | `promotion-debt-open` blocks later ladder advancement under existing law. |

---

## 4. Target standing lifecycle

### 4.1 Pre-implementation: audit

Every new-game spec and admission packet must identify the behavior-free infrastructure it expects to touch and compare that surface against:

1. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`;
2. lawful `engine-core` contract ergonomics;
3. registered `game-stdlib` scaffolding;
4. dev-only `game-test-support` harnesses; and
5. thin `wasm-api` adapters.

The audit outcome is one or more of:

- reuse an existing helper;
- not applicable, with rationale;
- use an accepted exception;
- retain a local shape with rationale;
- register a new first-use candidate; or
- stop and reroute behavior to the mechanic atlas or an ADR.

### 4.2 During implementation: reuse and preserve boundaries

New-game tasks must use the selected shared surface directly unless a documented adapter is needed to preserve a game-local type boundary. Adapters may translate types; they may not duplicate generic encoding, identifier grammar, visibility policy, stable-byte framing, or evidence geometry.

The implementation must keep game behavior local where the register's Non-Promotion List says it belongs. Deal/reveal/projection policy, betting/pot policy, trick lifecycle, teams, graph semantics, accounting, reaction windows, scoring, terminal outcome, legality, and hidden-state policy remain behavioral even when code looks similar.

### 4.3 Post-implementation: register freshness

Before official-game closeout:

- every new behavior-free shape has a register entry;
- every reused helper has a receipt link;
- every intentionally local parallel shape has an accepted rationale;
- affected hashes, visibility, determinism, and migration authority are stated; and
- the game-level evidence receipt and machine audit record agree.

### 4.4 Prior-game impact: queue or dispose

If the new implementation or a newly promoted helper identifies matching scaffolding in earlier official games, the closeout must name the migration set. A follow-on unit is required when migration or deeper characterization remains real work.

A follow-on unit is not required when the register proves one of these dispositions:

- the earlier site is not semantically identical;
- the shape is behavior-bearing and is rerouted to the mechanic atlas;
- extraction would reduce clarity or raise replay/visibility risk without enough benefit;
- the site is explicitly accepted `local-only`;
- the decision is `deferred` to a named trigger; or
- the candidate is `rejected`.

Every no-unit disposition must carry an owner and next-review trigger. A bare “later” or “not worth it” is not an accepted disposition.

### 4.5 Mechanical enforcement

The CI gate enforces what is tractable:

- every official game has an audit record;
- new games cannot use the historical 8C coverage exemption;
- required fields and evidence links are present;
- known promoted-shape fingerprints are either absent, routed through the shared helper, or covered by a register decision/accepted exception;
- referenced register entry IDs exist;
- required follow-on unit IDs exist in `specs/README.md`; and
- hash/visibility changes cite ADR 0009 migration authority.

It does **not** claim to decide arbitrary semantic equivalence. Novel-shape classification remains a reviewed engineering decision, made visible and mandatory by the receipt gate.

---

## 5. Authority-ordered per-file amendment set

### 5.1 Amendment disposition matrix

| File | Disposition | Why |
|---|---|---|
| `docs/README.md` | **Not applicable** | The register is already indexed in the correct authority position. No authority-order change is required. |
| `docs/FOUNDATIONS.md` | **Amend** | The standing audit, register-new duty, and queue-or-dispose closeout must become universal invariants and stop conditions. |
| `docs/ARCHITECTURE.md` | **Amend** | The reuse ownership matrix needs forward conformance, and the existing `game-test-support` owner should no longer be described as future. |
| `docs/ENGINE-GAME-DATA-BOUNDARY.md` | **Amend** | The boundary doc must define the forward scaffolding conformance process parallel to behavioral §13. |
| `docs/OFFICIAL-GAME-CONTRACT.md` | **Amend** | This is the primary workflow insertion point and official-game acceptance gate. |
| `docs/MECHANIC-ATLAS.md` | **Amend narrowly** | Add a parallel scaffolding check without changing §4 or §5A behavioral text. |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | **Amend** | Add first-use registration, standing per-game cadence, and automatic prior-game refactor scheduling. |
| `docs/AGENT-DISCIPLINE.md` | **Amend** | Make the audit and closeout mandatory for ordinary new-game tasks, not only pre-labeled scaffold-refactor work. |
| `docs/TESTING-REPLAY-BENCHMARKING.md` | **Amend** | Define the new CI check, its limits, and Gate 1 placement. |
| `docs/ROADMAP.md` | **Amend** | Add a blocking pre-Gate-18 governance interlock and Gate 18 admission requirement. |
| `docs/adr/0008-mechanical-scaffolding-governance.md` | **Amend append-only** | Extend the governing decision with the forward per-new-game lifecycle and newly affected sections. |
| `docs/adr/0009-replay-fixture-hash-taxonomy.md` | **Not applicable** | No byte, hash, fixture, export, RNG, or visibility migration is authorized. The new law only defers such changes to ADR 0009. |
| `docs/adr/ADR-TEMPLATE.md` | **Not applicable** | The existing template is adequate. This plan recommends extending ADR 0008 rather than authoring a new ADR by default. |
| `specs/README.md` | **Amend** | Add Unit 8F, block Gate 18 on it, and make the workflow run the audit before new-game spec authoring. |
| `templates/README.md` | **Amend** | Add the register to the authority/lifecycle guidance and clarify the two-stage audit receipt. |
| `templates/GAME-IMPLEMENTATION-ADMISSION.md` | **Amend** | Replace “if needed” with mandatory reuse-first and planned closeout rows. |
| `templates/GAME-MECHANICS.md` | **Amend** | Add the omitted register audit and required register-update target. |
| `templates/GAME-EVIDENCE.md` | **Amend** | Add post-implementation register freshness and prior-game refactor disposition. |
| `templates/AGENT-TASK.md` | **Amend** | Carry the audit/track obligation into every bounded new-game task. |
| `templates/PRIMITIVE-PRESSURE-LEDGER.md` | **Not applicable** | Its behavioral-scope-only redirect is already correct. Adding forward scaffolding fields here would blur the lane boundary. |

The following historical files remain evidence only and should not be edited by this governance unit: the prior change plan and brief, the archived 8M/8C/8C-R4 specs, and the 8C/R1–R4 characterization reports.

### 5.2 `docs/FOUNDATIONS.md`

**Location:** §11 “Universal acceptance invariants,” immediately after the current mechanical-scaffolding invariant; and §12 “Stop conditions,” after the existing promotion-debt stop conditions.

**Draft §11 text:**

```markdown
- Every new official game completes a mechanical-scaffolding reuse-first audit
  before serious implementation. The audit reviews the mechanical-scaffolding
  register and the lawful shared homes, reuses matching promoted scaffolding or
  records an accepted exception, and identifies any new behavior-free
  scaffolding the game will introduce.
- Every new behavior-free scaffolding shape introduced by an official game is
  recorded in the mechanical-scaffolding register with behavior exclusions,
  affected hash/visibility/determinism surfaces, a decision state, and a next
  review trigger. First use does not authorize promotion.
- When a new game's scaffolding makes an earlier official game a matching
  duplicate, the new-game closeout either queues a named bounded follow-on
  refactoring unit or records an accepted `local-only`, `deferred`, or `rejected`
  disposition with rationale and next review trigger. Existing pre-third-copy
  and promotion-debt blocking rules remain in force.
```

**Draft §12 text:**

```markdown
- a new official game starts serious implementation without a completed
  mechanical-scaffolding reuse-first audit;
- a new official game closes while a newly introduced behavior-free scaffolding
  shape is absent from the mechanical-scaffolding register;
- a new official game identifies matching prior-game scaffolding but leaves the
  retrofit as an unnamed TODO instead of a tracker unit or an accepted
  no-refactor disposition;
- a known promoted scaffolding helper is reimplemented locally without a
  register-backed exception;
```

**No change:** §4's behavioral first/second/third-use wording and §13's ADR triggers remain intact. This amendment strengthens an already-accepted lane; it does not authorize a new one.

### 5.3 `docs/ARCHITECTURE.md`

**Location 1:** §3A Reuse Ownership Matrix.

Change the dev-only owner label from “future `game-test-support` crate” to ``game-test-support``. The crate exists at the target commit; this is a truthfulness correction, not a new dependency permission.

**Location 2:** add the following section after §3A and before §3.1.

```markdown
### 3B. Forward mechanical-scaffolding conformance

Every new official game MUST perform a mechanical-scaffolding reuse-first audit
before serious implementation. The audit compares the game's planned
behavior-free infrastructure against the mechanical-scaffolding register and the
lawful shared homes in §3A.

The forward conformance sequence is:

1. reuse an existing registered/promoted helper when its accepted boundary fits;
2. record a register-backed exception before introducing a parallel local shape;
3. register every newly invented behavior-free scaffolding shape, including a
   first-use shape that remains local;
4. name earlier official games whose local code now matches the new shape; and
5. queue a bounded follow-on refactoring unit for those earlier sites, or record
   an accepted `local-only`, `deferred`, or `rejected` disposition with evidence
   and a next review trigger.

A queued unit is conformance work, not permission to broaden the helper. It MUST
preserve behavior by default and MUST follow ADR 0009 for any byte, hash,
fixture, RNG, export, or visibility migration.

This section does not govern behavioral mechanics. `ARCHITECTURE.md` §3.1 and
`MECHANIC-ATLAS.md` continue to govern promoted behavioral helpers, including
the unchanged third-use hard gate.
```

**Acceptance-check addition:** append this bullet to §14:

```markdown
- every new official game has a closed mechanical-scaffolding audit receipt,
  current register disposition, and a named prior-game retrofit unit or accepted
  no-refactor disposition;
```

### 5.4 `docs/ENGINE-GAME-DATA-BOUNDARY.md`

**Location:** add the following section after §13 and before §14.

```markdown
## 13A. Forward mechanical-scaffolding conformance boundary

The behavioral `game-stdlib` promotion process in §13 remains unchanged.
Mechanical scaffolding follows the separate ADR 0008 lane and the mechanical-
scaffolding register.

Before serious implementation, every new official game MUST audit its planned
behavior-free infrastructure against existing `engine-core` contract ergonomics,
registered `game-stdlib` scaffolding, dev-only `game-test-support` harnesses, and
thin `wasm-api` adapters. A matching lawful helper is reused unless the register
records an accepted exception.

A new behavior-free shape MUST be registered even on first use. Its entry names
the narrowest lawful home, behavior exclusions, exact current site, affected
hash/visibility/determinism surfaces, acceptance evidence, and next review
trigger. First-use registration is inventory and boundary control; it is not
promotion authority.

When the new game creates or exposes matching scaffolding in earlier official
games, its closeout MUST either queue a named bounded refactoring unit or record
an accepted `local-only`, `deferred`, or `rejected` disposition with rationale
and next review trigger. A third copy remains blocked by ADR 0008's hard
decision rule.

Any candidate that owns deal/reveal/projection policy, betting or pot semantics,
trick lifecycle, teams, graph semantics, accounting, reaction windows, scoring,
terminal outcome, legality, strategy, or hidden-state policy is not mechanical
scaffolding. Reject it from this lane and route it to the game-local behavioral
implementation, mechanic atlas, or an ADR.

No conformance action may silently change replay bytes, hashes, fixture/export
authority, RNG output, serialization order, or viewer authorization. Such a
change requires the applicable ADR 0009 migration authority and explicit
compatibility evidence.
```

### 5.5 `docs/OFFICIAL-GAME-CONTRACT.md`

**Location 1:** replace the §3 workflow code block with this complete block.

```text
rules research
  -> source notes
  -> original Rulepath rules summary
  -> variant scope and naming decision
  -> rule coverage matrix
  -> mechanic inventory
  -> primitive-pressure comparison
  -> mechanical-scaffolding reuse-first audit
  -> competent-player analysis if useful
  -> Level 2 strategy evidence pack if Level 2 bot planned
  -> typed Rust rules and tests using accepted shared scaffolding
  -> semantic effects and visibility tests
  -> replay/golden traces/serialization
  -> random legal simulation and benchmarks
  -> bot implementation and bot tests
  -> UI metadata and UI smoke
  -> mechanical-scaffolding closeout: reuse receipt, register update, and
     prior-game retrofit decision
  -> public polish review
```

**Location 2:** add this subsection immediately after the workflow explanation.

```markdown
### Mechanical-scaffolding forward obligation

The reuse-first audit is mandatory for every new official game. Before serious
implementation, the game compares its planned behavior-free plumbing against
`MECHANICAL-SCAFFOLDING-REGISTER.md` and the accepted shared homes in
`engine-core`, `game-stdlib`, `game-test-support`, and `wasm-api`.

The audit records:

- matching register entries and helpers that will be reused;
- accepted exceptions or local-only decisions and their rationale;
- newly invented behavior-free scaffolding that must receive a register entry;
- earlier official games that may contain the same shape; and
- expected replay/hash, visibility, determinism, and migration impact.

Before the game is marked official, its scaffolding closeout MUST update the
register for every new shape and decide every prior-game match. A real prior-game
migration set creates a named bounded follow-on unit in `specs/README.md` in the
same closeout. No unit is required only when the register records an accepted
`local-only`, `deferred`, or `rejected` disposition with evidence and a next
review trigger.

This obligation does not authorize mechanical extraction of behavioral rules.
The mechanic atlas and its behavioral third-use hard gate remain the governing
process for legality, scoring, reveal, turn, trick, team, graph, accounting,
reaction, outcome, and hidden-state policy.
```

**Location 3:** in §12, use this complete replacement cluster for the current mechanic/scaffolding acceptance bullets:

```markdown
- mechanic inventory is complete;
- behavioral atlas and primitive-pressure ledger evidence is current;
- the mechanical-scaffolding reuse-first audit is complete and linked from
  `GAME-EVIDENCE.md`;
- every matching promoted scaffolding helper is used, or a named register-backed
  exception explains why not;
- every new behavior-free scaffolding shape is present in
  `MECHANICAL-SCAFFOLDING-REGISTER.md` with a decision state and next review
  trigger;
- every matching prior-game site has a named follow-on tracker unit or an
  accepted `local-only`, `deferred`, or `rejected` disposition;
- open behavioral promotion debt and scaffolding promotion debt are closed or
  governed by an accepted exception before later ladder advancement;
```

### 5.6 `docs/MECHANIC-ATLAS.md`

**Locked preservation:** §4 and §5A remain word-for-word. In particular, retain:

> | Third official game | Hard gate. The game MUST NOT proceed until a primitive-pressure ledger decides reuse, promotion, explicit deferral/rejection, or ADR. |

**Location 1:** add this section after §5A.

```markdown
## 5B. Parallel mechanical-scaffolding check

Sections 4 and 5A govern behavioral mechanic pressure and are not weakened or
replaced by this section.

Every new official game also completes the separate mechanical-scaffolding
reuse-first audit governed by ADR 0008 and
`MECHANICAL-SCAFFOLDING-REGISTER.md`. Behavior-free plumbing is not counted as a
mechanic use and MUST NOT be added to the atlas merely to satisfy the scaffolding
workflow.

The game audit must:

- reuse matching registered scaffolding or record an accepted exception;
- register newly invented behavior-free scaffolding;
- identify prior official games that now contain matching scaffolding; and
- queue a bounded prior-game refactor unit or record an accepted no-refactor
  disposition.

If the candidate controls or interprets behavior, reject it from the scaffolding
lane and return it to this atlas, game-local code, or ADR review.
```

**Location 2:** append these bullets to §11 “Stage advancement check”:

```markdown
- the current game has a completed mechanical-scaffolding reuse-first audit;
- every new behavior-free scaffolding shape is registered;
- every prior-game scaffolding migration set has a named follow-on unit or an
  accepted register disposition;
- the scaffolding review did not reclassify a Non-Promotion List behavior as
  plumbing or alter the behavioral third-use gate;
```

### 5.7 `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`

**Location 1:** replace the `candidate` row in “Decision States” with this broader first-use-safe definition. The other states remain unchanged.

```markdown
| `candidate` | A new or repeated behavior-free scaffolding shape is recorded, but no shared-helper decision has landed. A first-use candidate MUST name its owning game/site, behavior exclusions, and second-use or other next review trigger. |
```

**Location 2:** replace the introductory paragraph under “Current Entries” with this complete text.

```markdown
The MSC-8C-001…010 entries and their R1–R4 receipts are the historical baseline
created by Unit 8C. Preserve them as shipped evidence.

For every new official game after the forward-governance interlock becomes
effective, register maintenance is part of the normal game lifecycle rather
than an optional reaction to a later ticket. A new behavior-free shape receives
a register entry before game closeout even when it remains a first-use
`candidate` or `local-only` shape. A game that introduces no new scaffolding
records that result in its game evidence and CI audit receipt; it does not need a
no-op candidate entry.
```

**Location 3:** add the following sections after “Non-Promotion List” and before “Current Entries.”

```markdown
## Forward Per-Game Maintenance Cadence

Every new official game completes two linked register checkpoints.

### Pre-implementation checkpoint

The game's reuse-first audit records:

| Field | Required content |
|---|---|
| Game and gate | Stable game id and roadmap/spec unit. |
| Audit evidence | Link to the filled `GAME-MECHANICS.md` audit and initialized `GAME-EVIDENCE.md` receipt. |
| Existing scaffolding reviewed | Matching MSC entry ids and accepted helpers in `engine-core`, `game-stdlib`, `game-test-support`, or `wasm-api`. |
| Planned disposition | Reuse, accepted exception, local-only, new candidate, rejected/rerouted, or not applicable with rationale. |
| Expected prior matches | Earlier official games/sites that may require characterization or migration. |
| Compatibility expectation | Hash, visibility, determinism, fixture/export, and ADR 0009 migration expectation. |

### Post-implementation checkpoint

Before official-game closeout:

1. update every reused entry whose migration evidence or next-review trigger
   changed;
2. add an entry for every newly invented behavior-free scaffolding shape;
3. record exact new and prior matching sites;
4. classify the prior-game migration set;
5. link the game evidence receipt and the machine audit record; and
6. name the follow-on tracker unit or accepted no-unit disposition.

A first-use entry is inventory, not extraction authority. It normally remains
`candidate`, `local-only`, or `rejected` until repeated evidence satisfies ADR
0008.

## Automatic Prior-Game Refactor Trigger

A bounded follow-on refactoring unit is required when a new game or newly
promoted helper leaves real characterization or migration work in earlier
official games. This includes:

- an exact semantic behavior-free shape now present in the new game and one or
  more earlier official games;
- a promoted helper whose migration set includes earlier games;
- a register entry that becomes `promotion-debt-open`; or
- a pre-third-copy decision whose accepted outcome requires consolidation.

The same closeout that records the migration set MUST add a named unit to
`specs/README.md`. The unit names the games, candidate/register entry, expected
hash and visibility impact, characterization evidence, rollback boundary, and
admission consequence.

A follow-on unit is not required only when the governing entry is explicitly
`local-only`, `deferred`, or `rejected` and records:

- why the sites are not semantically identical or why extraction is not worth
  the risk;
- the evidence supporting that decision;
- an owner; and
- a concrete next review trigger.

An unnamed TODO, issue reference without a tracker unit, or bare “review later”
does not satisfy this rule. Existing third-copy and promotion-debt blocking law
remains authoritative.
```

**Location 4:** append these checks to “Review Checklist.”

```markdown
- a new game's pre-implementation audit and post-implementation closeout are both linked;
- every new first-use scaffolding shape is registered without implying premature promotion;
- prior matching official-game sites are complete;
- a required follow-on unit exists in `specs/README.md`, or the accepted no-unit disposition carries rationale, owner, evidence, and next review trigger;
- the CI audit receipt agrees with the human evidence and register state;
```

### 5.8 `docs/AGENT-DISCIPLINE.md`

**Location:** add §8B after §8A.

```markdown
## 8B. New-game scaffolding reuse-and-track protocol

This protocol applies to every bounded task that creates or extends an official
game's Rust-owned production, bridge, test, replay, serialization, or evidence
plumbing. It applies even when `Task profile` is not `scaffold-refactor`.

Before implementing such plumbing, agents MUST:

1. read the relevant entries in `MECHANICAL-SCAFFOLDING-REGISTER.md` and inspect
   the accepted shared home named by those entries;
2. complete the task packet's reuse-first audit fields;
3. reuse a matching promoted helper, or link the accepted register exception
   before writing a parallel local shape; and
4. identify any genuinely new behavior-free scaffolding and any prior official
   games likely to contain the same shape.

During implementation, agents MUST keep adapters narrow. An adapter may translate
game-local types into a generic accepted API; it MUST NOT recreate generic seat
syntax, effect-envelope construction, action-tree framing, stable-byte framing,
visibility geometry, or evidence-profile driving behind a different name.

Before closeout, agents MUST:

1. update `GAME-EVIDENCE.md` with the reuse and new-scaffolding receipt;
2. add or update the governing register entry for every new shape;
3. update the machine scaffolding-audit record;
4. name the prior-game migration set; and
5. add the required follow-on unit to `specs/README.md`, or link the accepted
   `local-only`, `deferred`, or `rejected` disposition.

Agents MUST stop and reassess if the proposed scaffolding decides legality,
scoring, reveal, turn, trick, team, graph, accounting, reaction, outcome,
strategy, effect meaning, renderer policy, or hidden-state policy. Such work is
behavioral and does not belong in this lane.

Agents MUST NOT use a local wrapper, renamed copy, blanket `allow` list, skipped
CI job, or broad golden update to evade this protocol. Any byte, hash, fixture,
RNG, export, or visibility migration requires the authority and evidence
required by ADR 0009.
```

**Location:** append these bullets to §13 “Review check.”

```markdown
- every new-game task completed the scaffolding reuse-first audit;
- known promoted scaffolding was reused or covered by an accepted register exception;
- every new behavior-free scaffolding shape and prior-game migration set is registered;
- every required follow-on refactor unit is queued or explicitly disposed by an accepted register decision;
```


### 5.9 `docs/TESTING-REPLAY-BENCHMARKING.md`

**Location 1:** add this section immediately before §17 “CI expectations,” or as §17A immediately after it. The final numbering may be normalized during `/reassess-spec`.

```markdown
## Mechanical-scaffolding governance check

Gate 1 MUST run one repository-level mechanical-scaffolding governance check for
every pull request and push covered by the existing game-smoke workflow.

The check has two enforcement layers:

1. **receipt and register freshness** — every official game in `ci/games.json`
   has a scaffolding-audit record; new games use the current forward receipt;
   referenced register entries, evidence paths, exceptions, and follow-on spec
   units exist; and any hash/visibility migration names ADR 0009 authority; and
2. **known-shape linting** — high-confidence fingerprints of already-promoted
   generic scaffolding in game-local source are either absent, routed through the
   shared helper, or explicitly covered by the game's register-backed audit
   decision.

The check MUST NOT claim to prove arbitrary semantic equivalence. Textual
similarity, common control flow, or shared game nouns are not sufficient evidence
that code is behavior-free scaffolding. Novel shapes remain a human-reviewed
classification recorded in `GAME-MECHANICS.md`, `GAME-EVIDENCE.md`, and the
register.

False-positive control is mandatory:

- lint only stable, high-confidence generic-contract fingerprints already named
  by accepted register entries;
- report the signal id, file, line, expected shared home, and governing register
  entry;
- exclude behavior on the register Non-Promotion List from automatic
  scaffolding classification; and
- permit an exception only through a committed register decision with owner,
  rationale, evidence, and next review trigger.

There is no environment-variable, branch-name, or CI-label bypass. A temporary
or permanent exception is a repository decision and is reviewed in the same
change that needs it.

This check changes no game behavior, replay/hash bytes, visibility authority, or
fixture/export authority. It validates governance receipts and source
conformance only.
```

**Location 2:** add this bullet to §17 “CI expectations”:

```markdown
- mechanical-scaffolding governance receipt/register/fingerprint drift check;
```

### 5.10 `docs/ROADMAP.md`

**Location 1:** add the following subsection after Gate 17 and before Gate 18.

```markdown
### Pre-Gate-18: forward scaffolding-reuse governance

Purpose: convert the accepted ADR 0008 mechanical-scaffolding lane and completed
8C/R1-R4 retrofits into a standing per-new-game reuse-and-tracking obligation
before another official game is implemented.

Exit:

- `FOUNDATIONS.md`, the boundary/architecture docs, official-game workflow,
  register, agent law, testing/CI law, and templates require a reuse-first audit,
  registration of new behavior-free scaffolding, and a named prior-game refactor
  unit or accepted no-refactor disposition;
- the mechanical-scaffolding CI check is present in Gate 1 and proves audit
  receipt freshness, register linkage, accepted exceptions, known promoted-shape
  conformance, and follow-on tracker linkage;
- the 17 existing official games are represented by bounded legacy 8C/R1-R4
  receipt pointers rather than re-audited or rewritten;
- no game code, shared-helper API, trace, fixture, hash, RNG, export, visibility,
  or benchmark threshold changes in this governance unit; and
- `specs/README.md` marks the governance unit `Done` before Gate 18 can be
  authored or implemented.

Not allowed: re-proposing ADR 0008 or ADR 0009; weakening the behavioral
third-use gate; promoting new scaffolding; implementing Spades; adding YAML or a
DSL; putting game/mechanic nouns in `engine-core`; changing deterministic bytes
or viewer authorization; or silently regenerating evidence artifacts.
```

**Location 2:** prepend this admission sentence to Gate 18.

```markdown
Admission: the pre-Gate-18 forward scaffolding-reuse governance unit is `Done`,
the Gate 18 spec contains a completed reuse-first audit, and any prior-game
scaffolding impact has a named follow-on unit or accepted register disposition.
```

**Location 3:** append this Gate 18 exit bullet:

```markdown
- the game's mechanical-scaffolding closeout registers every new behavior-free
  shape, records all reused promoted scaffolding, and queues any required
  prior-game refactor unit before the game is marked official;
```

### 5.11 `docs/adr/0008-mechanical-scaffolding-governance.md`

The ADR mechanism and exact text are specified in §7. The amendment belongs here in authority order, but it should be applied append-only after the governance spec is reassessed and accepted.

### 5.12 `docs/adr/0009-replay-fixture-hash-taxonomy.md` — not applicable

No amendment is recommended. Every draft above explicitly preserves current bytes and routes any future replay/hash/fixture/export/RNG/visibility migration through ADR 0009. Reopening ADR 0009 would blur this governance-only unit and violate the locked scope.

### 5.13 `docs/adr/ADR-TEMPLATE.md` — not applicable

No template gap blocks this work. The proposed ADR 0008 extension can use its existing required fields, impact sections, migration matrix, and review checklist. If maintainers choose the fallback short ADR, the existing template is adequate.

### 5.14 `specs/README.md`

**Location 1:** in the active-epoch introduction, use this replacement paragraph.

```markdown
This table is the **living progress record** for the public scaling phase. A new
brainstorm that wants to “produce the next spec to continue the roadmap” reads
this first and picks the lowest unit whose status is not `Done`, honoring both
standing interlocks: open behavioral primitive-promotion debt in
`../docs/MECHANIC-ATLAS.md` closes before the next mechanic-ladder gate, and a
new-game spec completes the mechanical-scaffolding reuse-first audit in
`../docs/MECHANICAL-SCAFFOLDING-REGISTER.md` before serious implementation.
Required prior-game scaffolding refactors must appear as named tracker units or
accepted register dispositions. `docs/ROADMAP.md` records ladder law; this table
records progress.
```

**Location 2:** insert this row immediately after `8C-R4` and before Gate 18.

```markdown
| 8F | Pre-Gate-18 — forward scaffolding-reuse governance | _(seed; author from `../reports/forward-scaffolding-reuse-governance-change-plan.md`, then run `/reassess-spec` in place)_ | Not started | Blocking interlock before Gate 18. Extend ADR 0008 and land the authority-ordered workflow/template/register/CI amendments that require reuse-first audit, register-new, and queue-or-dispose prior-game refactors. Governance only: no game/helper/trace/fixture/hash/visibility migration. |
```

**Location 3:** replace the Gate 18 row with this complete row.

```markdown
| 9 | Gate 18 — Spades (partnerships) | _(seed; unwritten)_ | Not started | Blocked until **8F** is `Done`, in addition to the already-closed 8M, 8C, and 8C-R1…R4 obligations and the existing partnership/trick-taking atlas interlock. The Gate 18 spec must contain the first forward reuse-first audit and must name any new scaffolding register entries and prior-game follow-on units before implementation admission. Teams/partnership scoring + UI grouping stay game-local. |
```

**Location 4:** append this paragraph to “Spec format,” after the numbered structure.

```markdown
For every new-game spec, the Scope/Deliverables/Acceptance Evidence sections MUST
include a mechanical-scaffolding reuse-first audit, expected register updates,
and a prior-game retrofit disposition. The spec is incomplete when those fields
are silent. A `not applicable` result requires a rationale and evidence link.
```

**Location 5:** replace the current Workflow section with this complete block.

```markdown
## Workflow

1. Pick the lowest non-`Done` unit from the active-epoch tracker.
2. Before drafting a new mechanic-ladder game spec:
   - check `docs/MECHANIC-ATLAS.md` for open behavioral promotion debt and close
     it first unless an accepted exception or ADR says otherwise;
   - run the mechanical-scaffolding reuse-first audit against
     `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` and the accepted shared homes;
   - identify existing helpers to reuse, new behavior-free scaffolding to
     register, and prior official games that may require refactoring; and
   - ensure every required prior-game refactor has a named tracker unit, or the
     register carries an accepted no-refactor disposition.
3. Write the spec from the format above, grounded in ROADMAP and the foundation
   set. An advisory report is input to this step; it is not a substitute for the
   reassessed spec.
4. Run `/reassess-spec` on the saved spec in place, accept the corrected spec,
   then decompose it into `tickets/` AGENT-TASK packets with `/spec-to-tickets`.
5. Execute, gathering the acceptance evidence and keeping the scaffolding audit,
   register, CI receipt, and tracker disposition current.
6. When exit criteria pass, flip the index status to `Done` and admit the next
   unit.
```

### 5.15 `templates/README.md`

**Location 1:** add `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` immediately after `docs/MECHANIC-ATLAS.md` in the authority list.

**Location 2:** add this paragraph after “Recommended lifecycle order.”

```markdown
For every new official game, `GAME-MECHANICS.md` contains the pre-implementation
mechanical-scaffolding reuse-first audit before implementation admission.
`GAME-EVIDENCE.md` is initialized with that audit result and later receives the
post-implementation register-freshness and prior-game-refactor closeout. No
separate domain template is required: the register owns shared-scaffolding
decisions, while the evidence receipt owns per-game status and links.
```

**Location 3:** replace the `GAME-MECHANICS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `AGENT-TASK.md`, and `GAME-EVIDENCE.md` rows in the template index with these complete rows.

```markdown
| `GAME-MECHANICS.md` | Per-game mechanic inventory, behavioral primitive-pressure review, and mandatory mechanical-scaffolding reuse-first audit. | Every official game before implementation admission and whenever mechanics or scaffolding change. | `docs/MECHANIC-ATLAS.md`, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md` |
| `GAME-IMPLEMENTATION-ADMISSION.md` | Short gate receipt proving rules, behavioral pressure, scaffolding reuse-first audit, and boundary prerequisites are ready before serious coding. | Before serious implementation work starts for an official game. | `docs/FOUNDATIONS.md`, `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/ROADMAP.md` |
| `AGENT-TASK.md` | Universal bounded task packet, including mandatory new-game scaffolding reuse/track fields and the deeper scaffold-refactor profile when migration work is authorized. | Any bounded implementation, testing, docs, refactor, UI, bot, benchmark, or release-prep task. | `docs/AGENT-DISCIPLINE.md`, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` |
| `GAME-EVIDENCE.md` | Machine-friendly status and artifact-link receipt, including pre-code scaffolding audit, post-build register freshness, and prior-game refactor disposition. | Every official game; initialize at implementation admission and update whenever evidence or scaffolding disposition changes. | `docs/FOUNDATIONS.md`, `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/EVIDENCE-FIXTURE-CONTRACT.md` |
```

### 5.16 `templates/GAME-IMPLEMENTATION-ADMISSION.md`

**Location:** replace the “Novel Mechanics and Pressure” table with this complete table.

```markdown
## Novel Mechanics and Pressure

| Surface | Status | Evidence link | Blocks implementation? |
|---|---|---|---:|
| mechanic inventory complete enough to start | ready/blocked/constrained | `<GAME-MECHANICS.md>` | yes/no |
| behavioral primitive-pressure decision, if needed | ready/blocked/not applicable: `<rationale>` | `<PRIMITIVE-PRESSURE-LEDGER.md or atlas link>` | yes/no |
| mechanical-scaffolding reuse-first audit complete | ready/blocked | `<GAME-MECHANICS.md reuse-first audit section>` | yes |
| matching registered/promoted scaffolding will be reused, or accepted exceptions are linked | ready/blocked/not applicable: `<rationale>` | `<MSC entry ids / register exception>` | yes |
| newly anticipated behavior-free scaffolding has a planned register disposition | ready/blocked/not applicable: `<rationale>` | `<planned MSC entry or no-new-scaffolding rationale>` | yes |
| expected prior-game matching sites and follow-on/no-follow-on disposition are identified | ready/blocked/not applicable: `<rationale>` | `<GAME-MECHANICS.md / register link>` | yes |
| ADR needed for boundary-changing work | yes/no | `<docs/adr/... or rationale>` | yes/no |
```

**Location:** add these rows to “Required Evidence Profile.”

```markdown
| pre-implementation scaffolding audit receipt | yes | yes | `<GAME-EVIDENCE.md mechanic/scaffolding rows>` |
| post-implementation register freshness and prior-game refactor receipt | no | yes | `<GAME-EVIDENCE.md mechanic/scaffolding rows>` |
| CI scaffolding-audit record | no | yes | `<ci/scaffolding-audits.json game row>` |
```

**Location:** add this paragraph before “Admission Decision.”

```markdown
Admission is blocked when the reuse-first audit is missing, when a known matching
helper is being reimplemented without an accepted exception, or when anticipated
new scaffolding has no register/closeout plan. Admission does not require a
first-use candidate to be promoted.
```

### 5.17 `templates/GAME-MECHANICS.md`

**Location 1:** add this section after “Repeated-shape comparison” and before “Second-use note.”

```markdown
## Mechanical scaffolding reuse-first audit

Complete this table for every new official game before implementation admission.
A game with no relevant scaffolding surface records one explicit not-applicable
row with rationale.

| Planned surface | Existing MSC entry/shared symbol reviewed | Decision | Why the accepted boundary fits or does not fit | New register entry needed? | Earlier official-game matches | Expected follow-on unit or accepted no-unit disposition | Hash/visibility/determinism expectation |
|---|---|---|---|---:|---|---|---|
| `<effect/seat/action-tree/stable-byte/test/evidence/bridge surface>` | `<MSC id and symbol/path>` | reuse / accepted exception / local-only / new candidate / rejected-rerouted / not applicable | `<rationale>` | yes/no | `<game ids/sites or none>` | `<unit id / register decision / none>` | unchanged / ADR 0009 migration required: `<authority>` |

Audit rules:

- compare semantic responsibility, not only names or text;
- reuse a matching promoted helper before writing a parallel local shape;
- register every new behavior-free first-use shape without treating first use as
  promotion authority;
- route legality, scoring, reveal, turn, trick, team, graph, accounting,
  reaction, outcome, strategy, effect meaning, renderer policy, and hidden-state
  policy back to the behavioral lane; and
- identify prior-game refactoring work now, not after the game ships.
```

**Location 2:** replace “Required repo atlas update” with this complete section.

```markdown
## Required repo atlas/register update

Update `docs/MECHANIC-ATLAS.md` and the relevant
`PRIMITIVE-PRESSURE-LEDGER.md` instance when behavioral mechanic pressure
changes. Update `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` when scaffolding is
reused with new evidence, newly invented, kept local by decision, deferred,
rejected, promoted, or leaves prior-game migration work.

| Update target | Required? | Reason | Owner |
|---|---:|---|---|
| `docs/MECHANIC-ATLAS.md` | yes/no | `<behavioral pressure reason>` | `<owner>` |
| `PRIMITIVE-PRESSURE-LEDGER.md` | yes/no | `<behavioral pressure reason>` | `<owner>` |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | yes/no | `<reuse/new shape/prior-game migration/no-unit disposition>` | `<owner>` |
| `specs/README.md` follow-on unit | yes/no | `<prior-game migration set or accepted no-unit rationale>` | `<owner>` |
| ADR | yes/no | `<boundary/hash/visibility reason>` | `<owner>` |
```

**Location 3:** append these review checks.

```markdown
- The mechanical-scaffolding reuse-first audit is complete even when the result is not applicable.
- Every known promoted scaffolding match is reused or covered by an accepted exception.
- Every new behavior-free scaffolding shape has a planned register entry.
- Every prior-game match has a planned tracker unit or accepted no-unit disposition.
- The required repo update section names the scaffolding register explicitly.
```

### 5.18 `templates/GAME-EVIDENCE.md`

**Location:** replace the “Mechanic and Scaffolding Decisions” table with this complete table.

```markdown
## Mechanic and Scaffolding Decisions

| Decision surface | Status | Artifact link | Notes |
|---|---|---|---|
| Mechanic inventory | complete/partial/blocker | `<GAME-MECHANICS.md>` | `<notes>` |
| Primitive-pressure ledger | complete/not applicable: `<rationale>`/blocker | `<PRIMITIVE-PRESSURE-LEDGER.md or atlas link>` | `<notes>` |
| Pre-implementation mechanical-scaffolding reuse-first audit | complete/blocker | `<GAME-MECHANICS.md audit section>` | `<reused MSC ids, exceptions, anticipated new shapes>` |
| Existing registered/promoted scaffolding adoption | complete/not applicable: `<rationale>`/blocker | `<MSC entries and code/test evidence>` | `<notes>` |
| Post-implementation new-scaffolding/register-freshness receipt | no new scaffolding / register updated / blocker | `<MECHANICAL-SCAFFOLDING-REGISTER.md entry ids>` | `<new sites, decision states, next review triggers>` |
| Prior-game duplication/refactor disposition | no prior match / follow-on unit queued / accepted local-only / accepted deferred / accepted rejected / blocker | `<specs/README.md unit or register decision>` | `<migration set, owner, next review trigger>` |
| CI scaffolding-audit record | pass/fail/blocker | `<ci/scaffolding-audits.json row>` | `<known signal dispositions>` |
| Open behavioral promotion/scaffolding debt | none / blocker / deferred by accepted exception | `<artifact link>` | `<notes>` |
```

**Location:** append these receipt-review checks.

```markdown
- The pre-implementation audit and post-implementation register receipt are distinct and both current.
- Every new behavior-free shape has a register decision and next review trigger.
- Every prior-game match has a tracker unit or accepted no-unit disposition.
- The CI audit record agrees with this receipt and the register.
- Any byte/hash/fixture/export/RNG/visibility change cites ADR 0009 migration authority.
```

### 5.19 `templates/AGENT-TASK.md`

**Location 1:** add `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` to the required foundation/authority list immediately after `docs/MECHANIC-ATLAS.md`.

**Location 2:** add this section after “Mechanics and primitive-pressure status” and before “Goal.”

```markdown
## New-game scaffolding reuse/track status

Complete this section for every task that creates or changes an official game's
Rust-owned production, bridge, test, replay, serialization, or evidence
plumbing. Mark the whole section `not applicable` only with a rationale.

| Required field | Value/evidence |
|---|---|
| game/gate and lifecycle phase | `<game id / spec unit / admission, implementation, or closeout>` |
| reuse-first audit receipt | `<GAME-MECHANICS.md section and GAME-EVIDENCE.md row>` |
| matching registered helpers | `<MSC ids, symbols, and homes>` |
| reuse plan | `<exact call sites/adapters>` |
| accepted exceptions/local-only decisions | `<register link or none>` |
| new behavior-free scaffolding introduced by this task | `<shape/sites or none>` |
| register update owner | `<owner and expected entry id/status>` |
| prior official-game matching sites | `<games/paths or none>` |
| follow-on refactor disposition | `<tracker unit id / accepted no-unit register decision / none>` |
| hash/visibility/determinism disposition | unchanged / `<ADR 0009 migration authority>` |
| CI audit signal disposition | `<signal ids and receipt row>` |

A new-game task MUST NOT implement a known promoted scaffolding shape locally
until the accepted exception is linked. A task that discovers a new shape or
prior-game match owns the register/evidence update or explicitly hands it to a
named dependent task; it may not leave an unnamed cleanup note.
```

**Location 3:** add this row to “Acceptance evidence.”

```markdown
| scaffolding reuse/register/retrofit receipt | yes/no/not applicable: `<rationale>` | `<GAME-EVIDENCE.md rows, MSC ids, CI audit row, follow-on unit or no-unit decision>` |
```

**Location 4:** add this row to “Implementation boundaries.”

```markdown
| mechanical scaffolding | Reuse accepted behavior-free helpers first; register new shapes; queue or dispose prior-game refactors. No behavior may be reclassified as plumbing. | `<audit/register/CI/tracker evidence>` |
```

**Location 5:** add these rows to “Documentation required.”

```markdown
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | yes/no | `<reuse evidence/new entry/prior-game disposition>` |
| `ci/scaffolding-audits.json` | yes/no | `<game audit record change>` |
| `specs/README.md` follow-on unit | yes/no | `<unit row or accepted no-unit rationale>` |
```

**Location 6:** append these forbidden changes and review checks.

```markdown
- Do not recreate a known promoted scaffolding helper behind a local wrapper or renamed copy without an accepted register exception.
- Do not omit a new scaffolding shape or prior-game migration set from the register/evidence closeout.
- Do not bypass the scaffolding governance CI check with an environment variable, branch label, or unreviewed allow list.
```

```markdown
- New-game scaffolding reuse/track status is complete or explicitly not applicable with rationale.
- Register, game evidence, CI audit record, and tracker disposition agree.
```

The existing “Scaffold-Refactor Profile” remains. It is still required for the deeper migration tasks that a new-game closeout queues; the new section above is the lighter standing obligation that discovers and schedules those tasks.

### 5.20 `templates/PRIMITIVE-PRESSURE-LEDGER.md` — not applicable

No amendment is recommended. The template already says:

- it governs behavioral scope only;
- behavior-free plumbing must be rejected there and routed to the scaffolding register; and
- the behavioral third-game hard gate remains authoritative.

Adding forward-scaffolding lifecycle fields here would create two competing owners and risk weakening the behavioral/scaffolding distinction. The new workflow should link to this template only for behavioral pressure.

---

## 6. Mechanical CI gate specification

### 6.1 Recommendation

Add one repository-level checker:

```text
scripts/check-scaffolding-governance.mjs
```

Add one non-runtime evidence manifest:

```text
ci/scaffolding-audits.json
```

Add focused checker tests and fixtures:

```text
scripts/check-scaffolding-governance.test.mjs
scripts/testdata/scaffolding-governance/<case>/...
```

Wire the checker into `.github/workflows/gate-1-game-smoke.yml` in the `repo-checks` job immediately after `Engine boundary` and before documentation drift checks:

```yaml
- name: Mechanical scaffolding governance
  run: node scripts/check-scaffolding-governance.mjs
```

Gate 1 is the correct home because the check is repository-wide, requires Node, should run once rather than once per game, and is conceptually adjacent to `boundary-check.sh`, `check-doc-links.mjs`, and `check-catalog-docs.mjs`. Gate 0 and Gate 2 need no change.

### 6.2 What CI can and cannot prove

A fully precise static checker for “semantically identical behavior-free scaffolding” is infeasible here. Rust source can be textually different but semantically identical; similar loops can encode unrelated game behavior; and the behavior-free classification depends on ownership, visibility, replay, and policy boundaries. A checker that claimed otherwise would create false confidence and pressure maintainers to add broad suppressions.

The strongest tractable gate is therefore hybrid:

1. **mandatory reviewed receipts** for all scaffolding decisions, including novel shapes; and
2. **narrow high-confidence source fingerprints** for already-known promoted scaffolding, where the accepted register entry gives the checker a stable semantic target.

The gate proves process completeness and catches known regressions. It does not replace code review or the register decision.

### 6.3 Proposed audit-manifest shape

`ci/scaffolding-audits.json` is CI evidence metadata only. It is not loaded by games, Rust/WASM behavior, the browser, fixtures, or replay. It contains no selectors, formulas, rule conditions, triggers, or executable instructions.

Recommended top-level shape:

```json
{
  "schema_version": 1,
  "games": [
    {
      "id": "<game_id>",
      "coverage": "forward-v1",
      "evidence": "games/<game_id>/docs/GAME-EVIDENCE.md#mechanic-and-scaffolding-decisions",
      "audit": "games/<game_id>/docs/MECHANICS.md#mechanical-scaffolding-reuse-first-audit",
      "register_entries_reviewed": ["MSC-8C-001", "MSC-8C-002"],
      "register_decisions": ["<new-or-updated-MSC-id>"],
      "disposition": "reuse-only | no-new-scaffolding | register-updated | accepted-local-only | accepted-deferred | accepted-rejected",
      "prior_matching_games": ["<game_id>"],
      "follow_on_unit": "<spec unit id or null>",
      "no_follow_on_decision": "<MSC id or null>",
      "known_signal_dispositions": [
        {
          "signal": "MSC-8C-001.effect-envelope-literal",
          "decision": "reused | exception | not-present",
          "evidence": "<path or MSC decision>"
        }
      ],
      "compatibility": {
        "hashes": "unchanged | migration-authorized",
        "visibility": "unchanged | migration-authorized",
        "determinism": "unchanged | migration-authorized",
        "migration_authority": "<ADR-0009 ticket/spec or null>"
      }
    }
  ]
}
```

The checker should reject unknown fields by default so the receipt cannot silently become a second configuration language. Schema evolution requires a version bump and a bounded governance change.

### 6.4 Historical bootstrap without redoing 8C

The governance unit should add one compact legacy receipt for each of the 17 existing games. `coverage: "legacy-8c-covered"` is permitted only for this frozen set and only when the record links the existing Unit 8C/R1–R4 evidence:

| Historical coverage | Games |
|---|---|
| Unit 8C / pilot receipt, then R1 closeout where applicable | `race_to_n`, `draughts_lite` |
| 8C-R1 | `three_marks`, `column_four`, `directional_flip`, `token_bazaar` |
| 8C-R2 | `high_card_duel`, `secret_draft`, `poker_lite`, `masked_claims` |
| 8C-R3 | `plain_tricks`, `flood_watch`, `frontier_control`, `event_frontier` |
| 8C-R4 | `river_ledger`, `briar_circuit`, `vow_tide` |

The checker hard-codes or reads a committed `legacy_8c_games` set that cannot grow after Unit 8F. Any new `games/<id>` directory must use `coverage: "forward-v1"`. This makes Spades—or the neutral Rulepath game ID ultimately chosen for Gate 18—the first game that cannot claim historical coverage.

### 6.5 Required validations

The checker should fail on all of the following:

#### Enumeration and schema

- `ci/scaffolding-audits.json` is malformed or has an unsupported schema version;
- its game IDs are not set-equal to `ci/games.json` and the real `games/` directories;
- duplicate game IDs exist;
- a new game uses `legacy-8c-covered`;
- required fields are blank, contradictory, or use an unknown enum value;
- an evidence or audit path does not exist.

#### Register freshness

- a referenced `MSC-*` ID does not exist in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`;
- `disposition: register-updated` has no register decision;
- a new shape is declared in evidence but no register decision is linked;
- `accepted-local-only`, `accepted-deferred`, or `accepted-rejected` has no register decision;
- a no-follow-on disposition lacks its register-backed owner/rationale/next-review evidence link.

#### Prior-game refactor scheduling

- `prior_matching_games` is non-empty and neither `follow_on_unit` nor `no_follow_on_decision` is present;
- `follow_on_unit` is named but the unit ID is absent from `specs/README.md`;
- a register entry is `promotion-debt-open` but no closure unit or accepted exception is linked;
- the same prior game is assigned to conflicting follow-on decisions for the same entry.

#### Replay, hash, visibility, and determinism

- any compatibility field says `migration-authorized` while `migration_authority` is null;
- the authority does not reference ADR 0009 and a named bounded spec/ticket;
- the audit claims `unchanged` while the same change modifies committed trace/fixture/export artifacts without an explicit migration receipt. This last check may use the pull-request diff when available and fall back to committed consistency checks locally.

### 6.6 Known-shape fingerprint layer

The first version should deliberately cover only high-confidence, already-promoted generic shapes. It should not scan for “duplication” in the abstract.

| Signal ID | Initial detection target | Governing entry | Pass condition |
|---|---|---|---|
| `MSC-8C-001.effect-envelope-literal` | direct production `EffectEnvelope { visibility: ..., payload: ... }` construction under `games/*/src` where the accepted constructor fits | MSC-8C-001 | constructor use, or a linked exception explaining why the literal is semantically required |
| `MSC-8C-002.local-seat-grammar` | local canonical `seat_<n>` formatter/parser implementation rather than accepted seat-ID grammar/import helpers | MSC-8C-002 | shared grammar/helper use, or a linked alias/exception decision |
| `MSC-8C-004.local-action-tree-v1-framing` | local production implementation of the accepted ActionTree v1 byte/hash framing | MSC-8C-004 | use of `ActionTree::stable_bytes/stable_hash` or a linked exception |
| `MSC-8C-005.local-stable-byte-writer` | a game-local stable-byte writer or duplicated `RPSB` framing | MSC-8C-005 | use of the accepted writer or a linked exception |
| `MSC-8C-006.production-support-edge` | a normal/build dependency on `game-test-support` | MSC-8C-006 | no edge; this remains aligned with `boundary-check.sh` |

C-03 seat-count/ring helpers, C-07 pairwise no-leak geometry, C-08 evidence drivers, and C-09 bounded-index sampling should initially be **receipt-enforced only**. Broad regexes for modulo, loops, seat arithmetic, or profile code would false-positive on legitimate game-local behavior. Later expansion requires a demonstrated high-precision signature and checker fixtures.

The scanner should:

- restrict itself to named paths and production/test categories appropriate to the signal;
- use stable combinations of type names, symbols, and framing constants rather than one generic token;
- emit file and line context;
- keep the signal list in checker code, not a new YAML/DSL rule file; and
- require a test fixture for every signal and every accepted suppression shape.

### 6.7 Distinguishing scaffolding from legitimate game-local code

The gate uses three filters:

1. **accepted semantic scope:** a source fingerprint exists only when an accepted MSC entry already says the target is behavior-free;
2. **Non-Promotion List exclusion:** the scanner never treats deal/reveal/projection, betting/pot, trick, team, graph, accounting, reaction, scoring/outcome, legality, strategy, or hidden-state policy as mechanical scaffolding; and
3. **reviewed disposition:** ambiguous or novel code is classified in the human audit and register, not guessed by CI.

This is intentionally asymmetric. CI is strict about known paved-road regressions and strict about missing decisions; it is conservative about declaring a new abstraction.

### 6.8 Exception and override semantics

There is no `SKIP_SCAFFOLDING_CHECK`, branch-label waiver, comment directive, or untracked allow list.

An accepted exception must be committed and reviewable. It consists of:

- a register decision ID;
- exact game/path/symbol scope;
- rationale explaining why the accepted helper boundary does not fit or why local clarity/risk wins;
- hash, visibility, and determinism impact;
- owner; and
- next review trigger or explicit no-further-review decision.

The audit manifest points to that decision; it does not duplicate the rationale. Expired or dangling exceptions fail the checker.

### 6.9 Failure output

Each failure should be actionable, for example:

```text
scaffolding-governance check failed:
 - games/<id>/src/effects.rs:42 matches MSC-8C-001.effect-envelope-literal
   but <id> has no reuse or accepted-exception disposition in
   ci/scaffolding-audits.json
 - <id> names prior matching game <old_id> for MSC-<id>, but neither a
   follow_on_unit nor no_follow_on_decision is recorded
 - follow_on_unit 8F-R1 is absent from specs/README.md
```

The checker should end with a compact success summary naming the number of games, forward-v1 receipts, legacy receipts, known signals checked, register decisions linked, and follow-on units verified.

### 6.10 Proposed CI-file disposition

| File | Change |
|---|---|
| `scripts/check-scaffolding-governance.mjs` | **New.** Schema, set-equality, path/ID, follow-on, migration-authority, and known-signal checks. |
| `scripts/check-scaffolding-governance.test.mjs` | **New.** Node test suite for pass/fail fixtures and false-positive cases. |
| `scripts/testdata/scaffolding-governance/**` | **New.** Minimal synthetic repository fixtures; no game behavior. |
| `ci/scaffolding-audits.json` | **New.** Non-runtime evidence receipt, bootstrapped to existing 8C/R1–R4 evidence. |
| `.github/workflows/gate-1-game-smoke.yml` | **Amend.** Add one `repo-checks` step after boundary check. |
| `scripts/check-ci-games.mjs` | **Not applicable by default.** Keep game enumeration ownership there; the new checker independently consumes its validated manifest. A later consolidation is unnecessary for Unit 8F. |
| `.github/workflows/gate-0-hygiene.yml` | **Not applicable.** The gate needs repository/game context and Node, and Gate 1 already owns analogous drift checks. |
| `.github/workflows/gate-2-benchmarks.yml` | **Not applicable.** This is not a performance threshold. |

---

## 7. ADR mechanism recommendation

### 7.1 Required default: extend ADR 0008 in place, append-only

The default recommendation is to amend accepted ADR 0008 with a dated, append-only forward-obligation extension.

Reasons:

1. the lane, allowed homes, exclusions, semantic-identity requirement, and second-use/third-copy rule are already decided there;
2. the new work operationalizes the same decision rather than choosing a competing architecture;
3. ADR 0008 already says register maintenance becomes required and anticipates downstream workflow; the gap is incomplete wiring;
4. a separate ADR would make readers consult two decisions to understand one scaffolding lifecycle; and
5. append-only text can preserve the original 2026-06-22 context and consequences without pretending the forward lifecycle was part of the original acceptance.

This is an extension, not a reversal. If a repository policy outside the inspected files requires accepted ADRs to be immutable, use the fallback in §7.6.

### 7.2 Draft status-note addition

Append this paragraph to the existing Status note:

```markdown
Forward-obligation extension accepted on `<YYYY-MM-DD>` through Unit 8F. The
extension does not change the lane, allowed homes, Non-Promotion List,
second-use/pre-third-copy thresholds, or behavioral mechanic gate. It makes the
lane a standing per-new-game workflow: reuse-first audit, first-use registration,
and queue-or-dispose prior-game refactoring, with Gate 1 receipt enforcement.
```

### 7.3 Exact “Affected foundation sections” replacement

Replace the current affected-sections field with this complete field:

```markdown
- Affected foundation and governed sections:
  `FOUNDATIONS.md` §4 (lane context unchanged), §11, and §12;
  `ARCHITECTURE.md` §3A and new §3B;
  `ENGINE-GAME-DATA-BOUNDARY.md` §2A and new §13A;
  `OFFICIAL-GAME-CONTRACT.md` §3 and §12;
  `MECHANIC-ATLAS.md` new §5B and §11, with §§4–5A unchanged;
  `MECHANICAL-SCAFFOLDING-REGISTER.md` Decision States, Current Entries,
  Forward Per-Game Maintenance Cadence, Automatic Prior-Game Refactor Trigger,
  and Review Checklist;
  `AGENT-DISCIPLINE.md` new §8B and §13;
  `TESTING-REPLAY-BENCHMARKING.md` mechanical-scaffolding governance check and
  §17 CI expectations;
  `ROADMAP.md` pre-Gate-18 governance interlock and Gate 18 admission;
  `specs/README.md` active tracker, spec format, and workflow;
  `templates/README.md`, `GAME-IMPLEMENTATION-ADMISSION.md`,
  `GAME-MECHANICS.md`, `GAME-EVIDENCE.md`, and `AGENT-TASK.md`.
```

Also add these related documents:

```markdown
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
- `docs/ROADMAP.md`
- `specs/README.md`
- `templates/README.md`
- `templates/GAME-IMPLEMENTATION-ADMISSION.md`
- `templates/GAME-MECHANICS.md`
- `templates/GAME-EVIDENCE.md`
- `templates/AGENT-TASK.md`
```

### 7.4 Draft decision extension

Append this subsection at the end of ADR 0008's “Decision” section, before “Alternatives considered.”

```markdown
### Forward per-new-game obligation extension — `<YYYY-MM-DD>`

The mechanical-scaffolding lane is a standing obligation for every new official
game, not only a reaction to a later extraction task.

1. Before serious implementation, the game MUST complete a reuse-first audit of
   the scaffolding register and the lawful shared homes.
2. A matching promoted helper MUST be reused unless the register contains an
   accepted exception for the exact game/path/symbol scope.
3. Every newly invented behavior-free scaffolding shape MUST be registered on
   first use as `candidate`, `local-only`, or `rejected`, with exclusions and a
   next review trigger. First-use registration does not authorize promotion.
4. Before game closeout, the game MUST identify earlier official games with
   matching scaffolding. Real characterization or migration work MUST be queued
   as a named bounded unit in `specs/README.md`.
5. No follow-on unit is required only when the register records an accepted
   `local-only`, `deferred`, or `rejected` disposition with rationale, evidence,
   owner, and next review trigger.
6. A Gate 1 check MUST validate per-game audit receipt freshness, register and
   exception linkage, high-confidence known promoted-shape conformance, and
   follow-on tracker linkage. The check does not claim to infer arbitrary
   semantic equivalence.
7. Any byte, hash, fixture, export, RNG, serialization, or visibility migration
   remains governed by ADR 0009 and explicit migration evidence.

The behavioral third-use gate remains word-for-word effective. This extension
adds no new allowed home, changes no Non-Promotion List item, and authorizes no
new helper.
```

### 7.5 Draft migration-matrix additions

Append these rows to ADR 0008's migration matrix:

```markdown
| Forward foundation/workflow wiring | Add the standing audit, first-use registration, queue-or-dispose closeout, and stop conditions in the named docs | Unit 8F | Before Gate 18 authoring or implementation | Doc-link check, authority review, exact third-use-text comparison |
| New-game templates | Add mandatory pre-code audit and post-build register/refactor receipts | Unit 8F | Before Gate 18 admission | Filled-template fixture/review and link check |
| CI audit receipt | Add `ci/scaffolding-audits.json`, bootstrap only the frozen 17-game legacy set, and require `forward-v1` for future games | Unit 8F | Before Gate 18 game directory lands | Checker tests + set-equality check |
| Gate 1 enforcement | Add the repository-level scaffolding governance check | Unit 8F | Before Gate 18 implementation | Passing/failing checker fixtures + workflow run |
| Prior-game refactor scheduling | Require a tracker row or accepted no-unit register disposition when a new game exposes matching prior sites | Every future game closeout | Same closeout as the new game | Register/evidence/CI/tracker cross-check |
```

### 7.6 Fallback: short successor ADR

Use a new short ADR—recommended number `0010`, title “Forward Per-Game Mechanical-Scaffolding Obligation”—only if maintainers require accepted ADR text to be immutable or if reassessment discovers that the proposal changes the lane's architecture rather than operationalizing it.

The fallback ADR must:

- say it **extends** ADR 0008 and does not supersede its lane, homes, exclusions, or thresholds;
- reproduce the exact affected-sections list above;
- keep ADR 0008's historical text untouched;
- use the same migration matrix and impact statements; and
- avoid re-arguing alternatives already settled by ADR 0008.

A new ADR is **not** the default merely to create a new number.

---

## 8. Recommended governance spec unit

### 8.1 Unit identity and advisory status

Recommend this active-epoch unit:

```text
Unit ID: 8F
Title: Pre-Gate-18 — forward scaffolding-reuse governance
Proposed spec path: specs/pre-gate-18-forward-scaffolding-reuse-governance.md
Roadmap position: after 8C-R4 and before Gate 18
Nature: governance/doc/template/CI interlock; no game or helper implementation
```

`8F` is preferable to another `8C-R*` identifier. The R1–R4 series was the
bounded retroactive migration program owned by Unit 8C; this unit is a new
**forward** lifecycle interlock. It remains in the pre-Gate-18 `8*` lane without
pretending to be a fifth historical retrofit wave.

This change plan is the seed for that spec, not the spec itself. The maintainer
should save a separately authored spec under `specs/`, run `/reassess-spec` on
that in-place file, accept the corrected spec, and only then run
`/spec-to-tickets`. The advisory text below is intentionally concrete enough to
seed that work while leaving repository validation, ticket boundaries, owners,
and final acceptance wording to reassessment.

### 8.2 Draft objective

```markdown
## Objective

Institutionalize ADR 0008's mechanical-scaffolding lane as a standing,
forward, per-new-game obligation before Gate 18: every new official game must
complete a reuse-first audit, register every new behavior-free scaffolding
shape, and queue or explicitly dispose any prior-game refactoring exposed by
that work. Land the authority-ordered doctrine, workflow, template, evidence,
and Gate 1 enforcement changes without implementing a game, promoting a helper,
or changing deterministic or viewer-visible bytes.
```

### 8.3 Draft scope

#### In scope

```markdown
## Scope

### In scope

- append an explicitly dated forward-obligation extension to accepted ADR 0008,
  including the complete affected-section list and migration matrix;
- amend the authority spine, official-game workflow, register cadence, agent
  law, testing law, roadmap, and live tracker in the order named by
  `docs/README.md`;
- amend the per-game templates so admission, mechanics, evidence, and bounded
  tasks carry the same mandatory lifecycle;
- add `ci/scaffolding-audits.json` as a non-runtime evidence receipt;
- add `scripts/check-scaffolding-governance.mjs`, focused test fixtures, and its
  Gate 1 `repo-checks` invocation;
- represent the frozen 17-game corpus through explicit Unit 8C/R1–R4 evidence
  pointers, without re-auditing or rewriting those games;
- require `forward-v1` audit records for every game added after the frozen
  legacy set;
- prove that required prior-game follow-on units are present in
  `specs/README.md`, or that a valid register-backed no-unit disposition exists;
- reconcile doc links, tracker state, and final Unit 8F evidence.
```

#### Out of scope

```markdown
### Out of scope

- Gate 18 / Spades design, implementation, rules, UI, bots, fixtures, traces,
  benchmarks, or documentation;
- extraction, promotion, redesign, or relocation of any Rust helper;
- a new behavioral primitive or change to the mechanic atlas promotion state;
- a fresh 17-game semantic duplication audit;
- migration of existing game call sites beyond the evidence pointers already
  established by Unit 8C and R1–R4;
- generic clone detection, AST-wide semantic-equivalence inference, or an
  attempt to make CI decide architecture;
- any runtime configuration format or policy engine.
```

#### Not allowed

```markdown
### Not allowed

- changing, paraphrasing, weakening, or bypassing the behavioral third-use hard
  gate in `docs/MECHANIC-ATLAS.md` §4;
- changing ADR 0008's allowed homes, Non-Promotion List, semantic-identity rule,
  second-use review, or pre-third-copy hard-decision threshold;
- adding game, mechanic, genre, rule, scoring, trick, betting, pot, team,
  topology, accounting, reaction-window, deal, reveal, or projection nouns to
  `engine-core`;
- moving legality, outcome, visibility, or rule decisions into TypeScript;
- adding YAML, a selector/condition/trigger DSL, generated policy code, or a
  runtime scaffolding registry;
- changing RNG draw order, serialization, stable bytes, trace schema, fixture
  hashes, replay export, viewer authorization, no-leak behavior, or benchmark
  floors;
- regenerating goldens or fixtures to make the governance unit pass;
- using an environment variable, workflow flag, or undocumented allowlist to
  bypass the new check;
- authoring or implementing Gate 18 before Unit 8F is `Done`.
```

### 8.4 Draft deliverables

```markdown
## Deliverables

1. **ADR extension**
   - append-only dated extension to
     `docs/adr/0008-mechanical-scaffolding-governance.md`;
   - affected-section and migration-matrix updates;
   - no change to ADR 0009 except links from affected documents.

2. **Authority and workflow amendments**
   - `docs/FOUNDATIONS.md`;
   - `docs/ARCHITECTURE.md`;
   - `docs/ENGINE-GAME-DATA-BOUNDARY.md`;
   - `docs/OFFICIAL-GAME-CONTRACT.md`;
   - `docs/MECHANIC-ATLAS.md` without changing §4 or §5A;
   - `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`;
   - `docs/AGENT-DISCIPLINE.md`;
   - `docs/TESTING-REPLAY-BENCHMARKING.md`;
   - `docs/ROADMAP.md`;
   - `specs/README.md`.

3. **Template amendments**
   - `templates/README.md`;
   - `templates/GAME-IMPLEMENTATION-ADMISSION.md`;
   - `templates/GAME-MECHANICS.md`;
   - `templates/GAME-EVIDENCE.md`;
   - `templates/AGENT-TASK.md`;
   - explicit no-change proof for `templates/PRIMITIVE-PRESSURE-LEDGER.md`.

4. **Mechanical enforcement**
   - `ci/scaffolding-audits.json`;
   - `scripts/check-scaffolding-governance.mjs`;
   - `scripts/check-scaffolding-governance.test.mjs`;
   - `scripts/testdata/scaffolding-governance/**`;
   - Gate 1 workflow invocation.

5. **Legacy bootstrap and closeout evidence**
   - exact frozen 17-game legacy set;
   - one historical coverage pointer per game;
   - proof that a synthetic/new game cannot claim legacy coverage;
   - passing and failing checker-fixture receipts;
   - Unit 8F Outcome and tracker status flip after all exit criteria pass.
```

### 8.5 Candidate work breakdown / AGENT-TASK series

The final spec should keep each item independently reviewable. The following is
a candidate decomposition, not a pre-approved ticket series.

| Candidate task | Bounded objective | Main files | Dependency | Required evidence |
|---|---|---|---|---|
| `FSGOV-001` | Freeze the amendment inventory, legacy game set, accepted baseline, and exact behavioral-gate checksum/text fixture. | Unit 8F spec evidence only; optionally checker testdata | none | 17-game set equals `ci/games.json`; copied §4 gate text matches exact source; no proposed helper/game diff. |
| `FSGOV-002` | Extend ADR 0008 append-only and introduce the obligation at constitution level. | ADR 0008, `FOUNDATIONS.md` | 001 | authority review; affected sections complete; original ADR decision remains visible; exact §4 behavioral gate unchanged. |
| `FSGOV-003` | Land architecture, boundary, official-workflow, atlas-seam, and register-cadence amendments. | `ARCHITECTURE.md`, `ENGINE-GAME-DATA-BOUNDARY.md`, `OFFICIAL-GAME-CONTRACT.md`, `MECHANIC-ATLAS.md`, register | 002 | doc links; lane/home/Non-Promotion review; first-use and queue-or-dispose examples are internally consistent. |
| `FSGOV-004` | Land agent, testing, roadmap, and tracker law, including the blocking Gate 18 interlock. | `AGENT-DISCIPLINE.md`, `TESTING-REPLAY-BENCHMARKING.md`, `ROADMAP.md`, `specs/README.md` | 003 | lowest-non-Done order shows 8F before Gate 18; workflow includes both behavioral atlas and scaffolding audit. |
| `FSGOV-005` | Make the lifecycle executable in every affected per-game template. | five amended templates; explicit N/A review for primitive-pressure template | 003 | filled synthetic template packet proves pre-code audit, post-build registration, and prior-game disposition are all non-silent. |
| `FSGOV-006` | Add the versioned audit receipt and bounded historical bootstrap. | `ci/scaffolding-audits.json` | 003, 005 | all 17 existing games use frozen `legacy-8c-covered`; every pointer resolves; no future game can use that coverage. |
| `FSGOV-007` | Implement the receipt/register/tracker validator and high-confidence promoted-shape fingerprints. | checker, test file, synthetic fixtures | 006 | positive fixtures; failures for missing game, stale/missing paths, unknown IDs, unqueued prior site, invalid exception, and forbidden legacy claim; negative cases for intentional local/test repetition. |
| `FSGOV-008` | Wire the checker into Gate 1 and run the complete repository verification set. | Gate 1 workflow, any necessary package-script documentation | 007 | local checker pass; Node tests pass; existing Gate 1 checks pass; workflow syntax and ordering review. |
| `FSGOV-009` | Reconcile all documents, publish closeout evidence, and flip 8F to `Done` without admitting Gate 18 early. | Unit 8F spec Outcome, tracker, final doc links | 008 | exit-criteria matrix complete; no unauthorized code/data diff; Gate 18 remains `Not started`; exact URL/evidence provenance recorded for the implementation review. |

Task packets should carry the `Scaffold-Refactor Profile` as `Governance only`
or `not applicable`, not misuse it to imply a source migration. Every packet
must include the failing-test protocol already required by
`AGENT-DISCIPLINE.md`: first determine whether a failing check is valid, then
whether the fault is in the system under test or the test, then fix the correct
side. No task may weaken a checker fixture merely to close the series.

### 8.6 Draft exit criteria mapped to the three gaps

#### Gap 1 — reuse-first audit is standing and blocking

```markdown
- [ ] `FOUNDATIONS.md`, the official-game workflow, agent law, active tracker,
      and all affected new-game templates require a pre-implementation
      mechanical-scaffolding reuse-first audit.
- [ ] A `not applicable` audit requires a rationale and evidence link; omission
      is not accepted.
- [ ] Gate 1 fails when an official game lacks a valid audit record or when a
      post-8F game claims frozen legacy coverage.
- [ ] Known promoted-shape fingerprints fail when a parallel implementation has
      neither shared-helper use nor a valid register-backed exception.
```

#### Gap 2 — register-new is standing and closed at game completion

```markdown
- [ ] The register and official-game closeout require every newly invented
      behavior-free scaffolding shape to receive a first-use `candidate`,
      `local-only`, or `rejected` entry before the game closes.
- [ ] The mechanics and evidence templates name the register as a required
      update surface and carry a post-implementation freshness receipt.
- [ ] First-use registration is explicitly non-promotional and cannot place
      behavior from the Non-Promotion List into a scaffolding home.
- [ ] Gate 1 validates every cited register ID and every declared source/evidence
      path against committed files.
```

#### Gap 3 — prior-game refactoring is queued or explicitly disposed

```markdown
- [ ] The register, official-game contract, roadmap, tracker workflow, evidence
      template, and CI receipt all require prior matching official games to be
      named.
- [ ] Real characterization or migration work has a named bounded unit in
      `specs/README.md` before the new game closes.
- [ ] No-unit cases are accepted only through a register-backed `local-only`,
      `deferred`, or `rejected` decision with rationale, owner, evidence, and a
      next-review trigger.
- [ ] Gate 1 fails when a declared prior match has neither a tracker unit nor a
      valid no-unit disposition.
```

#### Cross-cutting preservation criteria

```markdown
- [ ] The exact behavioral third-use hard-gate sentence in
      `MECHANIC-ATLAS.md` §4 is byte-for-byte unchanged.
- [ ] ADR 0008's lane, allowed homes, Non-Promotion List, semantic-identity
      requirement, second-use review, and pre-third-copy decision threshold are
      unchanged.
- [ ] No game, helper, TypeScript behavior, trace, fixture, hash, RNG,
      serialization, visibility, no-leak, benchmark-threshold, or catalog entry
      changes as part of Unit 8F.
- [ ] `engine-core` remains noun-free and `game-test-support` remains dev-only.
- [ ] No YAML, DSL, selector, condition, trigger language, or runtime governance
      registry is introduced.
- [ ] Every byte/hash/visibility migration field in the new receipt either says
      `none` or cites ADR 0009 plus an explicit migration artifact.
- [ ] Gate 18 remains `Not started` until all Unit 8F criteria pass and the 8F
      tracker row is `Done`.
```

### 8.7 Acceptance evidence and commands

The final reassessed spec should pin exact tool versions/working directory where
needed, but this is the minimum command set:

```bash
node scripts/check-scaffolding-governance.mjs
node --test scripts/check-scaffolding-governance.test.mjs
node scripts/check-ci-games.mjs
node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
bash scripts/boundary-check.sh
cargo tree --workspace -e normal,build --invert game-test-support
git diff --check
```

Add targeted evidence beyond command success:

- a source comparison proving the behavioral third-use sentence is unchanged;
- a changed-file audit showing no `games/**`, runtime `crates/**`, `apps/web/**`,
  trace, fixture, benchmark-threshold, or generated-artifact mutation;
- a checker fixture in which a hypothetical new game reimplements a known
  promoted shape and fails;
- a checker fixture in which a legitimate behavior-bearing local shape is
  outside the fingerprint rule and passes with the correct register decision;
- a checker fixture in which a prior match names a nonexistent tracker unit and
  fails;
- a checker fixture in which a frozen legacy game points to its Unit 8C/R-wave
  receipt and passes; and
- a closeout matrix mapping every changed law/template/check to one of the three
  forward gaps.

### 8.8 Sequencing and admission

Unit 8F is the lowest non-`Done` row immediately after it is added. Its spec may
be authored from this plan, but Gate 18 must not be authored in parallel as a
way around the interlock. The intended sequence is:

```text
save advisory plan under reports/ (when the maintainer elects to commit it)
  -> author specs/pre-gate-18-forward-scaffolding-reuse-governance.md
  -> run /reassess-spec in place
  -> accept the corrected spec
  -> run /spec-to-tickets
  -> execute bounded Unit 8F tasks
  -> satisfy and record all exit criteria
  -> mark 8F Done
  -> author/reassess Gate 18 as the first forward-v1 game
```

Spades therefore exercises the mechanism rather than merely benefiting from a
later documentation cleanup. Its future spec must contain the first real
`forward-v1` audit, but Unit 8F must not pre-decide Spades-specific helper
adoption, exceptions, register entries, or retrofit units.

---

## 9. Recommended landing order and migration strategy

The amendments should land top-down in one governance unit, but not as one
opaque diff. This order makes contradictions fail early:

1. **Freeze baseline and safety fixtures.** Record the exact behavioral gate
   text, legacy game set, existing register IDs, lawful homes, Non-Promotion
   List, and no-change file boundaries.
2. **Extend the governing decision.** Append the dated ADR 0008 extension and
   update its affected-sections/migration matrix.
3. **Introduce constitutional stop conditions.** Amend `FOUNDATIONS.md`, then
   the architecture and engine/game/data boundary docs.
4. **Wire official workflow and registry lifecycle.** Amend the official-game
   contract, atlas seam, register cadence, and agent law.
5. **Admit the ladder interlock.** Amend testing law, roadmap, and
   `specs/README.md` before any lower-level template or CI rule is treated as
   authoritative.
6. **Make the obligation executable.** Amend the five affected templates while
   retaining the primitive-pressure ledger's behavioral-only boundary.
7. **Bootstrap evidence without rewriting history.** Add the 17 pointer-only
   legacy records. Do not manufacture modern `forward-v1` receipts for games
   that were governed by Unit 8C/R1–R4.
8. **Implement and test the tractable gate.** Land schema/set/link validation
   first, then high-confidence fingerprints, then Gate 1 wiring.
9. **Close the unit.** Run the full verification set, audit the changed-file
   surface, publish the Outcome, and mark 8F `Done` before Gate 18 authoring.

This is a governance migration, not a source migration. There is no rolling
runtime compatibility state and no reason for a feature flag. The only versioned
surface is the committed audit-receipt schema. Schema evolution should be
additive where possible; a breaking receipt-schema change needs a bounded
migration and checker support, not silent reinterpretation.

The legacy bootstrap must remain visibly historical. A future maintainer should
be able to tell the difference between “this game was closed by the 8C retrofit
program” and “this game passed the standing forward audit.” Allowing all games
to use one vague `covered` value would erase exactly the governance transition
Unit 8F is meant to establish.

---

## 10. Determinism, no-leak, and boundary impact statement

### 10.1 Behavioral authority

This plan adds no rule behavior and moves no rule authority. Rust remains the
sole owner of legality, transitions, outcome, visibility, replay state, and
deterministic effects. TypeScript/React remain presenters. The new CI receipt is
repository evidence metadata; it is not imported by a game, the WASM boundary,
or the web client.

### 10.2 Behavioral third-use gate

The behavioral gate remains word-for-word:

> | Third official game | Hard gate. The game MUST NOT proceed until a primitive-pressure ledger decides reuse, promotion, explicit deferral/rejection, or ADR. |

The scaffolding lane is parallel and narrower. It cannot be used to call a
behavior-bearing rule “plumbing,” bypass the primitive-pressure ledger, lower a
mechanic threshold, or justify a broad game-level framework.

### 10.3 Shared-code boundaries

- `engine-core` remains generic and noun-free. Only contract ergonomics with no
  game policy belong there.
- `game-stdlib` remains the narrow home for earned game-layer helpers admitted
  through the governing boundary and atlas/register process.
- `game-test-support` remains dev-only evidence infrastructure and may not enter
  production dependency closure.
- `wasm-api` remains a thin adapter and may not become a second rule engine.
- game-local behavior remains game-local, including every ADR 0008
  Non-Promotion List category.

A reuse-first audit may conclude “do not reuse” when semantic identity, type
ownership, visibility, or clarity does not support reuse. The obligation is to
make and record the decision—not to maximize shared code.

### 10.4 Determinism, replay, hashes, and fixtures

Unit 8F changes no execution path, RNG consumption, stable-byte writer call,
serialization shape, action-tree encoding, trace schema, replay export, fixture,
hash, or benchmark threshold. Legacy evidence pointers do not re-certify or
regenerate bytes.

Any future scaffolding adoption that changes bytes, hashes, fixture contents,
exports, RNG order, or serialization must be a separately scoped migration under
ADR 0009. Its audit receipt must cite that authority and the explicit migration
artifact. “The helper is cleaner” is never authority for silent regeneration.

### 10.5 Visibility and no-leak

The new lifecycle does not weaken viewer authorization, projection boundaries,
pairwise no-leak evidence, or private replay policy. A shared test harness may
reduce repeated test geometry; it may not replace game-specific viewer cases or
change which facts a viewer may observe. A proposed helper that owns
reveal/projection policy is behavior-bearing and belongs outside this lane.

### 10.6 Configuration and language surface

`ci/scaffolding-audits.json` is a finite, reviewed evidence receipt. It does not
select runtime behavior or express reusable conditions, triggers, predicates,
or game rules. No YAML, DSL, code generation, reflection framework, or dynamic
registry is authorized.

---

## 11. Risks, failure modes, and mitigations

| Risk | Why it matters | Required mitigation |
|---|---|---|
| CI overclaims semantic detection | Textual similarity cannot prove identical ownership, error, visibility, or deterministic semantics. | State the limit in testing law and checker output; use only high-confidence promoted-shape fingerprints; require a reviewed receipt for novel shapes. |
| The process becomes “reuse at any cost” | Forced abstraction can damage clarity and merge behavior with plumbing. | Preserve `local-only`, `deferred`, `rejected`, and exact-scope exceptions; require rationale/evidence rather than banning repetition. This is consistent with use-before-reuse and readable-test guidance.[^ext-rule-three][^ext-damp] |
| First-use registration bloats the register | Recording every tiny local helper would bury meaningful pressure. | Define a scaffolding shape as reusable lifecycle/infrastructure geometry, not any function; require a stable conceptual identity, boundary/exclusion statement, and next-review trigger. Trivial implementation details remain ordinary code. |
| Teams treat the receipt as paperwork | A complete JSON row can still lie about source semantics. | Require evidence paths, register links, reviewer ownership, and targeted fingerprints; make the receipt part of admission and closeout review, not an after-the-fact CI appeasement file. |
| Accepted exceptions become permanent escape hatches | Broad allowlists would restore silent duplication. | Exceptions are exact game/path/symbol/signal scoped, register-backed, owned, time/trigger bounded, and invalidated by drift. No environment bypass. |
| Follow-on units accumulate without execution | Auto-scheduling alone can convert duplication into stale debt. | Use existing promotion-debt/admission law: name dependency and blocking effect in the tracker; CI verifies existence, while the roadmap/tracker determine when debt blocks progression. |
| Legacy bootstrap is mistaken for a new audit | That would falsely claim evidence that was not produced under the forward process. | Freeze the 17-game set and use `legacy-8c-covered` with exact historical pointers; prohibit growth; require `forward-v1` for all later games. |
| Register and machine receipt drift apart | Two authorities could disagree about state. | The Markdown register remains the decision authority; the JSON receipt is a mechanically checked index that must resolve to existing register IDs and evidence. CI fails on mismatches. |
| Foundation and templates diverge | A template-only rule has no governing force; a foundation-only rule may be ignored in execution. | Land top-down, then cross-link every layer. The official-game contract owns the lifecycle; templates collect its receipts. |
| ADR history is rewritten | Editing old rationale as though it anticipated every later detail weakens decision provenance. | Append a dated extension rather than silently rewriting context/consequences. Use a successor ADR only if immutable-ADR policy requires it.[^ext-adr] |
| Golden path becomes stale | A supported path that does not match real work invites bypasses. | Give the register a mandatory per-game maintenance cadence and update the workflow/checker together when the accepted path changes.[^ext-golden-path] |

---

## 12. Recommendation and maintainer handoff

Adopt the plan as a seed for Unit 8F with these decisions held fixed:

1. extend ADR 0008 append-only by default;
2. introduce the obligation at constitution and boundary level before workflow,
   templates, and CI;
3. require first-use registration for meaningful behavior-free scaffolding but
   do not require first-use extraction;
4. define auto-scheduling as a mandatory tracker artifact or accepted
   register-backed no-unit disposition in the same game closeout;
5. enforce a versioned per-game receipt plus conservative known-shape
   fingerprints in Gate 1, without claiming general semantic clone detection;
6. freeze the 17-game Unit 8C/R1–R4 corpus as historical coverage and make Gate
   18 the first `forward-v1` game; and
7. prohibit any game/helper/byte/visibility migration inside the governance
   unit.

The immediate downstream artifact should be
`specs/pre-gate-18-forward-scaffolding-reuse-governance.md`, authored from this
plan and then reassessed in place. This document must not be copied wholesale
and marked `Planned`: the spec needs repository-current ticket boundaries,
owners, concrete base/changed-file evidence, test-fixture design, and final exit
wording established by `/reassess-spec`.

The sharpest test of the resulting law is simple: after Unit 8F, a reviewer
should be unable to admit a new game without seeing (a) what existing
scaffolding it audited and reused, (b) what new behavior-free shapes it added to
the register, and (c) which earlier games now require a named follow-on unit—or
why no such unit is justified. The CI check should make silence impossible,
while leaving the architecture decision itself reviewable and evidence-based.

---

## 13. Self-check against the locked brief

| Requirement | Result |
|---|---|
| Exactly one intermediate advisory change plan, not final repository law or a ticket-ready spec | Satisfied. The Unit 8F material is explicitly a seed requiring in-place `/reassess-spec` before `/spec-to-tickets`. |
| Implemented ADR 0008/register/third-use/8C/R1–R4/admission/evidence baseline acknowledged and not re-proposed | Satisfied in §§1–3 and throughout the amendment language. |
| Reuse-first audit closed | Satisfied in foundation stop conditions, official workflow, agent law, tracker/spec format, templates, receipt schema, CI, and Unit 8F exit criteria. |
| Register-new closed | Satisfied through first-use registration law, `GAME-MECHANICS` register update, post-build evidence receipt, register cadence, and CI ID/path validation. |
| Auto-schedule-refactor closed | Satisfied through queue-or-dispose law, mandatory tracker linkage, no-unit decision schema, CI failure semantics, and Unit 8F exit criteria. |
| Paste-ready amendments for every required doc/template | Satisfied in §5; required no-change files have explicit `Not applicable` dispositions and rationale. |
| Mechanical CI gate specified honestly | Satisfied in §6: receipt/register/tracker validation plus conservative known-shape fingerprints; arbitrary semantic equivalence is explicitly out of reach. |
| ADR mechanism selected with default and exact affected sections | Satisfied in §7: append-only ADR 0008 extension by default; successor ADR only under an immutable-ADR constraint. |
| Blocking pre-Gate-18 governance unit | Satisfied in §§5.10, 5.14, and 8; Gate 18 remains blocked until 8F is `Done`. |
| Behavioral third-use gate unchanged | Satisfied; exact text is preserved and made a Unit 8F comparison criterion. |
| Narrowest-layer-wins, noun-free `engine-core`, Non-Promotion List, no DSL/YAML | Satisfied in §§4–6, 8, and 10. |
| Determinism and no-leak preserved; ADR 0009 owns any future migration | Satisfied in §§5, 6, 8, and 10. |
| External claims that shape a decision cited | Satisfied through the primary/official sources below. |
| Repository claims use exact-commit evidence, with historical commit divergence flagged | Satisfied in the header, §2, and Appendix A. |

---

## External sources

[^ext-rule-three]: Ham Vocke, [“The Practical Test Pyramid”](https://martinfowler.com/articles/practical-test-pyramid.html), hosted by Martin Fowler. The article recommends “use before reuse” and invokes the Rule of Three as a restraint on premature test abstraction.

[^ext-damp]: Google Testing Blog, [“Tests Too DRY? Make Them DAMP!”](https://testing.googleblog.com/2019/12/testing-on-toilet-tests-too-dry-make.html). Used to calibrate intentional local repetition and readability exceptions, not to weaken production reuse obligations.

[^ext-golden-path]: Spotify Engineering, [“How We Use Golden Paths to Solve Fragmentation in Our Software Ecosystem”](https://engineering.atspotify.com/2020/08/how-we-use-golden-paths-to-solve-fragmentation-in-our-software-ecosystem). Used to support an explicit, maintained supported path rather than an unwritten specialist convention.

[^ext-adr]: Michael Nygard, [“Documenting Architecture Decisions”](https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions). Used to support preserving decision context and consequences through an append-only extension or a clearly linked successor.

---

## Appendix A — complete exact-commit acquisition ledger

The list below is append-only and contains every target-repository file URL used
for this analysis. All paths appeared exactly in the uploaded manifest, and all
requested files were successfully returned from the full exact-commit URL.

```text
Requested repository: joeloverbeck/rulepath
Target commit: 5ed1664de53eed9d51615786344905e3c05619d4
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run open(full exact raw URL); container.download(full exact raw URL) used for local retention where its MIME policy allowed
Requested file count: 73
Successfully verified file count: 73
Fetched repository files:
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/FOUNDATIONS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/ARCHITECTURE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/ENGINE-GAME-DATA-BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/OFFICIAL-GAME-CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/MECHANIC-ATLAS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/MECHANICAL-SCAFFOLDING-REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/AGENT-DISCIPLINE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/TESTING-REPLAY-BENCHMARKING.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/ROADMAP.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/adr/0008-mechanical-scaffolding-governance.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/adr/0009-replay-fixture-hash-taxonomy.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/docs/adr/ADR-TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/templates/README.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/templates/GAME-IMPLEMENTATION-ADMISSION.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/templates/GAME-MECHANICS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/templates/GAME-EVIDENCE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/templates/AGENT-TASK.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/templates/PRIMITIVE-PRESSURE-LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/reports/doc-and-template-overhaul-from-game-evidence-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/archive/specs/pre-gate-18-reuse-doctrine-and-evidence-realignment.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/archive/specs/8c-r4-n-seat-private-trick-scaffolding-intermediate-spec.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/reports/8c-r4-n-seat-private-trick-scaffolding-characterization.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/reports/8c-mechanical-scaffolding-characterization.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/reports/8c-r1-public-fixed-seat-scaffolding-characterization.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/scripts/boundary-check.sh
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/scripts/check-catalog-docs.mjs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/scripts/check-doc-links.mjs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/scripts/check-ci-games.mjs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/.github/workflows/gate-0-hygiene.yml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/.github/workflows/gate-1-game-smoke.yml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/.github/workflows/gate-2-benchmarks.yml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/ci/games.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/engine-core/src/action.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/engine-core/src/game.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/engine-core/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/engine-core/src/replay.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/engine-core/src/rng.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/game-stdlib/src/board_space.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/game-stdlib/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/game-stdlib/src/seat.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/game-stdlib/src/trick_taking.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/game-test-support/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/game-test-support/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/game-test-support/src/no_leak.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/crates/game-test-support/src/profiles.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/docs/GAME-IMPLEMENTATION-ADMISSION.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/docs/MECHANICS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/actions.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/bots.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/cards.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/effects.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/ids.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/replay_support.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/rules.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/scoring.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/setup.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/variants.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/src/visibility.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/tests/replay.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/tests/rules.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/tests/serialization.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/5ed1664de53eed9d51615786344905e3c05619d4/games/vow_tide/tests/visibility.rs
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

### Acquisition result

```text
Requested repository: joeloverbeck/rulepath
Target commit: 5ed1664de53eed9d51615786344905e3c05619d4
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: web.run open(full exact raw URL); container.download(full exact raw URL) for local retention where permitted
Requested file count: 73
Successfully verified file count: 73
Fetch-provenance contamination observed: no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
Substantive analysis began before acquisition completed: no
```
