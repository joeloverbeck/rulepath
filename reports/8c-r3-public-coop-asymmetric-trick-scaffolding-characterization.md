# Unit 8C-R3 Characterization Baseline

Date: 2026-06-24
Baseline commit: `b0be7a4157f8`
Ticket: `archive/tickets/8CR3PUBCOOASY-001.md`
Reference: `specs/8c-r3-public-coop-asymmetric-trick-scaffolding.md`

This report is the pre-migration baseline for Unit 8C-R3. It records current
surfaces only. It does not authorize any code, byte, hash, visibility, seat-ID,
RNG, fixture, golden-trace, or export change. Later tickets must cite this file
or append before/after receipts before migrating a selected surface.

## Authority And Determination

- `specs/README.md` records `8C-R2` as `Done` and `8C-R3` as the next public
  scaling unit. `8C-R4` and Gate 18 remain successor work.
- `docs/MECHANIC-ATLAS.md` section 10A records no open promotion debt. This is
  a mechanical-scaffolding retrofit, not a behavioral mechanic promotion.
- `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` seeds
  exactly the R3 game set: `plain_tricks`, `flood_watch`,
  `frontier_control`, and `event_frontier`.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` contains governing entries
  `MSC-8C-001` through `MSC-8C-010`. R3 appends receipts beneath those entries;
  it does not create rival register entries.
- None of the four R3 games is named as an 8C/C-08 pilot in the parent, R1, or
  R2 characterization material. There is no pilot-credit verdict in this unit.

## Four-Game Inventory

| Game | Cargo pre-state | Visibility shape | RNG setup shape | R3 role |
|---|---|---|---|---|
| `plain_tricks` | normal `engine-core`, `ai-core`, `game-stdlib`; no `game-test-support` | private hands plus public observer and seat viewers | local unbiased shuffle in `src/setup.rs` | trick-taking private-hand retrofit |
| `flood_watch` | normal `engine-core`, `ai-core`; no `game-stdlib`; no `game-test-support` | public/cooperative view with hidden future event-deck tail | local unbiased event-deck shuffle | cooperative public-observer retrofit |
| `frontier_control` | normal `engine-core`, `ai-core`; no `game-stdlib`; no `game-test-support` | fully public graph/faction game | RNG-free setup | asymmetric fully-public retrofit |
| `event_frontier` | normal `engine-core`, `ai-core`, `game-stdlib`; no `game-test-support` | public current/next events with hidden deeper deck tail | local unbiased epoch/deck shuffle | asymmetric event retrofit |

`rg -n "game-test-support" games/{plain_tricks,flood_watch,frontier_control,event_frontier}/Cargo.toml`
returned no matches at admission.

## Verdict Matrix

| Game | C-01 effects | C-02 seats | C-03 setup counts | C-04 tree v1 | C-05 writer v1 | C-06 dev support | C-07 no-leak | C-08 profiles | C-09 RNG | C-10 policy |
|---|---|---|---|---|---|---|---|---|---|---|
| `plain_tricks` | migrate public/private | migrate typed parser; WASM import aliases excepted | migrate roster; variant count N/A; range/ring N/A | migrate parallel surface | migrate only through action-tree v1 | migrate dev-only | migrate full private matrix | migrate replay/setup/domain/public-export/seat-private-export | migrate local unbiased sampler | local-only |
| `flood_watch` | migrate public; private N/A | shared WASM import/output exception | migrate roster, variant count, role-order count | migrate parallel surface | migrate only through action-tree v1 | migrate dev-only | migrate hidden future-deck matrix | migrate replay/setup/domain/public-export; seat-private-export N/A | migrate local unbiased sampler | local-only |
| `frontier_control` | migrate public; private N/A | shared WASM import/output exception | migrate roster and variant count; faction order exception | migrate parallel surface | migrate only through action-tree v1 | migrate dev-only | N/A plus public equality receipt | migrate replay/setup/domain/public-export; seat-private-export N/A | N/A, setup is RNG-free | local-only |
| `event_frontier` | migrate public; private N/A | shared WASM import/output exception | migrate roster and variant count; faction order exception | migrate parallel surface | migrate only through action-tree v1 | migrate dev-only | migrate hidden deeper-deck matrix | migrate replay/setup/domain/public-export; seat-private-export N/A | migrate local unbiased sampler | local-only |

## C-01 Effect Constructors

Current public constructors are local `EffectEnvelope` literals:

| Game | Current public owner | Current private owner | Verdict |
|---|---|---|---|
| `plain_tricks` | `games/plain_tricks/src/effects.rs::public_effect` | `games/plain_tricks/src/effects.rs::private_effect` | migrate public and private independently |
| `flood_watch` | `games/flood_watch/src/effects.rs::public_effect` | none | migrate public; private N/A |
| `frontier_control` | `games/frontier_control/src/effects.rs::public_effect` | none | migrate public; private N/A |
| `event_frontier` | `games/event_frontier/src/effects.rs::public_effect` | none | migrate public; private N/A |

Migration invariant: only envelope assembly may move to
`EffectEnvelope::public` / `EffectEnvelope::private_to`. Payloads, ordering,
recipient selection, reveal policy, filters, effect text, effect hashes,
replay checkpoints, and export bytes remain game-owned and unchanged.

## C-02 Seat Grammar And Boundary

| Surface | Current owner | Verdict |
|---|---|---|
| Plain typed parser | `games/plain_tricks/src/ids.rs::PlainTricksSeat::parse` matches `seat_0` and `seat_1` manually | migrate to strict `SeatId::parse_canonical` then map indices 0/1 |
| Plain WASM import | `crates/wasm-api/src/seats.rs::parse_plain_seat` | exception; import-only aliases remain bounded and output remains canonical |
| Flood/Frontier/Event WASM import | `parse_flood_seat`, `parse_frontier_seat`, `parse_event_frontier_seat` | exception; no game-local typed seat parser exists |
| Non-seat IDs | `TrickCardId`, `DistrictId`, `SiteId`, `EventKind`, `FactionId`, card/event IDs | excluded; not C-02 |

Canonical acceptance/rejection vectors for C-02 follow the spec: accept
`seat_0` and `seat_1`, reject missing prefix, empty index, leading zero, sign,
whitespace, non-ASCII digit, overflow, and out-of-game indices. WASM import
aliases are compatibility-only; TypeScript must not normalize seats.

## C-03 Setup Count Surfaces

| Game / predicate | Current owner | Current diagnostic / retained policy | Verdict |
|---|---|---|---|
| Plain roster | `games/plain_tricks/src/setup.rs::setup_match`, `seats.len() != 2` | `invalid_seat_count`, `plain_tricks requires exactly two seats` | migrate count wrapper |
| Plain variant count | no current enforcement in setup path | no new setup rule may be introduced | N/A |
| Flood roster | `games/flood_watch/src/setup.rs::setup_match` | `invalid_seat_count`, `flood_watch requires exactly two seats` | migrate count wrapper |
| Flood variant seat count | `validate_variant` | `invalid_variant_seat_count`, `flood_watch variants require exactly two seats` | migrate count wrapper |
| Flood role-order length | `validate_variant` | `invalid_variant_roles`, `flood_watch variants require exactly two roles`; role identities/order stay local | migrate count wrapper |
| Frontier roster | `games/frontier_control/src/setup.rs::setup_match` | `invalid_seat_count`, `frontier_control requires exactly two seats` | migrate count wrapper |
| Frontier variant seat count | `validate_variant` | `invalid_variant_seat_count`, `frontier_control variants require exactly two seats` | migrate count wrapper |
| Frontier faction order | `validate_variant` | `Garrison` then `Prospectors` remains game policy | exception |
| Event roster | `games/event_frontier/src/setup.rs::setup_match` | `invalid_seat_count`, `event_frontier requires exactly two seats` | migrate count wrapper |
| Event variant seat count | `validate_variant` | `invalid_variant_seat_count`, `event_frontier variants require exactly two seats` | migrate count wrapper |
| Event faction order | `validate_variant` | `Charter` then `Freeholders` remains game policy | exception |
| All ranges/ring helpers | no target game admits a true range or selected ring migration | turn/role/faction/leader rotation remains game-owned | N/A |

Flood Watch and Frontier Control may add a normal `game-stdlib` dependency only
for the behavior-free count wrapper. Exact diagnostics and setup acceptance
must remain unchanged.

## C-04/C-05 Action-Tree And Byte Surfaces

Current local action-tree hash owners:

| Game | Current local hash owner | Selected migration | Adjacent surfaces |
|---|---|---|---|
| `plain_tricks` | `games/plain_tricks/src/replay_support.rs::action_tree_hash` | add parallel `ActionTreeEncodingVersion::V1` bytes/hash | existing local tree hash, state/effect/view/replay/export/diagnostic bytes are exceptions |
| `flood_watch` | `games/flood_watch/src/visibility.rs::action_tree_hash` | add parallel v1 bytes/hash | existing debug-derived hash and all adjacent bytes are exceptions |
| `frontier_control` | `games/frontier_control/src/visibility.rs::action_tree_hash` | add parallel v1 bytes/hash | existing debug-derived hash and all adjacent bytes are exceptions |
| `event_frontier` | `games/event_frontier/src/visibility.rs::action_tree_hash` | add parallel v1 bytes/hash | existing debug-derived hash and all adjacent bytes are exceptions |

Representative tree coverage required by later tickets:

- Plain Tricks: opening trick, forced follow-suit, void/free discard, final
  play, terminal empty tree.
- Flood Watch: bail, levee placement, role power, early end, budget-exhausted
  automatic environment, terminal.
- Frontier Control: muster/reinforce, move, clash, stake/dismantle, early end,
  terminal.
- Event Frontier: full/limited operation, multi-site branch, event choice,
  pass, edict-blocked state, Reckoning/terminal.

Expected ADR-0009 class: `parallel-new-surface`. Replacing a current local hash
or changing branch/label/metadata/freshness order is outside ticket 001 and
outside C-04/C-05 unless separately admitted.

## C-06 Dev-Only Test Support

Current state: none of the four manifests contains `game-test-support`. R3 may
add it only under `[dev-dependencies]`. Production and normal/build reverse
dependencies are a stop condition and must be checked with:

```text
cargo tree --workspace -e normal --invert game-test-support
```

## C-07 Visibility And No-Leak Baseline

| Game | Current tests | R3 matrix verdict |
|---|---|---|
| `plain_tricks` | `tests/visibility.rs`, `tests/replay.rs`, and `tests/bots.rs` cover observer redaction, own-hand seat views, tail absence, public/seat export no-leak, and bot no-leak | migrate full pairwise matrix for both source seats |
| `flood_watch` | `tests/visibility.rs` checks public projection/action/diagnostic no-leak; `tests/replay.rs` checks public export redaction; `tests/bots.rs` checks hidden deck-order invariance | migrate hidden future-deck matrix across observer, seat 0, and seat 1 |
| `frontier_control` | `tests/visibility.rs` asserts observer, seat 0, and seat 1 projections are equal | N/A for hidden source; retain equality/public export receipt |
| `event_frontier` | `tests/visibility.rs`, `tests/replay.rs`, and `tests/bots.rs` check hidden deeper-deck absence from views, trees, diagnostics, effects, export, and bot rationale | migrate hidden deeper-deck matrix across observer, seat 0, and seat 1 |

Canaries, if added later, must be in-memory-only and absent from committed
traces, fixtures, exports, logs, snapshots, DOM/test IDs, browser storage,
accessibility artifacts, and screenshots.

## C-08 Evidence Profile Baseline

| Game | `replay-command-v1` | `setup-evidence-v1` | `domain-evidence-v1` | `public-export-v1` | `seat-private-export-v1` |
|---|---|---|---|---|---|
| `plain_tricks` | migrate; native command replay remains authority; visibility `internal-dev` | migrate standard fixture; setup/deal facts remain game-owned | migrate deck partition/trick-round fixture checks | migrate observer export/import/no-leak | migrate both seat viewers |
| `flood_watch` | migrate; native command replay remains authority; hidden deck order is internal | migrate standard and deluge fixtures | migrate levee/flood/event/budget evidence | migrate public export/import/no-leak | N/A, no official per-seat private timeline |
| `frontier_control` | migrate; fully public native command evidence | migrate standard and highlands fixtures | migrate graph/clash/connectivity/scoring evidence | migrate fully public export/import | N/A, no private timeline |
| `event_frontier` | migrate; native command evidence remains internal because setup has hidden deck order | migrate standard, hard-winter, and land-rush fixtures | migrate event/edict/funding/resource/scoring evidence | migrate public export/import/no hidden deeper deck | N/A, no official per-seat private timeline |

Drivers validate metadata and delegate to existing game/tool validators. They
must not parse commands, choose actions, project views, authorize exports,
score, or interpret fixture data as behavior.

## C-09 RNG Baseline

| Game | Current owner | Verdict | Identity evidence required later |
|---|---|---|---|
| `plain_tricks` | `setup::{shuffle_deck,next_bounded_index_unbiased}` | migrate to `DeterministicRng::next_index_unbiased_v1` | fixed RNG words, rejection paths, draw counts, deck order, hands/tail, effect/view/replay/export hashes |
| `flood_watch` | `setup::{shuffle_event_deck,next_bounded_index_unbiased}` | migrate | fixed RNG words, rejection paths, event deck order, forecast/current deck state, effect/view/replay/export hashes |
| `frontier_control` | no setup RNG | N/A | no synthetic RNG surface |
| `event_frontier` | `setup::{build_seeded_deck,shuffle_epoch,next_bounded_index_unbiased}` | migrate | fixed RNG words, rejection paths, per-epoch order, current/next/deeper tail, effect/view/replay/export hashes |

Any identity failure blocks the sampler migration. It does not authorize a new
RNG algorithm or broad golden refresh.

## C-10 Non-Promotion Baseline

R3 keeps these behavioral surfaces local or explicitly rejected for mechanical
scaffolding: trick lifecycle, follow-suit/winner-leads/scoring, cooperative
role powers, faction identity/order, graph topology/connectivity, movement and
clash, events/edicts, budget/resource accounting, projection/redaction,
terminal scoring, outcome rationale, bot strategy, and diagnostics. No YAML,
DSL, static rule behavior, TypeScript legality, or private licensed content is
admitted.

## Fixture And Golden Trace SHA-256 Baseline

The command below was run on 2026-06-24 before any R3 migration:

```text
sha256sum games/plain_tricks/data/fixtures/*.json games/plain_tricks/tests/golden_traces/*.json games/flood_watch/data/fixtures/*.json games/flood_watch/tests/golden_traces/*.json games/frontier_control/data/fixtures/*.json games/frontier_control/tests/golden_traces/*.json games/event_frontier/data/fixtures/*.json games/event_frontier/tests/golden_traces/*.json
```

### Plain Tricks

| Path | SHA-256 |
|---|---|
| `games/plain_tricks/data/fixtures/plain_tricks_standard.fixture.json` | `fb2f907a073cf70852a509922c0228ec359be2d5eb675180d42203c56521577e` |
| `games/plain_tricks/tests/golden_traces/bot-action.trace.json` | `e4e5a09c1e53e8d50d7f7d9b338c0f35f774cceda73ebc8aad84527f7269a34e` |
| `games/plain_tricks/tests/golden_traces/deal-private-no-leak.trace.json` | `608443421becb79374ce2919346afbc02363f2fec116e7606e0e2f20083f5f08` |
| `games/plain_tricks/tests/golden_traces/follow-suit-forced.trace.json` | `33c1071971f78bc91d10838299f011566741b1b378afb99d3df5268c7c8c32fa` |
| `games/plain_tricks/tests/golden_traces/invalid-must-follow-diagnostic.trace.json` | `a3b57c28168ca9663089d4203f86f62125d75883a5c4df37ac0bdd30854df12f` |
| `games/plain_tricks/tests/golden_traces/invalid-stale-diagnostic.trace.json` | `dee59c60047c2f034d6a82b66d3f7214c48803983063c2bcce68207161df059d` |
| `games/plain_tricks/tests/golden_traces/invalid-wrong-seat-diagnostic.trace.json` | `53c8bf02b404b75a8a3913e670b78b38369fc44bc8de57815180835858cc8848` |
| `games/plain_tricks/tests/golden_traces/no-leak-public-observer.trace.json` | `f87ed9e39cde8d81df6899eabb402b0f2a7422bc3d07efdb8842fcec1e3caeff` |
| `games/plain_tricks/tests/golden_traces/off-suit-never-wins.trace.json` | `b289ce0b6596aa2f3b30d9d259876b6e109ade4da01961db24a3e14484aabb9b` |
| `games/plain_tricks/tests/golden_traces/public-replay-export-import.trace.json` | `fc7fb3b30b8ce6ca5c03a25e951dbfe315d637b85e0ca74ff21a44cea845aca2` |
| `games/plain_tricks/tests/golden_traces/round-close-deal-rotation.trace.json` | `5e68a08692fa928ac1f21b5a0a248944247f6bf3adb51e10296023d47950c9fb` |
| `games/plain_tricks/tests/golden_traces/seat-private-view.trace.json` | `d1eea0f04e46920e04dc1c66c1183e72edf50ffaffd2c374628f7a4102a02c87` |
| `games/plain_tricks/tests/golden_traces/terminal-most-points-win.trace.json` | `7cb04a1c3a5ad7e529d78ea0bb6771fc85667af5085a234b65955eac80aa927a` |
| `games/plain_tricks/tests/golden_traces/tie-split.trace.json` | `3b9e9424e0d9812c605a7aae19a76b23deb6bb009a69d5c6509486a905165424` |
| `games/plain_tricks/tests/golden_traces/trick-winner-leads-next.trace.json` | `7a8676bad051a8e0a736407c87f7094919e37d52b6bf42599583556538e6584d` |
| `games/plain_tricks/tests/golden_traces/void-free-discard.trace.json` | `74ede3806db67d4f388f802504a2b7cdb1c90d27c8e2deea690979c558f60818` |
| `games/plain_tricks/tests/golden_traces/wasm-exported.trace.json` | `451a92b69e3c7a3d1034bcbebe393642fcba261bb9a2ba172b639a6c6b41ea0a` |

### Flood Watch

| Path | SHA-256 |
|---|---|
| `games/flood_watch/data/fixtures/flood_watch_deluge.fixture.json` | `c0e578e84fc5ab0a5a473e4867e4f742df40d3828b31629b4011dc144e71a6df` |
| `games/flood_watch/data/fixtures/flood_watch_standard.fixture.json` | `2e2c8548c83cc9fe92545e212077e39a2bfd10a26410913bb13ecc74c3fa2f7f` |
| `games/flood_watch/tests/golden_traces/bail-dry-district-diagnostic.trace.json` | `4787310ca492f81d8bfe90187e8ce186fd4e9985c350daf78bdfb0a4284a0243` |
| `games/flood_watch/tests/golden_traces/bot-coop-full-game.trace.json` | `762685280c459ef314022ff75af6c24538e6835fe8ed7458d52a1747150da5c1` |
| `games/flood_watch/tests/golden_traces/budget-exhaustion-auto-environment.trace.json` | `afa8e569e3a9831cf301e844d44aedabb9e06f204e8fd42c011c320688ededc8` |
| `games/flood_watch/tests/golden_traces/early-end-turn.trace.json` | `232595bbe83772981d340ce3ed73e42c4ad60ebc433a38bccdde6fd77bff3e6d` |
| `games/flood_watch/tests/golden_traces/forecast-public-reveal.trace.json` | `b133cc5e8b91f533198b0bf7a42c439e0cb3d467634f2cb605fed835404cd1c9` |
| `games/flood_watch/tests/golden_traces/levee-absorption.trace.json` | `ce4202572f8f02c10dc471e55f015f995daf02e652aedf00b53db6aa1b6d0b9e` |
| `games/flood_watch/tests/golden_traces/loss-by-inundation.trace.json` | `a3c25edf486ade4ed0043c5259a6b14be333d294e1e62281b52c825cb1f386e0` |
| `games/flood_watch/tests/golden_traces/mid-phase-early-stop.trace.json` | `aba8b9f03a93c5fda7bafdf8018ec8373725263d10f9b61cf16ea421870d0c3a` |
| `games/flood_watch/tests/golden_traces/out-of-budget-diagnostic.trace.json` | `514f6329ba10683a96b55bcc6e27f3834d802477d0876e46426cd4dcc044c8ef` |
| `games/flood_watch/tests/golden_traces/public-observer-no-leak.trace.json` | `af5f2eb482f680ebd22599e49362692a6fab2362879dd978356bdcc196e0d8e5` |
| `games/flood_watch/tests/golden_traces/public-replay-export-import.trace.json` | `f7dffb995f1c985824267718f7cf589a4e009e1292571892ddc6ec9854073545` |
| `games/flood_watch/tests/golden_traces/reprieve-no-op.trace.json` | `ef99135dd4f4c8cc352f605afc2aec906b0da4c14253e61b413ffec422ef5d86` |
| `games/flood_watch/tests/golden_traces/role-power-levee-warden.trace.json` | `72c63840443b1f51544da9ddb75713d32573a5613333b85cd3a304c4c8605685` |
| `games/flood_watch/tests/golden_traces/role-power-pumpwright.trace.json` | `56599a8e1afa8f36365638096629fec0fe06ae9ed95058214644c1637f1fe628` |
| `games/flood_watch/tests/golden_traces/scenario-deluge-setup.trace.json` | `5dc21b8773cbf74416f3143bf39b3f0827a9bfaa42988da4b5a9ada9caa25dc4` |
| `games/flood_watch/tests/golden_traces/standard-win.trace.json` | `0e8911cea1034e1d3371e0002526fd3ecd75329e4b1d689173f2fb308ede4451` |
| `games/flood_watch/tests/golden_traces/storm-surge-double-rise.trace.json` | `dcabccc14f4a345fe25ccb782cd656dd5d780ef398210dc9cb34a47a1914941f` |
| `games/flood_watch/tests/golden_traces/wrong-seat-diagnostic.trace.json` | `aaa066744023d4f5293276f668609f17c3f4e6497bb10e706b0bf59f99771c86` |

### Frontier Control

| Path | SHA-256 |
|---|---|
| `games/frontier_control/data/fixtures/frontier_control_highlands.fixture.json` | `08c1985b4a937ec84fea2ab0e57faed63d7b8bd93341209f8cfbacd085fe101d` |
| `games/frontier_control/data/fixtures/frontier_control_standard.fixture.json` | `e3b2527d032666c9c08a861f8f0dc5f56efd20086fa4e81cfed2aa1287f73355` |
| `games/frontier_control/tests/golden_traces/bot-vs-bot-full-game.trace.json` | `6822c2b5fa78b2b1a05e18d9d858b4688642c7f9997a62e16c5499b64b6ba500` |
| `games/frontier_control/tests/golden_traces/budget-exhaustion-auto-end.trace.json` | `32d5a5e0bd344049146401ddacf6d00360f5ec636aaf4cb3f9bdc27a16f3e63a` |
| `games/frontier_control/tests/golden_traces/clash-crew-into-guards.trace.json` | `4b49887c9298a5b2635b7d8f005682188e31d53604c9b17c61894f99838b68f5` |
| `games/frontier_control/tests/golden_traces/clash-guard-into-crews.trace.json` | `060d8067854af7b1002b744b5981cdc13bb17c89bd51422c041611a5099804f0` |
| `games/frontier_control/tests/golden_traces/early-end-turn.trace.json` | `90850636826c5d8c50e46d2b031bcd40132f822e6f4db74222acc31eb695f50a` |
| `games/frontier_control/tests/golden_traces/highlands-setup.trace.json` | `1ec1d8e4db6dfa4bcde0f5ff2411b4a11414a13a28bee12591125a6f723ed929` |
| `games/frontier_control/tests/golden_traces/muster-and-reinforce-caps.trace.json` | `6d05b49f982d4d50fa68f4b499cbbb60321554452fdbece64c4866c2341c6849` |
| `games/frontier_control/tests/golden_traces/non-adjacent-move-diagnostic.trace.json` | `3ce305677b48ad6446d988910235f4352e4ba49cfbd0fa44296480e4a684c5ac` |
| `games/frontier_control/tests/golden_traces/replay-export-import.trace.json` | `3352d26701c3ee7ef1a30d0ea5faf3801eb6e56da252f25eafff908ba536738b` |
| `games/frontier_control/tests/golden_traces/round-scoring-breakdown.trace.json` | `cf8af7938baaa6e4b04164f216043cf8b33216a45e60f3338bf11e59032d1679` |
| `games/frontier_control/tests/golden_traces/stake-and-dismantle.trace.json` | `46440b72423e1a9e7fefe48d76d3218fec312e853492a106555e0f248427d118` |
| `games/frontier_control/tests/golden_traces/stake-on-guarded-site-diagnostic.trace.json` | `ce0b80c342ead52de6be274357b2fa1b53a1df9421cb638dcb4cbe6f3428b5f6` |
| `games/frontier_control/tests/golden_traces/standard-garrison-win.trace.json` | `baef8ea894a32055951a837730ac59723785dcdc308740137bc8478bb6397f5c` |
| `games/frontier_control/tests/golden_traces/standard-prospector-win.trace.json` | `2cfd2936a42382e75f9c61adf8a58a9fe498c713010e1f2e4ba109596ba42131` |
| `games/frontier_control/tests/golden_traces/supply-cut-scores-zero.trace.json` | `5c8bbb62cd2350ffd8fc65706a12c99ec12a70f8236dc55dccabfdac8d06711a` |
| `games/frontier_control/tests/golden_traces/tie-garrison-tiebreak.trace.json` | `582ca7a657ed60cc72cec280b058225575dcdef4182a1d17bf5e9d68fab4fdc2` |
| `games/frontier_control/tests/golden_traces/wrong-faction-diagnostic.trace.json` | `dc6206d3f5bd8fcbae5df4a789b2f9c2d019c4826e5fb956f44e4329e6a83ee3` |

### Event Frontier

| Path | SHA-256 |
|---|---|
| `games/event_frontier/data/fixtures/event_frontier_hard_winter.fixture.json` | `4e3dd69fbdf94cd3c7c2f79dc93a0f30ab210989e19faebaafc64d99ea30acb1` |
| `games/event_frontier/data/fixtures/event_frontier_land_rush.fixture.json` | `c304c7f37ad50486d31f183dcf10b08c39a1f1c1790b7700dd20cd04931546bc` |
| `games/event_frontier/data/fixtures/event_frontier_standard.fixture.json` | `73abbbec347a1ab97e8d3f27898c1a8898ea10d8b33c7e7caf7d7242d4db95c6` |
| `games/event_frontier/tests/golden_traces/bot-vs-bot-full-game.trace.json` | `be3fd9360718a92329ba14267666a16a23262ce552198c709981510b5235c00d` |
| `games/event_frontier/tests/golden_traces/double-pass-discards-card.trace.json` | `2c3b5db1c1dab6bdfa46da263318bd0ebc364b138859bca065c6ca2c333b5c96` |
| `games/event_frontier/tests/golden_traces/edict-activation-and-expiry.trace.json` | `5b1cc57dc913375c0af501bc8c8ea7084e08e2d4d0ec81b2ed1628e1d8c433a8` |
| `games/event_frontier/tests/golden_traces/edict-blocks-action-diagnostic.trace.json` | `58bbfc2bc107698700a62c14bece3af84ac56640decf659f3c647906955a279a` |
| `games/event_frontier/tests/golden_traces/event-choice-resolves-card.trace.json` | `9d335b58fa74ab4cf23c6e2c84e4939e0aa084d43596110123a9ea911464233f` |
| `games/event_frontier/tests/golden_traces/final-reckoning-fallback.trace.json` | `6cd6e78ab154875c11e4e8a6fdbe83ad92b8e3260156c90e4fa744e78a5c82b4` |
| `games/event_frontier/tests/golden_traces/hard-winter-setup.trace.json` | `4627fe8bdcffba9abcb94ce2514d35c0ee56319adcba9b02ce89b3ef12792ce5` |
| `games/event_frontier/tests/golden_traces/ineligible-faction-diagnostic.trace.json` | `64cd9ce338284a0f31a0b491fd7f3587b4d0f3f780314cbb0f93eb88874d90da` |
| `games/event_frontier/tests/golden_traces/land-rush-setup.trace.json` | `548480e4eda4f5f013cd0d8d330677e40094607c6c1088fd4ae4806853dd14f7` |
| `games/event_frontier/tests/golden_traces/limited-op-after-full-op.trace.json` | `285d02042391e0e26ee65f11fed6e0c8af7b003bf3e25158783e4c29ae0fd5ae` |
| `games/event_frontier/tests/golden_traces/no-eligible-faction-discard.trace.json` | `8d97bf936f895b40184ccbce47c3ff04917c19b8c0afacfe99e821a6e21c5a9c` |
| `games/event_frontier/tests/golden_traces/op-full-multi-site.trace.json` | `1590327ee3483491a6291464ac845487c2b8d550224686e493eb8222d5c3b60f` |
| `games/event_frontier/tests/golden_traces/pass-keeps-eligibility.trace.json` | `cc25e5accd60341dd1f99386011c407f9f7306588f1f9b5021ac40def8e02711` |
| `games/event_frontier/tests/golden_traces/reckoning-never-first-in-epoch.trace.json` | `93b6599a9cd5016ace750b21ee3a1923cdb4d976339cc877004ed0ae65a7674e` |
| `games/event_frontier/tests/golden_traces/reckoning-scoring-breakdown.trace.json` | `2db01e035643fa74539a74ac080d8f97f3e2f002fb97f7d9bda76e7197a5fa13` |
| `games/event_frontier/tests/golden_traces/replay-export-import-no-deck-leak.trace.json` | `f76dcb1b75baeede1235a12e6a8c4870e6b69a19fcdd43dd5568d10bb6c66a47` |
| `games/event_frontier/tests/golden_traces/standard-charter-instant-win.trace.json` | `cd03a926c5a6b2881f7d826c5d4ce792d591706cb57f4d8ebb836fc6c2392759` |
| `games/event_frontier/tests/golden_traces/standard-freeholder-cache-win.trace.json` | `0dc43dbcc555528c28647422c12bb3dd38c914d4f73977e891f900a2af018b9e` |

## Command Evidence Ledger

Ticket 001 command evidence is recorded here. Later tickets append their own
focused before/after evidence and the final capstone records the full §7.1 set.

| Command | Status | Relevant output / classification |
|---|---|---|
| `git status --short` | passed before ticket archival | Output showed only `?? reports/8c-r3-public-coop-asymmetric-trick-scaffolding-characterization.md`. |
| `cargo test --workspace` | passed, exit 0 | Workspace tests and doc-tests passed. The command waited briefly for the build-directory lock while parallel replay checks compiled, then completed green. |
| `cargo run -p replay-check -- --game plain_tricks --all` | passed, exit 0 | All Plain Tricks traces passed; `bot-action` was accepted as not-applicable and the remaining 15 traces reported `ok`. |
| `cargo run -p replay-check -- --game flood_watch --all` | passed, exit 0 | All 18 Flood Watch traces were accepted; final line `replay-check: all traces passed`. |
| `cargo run -p replay-check -- --game frontier_control --all` | passed, exit 0 | All 17 Frontier Control traces were accepted; final line `replay-check: all traces passed`. |
| `cargo run -p replay-check -- --game event_frontier --all` | passed, exit 0 | All 18 Event Frontier traces were accepted; final line `replay-check: all traces passed`. |

## Changed-Artifact Policy

Authorized ticket 001 change: this report only. Existing production code,
tests, fixtures, traces, snapshots, exports, specs, and register entries remain
unchanged until their own ticket. The default authorized changes to existing
golden traces, fixtures, snapshots, or export bytes are none.

## Migration Receipts

### 8CR3PUBCOOASY-101 - Plain Tricks public effect constructor

Completed: 2026-06-24

- Selected surface: `games/plain_tricks/src/effects.rs::public_effect`.
- Change: replaced the local public envelope literal with
  `EffectEnvelope::public(payload)`.
- ADR-0009 classification: `unchanged`; no trace, fixture, export, hash,
  schema, seat spelling, RNG, or visibility byte was intentionally migrated.
- Compatibility / rollback: restore only the local public literal constructor.
  `private_effect`, `hand_dealt_effect`, recipient selection, payload
  formation, effect order, filtering, and reveal policy were untouched.
- Verification:
  - `cargo test -p plain_tricks` passed.
  - `cargo run -p replay-check -- --game plain_tricks --all` passed; all
    Plain Tricks traces passed with existing expected hashes.
  - `cargo run -p fixture-check -- --game plain_tricks` passed.
  - No golden trace, fixture, export, or test file changed.

### 8CR3PUBCOOASY-102 - Plain Tricks seat-private effect constructor

Completed: 2026-06-24

- Selected surface: `games/plain_tricks/src/effects.rs::private_effect`.
- Change: replaced the local seat-private envelope literal with
  `EffectEnvelope::private_to(owner_seat_id, payload)`.
- ADR-0009 classification: `unchanged`; no trace, fixture, export, hash,
  schema, seat spelling, RNG, or visibility byte was intentionally migrated.
- Compatibility / rollback: restore only the local private literal constructor.
  `hand_dealt_effect` still supplies the owner `SeatId` and the exact
  `HandDealt` payload; public effects, filtering, reveal policy, effect order,
  and export policy were untouched.
- Verification:
  - `cargo test -p plain_tricks` passed, including the seat-private visibility
    and effect-scope tests.
  - `cargo run -p replay-check -- --game plain_tricks --all` passed; all
    Plain Tricks traces passed with existing expected hashes.
  - `cargo run -p fixture-check -- --game plain_tricks` passed.
  - No golden trace, fixture, export, or test file changed.

### 8CR3PUBCOOASY-103 - Flood Watch public effect constructor

Completed: 2026-06-24

- Selected surface: `games/flood_watch/src/effects.rs::public_effect`.
- Change: replaced the local public envelope literal with
  `EffectEnvelope::public(payload)`.
- ADR-0009 classification: `unchanged`; no trace, fixture, export, hash,
  schema, seat spelling, RNG, or visibility byte was intentionally migrated.
- Compatibility / rollback: restore only the local public literal constructor.
  Forecast/event/levee payload formation, public visibility policy, hidden
  future-deck redaction, effect order, and export policy were untouched.
- Verification:
  - `cargo test -p flood_watch` passed.
  - `cargo run -p replay-check -- --game flood_watch --all` passed; all Flood
    Watch traces were accepted.
  - `cargo run -p fixture-check -- --game flood_watch` passed.
  - No golden trace, fixture, export, or test file changed.

### 8CR3PUBCOOASY-104 - Frontier Control public effect constructor

Completed: 2026-06-24

- Selected surface: `games/frontier_control/src/effects.rs::public_effect`.
- Change: replaced the local public envelope literal with
  `EffectEnvelope::public(payload)`.
- ADR-0009 classification: `unchanged`; no trace, fixture, export, hash,
  schema, seat spelling, RNG, or visibility byte was intentionally migrated.
- Compatibility / rollback: restore only the local public literal constructor.
  Graph/clash/scoring payload formation, public visibility policy, effect
  order, and export policy were untouched.
- Verification:
  - `cargo test -p frontier_control` passed.
  - `cargo run -p replay-check -- --game frontier_control --all` passed; all
    Frontier Control traces were accepted.
  - `cargo run -p fixture-check -- --game frontier_control` passed.
  - No golden trace, fixture, export, or test file changed.

### 8CR3PUBCOOASY-105 - Event Frontier public effect constructor

Completed: 2026-06-24

- Selected surface: `games/event_frontier/src/effects.rs::public_effect`.
- Change: replaced the local public envelope literal with
  `EffectEnvelope::public(payload)`.
- ADR-0009 classification: `unchanged`; no trace, fixture, export, hash,
  schema, seat spelling, RNG, or visibility byte was intentionally migrated.
- Compatibility / rollback: restore only the local public literal constructor.
  Current/next card reveal payloads, hidden-tail redaction, event/edict payload
  formation, effect order, and export policy were untouched.
- Verification:
  - `cargo test -p event_frontier` passed.
  - `cargo run -p replay-check -- --game event_frontier --all` passed; all
    Event Frontier traces were accepted.
  - `cargo run -p fixture-check -- --game event_frontier` passed.
  - No golden trace, fixture, export, or test file changed.

### 8CR3PUBCOOASY-201 - Plain Tricks typed seat parser

Completed: 2026-06-24

- Selected surface: `games/plain_tricks/src/ids.rs::PlainTricksSeat::parse`.
- Change: replaced the game-local two-string parser with delegation to
  `SeatId::parse_canonical`, then mapped the canonical zero-based index through
  `PlainTricksSeat::from_index`.
- ADR-0009 classification: `unchanged`; accepted output spellings, trace bytes,
  replay hashes, and public/private exports were not intentionally migrated.
- Compatibility / rollback: restore only the manual `"seat_0"` / `"seat_1"`
  parser. `from_index`, `as_str`, trace spellings, non-seat ID parsers, and the
  WASM import-alias boundary were untouched.
- Verification:
  - `cargo test -p plain_tricks` passed, including canonical acceptance,
    malformed grammar, overflow, and out-of-game index parser cases.
  - `cargo run -p replay-check -- --game plain_tricks --all` passed; all Plain
    Tricks traces passed with existing expected hashes.
  - No golden trace, fixture, export, or non-seat ID parser changed.

### 8CR3PUBCOOASY-202 - WASM seat boundary conformance

Completed: 2026-06-24

- Selected surface: `crates/wasm-api/src/seats.rs` inline conformance tests.
- Change: added R3 seat-boundary vectors for Plain Tricks, Flood Watch,
  Frontier Control, and Event Frontier covering canonical input, retained
  import aliases, malformed/out-of-game rejection, and canonical output
  spelling.
- ADR-0009 classification: `unchanged`; no production seat adapter, canonical
  output, trace, replay hash, or TypeScript behavior was intentionally migrated.
- Compatibility / rollback: remove only the R3 conformance tests. Existing
  import aliases, canonical Rust output helpers, and game parsers remain
  unchanged.
- Verification:
  - `cargo test -p wasm-api` passed.
  - `cargo run -p replay-check -- --game plain_tricks --all` passed.
  - `cargo run -p replay-check -- --game flood_watch --all` passed.
  - `cargo run -p replay-check -- --game frontier_control --all` passed.
  - `cargo run -p replay-check -- --game event_frontier --all` passed.
  - Targeted TypeScript grep
    `rg -n "normalizeSeat|parseSeat|parse_seat|repairSeat|canonicalSeat|seatId.*replace|replace\\([^)]*seat-" apps/web --glob '*.{ts,tsx,js,jsx}'`
    returned no matches; no TypeScript seat normalization/repair path was
    found.

### 8CR3PUBCOOASY-301 - Plain Tricks roster count

Completed: 2026-06-24

- Selected surface: `games/plain_tricks/src/setup.rs::setup_match` roster
  predicate.
- Change: replaced the bare `seats.len()` comparison with
  `SeatCount::new(seats.len()).map(SeatCount::get)` compared against the
  game-owned `STANDARD_SEAT_COUNT`.
- ADR-0009 classification: `unchanged`; accepted/rejected counts, diagnostics,
  setup state, replay hashes, fixtures, and exports were not intentionally
  migrated.
- Compatibility / rollback: restore only the bare roster-length predicate.
  Variant policy, deal/leader rotation, RNG sampling, exact two-seat policy,
  and diagnostics stay game-owned.
- Verification:
  - `cargo test -p plain_tricks` passed, including exact diagnostic checks for
    0, 1, and 3 seats.
  - `cargo run -p replay-check -- --game plain_tricks --all` passed; all Plain
    Tricks traces passed with existing expected hashes.
  - `cargo run -p fixture-check -- --game plain_tricks` passed.
  - No golden trace, fixture, export, or setup policy file changed.

### 8CR3PUBCOOASY-302 - Flood Watch roster count and game-stdlib edge

Completed: 2026-06-24

- Selected surfaces: `games/flood_watch/Cargo.toml` and
  `games/flood_watch/src/setup.rs::setup_match` roster predicate.
- Change: added a normal `game-stdlib` dependency and replaced the bare
  `seats.len()` comparison with
  `SeatCount::new(seats.len()).map(SeatCount::get)` compared against the
  game-owned `STANDARD_SEAT_COUNT`.
- ADR-0009 classification: `unchanged`; accepted/rejected counts, diagnostics,
  setup/deck state, replay hashes, fixtures, and exports were not intentionally
  migrated.
- Compatibility / rollback: remove only the `game-stdlib` edge if no later
  Flood R3 task needs it and restore only the bare roster-length predicate.
  Variant seat count, role order, deck setup, RNG sampling, and cooperative
  two-seat policy stay game-owned.
- Verification:
  - `cargo test -p flood_watch` passed, including exact diagnostic checks for
    0, 1, and 3 seats.
  - `cargo run -p replay-check -- --game flood_watch --all` passed; all Flood
    Watch traces were accepted.
  - `bash scripts/boundary-check.sh` passed; `engine-core` stayed noun-free and
    `game-test-support` stayed dev-only.
  - No golden trace, fixture, export, or variant policy file changed.

### 8CR3PUBCOOASY-303 - Flood Watch variant seat-count predicate

Completed: 2026-06-24

- Selected surface: `games/flood_watch/src/setup.rs::validate_variant`
  `variant.seat_count` predicate.
- Change: replaced the bare variant seat-count comparison with
  `SeatCount::new(variant.seat_count as usize).map(SeatCount::get)` compared
  against the game-owned `STANDARD_SEAT_COUNT`.
- ADR-0009 classification: `unchanged`; variant acceptance/diagnostic, setup
  state, replay hashes, fixtures, and exports were not intentionally migrated.
- Compatibility / rollback: restore only the bare `variant.seat_count`
  comparison. Roster validation, role-order validation, variant policy, and
  event-deck setup stay game-owned.
- Verification:
  - `cargo test -p flood_watch` passed, including exact diagnostic checks for
    variant seat counts 0, 1, and 3.
  - `cargo run -p replay-check -- --game flood_watch --all` passed; all Flood
    Watch traces were accepted.
  - `cargo run -p fixture-check -- --game flood_watch` passed.
  - No golden trace, fixture, export, or variant policy file changed.

### 8CR3PUBCOOASY-304 - Flood Watch role-order cardinality predicate

Completed: 2026-06-24

- Selected surface: `games/flood_watch/src/setup.rs::validate_variant`
  `variant.role_order.len()` predicate.
- Change: replaced the bare role-order length comparison with
  `SeatCount::new(variant.role_order.len()).map(SeatCount::get)` compared
  against the game-owned `STANDARD_SEAT_COUNT`.
- ADR-0009 classification: `unchanged`; role identities, order, powers,
  assignment, setup hashes, replay hashes, fixtures, and exports were not
  intentionally migrated.
- Compatibility / rollback: restore only the bare `variant.role_order.len()`
  comparison. Roster validation, variant seat-count validation, role
  identities/order/powers, and event-deck setup stay game-owned.
- Verification:
  - `cargo test -p flood_watch` passed.
  - `cargo run -p replay-check -- --game flood_watch --all` passed; all Flood
    Watch traces were accepted.
  - `cargo run -p fixture-check -- --game flood_watch` passed.
  - No golden trace, fixture, export, role policy, or variant policy file
    changed.

### 8CR3PUBCOOASY-305 - Frontier Control roster count and game-stdlib edge

Completed: 2026-06-24

- Selected surfaces: `games/frontier_control/Cargo.toml` and
  `games/frontier_control/src/setup.rs::setup_match` roster predicate.
- Change: added a normal `game-stdlib` dependency and replaced the bare
  `seats.len()` comparison with
  `SeatCount::new(seats.len()).map(SeatCount::get)` compared against the
  game-owned `STANDARD_SEAT_COUNT`.
- ADR-0009 classification: `unchanged`; accepted/rejected counts, diagnostics,
  setup state, replay hashes, fixtures, and exports were not intentionally
  migrated.
- Compatibility / rollback: remove only the `game-stdlib` edge if no later
  Frontier R3 task needs it and restore only the bare roster-length predicate.
  Variant seat count, faction identity/order, graph setup, and asymmetric
  two-seat policy stay game-owned.
- Verification:
  - `cargo test -p frontier_control` passed, including exact diagnostic checks
    for 0, 1, and 3 seats.
  - `cargo run -p replay-check -- --game frontier_control --all` passed; all
    Frontier Control traces were accepted.
  - `bash scripts/boundary-check.sh` passed; `engine-core` stayed noun-free and
    `game-test-support` stayed dev-only.
  - No golden trace, fixture, export, or variant policy file changed.

### 8CR3PUBCOOASY-306 - Frontier Control variant seat-count predicate

Completed: 2026-06-24

- Selected surface: `games/frontier_control/src/setup.rs::validate_variant`
  `variant.seat_count` predicate.
- Change: replaced the bare variant seat-count comparison with
  `SeatCount::new(variant.seat_count as usize).map(SeatCount::get)` compared
  against the game-owned `STANDARD_SEAT_COUNT`.
- ADR-0009 classification: `unchanged`; variant acceptance/diagnostic, faction
  identity/order, graph setup, replay hashes, fixtures, and exports were not
  intentionally migrated.
- Compatibility / rollback: restore only the bare `variant.seat_count`
  comparison. Roster validation, faction identity/order, graph setup, and
  scoring stay game-owned.
- Verification:
  - `cargo test -p frontier_control` passed, including exact diagnostic checks
    for variant seat counts 0, 1, and 3.
  - `cargo run -p replay-check -- --game frontier_control --all` passed; all
    Frontier Control traces were accepted.
  - `cargo run -p fixture-check -- --game frontier_control` passed.
  - No golden trace, fixture, export, or variant policy file changed.

### 8CR3PUBCOOASY-307 - Event Frontier roster count

Completed: 2026-06-24

- Selected surface: `games/event_frontier/src/setup.rs::setup_match` roster
  predicate.
- Change: replaced the bare `seats.len()` comparison with
  `SeatCount::new(seats.len()).map(SeatCount::get)` compared against the
  game-owned `STANDARD_SEAT_COUNT`.
- ADR-0009 classification: `unchanged`; accepted/rejected counts, diagnostics,
  deck setup, replay hashes, fixtures, and exports were not intentionally
  migrated.
- Compatibility / rollback: restore only the bare roster-length predicate.
  Variant seat count, faction identity/order, event/resource setup, and
  asymmetric two-seat policy stay game-owned.
- Verification:
  - `cargo test -p event_frontier` passed, including exact diagnostic checks for
    0, 1, and 3 seats.
  - `cargo run -p replay-check -- --game event_frontier --all` passed; all
    Event Frontier traces were accepted.
  - `cargo run -p fixture-check -- --game event_frontier` passed.
  - No golden trace, fixture, export, or variant policy file changed.

### 8CR3PUBCOOASY-308 - Event Frontier variant seat-count predicate

Completed: 2026-06-24

- Selected surface: `games/event_frontier/src/setup.rs::validate_variant`
  `variant.seat_count` predicate.
- Change: replaced the bare variant seat-count comparison with
  `SeatCount::new(variant.seat_count as usize).map(SeatCount::get)` compared
  against the game-owned `STANDARD_SEAT_COUNT`.
- ADR-0009 classification: `unchanged`; variant acceptance/diagnostic, faction
  identity/order, event/resource setup, replay hashes, fixtures, and exports
  were not intentionally migrated.
- Compatibility / rollback: restore only the bare `variant.seat_count`
  comparison. Roster validation, faction identity/order, event/resource setup,
  and deck setup stay game-owned.
- Verification:
  - `cargo test -p event_frontier` passed, including exact diagnostic checks for
    variant seat counts 0, 1, and 3.
  - `cargo run -p replay-check -- --game event_frontier --all` passed; all
    Event Frontier traces were accepted.
  - `cargo run -p fixture-check -- --game event_frontier` passed.
  - No golden trace, fixture, export, or variant policy file changed.

### 8CR3PUBCOOASY-401 - Plain Tricks action-tree v1 parallel surface

Completed: 2026-06-24

- Selected surfaces: `games/plain_tricks/src/replay_support.rs` and
  `games/plain_tricks/tests/replay.rs`.
- Change: added parallel `action_tree_v1_bytes` and `action_tree_v1_hash`
  helpers using `ActionTreeEncodingVersion::V1`, while retaining the existing
  local `action_tree_hash` unchanged. Added representative v1 vectors for
  opening trick, forced follow-suit, void/free discard, final play, and terminal
  empty tree.
- ADR-0009 classification: `unchanged` for existing surfaces; the v1
  action-tree bytes/hash are additive. Local action-tree hash, state/effect/view
  hashes, traces, fixtures, exports, legal choices, metadata, labels, and
  branch order were not intentionally migrated.
- Compatibility / rollback: remove only the v1 helper functions and replay
  vector test. Existing local hash and all replay/export consumers remain
  unchanged.
- Verification:
  - `cargo test -p plain_tricks` passed, including the new v1 vectors:
    opening `(bytes=3209, hash=10760653848758353227, local=9608973152758876482)`,
    forced-follow `(bytes=1850, hash=10249125325511701213, local=11988930228804901292)`,
    void/free `(bytes=1874, hash=13864411618449214495, local=2830033628787621803)`,
    final-play `(bytes=932, hash=10622526245863211658, local=12733681326737878192)`,
    terminal-empty `(bytes=64, hash=17407510006563527667, local=117586594652395198)`.
  - `cargo run -p replay-check -- --game plain_tricks --all` passed; all Plain
    Tricks traces passed with existing expected hashes.
  - `cargo run -p fixture-check -- --game plain_tricks` passed.
  - No golden trace, fixture, export, state/effect/view hash, or local
    action-tree hash surface changed.

### 8CR3PUBCOOASY-402 - Flood Watch action-tree v1 parallel surface

Completed: 2026-06-24

- Selected surfaces: `games/flood_watch/src/visibility.rs`,
  `games/flood_watch/src/lib.rs`, and `games/flood_watch/tests/replay.rs`.
- Change: added parallel `action_tree_v1_bytes` and `action_tree_v1_hash`
  helpers using `ActionTreeEncodingVersion::V1`, while retaining the existing
  debug-derived local `action_tree_hash` unchanged. Added representative v1
  vectors for bail/place-levee, Levee Warden role-power tree, early end next
  turn, budget-exhausted/automatic-environment empty tree, and terminal empty
  tree.
- ADR-0009 classification: `unchanged` for existing surfaces; the v1
  action-tree bytes/hash are additive. Local action-tree hash, state/effect/view
  hashes, traces, fixtures, exports, legal choices, metadata, labels, and
  branch order were not intentionally migrated.
- Compatibility / rollback: remove only the v1 helper functions, re-export, and
  replay vector test. Existing local hash and all replay/export consumers remain
  unchanged.
- Verification:
  - `cargo test -p flood_watch` passed, including the new v1 vectors:
    bail/levee `(bytes=3920, hash=2247660004428458771, local=4425850002041434203)`,
    role-power `(bytes=3920, hash=4532944654053335564, local=8946559128574054524)`,
    early-end `(bytes=4375, hash=6356390137971522057, local=13133754107875012264)`,
    budget-empty `(bytes=64, hash=828296343441045014, local=9791162161922510910)`,
    terminal-empty `(bytes=64, hash=828296343441045014, local=9791162161922510910)`.
  - `cargo run -p replay-check -- --game flood_watch --all` passed; all Flood
    Watch traces were accepted.
  - `cargo run -p fixture-check -- --game flood_watch` passed.
  - No golden trace, fixture, export, state/effect/view hash, or local
    action-tree hash surface changed.

### 8CR3PUBCOOASY-403 - Frontier Control action-tree v1 parallel surface

Completed: 2026-06-24

- Selected surfaces: `games/frontier_control/src/visibility.rs`,
  `games/frontier_control/src/lib.rs`, and
  `games/frontier_control/tests/replay.rs`.
- Change: added parallel `action_tree_v1_bytes` and `action_tree_v1_hash`
  helpers using `ActionTreeEncodingVersion::V1`, while retaining the existing
  debug-derived local `action_tree_hash` unchanged. Added representative v1
  vectors for opening moves, move/clash branch with muster and stake,
  stake-available tree, dismantle/reinforce tree, early end next turn, and
  terminal empty tree.
- ADR-0009 classification: `unchanged` for existing surfaces; the v1
  action-tree bytes/hash are additive. Local action-tree hash, state/effect/view
  hashes, traces, fixtures, exports, legal choices, metadata, labels, and
  branch order were not intentionally migrated.
- Compatibility / rollback: remove only the v1 helper functions, re-export, and
  replay vector test. Existing local hash and all replay/export consumers remain
  unchanged.
- Verification:
  - `cargo test -p frontier_control` passed, including the new v1 vectors:
    opening `(bytes=1291, hash=14934942909345403747, local=16277890795749786444)`,
    move/clash `(bytes=3310, hash=4769522588459725601, local=8239912348712405228)`,
    stake `(bytes=2601, hash=12908324649299837008, local=11013731039854121046)`,
    dismantle `(bytes=5890, hash=4031145394212002295, local=26708586450493490)`,
    early-end `(bytes=4092, hash=480402586032591446, local=16861215057075239797)`,
    terminal-empty `(bytes=64, hash=17387353871007407771, local=10022657772393329959)`.
  - `cargo run -p replay-check -- --game frontier_control --all` passed; all
    Frontier Control traces were accepted.
  - `cargo run -p fixture-check -- --game frontier_control` passed.
  - No golden trace, fixture, export, state/effect/view hash, or local
    action-tree hash surface changed.

### 8CR3PUBCOOASY-404 - Event Frontier action-tree v1 parallel surface

Completed: 2026-06-24

- Selected surfaces: `games/event_frontier/src/visibility.rs`,
  `games/event_frontier/src/lib.rs`, and
  `games/event_frontier/tests/replay.rs`.
- Change: added parallel `action_tree_v1_bytes` and `action_tree_v1_hash`
  helpers using `ActionTreeEncodingVersion::V1`, while retaining the existing
  debug-derived local `action_tree_hash` unchanged. Added representative v1
  vectors for full multi-site operation, limited second-choice operation, event
  choice, pass-after-event, Survey Ban blocked branch, Reckoning empty tree, and
  terminal empty tree.
- ADR-0009 classification: `unchanged` for existing surfaces; the v1
  action-tree bytes/hash are additive. Local action-tree hash, state/effect/view
  hashes, traces, fixtures, exports, legal choices, metadata, labels, branch
  order, and hidden deck-order surfaces were not intentionally migrated.
- Compatibility / rollback: remove only the v1 helper functions, re-export, and
  replay vector test. Existing local hash and all replay/export consumers remain
  unchanged.
- Verification:
  - `cargo test -p event_frontier` passed, including the new v1/no-hidden-deck
    vectors: full-operation `(bytes=5724, hash=12263323764607805373, local=12025048674674442718)`,
    limited-operation `(bytes=2688, hash=5287035841278219952, local=1262519681689202196)`,
    event-choice `(bytes=3776, hash=6239437208328345357, local=12651858397689234283)`,
    pass `(bytes=5418, hash=18107612798635470515, local=9761797406534023113)`,
    edict-blocked `(bytes=2924, hash=17452768486966187756, local=16133410113579192678)`,
    reckoning-empty `(bytes=64, hash=17387353871007407771, local=10022657772393329959)`,
    terminal-empty `(bytes=64, hash=17387353871007407771, local=10022657772393329959)`.
  - `cargo run -p replay-check -- --game event_frontier --all` passed; all
    Event Frontier traces were accepted.
  - `cargo run -p fixture-check -- --game event_frontier` passed.
  - No golden trace, fixture, export, state/effect/view hash, local action-tree
    hash, or hidden deck-order surface changed.
