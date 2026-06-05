# <game_id> AI

Game ID: `<game_id>`

Rules version: <version>

Last updated: YYYY-MM-DD

## Bot summary

| Bot | Level | Public default? | Information access | Status |
|---|---:|---:|---|---|
| random legal | 0 | no/yes | legal action tree only | required |
| baseline | 1 | no/yes | allowed seat view | <status> |
| authored policy | 2 | no/yes | allowed seat view | requires strategy evidence pack |
| shallow search | 3 | no/yes | allowed seat view | small perfect-information only |

## Level 0: random legal bot

- Legal action API used:
- Deterministic seed behavior:
- Simulation tests:
- Known limitations:
- Explanation text:

## Level 1: rule-informed baseline bot

- Policy name/version:
- Decision order:
- Immediate tactics:
- Mandatory rule handling:
- Tie-break method:
- Explanation examples:
- Tests:
- Benchmarks:

## Level 2: authored policy bot

Required evidence pack: `BOT-STRATEGY-EVIDENCE-PACK.md` instance at <path>.

- Policy name/version:
- Phase model:
- Candidate extraction:
- Tactical priorities:
- Lexicographic ranking plan:
- Bounded scoring tie-breakers:
- Deterministic seeded tie-break:
- Explanation contract:
- Public default suitability:

## Information access

| Information | Human seat sees? | Bot sees? | Notes / tests |
|---|---:|---:|---|
| <information> | yes/no | yes/no | <notes> |

Bots MUST NOT receive actual hidden information unavailable to the acting seat.

## Decision order

1. <priority>
2. <priority>
3. <fallback>

## Style profiles

One strong default bot comes first. Optional style profiles MAY be added later.

| Profile | Policy variation | Hidden-info safe? | Status |
|---|---|---:|---|
| <profile> | <variation> | yes/no | <status> |

## Explanations

| Situation | Example explanation | Hidden-info safe? |
|---|---|---:|
| <situation> | <explanation> | yes |

Public mode MAY show a small “why?” affordance or recent-bot-action explanation. Full candidate ranking is dev-mode only.

## Known weaknesses

- <weakness>

Do not hide weaknesses behind magic weights. Document them.

## Tests

| Test | Purpose | Status |
|---|---|---|
| legality over seeds | bot chooses only legal action paths | <status> |
| determinism | fixed seed/view/limits produce fixed decision | <status> |
| explanation smoke | non-random bots explain decisions | <status> |
| no-leak view | hidden-info games only | <status> |
| no-leak explanation/ranking | hidden-info games only | <status> |

## Benchmarks

| Benchmark | Target | Current baseline | Notes |
|---|---:|---:|---|
| legal action generation | <target> | <baseline> | <notes> |
| bot decision latency | <target> | <baseline> | <notes> |
| playout throughput | <target> | <baseline> | <notes> |
