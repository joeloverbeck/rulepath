# Primitive Pressure Ledger

Candidate name: <candidate primitive>

Status: local-only | repeated-shape candidate | extraction required | promoted primitive | rejected/deferred with rationale | ADR-required

Last updated: YYYY-MM-DD

## Mechanic shape

Describe the repeated mechanic shape in prose.

Do not describe a universal behavior language. Do not use game-specific brand names.

## Games exerting pressure

| Game | Ladder stage | Local implementation | Pressure type | Notes |
|---|---:|---|---|---|
| <game> | <stage> | <module/helper> | first / second / third / benchmark / bug | <notes> |

## Local implementations compared

| Aspect | Game A | Game B | Game C | Same shape? |
|---|---|---|---|---:|
| state shape | <notes> | <notes> | <notes> | yes/no |
| action shape | <notes> | <notes> | <notes> | yes/no |
| validation | <notes> | <notes> | <notes> | yes/no |
| effects | <notes> | <notes> | <notes> | yes/no |
| visibility | <notes> | <notes> | <notes> | yes/no |
| UI pattern | <notes> | <notes> | <notes> | yes/no |
| bot use | <notes> | <notes> | <notes> | yes/no |
| benchmark pressure | <notes> | <notes> | <notes> | yes/no |

## Similarities

- <similarity>

## Differences

- <difference>

## Extraction decision

Decision: reuse / promote / defer / reject / ADR-required

Rationale:

- <rationale>

## Rejected alternatives

| Alternative | Why rejected |
|---|---|
| <alternative> | <reason> |

## API sketch in prose only

Describe the narrow typed helper without writing implementation code.

- Inputs:
- Outputs:
- Error/diagnostic behavior:
- Determinism requirements:
- Visibility requirements:
- Non-goals:
- Examples:
- Anti-examples:

## Tests required

| Test | Required before promotion? | Notes |
|---|---:|---|
| unit tests | yes | <notes> |
| compatibility tests in each back-ported game | yes | <notes> |
| golden trace preservation/update notes | yes | <notes> |
| property tests | yes/no | <notes> |
| visibility/no-leak tests | if relevant | <notes> |
| benchmark tests | yes | <notes> |

## Traces affected

| Trace | Preserve or update? | Reason |
|---|---|---|
| <trace> | preserve/update | <reason> |

## Benchmarks affected

| Benchmark | Expected impact | Required threshold |
|---|---|---:|
| <benchmark> | <impact> | <threshold> |

## Examples

Good fits:

- <example>

## Anti-examples

Not a fit:

- <anti-example>

## ADR need

ADR required? yes/no

Reason:

- <reason>

## Review checklist

- Third-game hard gate satisfied.
- No game noun enters `engine-core`.
- Helper belongs in `game-stdlib` or stays local.
- No untyped behavior language is created.
- Existing games are back-ported if promoted.
- Traces are preserved or intentionally updated.
- Benchmarks are measured.
- Examples and anti-examples are documented.
