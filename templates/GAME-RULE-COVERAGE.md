# <game_id> Rule Coverage

Game ID: `<game_id>`

Rules version: <version>

Last updated: YYYY-MM-DD

Every rule in `RULES.md` must be mapped here. Silent gaps are not allowed.

| Rule section | Summary | Implementation | Unit tests | Rule tests | Golden traces | Property / simulation coverage | UI / replay coverage | Notes |
|---|---|---|---|---|---|---|---|---|
| 1 | purpose/scope | <module> | <tests> | <tests> | <traces> | <coverage> | <coverage> | <notes> |
| 2 | components/state vocabulary | <module> | <tests> | <tests> | <traces> | <coverage> | <coverage> | <notes> |
| 3 | setup | <module> | <tests> | <tests> | <traces> | <coverage> | <coverage> | <notes> |
| 4 | turn sequence | <module> | <tests> | <tests> | <traces> | <coverage> | <coverage> | <notes> |
| 5 | legal actions | <module> | <tests> | <tests> | <traces> | <coverage> | <coverage> | <notes> |
| 6 | forced actions/restrictions | <module> | <tests> | <tests> | <traces> | <coverage> | <coverage> | <notes> |
| 7 | scoring | <module> | <tests> | <tests> | <traces> | <coverage> | <coverage> | <notes> |
| 8 | terminal conditions | <module> | <tests> | <tests> | <traces> | <coverage> | <coverage> | <notes> |
| 9 | visibility | <module> | <tests> | <tests> | <traces> | <coverage> | <coverage> | <notes> |
| 10 | bot strategy notes | `bots` docs | <tests> | <tests> | <traces> | <simulation> | <explanations> | <notes> |
| 11 | ambiguities | <module/docs> | <tests> | <tests> | <traces> | <coverage> | <coverage> | <notes> |

## Known gaps

| Gap | Status | Rationale | Required before release? |
|---|---|---|---|
| <gap> | not applicable / intentionally deferred / unsupported / open question | <rationale> | yes/no |

## Golden trace index

| Trace | Purpose | Rules covered | Update policy |
|---|---|---|---|
| <trace file> | <purpose> | <rules> | preserve / update only with note |

## Coverage review checklist

- Every `RULES.md` section has a row.
- Every omitted rule is explicitly marked.
- Golden traces cover normal play, terminal state, invalid/stale diagnostics, bot action, and hidden/stochastic cases where applicable.
- Property/simulation tests cover broad invariants.
- UI smoke exists once web-exposed.
