# Ticket Authoring Contract

This directory contains active implementation tickets.

To keep architecture clean, robust, and extensible, every new ticket must be created from `tickets/_TEMPLATE.md` and must satisfy the checks below.

## Core Architectural Contract

1. No backwards-compatibility shims or alias paths in new work.
2. If current code and ticket assumptions diverge, update the ticket first before implementation.
3. `docs/FOUNDATIONS.md` is the non-negotiable design contract. Tickets must align with its §2 behavior authority, §3 `engine-core` contract-kernel boundary, §4 `game-stdlib`-is-earned rule, §5 static-data-is-not-behavior rule, §11 universal acceptance invariants, §12 stop conditions, and §13 ADR triggers.

## Required Ticket Sections

1. `Assumption Reassessment (YYYY-MM-DD)`:
   - Validate ticket assumptions against current code, skills, and specs.
   - Explicitly call out mismatches and corrected scope.
   - Cite exact files, symbols, crate/module names, or schema fields for any non-trivial architectural claim.
   - For cross-crate or cross-artifact tickets, name the exact shared boundary, contract, or schema under audit before implementation.
   - Classify newly exposed adjacent contradictions as required consequences of the intended change, separate bugs uncovered during reassessment, or future cleanup that must become its own ticket.
   - For tickets touching FOUNDATIONS-aligned enforcement surfaces (acceptance invariants, fail-closed validation, the hidden-information no-leak firewall, deterministic replay/hash, the third-use mechanic hard gate), restate the FOUNDATIONS principle under audit before trusting the spec narrative.
2. `Architecture Check`:
   - Explain why the proposed design is cleaner than alternatives.
   - State that no backwards-compatibility aliasing/shims are introduced.
   - Confirm `engine-core` stays free of mechanic nouns and `game-stdlib` changes are earned via the mechanic atlas.
3. `Verification Layers`:
   - Required for any cross-crate or cross-artifact ticket.
   - Map each important invariant to the exact verification surface that proves it.
   - Use one line per invariant. Valid verification surfaces for Rulepath tickets:
     - codebase grep-proof (symbol existence, rename/removal confirmation, schema field presence)
     - schema/serialization validation (action-tree / command-envelope / effect-envelope / public-private-view conformance against `docs/ARCHITECTURE.md` and the boundary in `docs/ENGINE-GAME-DATA-BOUNDARY.md`; stable serialization order)
     - golden trace / deterministic replay-hash check (per `docs/TESTING-REPLAY-BENCHMARKING.md`)
     - no-leak visibility test (hidden information does not reach payloads, DOM, logs, previews, effect logs, bot explanations, candidate rankings, or replay exports)
     - bot legality check (bot uses the normal legal action API and allowed views only)
     - simulation/CLI run (game driven end-to-end via the simulation CLI; deliverable inspected without commit)
     - benchmark check (per `docs/TESTING-REPLAY-BENCHMARKING.md`)
     - FOUNDATIONS alignment check (principle, invariant, or stop condition cited by section)
     - skill/template dry-run (skill invoked with a representative input; deliverable inspected without commit)
     - manual review (prose quality, UI play-first audit, IP-conservatism audit)
   - Do not collapse multiple layers into one generic "validation" or "review" surface.
4. `Tests`:
   - List new/modified tests, traces, validators, or dry-run commands and rationale per test.
   - Include targeted and full-pipeline verification commands.
   - Commands must be copy-paste runnable against real crate/module paths, real file paths, or real validation/simulation commands, not approximate filters.

## Mandatory Pre-Implementation Checks

1. Dependency references point to existing repository files (active or archived paths are both valid when explicit).
2. Type, schema, and data-contract references match current code.
3. Files-to-touch list matches current file layout and ownership.
4. Scope does not duplicate already-delivered architecture.
5. Test/verification commands have been dry-run checked or verified against the current pipeline layout.
6. Claimed helper, type, or function usage is verified against the exact current symbol location, not inferred from a similarly named artifact elsewhere in the repo.
7. For cross-crate or cross-artifact tickets, confirm the intended invariant, the exact shared boundary under audit, and whether adjacent contradictions belong to this ticket or a follow-up before implementation begins.
8. For information-path refactors (where the same fact is currently transported through multiple paths), confirm whether current code still has multiple lawful transport paths for the same fact, name the canonical end-state path, and verify that the planned proof surface remains strong enough to debug that canonical path after the change.
9. For tickets touching the third-use mechanic hard gate (§4), fail-closed acceptance invariants (§11), or deterministic replay/hash & serialization surfaces, verify the change does not leak hidden information (FOUNDATIONS §11 no-leak firewall) or break deterministic replay/hash (§11/§13).
10. For tickets extending an existing schema or contract (action tree, command/effect envelope, public/private view, golden trace, checkpoint, serialized save, static-data manifest entry), verify consumers of that schema have been updated, or the extension is additive-only (new optional field with a default).

## Archival Reminder

Follow `docs/archival-workflow.md` as the canonical archival process.
