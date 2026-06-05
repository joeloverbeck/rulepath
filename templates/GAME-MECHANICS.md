# <game_id> Mechanics Inventory

Game ID: `<game_id>`

Ladder stage: <stage number and name>

Rules version: <version>

Last updated: YYYY-MM-DD

## Mechanic tags

Check all that apply and add notes.

| Category | Tags / notes |
|---|---|
| topology / spatial model | <none / grid / graph / track / custom> |
| component / zone model | <pieces / cards / decks / hands / public zones / private zones> |
| action shape | <flat / action tree / progressive / simultaneous / reaction> |
| turn / phase model | <alternating / phases / rounds / interrupts / cleanup> |
| randomness / chance | <none / setup shuffle / draws / event samples> |
| visibility / hidden information | <perfect / private hands / commitments / roles / redacted logs> |
| resource / accounting | <none / counters / payments / pots / score economy> |
| movement / capture / placement | <placement / movement / capture / flip / promotion> |
| pattern / line / directional scanning | <none / lines / scanning / adjacency> |
| commitment / reveal | <none / simultaneous / delayed reveal> |
| reaction / window / pending response | <none / pending player / challenge / interrupt> |
| scoring / outcome | <points / terminal line / shared win/loss / asymmetric victory> |
| semantic effect shape | <placement / movement / reveal / grouped effects / automation> |
| UI interaction pattern | <direct click / drag optional / progressive construction / replay-heavy> |
| bot policy pattern | <random / baseline / authored policy / shallow search> |
| benchmark / performance pressure | <hot paths and targets> |

## Repeated-shape check

| Mechanic shape | Already appears in | Same shape? | Notes |
|---|---|---:|---|
| <shape> | <games> | yes/no/unclear | <comparison> |

## Primitives reused

| Primitive | Source | Why reused | Tests proving compatibility |
|---|---|---|---|
| <primitive> | `game-stdlib` / local helper | <reason> | <tests> |

## Local mechanics

| Local mechanic | Why local | Extraction risk | Tests |
|---|---|---|---|
| <mechanic> | <reason> | <low/medium/high> | <tests> |

## Primitive candidates

| Candidate | Status | Games exerting pressure | Required next step |
|---|---|---|---|
| <candidate> | local-only / repeated-shape candidate / extraction required / promoted primitive / rejected-deferred / ADR-required | <games> | <next step> |

## Extraction or defer rationale

Explain why each repeated shape is staying local, being reused, being promoted, being deferred, or requiring ADR.

## Effects / UI / bot mechanic notes

- Semantic effects required:
- UI interaction pattern:
- Bot policy pattern:
- Visibility and no-leak considerations:
- Benchmark pressure:

## Repo-level atlas update instructions

Update `docs/MECHANIC-ATLAS.md` and any `templates/PRIMITIVE-PRESSURE-LEDGER.md` instance when:

- this game repeats a mechanic shape from another official game;
- this game is the third official game with the same mechanic shape;
- a primitive is reused, promoted, rejected, or deferred;
- traces, benchmarks, or examples/anti-examples change.
