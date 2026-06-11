# Flood Watch Competent Player Analysis

Game ID: `flood_watch`

Implemented variants: `flood_watch_standard`, `flood_watch_deluge`

Rules version checked: `flood-watch-rules-v1`

Date: 2026-06-11

## Purpose and Authority

This is strategy analysis for the implemented Flood Watch variants. It
documents competent cooperative play and the Level 1 bot posture. It does not
define rules. If this document conflicts with [RULES.md](RULES.md), the rules
win.

## Sources and References

| Source/reference | Date consulted | Source quality | Used for | Copied prose status | Notes |
|---|---|---|---|---|---|
| [RULES.md](RULES.md) | 2026-06-11 | rules authority | turn flow, event resolution, terminal conditions, bot boundaries | none | Local Rulepath prose. |
| [SOURCES.md](SOURCES.md) | 2026-06-11 | source and IP record | original-design boundaries | none | Strategy examples are Rulepath-authored. |
| [bots.rs](../src/bots.rs) | 2026-06-11 | implementation evidence | Level 0 and Level 1 policy behavior | none | Rust owns bot choices. |
| [bots.rs tests](../tests/bots.rs) | 2026-06-11 | executable evidence | legality, determinism, hidden-order invariance, rationale no-leak | none | Native bot suite. |
| [BENCHMARKS.md](BENCHMARKS.md) | 2026-06-11 | benchmark posture | playout smoke and calibration status | none | Win-rate calibration is a named follow-up after simulator registration. |

## Competent-Player Summary

Competent Flood Watch play is role-efficient cooperation under event pressure:

- spend the active seat's budget through the Rust legal action tree only;
- bail districts at level 2 before they can become shared losses;
- use the Pumpwright's two-level bail power to recover flooded districts;
- use the Levee Warden's two-levee reinforce power to prepare high-pressure
  districts;
- treat a public forecast as the next concrete threat and mitigate it before
  lower-priority improvements;
- use remaining-composition counts as pressure estimates, never as deck-order
  knowledge;
- forecast with spare budget when no public district action improves the board;
- end the turn when no legal public action improves the position.

## Situations

| Situation | What competent players notice | Important rules | Notes |
|---|---|---|---|
| Active action phase | Remaining budget, active role, public district levels/levees, forecast, composition counts. | `FW-ACT-001` through `FW-ACT-006` | The teammate waits; TypeScript does not synthesize actions. |
| District at level 2 | One unabsorbed rise causes shared loss. | `FW-END-001`, `FW-ENV-005` | Bail or reinforce the specific threat before spending on lower-pressure work. |
| Public forecast | The next card kind is public to both seats and observers. | `FW-ACT-004`, `FW-ENV-003` | Forecast card identity is allowed public information; the rest of the deck remains hidden. |
| High expected pressure | Remaining composition has more downpours/surges for a district. | `FW-VIS-004`, `FW-BOT-002` | Count remaining public composition, not order. |
| Terminal | Outcome is shared: team won or team lost. | `FW-END-001` through `FW-END-003` | There are no per-seat winners or tiebreaks. |

## Level 1 Bot Mapping

The implemented Level 1 policy in [bots.rs](../src/bots.rs) follows this
priority order:

1. Rescue level-2 districts with legal bail actions.
2. Mitigate a public forecast that would otherwise inundate a district.
3. Reinforce the district with the highest public expected pressure.
4. Forecast with spare budget.
5. End turn when no public legal action improves the position.

Stable district order breaks ties. The bot consumes the public projection,
remaining-composition counts, the legal action tree, and its declared seed. It
does not read the undrawn deck order.

## Balance Evidence and Calibration

Current executable evidence:

| Evidence | Command or file | Status |
|---|---|---|
| Bot legality, determinism, both seats/roles, hidden-order invariance | `cargo test -p flood_watch --test bots` | passing |
| Full package rule/replay/visibility/bot suite | `cargo test -p flood_watch` | passing |
| Legal cooperative playout smoke | `cargo bench -p flood_watch -- random_playout` | passing through the benchmark harness |
| Simulator win-rate evidence | `cargo run -p simulate -- --game flood_watch --games 1000` | pending GAT12FLOWATCOO-015 registration |

The target Level 1 + Level 1 standard-scenario band is approximately 35-75%.
Because `simulate` registration is owned by GAT12FLOWATCOO-015, no numeric
win-rate is claimed here. Once registration lands, an out-of-band result must
trigger a scenario-constant retune before public polish, recorded here and in
[BENCHMARKS.md](BENCHMARKS.md).

## Review Checklist

- Strategy prose is original.
- Rules authority is separate from strategy.
- Hidden-information boundaries are explicit.
- No strategy claim requires MCTS, ISMCTS, Monte Carlo, ML, RL, hidden-state
  sampling, or undrawn-deck peeking.
