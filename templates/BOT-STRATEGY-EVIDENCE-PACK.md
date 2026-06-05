# Bot Strategy Evidence Pack

Game ID: `<game_id>`

Variant: <variant>

Bot target: Level 2 authored policy

Prepared by: <name/agent>

Date: YYYY-MM-DD

## 1. Sources consulted

| Source | URL | Date consulted | Used for | Copied prose? |
|---|---|---|---|---|
| <source> | <URL> | YYYY-MM-DD | strategy/rules/variant context | none |

## 2. Competent-player principles

Summarize practical principles in original language.

- <principle>

## 3. Tactical priorities

Order matters. Prefer lexicographic priorities over weighted soup.

1. <priority>
2. <priority>
3. <priority>
4. seeded deterministic tie-break

## 4. Phase model

| Phase / situation | Policy focus | Notes |
|---|---|---|
| <phase> | <focus> | <notes> |

## 5. Candidate features

| Feature | Visible to bot? | Used for | Hidden-info risk |
|---|---:|---|---|
| <feature> | yes/no | <decision/tie-break/explanation> | <risk> |

## 6. Lexicographic ranking plan

| Rank slot | Meaning | Higher is better? | Tests |
|---|---|---:|---|
| 1 | <category> | yes/no | <tests> |
| 2 | <category> | yes/no | <tests> |
| 3 | deterministic tie-break | yes | <tests> |

## 7. Permitted tie-breakers

Allowed:

- small bounded scores with documented meaning;
- deterministic seeded tie-break;
- style profile tie-breaks after mandatory/terminal priorities.

Forbidden:

- actual hidden state;
- future random outcomes;
- unbounded magic weights;
- static data conditions that become behavior;
- random blunder injection by default.

## 8. Forbidden hidden information

| Information | Why forbidden | No-leak test |
|---|---|---|
| <info> | unavailable to seat | <test> |

## 9. Decision examples

| Situation | Candidate choices | Expected choice | Explanation |
|---|---|---|---|
| <situation> | <choices> | <choice> | <explanation> |

## 10. Explanation examples

Public “why?” examples:

- <example>

Dev-mode ranking examples:

- <example>

## 11. Known weaknesses

- <weakness>

These weaknesses are acceptable because:

- <rationale>

## 12. Test plan

- legality over many seeds;
- determinism for fixed seed/view/limits;
- priority unit tests;
- explanation smoke tests;
- simulation/fuzz tests;
- no-leak tests if hidden information exists;
- regression tests for listed decision examples.

## 13. Benchmark plan

| Benchmark | Target | Notes |
|---|---:|---|
| legal action generation | <target> | <notes> |
| candidate extraction | <target> | <notes> |
| decision latency | <target> | <notes> |
| playout throughput | <target> | <notes> |

## 14. Public UX note

Describe how the public UI should expose the bot's recent decision or “why?” explanation without turning the game into a debug console.
