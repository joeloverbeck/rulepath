# <game_id> Event Coverage Matrix

Game ID: `<game_id>`

Rules version: `<rules_version>`

Data/manifest version: `<data_or_manifest_version>`

Engine version: `<engine_version>`

Prepared by: `<name/agent>`

Last updated: YYYY-MM-DD

Evidence receipt: [`GAME-EVIDENCE.md`](GAME-EVIDENCE.md)

Rule coverage index: [`GAME-RULE-COVERAGE.md`](GAME-RULE-COVERAGE.md)

Template realignment mapping: report `B-05 -> GAME-EVENT-COVERAGE.md`.

## Purpose

This matrix records coverage for large card/event effect sets without turning
coverage into executable behavior. It uses placeholder event IDs and evidence
IDs only. Public templates MUST NOT include private licensed card text, event
names, rules prose, screenshots, fixture names, e2e names, catalog strings, or
source expression.

Event behavior remains typed Rust. This matrix may cite private event IDs,
branch kinds, source receipt IDs, Rust behavior owners, named tests, traces,
visibility evidence, replay/hash evidence, and status. It MUST NOT encode
selectors, conditions, triggers, target filters, rule overrides, effect
formulas, legality, state transitions, visibility policy, or bot tactics.

## Status labels

| Status | Meaning |
|---|---|
| covered | Rust implementation and required evidence exist. |
| partial | Some branches or proof surfaces remain incomplete. |
| not started | Known event branch; no implementation/evidence yet. |
| intentionally deferred | Deferred by documented milestone/profile decision. |
| unsupported | Explicitly not implemented for this variant/profile. |
| not applicable | Truly not applicable, with rationale. |
| blocked | Cannot proceed until stated blocker is resolved. |

## Event-effect coverage matrix

Use stable evidence IDs so `GAME-EVIDENCE.md`, private release checklists, and
future tools can link to proof without copying private card text.

| Event evidence ID | Private event ID | Branch kind | Behavior owner | Source receipt/private reference | Named tests | Golden traces / replay fixtures | Visibility/no-leak evidence | Replay/hash impact | Status | Notes |
|---|---|---|---|---|---|---|---|---|---|---|
| `EVT-001` | `<private_event_id_or_placeholder>` | operation / special / event branch / persistent effect / temporary effect / cleanup / not applicable | `<Rust module/function/match arm/trait>` | `<private source id or not applicable>` | `<tests>` | `<traces/fixtures>` | `<LEAK-* or not applicable>` | preserve / private migration / not applicable | not started | `<notes>` |

## Branch taxonomy

| Branch kind | Required proof |
|---|---|
| target choice | Rust-owned target selection, invalid-target diagnostic, no hidden target leakage. |
| conditional branch | Rust-owned condition evaluation, both true/false branch evidence when reachable. |
| rule override | Named rule IDs affected, expiry/reset evidence, replay/hash note. |
| persistent effect | Creation, duration, expiry, serialization, visibility, and cleanup evidence. |
| temporary effect | Scope, single-use or phase expiry, and diagnostic evidence. |
| free/bonus action | Legal action tree ownership, validation, sequencing, and replay evidence. |
| periodic round interaction | Setup/round/reset/victory interaction and deterministic trace evidence. |

## Deferred, unsupported, or not applicable events

| Event evidence ID or range | Status | Rationale | Evidence required later | Required before private release candidate? | Owner |
|---|---|---|---|---:|---|
| `<EVT-range>` | intentionally deferred / unsupported / not applicable | `<rationale>` | `<future evidence>` | yes/no | `<owner>` |

## Coverage review checklist

- Every private event ID or event branch in scope has one row or a documented
  deferred/not-applicable range.
- Rows use placeholder IDs or private-source receipt IDs, not copied licensed
  event names or card text.
- Behavior owner is Rust code, never YAML, JSON, TOML, RON, CSV, markdown, or
  table-row behavior.
- Selectors, conditions, triggers, target filters, rule overrides, and effect
  formulas are not encoded in this file.
- Unknown fields in any machine-readable derivative are rejected by default.
- Visibility/no-leak evidence covers public observer, each relevant seat, replay
  export, diagnostics, effect logs, DOM/test IDs, and bot explanations where
  applicable.
- Replay/hash impact is recorded for every branch that changes command streams,
  state bytes, effect bytes, public/private views, or exports.
- `GAME-RULE-COVERAGE.md` links this matrix and keeps the stable rule-to-proof
  index current.
