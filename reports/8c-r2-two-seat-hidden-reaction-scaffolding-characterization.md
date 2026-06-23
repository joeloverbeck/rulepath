# Unit 8C-R2 Characterization Baseline

Date: 2026-06-23
Baseline commit: `51a5c12636696d974b9491cc49bcff5590fca64b`
Ticket: `tickets/UNI8CR2TWOSEA-001.md`
Reference: `specs/unit-8c-r2-two-seat-hidden-reaction-scaffolding-intermediate-spec.md`

This report is the pre-migration baseline for Unit 8C-R2. It records current
surfaces only; it does not authorize any byte, hash, visibility, seat-ID, RNG,
fixture, or golden-trace change. Later tickets must amend their before/after
evidence here or cite this file before migrating a selected surface.

## Authority And Determination

- `specs/README.md` records `8C-R1` as `Done` and `8C-R2` as the lowest active
  not-started C-11 follow-on row. `8C-R3`, `8C-R4`, and Gate 18 remain successor
  work.
- `docs/MECHANIC-ATLAS.md` section 10A records `Current debt: _None_`.
- `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` seeds
  exactly the R2 game set: `high_card_duel`, `secret_draft`, `poker_lite`, and
  `masked_claims`.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` contains governing entries
  `MSC-8C-001` through `MSC-8C-010`. R2 appends receipts beneath those entries;
  it does not create a rival register.
- High Card Duel's Unit 8C pilot is treated as discharging only the named C-07
  no-leak geometry surface. High Card C-04/C-05 action-tree v1 remains a
  `migrate` surface for this unit.

## Verdict Matrix

| Game | C-01 effects | C-02 seats | C-03 count | C-04 tree v1 | C-05 writer v1 | C-06 dev support | C-07 no-leak | C-08 profiles | C-09 RNG | C-10 policy |
|---|---|---|---|---|---|---|---|---|---|---|
| `high_card_duel` | migrate public/private | migrate parser; WASM import adapter already shipped; legacy roster exception | migrate | migrate | migrate | already-discharged-by-8C-pilot | already-discharged-by-8C-pilot | migrate, with seat-private export N/A | migrate | local-only |
| `secret_draft` | migrate public; private constructor N/A | migrate parser; WASM import adapter already shipped; legacy roster exception | migrate | migrate | migrate | migrate dev-only | migrate | migrate, including seat-private export profile | N/A | local-only |
| `poker_lite` | migrate public/private | migrate parser; WASM import adapter already shipped; legacy roster exception | migrate | migrate | migrate | migrate dev-only | migrate | migrate, including seat-private export profile | migrate | local-only |
| `masked_claims` | migrate public; private constructor N/A | migrate parser; adapter/output already canonical | migrate | migrate | migrate | migrate dev-only | migrate | migrate, with seat-private export N/A | migrate | local-only |

## Owner Paths And Symbols

| Surface | High Card Duel | Secret Draft | Poker Lite | Masked Claims |
|---|---|---|---|---|
| C-01 public effect | `games/high_card_duel/src/effects.rs::public_effect` | `games/secret_draft/src/effects.rs::public_effect` | `games/poker_lite/src/effects.rs::public_effect` | `games/masked_claims/src/effects.rs::public_effect` |
| C-01 private effect | `games/high_card_duel/src/effects.rs::private_effect` | N/A, no seat-private effect constructor | `games/poker_lite/src/effects.rs::private_effect` | N/A, no seat-private effect constructor |
| C-02 parser | `games/high_card_duel/src/ids.rs::HighCardDuelSeat::parse` | `games/secret_draft/src/ids.rs::SecretDraftSeat::parse` | `games/poker_lite/src/ids.rs::PokerLiteSeat::parse` | `games/masked_claims/src/ids.rs::MaskedClaimsSeat::parse` |
| C-02 WASM import | `crates/wasm-api/src/seats.rs::parse_high_card_seat` | `crates/wasm-api/src/seats.rs::parse_secret_seat` | `crates/wasm-api/src/seats.rs::parse_poker_seat` | `crates/wasm-api/src/seats.rs::parse_masked_seat` |
| C-03 setup count | `games/high_card_duel/src/setup.rs::setup_match` | `games/secret_draft/src/setup.rs::setup_match` | `games/poker_lite/src/setup.rs::setup_match` | `games/masked_claims/src/setup.rs::setup_match` |
| C-04/C-05 action evidence | `games/high_card_duel/src/replay_support.rs`; `legal_action_tree` | `games/secret_draft/src/replay_support.rs::action_tree_hash` | `games/poker_lite/src/replay_support.rs::action_tree_hash` | `games/masked_claims/src/replay_support.rs`; `games/masked_claims/src/actions.rs::legal_action_tree` |
| C-07 no-leak evidence | `games/high_card_duel/tests/visibility.rs`; `tests/bots.rs` | `games/secret_draft/tests/visibility.rs`; `tests/bots.rs`; `tests/replay.rs` | `games/poker_lite/tests/visibility.rs`; `tests/bots.rs`; `tests/replay.rs` | `games/masked_claims/tests/visibility.rs`; `tests/bots.rs`; `tests/replay.rs` |
| C-08 replay/profile evidence | `src/replay_support.rs`; `tests/replay.rs` | `src/replay_support.rs`; `tests/replay.rs` | `src/replay_support.rs`; `tests/replay.rs` | `src/replay_support.rs`; `tests/replay.rs` |
| C-09 RNG | `games/high_card_duel/src/setup.rs::next_bounded_index_unbiased` | N/A | `games/poker_lite/src/setup.rs::next_bounded_index_unbiased` | `games/masked_claims/src/setup.rs::next_bounded_index_unbiased` |

Shared helpers currently available:

- `engine_core::EffectEnvelope::public`
- `engine_core::EffectEnvelope::private_to`
- `engine_core::SeatId::parse_canonical`
- `game_stdlib::seat::SeatCount`
- `game_test_support::no_leak::assert_pairwise_no_leak`
- `game_test_support::profiles::{ReplayCommandV1Driver, PublicExportV1Driver, SeatPrivateExportV1Driver, SetupEvidenceV1Driver}`
- `engine_core::DeterministicRng::next_index_unbiased_v1`

## Current Seat Grammar And Compatibility

All four game-local seat parsers currently accept only canonical Rust seat IDs:
`seat_0` and `seat_1`. They reject role labels, aliases, hyphenated spellings,
out-of-range seats, and ambiguous strings in game crates.

WASM import aliases are contained in `crates/wasm-api/src/seats.rs`.
`parse_high_card_seat`, `parse_secret_seat`, and `parse_poker_seat` keep legacy
runtime-roster/trace compatibility exceptions; `parse_masked_seat` is already
canonical on output. TypeScript remains presentation-only and performs no seat
normalization.

## C-01 Effect Baseline

Current constructors use local literals:

- `public_effect` constructs `EffectEnvelope { visibility: VisibilityScope::Public, payload }`.
- High Card and Poker `private_effect` construct
  `EffectEnvelope { visibility: VisibilityScope::PrivateToSeat(owner_seat_id), payload }`.

Migration scope is constructor-only. Payload formation, effect ordering,
recipient selection, reveal policy, redaction, and stable effect strings remain
game-owned. Secret Draft and Masked Claims have no seat-private effect
constructor at this baseline; that is a C-01 N/A, not permission to weaken
no-leak evidence.

## C-03 Setup Count Baseline

| Game | Current count check | Diagnostic baseline |
|---|---|---|
| `high_card_duel` | two seats compared through current `setup_match` and variant expectations | preserve existing invalid count diagnostic, state bytes, shuffle/deal behavior |
| `secret_draft` | `seats.len() != STANDARD_SEAT_COUNT as usize` | `code=invalid_seat_count`, message `secret_draft requires exactly two seats` |
| `poker_lite` | fixed-two setup predicate in `setup_match` | preserve current `invalid_seat_count` diagnostic and setup/deal bytes |
| `masked_claims` | fixed-two setup predicate in `setup_match` | preserve current `invalid_seat_count` diagnostic and setup/deal bytes |

`SeatCount::next_ring_index` is not applicable to R2. Existing two-seat
`other()` helpers and phase-order rules remain game-local.

## C-04/C-05 Stable Byte Baseline

Selected surface: action-tree v1 bytes/hash only. Adjacent state, effect, view,
replay-command, export, and diagnostic byte/hash surfaces are exceptions for
this unit unless a later ticket explicitly records a separate ADR-0009 packet.

Current action-tree authorities:

- High Card: game replay tests compute current action-tree hashes from
  `legal_action_tree`; no game-owned v1 wrapper exists.
- Secret Draft: `games/secret_draft/src/replay_support.rs::action_tree_hash`
  is the current legacy string hash.
- Poker Lite: `games/poker_lite/src/replay_support.rs::action_tree_hash` is the
  current legacy string hash.
- Masked Claims: `games/masked_claims/src/actions.rs::legal_action_tree` drives
  compound claim and pending-response trees; no game-owned canonical tree hash
  wrapper exists.

Expected ADR-0009 class for C-04/C-05 migration tickets:

- High Card and Masked Claims: `parallel-new-surface`.
- Secret Draft and Poker Lite: legacy hash remains readable and authoritative;
  add a parallel v1 surface.

## C-07 No-Leak Geometry Baseline

High Card already uses `game_test_support::no_leak::assert_pairwise_no_leak` in
`games/high_card_duel/tests/visibility.rs::no_leak_harness_covers_public_seat_replay_effect_and_bot_surfaces`.
Its C-07 receipt is verified, not rebuilt. Existing probes cover observer,
seat_0, and seat_1 over view, action tree, diagnostic, filtered effect, public
export, bot input, and reveal-safe surfaces.

R2 still requires Secret Draft, Poker Lite, and Masked Claims to adopt the
shared geometry while retaining their game-specific reveal/reaction assertions.
Canaries must be constructed in memory only and must not be written to traces,
fixtures, snapshots, logs, DOM/test IDs, browser storage, or accessibility
artifacts.

### Matrix Expectations

Abbreviations: `A` absent, `P` present, `N/A` not applicable. Each source row
is run for `seat_0` and `seat_1` where the game owns the source datum.

| Game / datum and surface | Observer | Owner | Opponent |
|---|---:|---:|---:|
| HCD unrevealed private card -> projected view | A | P | A |
| HCD unrevealed private card -> action/diagnostic/public export/bot rationale | A | A | A |
| HCD private deal/commit effect -> filtered effects | A | P | A |
| HCD after authorized reveal -> public view/effect/export | P | P | P |
| Secret pre-reveal committed item -> view/action/diagnostic/effect/public export/seat-private export/bot rationale | A | A | A |
| Secret internal command trace | N/A | N/A | N/A |
| Secret after synchronized reveal -> public view/effect/export | P | P | P |
| Poker private crest before showdown -> projected view | A | P | A |
| Poker private crest before showdown -> action/diagnostic/public export/bot rationale | A | A | A |
| Poker private setup/choice effect -> filtered effects | A | P | A |
| Poker seat-private export before showdown | A | P | A |
| Poker showdown reveal -> public view/export | P | P | P |
| Poker yielded losing crest -> public/opponent export | A | P or N/A | A |
| Masked hand tile or pending claim identity -> view/action/diagnostic/effect/public export/bot rationale | A | P or N/A per current owner view | A |
| Masked accepted secret resolution -> public/opponent surfaces | A | N/A | A |
| Masked challenged tile after authorized reveal -> public view/effect/export | P | P | P |

## C-08 Evidence Profiles

| Profile | High Card Duel | Secret Draft | Poker Lite | Masked Claims |
|---|---|---|---|---|
| `replay-command-v1` | migrate; internal-dev trace bytes remain authority | migrate; internal-dev | migrate; internal-dev | migrate; existing rule/replay builder only |
| `setup-evidence-v1` | migrate metadata only; private deal assertions remain internal-dev | migrate setup pool/empty commitment metadata | migrate setup/deck metadata | migrate mask ordering/status metadata |
| `public-export-v1` | migrate observer-only `export_public_observer_replay` | migrate observer invocation of `export_public_replay` | migrate observer invocation of `export_public_replay` | migrate observer-only `PublicReplayExport` path |
| `seat-private-export-v1` | N/A: no official seat-private replay exporter | migrate for `seat_0` and `seat_1`, preserving pre-reveal redaction | migrate for `seat_0` and `seat_1`, preserving own-hand-only access | N/A: official import/export path is observer-only |
| `domain-evidence-v1` | N/A | N/A | N/A | N/A |

Profile drivers validate metadata and delegate behavior to existing game/tool
validators. They do not parse commands, project views, authorize exports, or
interpret fixture behavior.

## C-06/C-09/C-10 Checkpoints

| Game | C-06 dev-only support | C-09 bounded index | C-10 non-promotion |
|---|---|---|---|
| `high_card_duel` | already has `game-test-support` under `[dev-dependencies]` only | migrate local unbiased sampler; existing fixed vector covers upper bound 3 -> index 1 and zero -> None | keep shuffle/deal/commit/reveal/outcome local |
| `secret_draft` | add `game-test-support` as dev-only when tests need it | not applicable, no RNG/bounded-index rule surface | keep simultaneous commitment/reveal and visible-pool resolution local |
| `poker_lite` | add `game-test-support` as dev-only when tests need it | migrate local rejection sampler | keep pledge rounds, showdown, pool allocation, and scoring local |
| `masked_claims` | add `game-test-support` as dev-only when tests need it | migrate local rejection sampler | keep reaction window, pending responder, redaction, challenge reveal, and outcome local |

Every C-09 migration must prove identical returned indices, rejection draw
counts, complete shuffle/deal vectors, state/effect/view/export hashes, and
golden traces. Expected ADR-0009 class is `unchanged`; any divergence blocks
the game-specific migration.

## Accepted Exceptions And Triggers

| Surface | Owner | Compatibility window | Rollback | Next trigger |
|---|---|---|---|---|
| HCD/Secret/Poker legacy runtime roster spelling | `crates/wasm-api` | preserved through R2 | evidence-only; no output flip in parser tasks | dedicated WASM runtime-seat migration with state/effect/view/hash compatibility |
| Legacy Secret/Poker action-tree hash | owning game `replay_support.rs` | readable alongside new v1 parallel evidence | remove only v1 adapter/tests | future authority flip with ADR-0009 packet |
| State/effect/view/replay/export/diagnostic bytes | owning game and tool validators | unchanged through R2 | revert selected surface only | separately named migration per surface |
| HCD/Masked seat-private replay export | owning game export APIs | N/A until official exporter exists | no code created | future exporter admission, if any |
| All `domain-evidence-v1` rows | no distinct domain fixture | N/A | no code created | future distinct domain-evidence fixture |

## Fixture And Golden Trace SHA-256 Baseline

### High Card Duel

| Path | SHA-256 |
|---|---|
| `games/high_card_duel/data/fixtures/high_card_duel_standard.fixture.json` | `b472fe99ab55d9cdc9c3bf7746d01ad16265d39fea3c5a8d020aa6392810991e` |
| `games/high_card_duel/tests/golden_traces/bot-action.trace.json` | `f9b41953df0d9a0ca9801de2508a4c69b732028943b12b3d42d5625d9723ee5e` |
| `games/high_card_duel/tests/golden_traces/hidden-info-public-observer.trace.json` | `32867983cc886d53f889f6fb9a14120064f84a9d11f684872679420ef4dff0dd` |
| `games/high_card_duel/tests/golden_traces/invalid-private-card-redacted.trace.json` | `ec743b034e4e537a30a94838be38cfe7b82abf43708b6919d8cb004b2ba9b4cb` |
| `games/high_card_duel/tests/golden_traces/invalid-wrong-seat-diagnostic.trace.json` | `90b4e11e5c5b7860adfaadfa8eb1d5d118b3fce2edf1714dc8d3f27d48311ed2` |
| `games/high_card_duel/tests/golden_traces/public-replay-export-import.trace.json` | `ce76d6b594b9c3191aab32e295bf6b6f4b33bcb9153a88b1e70308c89acd361f` |
| `games/high_card_duel/tests/golden_traces/seat-private-view.trace.json` | `3d012887057d2af1c6826bc87658972bd74dca9ffd4a8eecc6c531e9c71e5b9f` |
| `games/high_card_duel/tests/golden_traces/shortest-normal.trace.json` | `bc77f5bc76c2459ddec1a48115eb5d00c3b4e646d6d281250acdd1e7c63b9db6` |
| `games/high_card_duel/tests/golden_traces/stale-diagnostic.trace.json` | `41ef1ad5d71505dfca8d5b49765cc52109d162cfd673a9126719446aa4caf6d1` |
| `games/high_card_duel/tests/golden_traces/terminal.trace.json` | `d5c3615118fb6eb4f6192177783981ee1e5cced2dcb276e0055001b392072701` |
| `games/high_card_duel/tests/golden_traces/tie-round.trace.json` | `6a450a4521889070933871d134a78d5c9b3d4e74e79a4fcb2a7112f1c5e27f79` |

### Secret Draft

| Path | SHA-256 |
|---|---|
| `games/secret_draft/data/fixtures/secret_draft_standard.fixture.json` | `a3dd7a8e00b4c8d9335b55d0eaf615d40bdfc15e25a4260ec919d561defd7391` |
| `games/secret_draft/tests/golden_traces/already-committed-diagnostic.trace.json` | `507a4e791e062ef2c38bc68e12bcd3deabd490b690d35a8b44c57e53a5a747d4` |
| `games/secret_draft/tests/golden_traces/bot-action.trace.json` | `af18b1a71bec33518a86de7cf5b13cfb94633203e479f066e76bd74f5fba6e64` |
| `games/secret_draft/tests/golden_traces/contested-pick-fallback.trace.json` | `f133cec43457d90689d26700ffd3a42d0b7e20675a22c92f5430e29b14ff3b37` |
| `games/secret_draft/tests/golden_traces/draw-after-tie-breaks.trace.json` | `4787d8a29a1ab2248e8ea914720edc3bb81f1b1d47db0132da3fb1b12c979203` |
| `games/secret_draft/tests/golden_traces/first-commit-pending.trace.json` | `114f6e89df958ea92207f86847ac64a4baf096eae74d010dc5c1ba3f117d713b` |
| `games/secret_draft/tests/golden_traces/public-observer-no-leak.trace.json` | `f7200a691357da86f92f7d22c2bd8299263f5694afe6c277765e54e184699009` |
| `games/secret_draft/tests/golden_traces/public-replay-export-import.trace.json` | `6c96f20571ac813076428bad14fb2c9fcbb35ae3a0f783aeaf6ad21b764a04f2` |
| `games/secret_draft/tests/golden_traces/seat-private-no-prereveal-choice.trace.json` | `1b170d28025460013cc3664ca1d7d3aaa2bf76e425d4fcc42eaf3dbbd89c5511` |
| `games/secret_draft/tests/golden_traces/shortest-normal.trace.json` | `f0cf0b114d2b9bb38e60bc5b45e3a4a2d72d80b1f0e509926bdf4f48d3b2f8b6` |
| `games/secret_draft/tests/golden_traces/simultaneous-reveal-batch.trace.json` | `aada04b2c87b95a51a95175f5f45591b1bb3fdd62bd4efc6fc1fedca39a051de` |
| `games/secret_draft/tests/golden_traces/stale-diagnostic.trace.json` | `f6b68e6ad1db565817482245fb9ef65a7d290bdb6fcd7b092ed05095f6ad47d1` |
| `games/secret_draft/tests/golden_traces/terminal-tie-break.trace.json` | `243d99b5898c34bb39f58ac9951a6c2a6e2725c6b4c35fcb7b106f093a2cf333` |
| `games/secret_draft/tests/golden_traces/unavailable-item-diagnostic.trace.json` | `d3c9512d1f2c23d88b2ec6e64153116869aefc56ba5abddb907f73572ad1fdd5` |
| `games/secret_draft/tests/golden_traces/wasm-exported.trace.json` | `9f0747169ef9c3d1bf554bad8f7889389b26e94e4f0b27759112e4301037d557` |

### Poker Lite

| Path | SHA-256 |
|---|---|
| `games/poker_lite/data/fixtures/poker_lite_standard.fixture.json` | `d556ed6e68cfc979ef6c28d9dc984a49491a7a4e9c254348b7623ecebb527850` |
| `games/poker_lite/tests/golden_traces/bot-action.trace.json` | `ffa8693568912f719bc9ccdec354f72efb4124fd996cb46fa6ba8f7eaa205713` |
| `games/poker_lite/tests/golden_traces/deal-private-no-leak.trace.json` | `43ec409f8e12d1e4329be6859b28d3f08ae8b40fca5e655c557fe105592e9afb` |
| `games/poker_lite/tests/golden_traces/high-card-showdown.trace.json` | `1b7c1378ddc55063cbbbee99c95f033a5e17b84efc9d193fbad3750d9f954dbd` |
| `games/poker_lite/tests/golden_traces/hold-hold-center-reveal.trace.json` | `0a3548e2e5eb5f2103cc62faa835657bb3e2ee784b32c528132c99f78cf12fc2` |
| `games/poker_lite/tests/golden_traces/invalid-lift-cap-diagnostic.trace.json` | `93f595d00b5c1a828c39110749ef806bb7e85b0c3d8af061c198750b60a712a5` |
| `games/poker_lite/tests/golden_traces/invalid-private-card-redacted.trace.json` | `de7428b335ee9e5f76a1fcdeb7304bbc130befbe8908e1a32f8e1b53ca30195a` |
| `games/poker_lite/tests/golden_traces/invalid-stale-diagnostic.trace.json` | `66377353e78fa4ca04608dd71798b7aa3f835d1c1f92b0334956ac1e55c139fe` |
| `games/poker_lite/tests/golden_traces/invalid-wrong-seat-diagnostic.trace.json` | `4e21de3d08f743406ac54cb923199483c1182d327e73f37b8f37a4582d793fff` |
| `games/poker_lite/tests/golden_traces/lift-match-showdown.trace.json` | `1d162377def79145968a52c097a3578036c3223c73dc49f18458aa6e9269e1d6` |
| `games/poker_lite/tests/golden_traces/no-leak-public-observer.trace.json` | `614d2ddbf0dfb77e80efe1eec4d467254fbd7c08c442727c1a4b98a02551ad55` |
| `games/poker_lite/tests/golden_traces/pair-beats-high-card.trace.json` | `d2be9d74f529c04976834a396a3e3031c7ca004bb50d82657292a4f5c8562e3f` |
| `games/poker_lite/tests/golden_traces/press-match-showdown-reveal.trace.json` | `befdf890f0990ad3a808b59e7d799c560b41fc514059d307759dd57a886cdabf` |
| `games/poker_lite/tests/golden_traces/public-replay-export-import.trace.json` | `b862902506147fee07af7f0338682eb25b08819f4cdf7a9678193bda053c20ac` |
| `games/poker_lite/tests/golden_traces/seat-private-view.trace.json` | `ea27370b0f32265e4e783d38ab16cd8a035eb523b694522b197de7b6d6eaf514` |
| `games/poker_lite/tests/golden_traces/tie-split.trace.json` | `b97a1ad6a29fe35e27d56b71f2b07d601e56c1bcfd125f6aa291f0a8a09ba10b` |
| `games/poker_lite/tests/golden_traces/wasm-exported.trace.json` | `2282078e860cdb36152dc83b4c23d7ede1646987fed2e92242cab2d1abf517a0` |
| `games/poker_lite/tests/golden_traces/yield-terminal-no-showdown.trace.json` | `94ee4060c9693764a4d349b7d14ee4eda31f23d2bb88590986828e22912a66a9` |

### Masked Claims

| Path | SHA-256 |
|---|---|
| `games/masked_claims/data/fixtures/masked_claims_standard.fixture.json` | `68eb8bcdb5ee398163a78cb442cd1fe18c728fa929aae1c9a7b3881912ad82b2` |
| `games/masked_claims/tests/golden_traces/accept-resolution.trace.json` | `91397c6793a1b90670bf5af4ede17227e9e15fd5b5b4789b092abbf0e33f32ac` |
| `games/masked_claims/tests/golden_traces/accepted-mask-never-revealed.trace.json` | `468a7cfb887e1d8739b13037f84a98f418c8159dccf09a98b024433a5932059c` |
| `games/masked_claims/tests/golden_traces/bot-claim-and-response.trace.json` | `d9692e4eb44f82b6f67411e80b73886d4c95da48106414f47e335a546c1e430c` |
| `games/masked_claims/tests/golden_traces/certain-lie-challenge.trace.json` | `cf10ae66b6af1686ff071d65c22bd9b62645d2e0652c8c4200da63bed28de10b` |
| `games/masked_claims/tests/golden_traces/challenge-exposed-lie.trace.json` | `dadb9c74447f3cbd41a6909368f6e7abcc310f3693dc8d627b392261b8534c4a` |
| `games/masked_claims/tests/golden_traces/challenge-honest-reveal.trace.json` | `32244238eb4b8b97b7742a1430dd9ef87207b7bbb74dd806512771822d29c02c` |
| `games/masked_claims/tests/golden_traces/claim-pending-window.trace.json` | `3a61da9c6167fab5c639f7c8dadc21085c62c4a1e70bd3028634377d8d8e048b` |
| `games/masked_claims/tests/golden_traces/draw-after-tie-breaks.trace.json` | `b4c3c117a69def7944070ac0e8968d5c4a68d3aeafb9bb3b496c91a052b4bb98` |
| `games/masked_claims/tests/golden_traces/public-observer-no-leak.trace.json` | `ff4352da676557e8b77b1826e3bbccdc73ccd59104164e2fe6e6a3210f6b36ac` |
| `games/masked_claims/tests/golden_traces/public-replay-export-import.trace.json` | `2d2bfab48767bf89224d7b1a02141618c2430fd290d455db3f5c523c209b5fb6` |
| `games/masked_claims/tests/golden_traces/shortest-normal.trace.json` | `9619a6015a784bc3ce834ea74288b4994448f1fbf75f15190d872c2e26460316` |
| `games/masked_claims/tests/golden_traces/stale-diagnostic.trace.json` | `fef9640eba883ee1c2890ccc40b3ea756814fbdea213decb11294fb34c22e05e` |
| `games/masked_claims/tests/golden_traces/terminal-tie-break.trace.json` | `6851e9c454d238ef3e8416c6cae82a5b3e8046fbf2db36f469344f36155372c4` |
| `games/masked_claims/tests/golden_traces/underclaim-trap-reveal.trace.json` | `e70a93d15e6ad70bb0e21799468ae98aabc079704c2f7567453a5c4bfa118a5c` |
| `games/masked_claims/tests/golden_traces/unowned-tile-diagnostic.trace.json` | `3056bb1d2c0c4e5fec0baba3020ea7c844ccc35b5f0317a84afefd06644d81c1` |
| `games/masked_claims/tests/golden_traces/wrong-phase-claim-diagnostic.trace.json` | `77322e574f4afb6d0c097cc079de29f5b5d8ea25bbcd629df301596a0e50b936` |
| `games/masked_claims/tests/golden_traces/wrong-seat-response-diagnostic.trace.json` | `40e31c9b3444b516c90f251ad5af252295aabb8e005d67c992a2a5b2fa19d743` |

## Stable Hash Surface Anchors

The golden traces above contain the current `expected_state_hashes`,
`expected_effect_hashes`, `expected_action_tree_hashes`,
`expected_public_view_hashes`, `expected_private_view_hashes`, and
`expected_replay_hashes` where each game supports those surfaces. They remain
the current byte/hash authority until an owning ticket records a before/after
packet. Representative anchors:

- HCD `shortest-normal`: state `4231217801294566961`, effect
  `4913717750046711716`, action tree `18301130017473991288`, public view
  `8209246180121319086`, replay `8132355332930551567`.
- HCD `seat-private-view`: private view `seat_0=6854759188159202200`.
- Secret `shortest-normal`: state `15264312985025926927`, effect
  `6300494095297500087`, action tree `13944979233763565434`, replay
  `13982334687289115299`.
- Poker `public-replay-export-import`: all hash fields remain in
  `games/poker_lite/tests/golden_traces/public-replay-export-import.trace.json`;
  the SHA-256 above is the ticket-001 file anchor.
- Masked `public-observer-no-leak`: state `11401`, effect `11402`,
  action tree `11403`, public view `11404`, replay `11405`.

## Verification Evidence

Baseline commands run for ticket 001:

| Command | Status | Notes |
|---|---|---|
| `cargo run -p replay-check -- --game high_card_duel --all` | pass | 10 HCD traces checked; output ended `replay-check: all traces passed`. |
| `cargo run -p replay-check -- --game secret_draft --all` | pass | 14 Secret Draft traces checked; output ended `replay-check: all traces passed`. |
| `cargo run -p replay-check -- --game poker_lite --all` | pass | Poker traces checked; output included `bot-action.trace.json: not-applicable trace accepted` and `wasm-exported.trace.json: public export fixture accepted`, then `replay-check: all traces passed`. |
| `cargo run -p replay-check -- --game masked_claims --all` | pass | 17 Masked Claims traces accepted through the current not-applicable trace path; output ended `replay-check: all traces passed`. |
| `cargo test --workspace --all-targets` | pass | Cargo exited 0. This command also executed benchmark target binaries under `--all-targets`; some older benchmark rows printed `pass=false` as benchmark data, but no Rust test failed. |

Changed-artifact inventory at report creation:

- New file: `reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md`.
- No production code, fixtures, golden traces, schemas, or WASM/browser files
  were changed for this report-only ticket.

## Migration Evidence Ledger

### UNI8CR2TWOSEA-002 - High Card Duel public effect constructor

Selected surface: `games/high_card_duel/src/effects.rs::public_effect`.

Before state: local literal constructor
`EffectEnvelope { visibility: VisibilityScope::Public, payload }`.

After state: `EffectEnvelope::public(payload)`.

ADR-0009 classification: `unchanged`. This changes only generic envelope
construction and preserves payload text, visibility scope, effect ordering,
stable effect strings, replay hashes, and viewer filtering. Rollback removes
only this constructor call and restores the local literal.

Evidence:

- `games/high_card_duel/tests/serialization.rs::public_effect_constructor_preserves_public_scope_and_payload_text`
  pins public scope and representative stable public payload text.
- `cargo fmt --all --check` passed.
- `cargo test -p high_card_duel` passed.
- `cargo run -p replay-check -- --game high_card_duel --all` passed; 10 traces
  checked and `replay-check: all traces passed`.

### UNI8CR2TWOSEA-003 - High Card Duel private effect constructor

Selected surface: `games/high_card_duel/src/effects.rs::private_effect`.

Before state: local literal constructor
`EffectEnvelope { visibility: VisibilityScope::PrivateToSeat(owner_seat_id), payload }`.

After state: `EffectEnvelope::private_to(owner_seat_id, payload)`.

ADR-0009 classification: `unchanged`. This changes only generic seat-private
envelope construction and preserves owner `SeatId`, payload formation,
filtered-effect projection, private diagnostics/deal/commit payloads, effect
hashes, and observer/opponent filtering. Rollback removes only this constructor
call and restores the local literal.

Evidence:

- Existing `games/high_card_duel/tests/visibility.rs` coverage pins this
  surface: `effect_private_card_identity_is_private_to_owner`,
  `effect_filtering_returns_correct_sets_for_observer_seat0_seat1`,
  `effect_visibility_scopes_match_spec`, and the pairwise no-leak harness.
- `cargo fmt --all --check` passed.
- `cargo test -p high_card_duel` passed.
- `cargo run -p replay-check -- --game high_card_duel --all` passed; 10 traces
  checked and `replay-check: all traces passed`.

### UNI8CR2TWOSEA-004 - Secret Draft public effect constructor

Selected surface: `games/secret_draft/src/effects.rs::public_effect`.

Before state: local literal constructor
`engine_core::EffectEnvelope { visibility: engine_core::VisibilityScope::Public, payload }`.

After state: `engine_core::EffectEnvelope::public(payload)`.

ADR-0009 classification: `unchanged`. This changes only generic public
envelope construction and preserves payload formation, commitment/reveal
policy, redaction, stable public effect strings, replay hashes, and viewer
filtering. Secret Draft has no seat-private effect constructor at this
baseline; that N/A remains a report/register receipt, not synthetic code.

Evidence:

- `games/secret_draft/src/effects.rs::public_effect_constructor_preserves_public_scope_and_redacted_payload`
  pins public scope and confirms pre-reveal public effect payloads omit every
  `DraftItemId`.
- `cargo fmt --all --check` passed.
- `cargo test -p secret_draft` passed.
- `cargo run -p replay-check -- --game secret_draft --all` passed; 14 traces
  checked and `replay-check: all traces passed`.

### UNI8CR2TWOSEA-019 - Poker Lite parallel action-tree v1 bytes/hash

Selected surface: `games/poker_lite/src/replay_support.rs` additive
action-tree v1 adapter alongside the retained legacy `action_tree_hash`.

Before state: Poker Lite had a local legacy string encoder `action_tree_hash`
used by replay evidence, but no version-pinned v1 byte/hash surface.

After state: `action_tree_v1_bytes` and `action_tree_v1_hash` expose
`ActionTreeEncodingVersion::V1` bytes/hash for existing legal action trees.
The legacy `action_tree_hash` function remains intact.

ADR-0009 classification: `parallel-new-surface` with legacy `exception`. This
adds explicit v1 action-tree evidence for opening pledge, outstanding response,
and second-round pledge trees without reinterpreting the legacy hash, changing
legal choices, or altering trace bytes.

Evidence:

- `games/poker_lite/tests/replay.rs::action_tree_v1_bytes_and_hashes_are_pinned_across_pledge_phases`
  pins opening pledge choices `hold, press` with legacy/v1 values
  `2134463419946389911` / `1144` / `4146366381206085604`,
  outstanding response choices `lift, match, yield` with legacy/v1 values
  `5240408035218415049` / `1715` / `15898457577120528969`, and
  second-round pledge choices `hold, press` with legacy/v1 values
  `10376176577096665250` / `1142` / `12557641340017326258`.
- `cargo fmt --all --check` passed.
- `cargo test -p poker_lite` passed.
- `cargo run -p replay-check -- --game poker_lite --all` passed; all Poker
  Lite traces passed, including the public export fixture.

### UNI8CR2TWOSEA-020 - Masked Claims parallel action-tree v1 bytes/hash

Selected surface: `games/masked_claims/src/replay_support.rs` additive
action-tree v1 adapter.

Before state: Masked Claims had compound claim and flat response legal action
trees, but no game-owned version-pinned v1 byte/hash wrapper.

After state: `action_tree_v1_bytes` and `action_tree_v1_hash` expose
`ActionTreeEncodingVersion::V1` bytes/hash for existing legal action trees.
The adapter is additive and independently removable.

ADR-0009 classification: `parallel-new-surface`. This adds explicit v1
action-tree evidence for the nested claim shape and pending-response shape.
Existing legality, reaction-window ownership, pending-responder policy, state
bytes, public export bytes, and trace bytes are unchanged.

Evidence:

- `games/masked_claims/tests/replay.rs::action_tree_v1_bytes_and_hashes_are_pinned_for_claim_and_response_shapes`
  pins the claim tree root choice `claim` with v1 length/hash `15326` /
  `3772732430772540101`, and the response choices `respond/accept,
  respond/challenge` with v1 length/hash `1100` / `689297409234037920`.
- `cargo fmt --all --check` passed.
- `cargo test -p masked_claims` passed.
- `cargo run -p replay-check -- --game masked_claims --all` passed; all Masked
  Claims traces passed.

### UNI8CR2TWOSEA-005 - Poker Lite public effect constructor

Selected surface: `games/poker_lite/src/effects.rs::public_effect`.

Before state: local literal constructor
`EffectEnvelope { visibility: VisibilityScope::Public, payload }`.

After state: `EffectEnvelope::public(payload)`.

ADR-0009 classification: `unchanged`. This changes only generic public
envelope construction and preserves payload formation, reveal timing, pot and
showdown policy, public effect bytes, replay hashes, and viewer filtering.
Poker's private effect constructor is unchanged and remains ticket 006's
surface.

Evidence:

- `games/poker_lite/tests/serialization.rs::public_effect_constructor_preserves_public_scope_and_payload`
  pins public scope and exact representative payload preservation.
- `cargo fmt --all --check` passed after rustfmt adjusted import ordering.
- `cargo test -p poker_lite` passed.
- `cargo run -p replay-check -- --game poker_lite --all` passed; output
  included `bot-action.trace.json ... not-applicable trace accepted` and
  `wasm-exported.trace.json: public export fixture accepted`, then
  `replay-check: all traces passed`.

### UNI8CR2TWOSEA-006 - Poker Lite private effect constructor

Selected surface: `games/poker_lite/src/effects.rs::private_effect`.

Before state: local literal constructor
`EffectEnvelope { visibility: VisibilityScope::PrivateToSeat(owner_seat_id), payload }`.

After state: `EffectEnvelope::private_to(owner_seat_id, payload)`.

ADR-0009 classification: `unchanged`. This changes only generic seat-private
envelope construction and preserves owner `SeatId`, payload formation, private
setup-card delivery, bot-choice visibility, filtered-effect projection, replay
hashes, and observer/opponent filtering.

Evidence:

- `games/poker_lite/src/effects.rs::private_effect_constructor_preserves_owner_scope_and_payload`
  pins private scope and exact representative payload preservation.
- Existing Poker Lite no-leak coverage remains green, including
  `tests/visibility.rs` and `tests/bots.rs`.
- `cargo fmt --all --check` passed.
- `cargo test -p poker_lite` passed.
- `cargo run -p replay-check -- --game poker_lite --all` passed; output
  included `deal-private-no-leak.trace.json`, `seat-private-view.trace.json`,
  and `wasm-exported.trace.json: public export fixture accepted`, then
  `replay-check: all traces passed`.

### UNI8CR2TWOSEA-007 - Masked Claims public effect constructor

Selected surface: `games/masked_claims/src/effects.rs::public_effect`.

Before state: local literal constructor
`EffectEnvelope { visibility: VisibilityScope::Public, payload }`.

After state: `EffectEnvelope::public(payload)`.

ADR-0009 classification: `unchanged`. This changes only generic public
envelope construction and preserves payload formation, claim/reaction redaction,
reveal timing, public effect bytes, replay hashes, and viewer filtering. Masked
Claims still has no seat-private effect constructor; that N/A remains a
report/register receipt rather than new code.

Evidence:

- `games/masked_claims/src/effects.rs::public_effect_constructor_preserves_public_scope_and_payload`
  pins public scope and exact representative payload preservation.
- Existing Masked Claims no-leak coverage remains green, including
  `tests/visibility.rs` and effect tests that keep claim/window/accept/terminal
  payloads free of hidden tile IDs.
- `cargo fmt --all --check` passed.
- `cargo test -p masked_claims` passed.
- `cargo run -p replay-check -- --game masked_claims --all` passed; output
  included `public-observer-no-leak.trace.json`, `claim-pending-window.trace.json`,
  and `public-replay-export-import.trace.json`, then all traces passed.

### UNI8CR2TWOSEA-008 - High Card Duel canonical seat parser adoption

Selected surface: `games/high_card_duel/src/ids.rs::HighCardDuelSeat::parse`.

Before state: local manual match accepted only `"seat_0"` and `"seat_1"`.

After state: `SeatId::parse_canonical(value)` handles canonical grammar
acceptance, then `canonical_zero_based_index()` maps through
`HighCardDuelSeat::from_index`.

ADR-0009 classification: `unchanged`. This changes only the parser authority
for canonical grammar and preserves the game-local two-seat bound. The game
crate still rejects hyphen IDs, symbolic aliases, ambiguous labels,
out-of-range IDs, leading-zero spellings, Unicode lookalikes, and role names.
Legacy aliases remain import-only in the WASM adapter.

Evidence:

- `games/high_card_duel/src/ids.rs::seat_parser_rejects_non_canonical_and_out_of_range_ids`
  pins canonical acceptance and strict game-crate rejection vectors.
- `cargo fmt --all --check` passed after rustfmt formatting was applied.
- `cargo test -p high_card_duel` passed.
- `cargo run -p replay-check -- --game high_card_duel --all` passed; 10 traces
  checked and `replay-check: all traces passed`.

### UNI8CR2TWOSEA-009 - Secret Draft canonical seat parser adoption

Selected surface: `games/secret_draft/src/ids.rs::SecretDraftSeat::parse`.

Before state: local manual match accepted only `"seat_0"` and `"seat_1"`.

After state: `SeatId::parse_canonical(value)` handles canonical grammar
acceptance, then `canonical_zero_based_index()` maps through
`SecretDraftSeat::from_index`.

ADR-0009 classification: `unchanged`. This changes only the parser authority
for canonical grammar and preserves the game-local two-seat bound. The game
crate still rejects hyphen IDs, symbolic aliases, ambiguous labels,
out-of-range IDs, leading-zero spellings, Unicode lookalikes, and role names.
Legacy aliases remain import-only in the WASM adapter.

Evidence:

- `games/secret_draft/src/ids.rs::seat_parser_rejects_non_canonical_and_out_of_range_ids`
  pins canonical acceptance and strict game-crate rejection vectors.
- `cargo fmt --all --check` passed after rustfmt formatting was applied.
- `cargo test -p secret_draft` passed.
- `cargo run -p replay-check -- --game secret_draft --all` passed; 14 traces
  checked and `replay-check: all traces passed`.

### UNI8CR2TWOSEA-010 - Poker Lite canonical seat parser adoption

Selected surface: `games/poker_lite/src/ids.rs::PokerLiteSeat::parse`.

Before state: local manual match accepted only `"seat_0"` and `"seat_1"`.

After state: `SeatId::parse_canonical(value)` handles canonical grammar
acceptance, then `canonical_zero_based_index()` maps through
`PokerLiteSeat::from_index`.

ADR-0009 classification: `unchanged`. This changes only the parser authority
for canonical grammar and preserves the game-local two-seat bound. The game
crate still rejects hyphen IDs, symbolic aliases, ambiguous labels,
out-of-range IDs, leading-zero spellings, Unicode lookalikes, and role names.
Legacy aliases remain import-only in the WASM adapter.

Evidence:

- `games/poker_lite/src/ids.rs::seat_parser_rejects_non_canonical_and_out_of_range_ids`
  pins canonical acceptance and strict game-crate rejection vectors.
- `cargo fmt --all --check` passed.
- `cargo test -p poker_lite` passed.
- `cargo run -p replay-check -- --game poker_lite --all` passed; output
  included `deal-private-no-leak.trace.json`, `seat-private-view.trace.json`,
  and `wasm-exported.trace.json: public export fixture accepted`, then
  `replay-check: all traces passed`.

### UNI8CR2TWOSEA-011 - Masked Claims canonical seat parser adoption

Selected surface: `games/masked_claims/src/ids.rs::MaskedClaimsSeat::parse`.

Before state: local manual match accepted only `"seat_0"` and `"seat_1"`.

After state: `SeatId::parse_canonical(value)` handles canonical grammar
acceptance, then `canonical_zero_based_index()` maps through
`MaskedClaimsSeat::from_index`.

ADR-0009 classification: `unchanged`. This changes only the parser authority
for canonical grammar and preserves the game-local two-seat bound. The game
crate still rejects hyphen IDs, symbolic aliases, ambiguous labels,
out-of-range IDs, leading-zero spellings, Unicode lookalikes, and role names.
Masked Claims' WASM adapter/output remain unchanged and already canonical.

Evidence:

- `games/masked_claims/src/ids.rs::seat_parser_rejects_non_canonical_and_out_of_range_ids`
  pins canonical acceptance and strict game-crate rejection vectors.
- `cargo fmt --all --check` passed.
- `cargo test -p masked_claims` passed.
- `cargo run -p replay-check -- --game masked_claims --all` passed; output
  included `public-observer-no-leak.trace.json`, `claim-pending-window.trace.json`,
  and `public-replay-export-import.trace.json`, then all traces passed.

### UNI8CR2TWOSEA-012 - WASM seat compatibility receipt

Selected surface: `crates/wasm-api/src/seats.rs` import adapter and roster
helpers.

Before state: bounded import adapter already accepted canonical underscore,
legacy hyphen, and symbolic aliases, while game crates remained canonical-only.
HCD/Secret/Poker replay cursors used the legacy hyphen roster and Masked Claims
already emitted canonical underscore IDs.

After state: added focused receipt tests only. No runtime roster, trace helper,
state, effect, view, export, or hash byte changed.

ADR-0009 classification: `unchanged`. The HCD/Secret/Poker runtime roster
exception remains owned by `wasm-api`, preserved through C-11, with the next
trigger set to a dedicated WASM runtime-seat migration. Masked Claims remains
canonical on output.

Evidence:

- `crates/wasm-api/src/seats.rs::import_adapter_accepts_canonical_hyphen_and_symbolic_aliases`
  covers HCD, Secret Draft, Poker Lite, and Masked Claims alias imports.
- `crates/wasm-api/src/seats.rs::masked_output_helpers_emit_canonical_seat_ids`
  pins Masked Claims canonical output helpers.
- `crates/wasm-api/src/seats.rs::existing_trace_and_roster_helpers_keep_legacy_outputs`
  guards the legacy roster/trace spelling exception.
- `cargo fmt --all --check` passed.
- `cargo test -p wasm-api` passed.
- `cargo run -p replay-check -- --game high_card_duel --all` passed; 10 traces
  checked and `replay-check: all traces passed`.
- `cargo run -p replay-check -- --game secret_draft --all` passed; 14 traces
  checked and `replay-check: all traces passed`.
- `cargo run -p replay-check -- --game poker_lite --all` passed; output
  included `wasm-exported.trace.json: public export fixture accepted`, then
  `replay-check: all traces passed`.
- `cargo run -p replay-check -- --game masked_claims --all` passed; output
  included `public-observer-no-leak.trace.json` and
  `public-replay-export-import.trace.json`, then all traces passed.

### UNI8CR2TWOSEA-013 - High Card Duel exact-two-seat structural validation

Selected surface: `games/high_card_duel/src/setup.rs::setup_match`.

Before state: local predicate compared `seats.len()` directly against
`options.variant.seat_count`.

After state: `SeatCount::new(seats.len())` validates nonzero structure and the
resulting count is compared against the game-owned variant expected count.

ADR-0009 classification: `unchanged`. This changes only the structural count
helper used by setup validation and preserves the exact diagnostic
code/message, variant expected-count policy, setup shuffle/deal behavior, state
bytes, and replay hashes. `SeatCount::next_ring_index` remains not applicable.

Evidence:

- `games/high_card_duel/src/setup.rs::setup_accepts_exact_variant_seat_count`
  pins accepted two-seat setup.
- `games/high_card_duel/src/setup.rs::setup_rejects_non_two_seat_counts_with_exact_diagnostic`
  pins 0/1/3 rejection with the exact diagnostic.
- `cargo fmt --all --check` passed.
- `cargo test -p high_card_duel` passed.
- `cargo run -p replay-check -- --game high_card_duel --all` passed; 10 traces
  checked and `replay-check: all traces passed`.

### UNI8CR2TWOSEA-014 - Secret Draft exact-two-seat structural validation

Selected surface: `games/secret_draft/src/setup.rs::setup_match` and the
normal `game-stdlib` dependency edge.

Before state: local predicate compared `seats.len()` directly against
`STANDARD_SEAT_COUNT`; `secret_draft` had no normal `game-stdlib` dependency.

After state: `SeatCount::new(seats.len())` validates nonzero structure and the
resulting count is compared against the game-owned `STANDARD_SEAT_COUNT`.
`games/secret_draft/Cargo.toml` and `Cargo.lock` record the normal
`game-stdlib` dependency.

ADR-0009 classification: `unchanged`. This changes only the structural count
helper used by setup validation and preserves the exact diagnostic
code/message, standard expected-count policy, setup state bytes, and replay
hashes. `SeatCount::next_ring_index` remains not applicable.

Evidence:

- `games/secret_draft/src/setup.rs::setup_accepts_exact_standard_seat_count`
  pins accepted two-seat setup.
- `games/secret_draft/src/setup.rs::setup_rejects_wrong_seat_counts_with_exact_diagnostic`
  pins 0/1/3 rejection with the exact diagnostic.
- `cargo fmt --all --check` passed.
- `cargo test -p secret_draft` passed.
- `cargo run -p replay-check -- --game secret_draft --all` passed; 14 traces
  checked and `replay-check: all traces passed`.

### UNI8CR2TWOSEA-021 - Secret Draft game-test-support dev-only dependency

Selected surface: `games/secret_draft/Cargo.toml` `[dev-dependencies]` and
the corresponding `Cargo.lock` package dependency list.

Before state: Secret Draft had no `game-test-support` dependency.

After state: Secret Draft lists `game-test-support` only under
`[dev-dependencies]`; the lockfile records the package dependency edge.

ADR-0009 classification: `unchanged`. This is a dev-only test-infrastructure
edge for later no-leak/profile harness use. No normal/build/WASM/tool edge was
added, and no production code or runtime behavior changed.

Evidence:

- `cargo tree --workspace -e normal --invert game-test-support` output showed
  only `game-test-support v0.1.0`, with no `secret_draft` normal edge.
- `bash scripts/boundary-check.sh` passed and reported
  `game-test-support dev-only boundary check passed`.
- `cargo test -p secret_draft` passed.

### UNI8CR2TWOSEA-025 - Secret Draft C-07 pairwise no-leak geometry

Selected surface: `games/secret_draft/tests/visibility.rs` pairwise no-leak
matrix using `game_test_support::no_leak`.

Before state: Secret Draft had focused pre-reveal redaction, bot, public
export, and internal-trace authority tests, but no shared pairwise matrix over
both source seats and viewer classes.

After state:
`pairwise_no_leak_matrix_covers_pre_commit_and_post_reveal_surfaces` enumerates
both source seats across observer, seat 0, and seat 1 viewers. Pre-reveal
surfaces cover commitment/private view fields, action metadata, diagnostics,
effects, public export, seat-private export, and bot rationale. Post-reveal
surfaces cover view, effect, public export, and seat-private export.

ADR-0009 classification: `unchanged`. This adds only deterministic no-leak
evidence. It keeps synchronized reveal policy in Secret Draft, keeps the
visible pool public pre-reveal, treats raw internal command traces as
`internal-dev`, and adds no golden trace, fixture, or committed canary.

Evidence:

- The matrix models committed-choice secrecy separately from public visible
  pool membership, because a still-visible item may remain a legal public
  choice before synchronized reveal.
- `games/secret_draft/tests/visibility.rs::pairwise_no_leak_matrix_covers_pre_commit_and_post_reveal_surfaces`
  passed.
- Existing
  `games/secret_draft/tests/bots.rs::level1_uses_only_public_information_when_opponent_commitment_differs`
  and
  `games/secret_draft/tests/visibility.rs::raw_internal_trace_is_the_only_checked_surface_that_keeps_private_command_authority`
  remained green.
- `cargo fmt --all --check` passed.
- `cargo test -p secret_draft` passed.
- `cargo run -p replay-check -- --game secret_draft --all` passed; 14 traces
  checked and `replay-check: all traces passed`.

### UNI8CR2TWOSEA-029 - Secret Draft replay-command-v1 profile driver

Selected surface: `games/secret_draft/tests/replay.rs` profile-driver test over
the existing internal command trace.

Before state: Secret Draft had internal command trace generation and replay
validation, but no `ReplayCommandV1Driver` receipt.

After state:
`replay_command_v1_profile_driver_wraps_internal_trace_validator` validates
the `replay-command-v1` metadata (`v1`, `internal-dev`,
`validator_owner = secret_draft`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing generated internal trace
validator.

ADR-0009 classification: `unchanged`. This adds typed profile evidence only.
The private command trace remains `internal-dev`, existing trace bytes remain
authoritative, no canonical byte claim is made, and no artifact is rewritten.

Evidence:

- Valid profile metadata reports `replay-command-v1`, `v1`, `internal-dev`,
  and `secret_draft`.
- `validate_with` returns the existing trace stable hash from
  `replay_internal_full_trace`.
- Wrong profile id, wrong validator owner, and illegal profile field are
  rejected.
- `cargo fmt --all --check` passed.
- `cargo test -p secret_draft` passed.
- `cargo run -p replay-check -- --game secret_draft --all` passed; 14 traces
  checked and `replay-check: all traces passed`.

### UNI8CR2TWOSEA-030 - Poker Lite replay-command-v1 profile driver

Selected surface: `games/poker_lite/tests/replay.rs` profile-driver test over
the existing internal command trace.

Before state: Poker Lite had internal command trace generation and replay
validation, but no `ReplayCommandV1Driver` receipt.

After state:
`replay_command_v1_profile_driver_wraps_internal_trace_validator` validates
the `replay-command-v1` metadata (`v1`, `internal-dev`,
`validator_owner = poker_lite`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing generated internal trace
validator.

ADR-0009 classification: `unchanged`. This adds typed profile evidence only.
The private command trace remains `internal-dev`, existing trace bytes remain
authoritative, no canonical byte claim is made, and no artifact is rewritten.

Evidence:

- Valid profile metadata reports `replay-command-v1`, `v1`, `internal-dev`,
  and `poker_lite`.
- `validate_with` returns the existing trace stable hash from
  `replay_internal_full_trace`.
- Wrong profile id, wrong validator owner, and illegal profile field are
  rejected.
- `cargo test -p poker_lite` passed.
- `cargo run -p replay-check -- --game poker_lite --all` passed; all Poker
  Lite traces passed.

### UNI8CR2TWOSEA-031 - Masked Claims replay-command-v1 profile driver

Selected surface: `games/masked_claims/tests/replay.rs` profile-driver test
over the existing deterministic rule/replay evidence builder.

Before state: Masked Claims had deterministic replay evidence through
`replay_run` and golden trace presence/no-leak checks, but no
`ReplayCommandV1Driver` receipt.

After state:
`replay_command_v1_profile_driver_wraps_rule_replay_evidence` validates the
`replay-command-v1` metadata (`v1`, `internal-dev`,
`validator_owner = masked_claims`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing rule/replay evidence builder.

ADR-0009 classification: `unchanged`. This adds typed profile evidence only.
Masked Claims keeps its existing rule/replay construction, no omniscient export
is introduced, no canonical byte claim is made, and no artifact is rewritten.

Evidence:

- Valid profile metadata reports `replay-command-v1`, `v1`, `internal-dev`,
  and `masked_claims`.
- `validate_with` returns a deterministic hash of the existing `replay_run`
  evidence for seed 31.
- Wrong profile id, wrong validator owner, wrong visibility class, and illegal
  profile field are rejected.
- `cargo test -p masked_claims` passed.
- `cargo run -p replay-check -- --game masked_claims --all` passed; all
  Masked Claims traces were accepted under the current not-applicable baseline.

### UNI8CR2TWOSEA-032 - High Card Duel setup-evidence-v1 profile driver

Selected surface: `games/high_card_duel/tests/serialization.rs` setup-evidence
profile-driver test over the existing read-only fixture metadata.

Before state: High Card Duel had a metadata-only fixture and internal setup
assertions, but no `SetupEvidenceV1Driver` receipt for public setup evidence.

After state:
`setup_evidence_v1_profile_driver_wraps_public_fixture_metadata` validates the
`setup-evidence-v1` metadata (`v1`, `public`,
`validator_owner = high_card_duel`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing fixture bytes.

ADR-0009 classification: `unchanged`. This adds typed setup-profile evidence
only. The fixture remains read-only, private deal assertions stay internal-dev,
no canonical byte claim is made, and no fixture artifact is rewritten.

Evidence:

- Valid profile metadata reports `setup-evidence-v1`, `v1`, `public`, and
  `high_card_duel`.
- The fixture still contains public fixture id, game, variant, rules version,
  and fixture-kind metadata only.
- The fixture contains no `private_deal` field and no private card command
  token such as `hcd:r`.
- Wrong profile id, wrong validator owner, wrong visibility class, and illegal
  profile field are rejected.
- `cargo test -p high_card_duel` passed.
- `cargo run -p fixture-check -- --game high_card_duel` passed with
  `fixture-check: all fixtures passed`.

### UNI8CR2TWOSEA-033 - Secret Draft setup-evidence-v1 profile driver

Selected surface: `games/secret_draft/tests/serialization.rs` setup-evidence
profile-driver test over the existing read-only fixture metadata.

Before state: Secret Draft had public setup fixture parsing and empty
commitment assertions, but no `SetupEvidenceV1Driver` receipt for public setup
evidence.

After state:
`setup_evidence_v1_profile_driver_wraps_public_fixture_metadata` validates the
`setup-evidence-v1` metadata (`v1`, `public`,
`validator_owner = secret_draft`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing fixture bytes.

ADR-0009 classification: `unchanged`. This adds typed setup-profile evidence
only. The fixture remains read-only, commitments remain empty, no reveal
behavior is encoded in data, no canonical byte claim is made, and no fixture
artifact is rewritten.

Evidence:

- Valid profile metadata reports `setup-evidence-v1`, `v1`, `public`, and
  `secret_draft`.
- The fixture still contains public fixture id, game, variant, rules version,
  visible-pool metadata, and `seat_0_commitment` / `seat_1_commitment` set to
  `none`.
- The fixture contains no selector, trigger, or reveal behavior field.
- Wrong profile id, wrong validator owner, wrong visibility class, and illegal
  profile field are rejected.
- `cargo test -p secret_draft` passed.
- `cargo run -p fixture-check -- --game secret_draft` passed with
  `fixture-check: all fixtures passed`.

### UNI8CR2TWOSEA-034 - Poker Lite setup-evidence-v1 profile driver

Selected surface: `games/poker_lite/tests/serialization.rs` setup-evidence
profile-driver test over the existing read-only fixture metadata.

Before state: Poker Lite had public setup fixture parsing and deck/setup
metadata assertions, but no `SetupEvidenceV1Driver` receipt for public setup
evidence.

After state:
`setup_evidence_v1_profile_driver_wraps_deck_fixture_metadata` validates the
`setup-evidence-v1` metadata (`v1`, `public`,
`validator_owner = poker_lite`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing fixture bytes.

ADR-0009 classification: `unchanged`. This adds typed setup-profile evidence
only. The fixture remains read-only, dealt private cards stay internal-dev, no
canonical byte claim is made, and no fixture artifact is rewritten.

Evidence:

- Valid profile metadata reports `setup-evidence-v1`, `v1`, `public`, and
  `poker_lite`.
- The fixture still contains public fixture id, game, variant, rules version,
  deck-order metadata, `private_cards = hidden_by_setup`, and hidden center
  status.
- The fixture contains no seat-private hand field such as `seat_0_private`,
  `seat_1_private`, or `private_hand`.
- Wrong profile id, wrong validator owner, wrong visibility class, and illegal
  profile field are rejected.
- `cargo test -p poker_lite` passed.
- `cargo run -p fixture-check -- --game poker_lite` passed with
  `fixture-check: all fixtures passed`.

### UNI8CR2TWOSEA-035 - Masked Claims setup-evidence-v1 profile driver

Selected surface: `games/masked_claims/tests/serialization.rs` setup-evidence
profile-driver test over the existing read-only fixture metadata.

Before state: Masked Claims had public fixture parsing and mask metadata
checks, but no `SetupEvidenceV1Driver` receipt for public setup evidence.

After state:
`setup_evidence_v1_profile_driver_wraps_mask_fixture_metadata` validates the
`setup-evidence-v1` metadata (`v1`, `public`,
`validator_owner = masked_claims`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing fixture bytes.

ADR-0009 classification: `unchanged`. This adds typed setup-profile evidence
only. The fixture remains read-only, hand and reserve identity stay hidden or
internal-only, no reaction policy is encoded in data, no canonical byte claim
is made, and no fixture artifact is rewritten.

Evidence:

- Valid profile metadata reports `setup-evidence-v1`, `v1`, `public`, and
  `masked_claims`.
- The fixture still contains public fixture id, game, variant, rules version,
  mask-order metadata, `hand_status = hidden_by_setup`, and
  `reserve_status = internal_only`.
- The fixture contains no selector, trigger, or reaction policy field.
- Wrong profile id, wrong validator owner, wrong visibility class, and illegal
  profile field are rejected.
- `cargo test -p masked_claims` passed.
- `cargo run -p fixture-check -- --game masked_claims` passed with
  `fixture-check: all fixtures passed`.

### UNI8CR2TWOSEA-036 - High Card Duel public-export-v1 profile driver

Selected surface: `games/high_card_duel/tests/replay.rs` profile-driver test
over the existing observer-only public replay export, plus the existing
visibility no-leak surface in `games/high_card_duel/tests/visibility.rs`.

Before state: High Card Duel pinned the observer export hash and had export
no-leak coverage, but no `PublicExportV1Driver` receipt for the public export
profile.

After state:
`public_export_v1_profile_driver_wraps_observer_export_validator` validates the
`public-export-v1` metadata (`v1`, `public`,
`validator_owner = high_card_duel`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing observer export hash.

ADR-0009 classification: `unchanged`. This adds typed public-export profile
evidence only. The export remains observer-only, existing export bytes remain
authoritative, no canonical byte claim is made, and no export artifact is
rewritten.

Evidence:

- Valid profile metadata reports `public-export-v1`, `v1`, `public`, and
  `high_card_duel`.
- `validate_with` returns the existing observer export stable hash
  `11079559833511455730`.
- Wrong profile id, wrong validator owner, wrong visibility class, and illegal
  profile field are rejected.
- The observer export still omits seed material, private commit tokens, and
  unrevealed deck-tail identities.
- `cargo test -p high_card_duel` passed.
- `cargo run -p replay-check -- --game high_card_duel --all` passed; all HCD
  traces passed.

### UNI8CR2TWOSEA-037 - Secret Draft public-export-v1 profile driver

Selected surface: `games/secret_draft/tests/replay.rs` profile-driver test
over the existing observer public replay export, plus the existing pre-reveal
redaction surface in `games/secret_draft/tests/visibility.rs`.

Before state: Secret Draft had observer public export round-trip and pre-reveal
redaction tests, but no `PublicExportV1Driver` receipt for the public export
profile.

After state:
`public_export_v1_profile_driver_wraps_observer_export_validator` validates the
`public-export-v1` metadata (`v1`, `public`,
`validator_owner = secret_draft`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing pre-reveal observer export
hash.

ADR-0009 classification: `unchanged`. This adds typed public-export profile
evidence only. The observer export path remains unchanged, existing export
bytes remain authoritative, no canonical byte claim is made, and no export
artifact is rewritten.

Evidence:

- Valid profile metadata reports `public-export-v1`, `v1`, `public`, and
  `secret_draft`.
- `validate_with` returns the existing pre-reveal observer export stable hash
  `5995340232186846963`.
- Wrong profile id, wrong validator owner, wrong visibility class, and illegal
  profile field are rejected.
- The observer export still emits `commit_redacted` and omits the committed
  item id/path plus seed material.
- `cargo test -p secret_draft` passed.
- `cargo run -p replay-check -- --game secret_draft --all` passed; all Secret
  Draft traces passed.

### UNI8CR2TWOSEA-038 - Poker Lite public-export-v1 profile driver

Selected surface: `games/poker_lite/tests/replay.rs` profile-driver test over
the existing observer public replay export, plus the existing yield no-reveal
surface in `games/poker_lite/tests/visibility.rs`.

Before state: Poker Lite had observer public export round-trip, yield
non-reveal, and private-crest redaction tests, but no `PublicExportV1Driver`
receipt for the public export profile.

After state:
`public_export_v1_profile_driver_wraps_observer_export_validator` validates the
`public-export-v1` metadata (`v1`, `public`,
`validator_owner = poker_lite`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing yield-terminal observer
export hash.

ADR-0009 classification: `unchanged`. This adds typed public-export profile
evidence only. The observer export path remains unchanged, existing export
bytes remain authoritative, no canonical byte claim is made, and no export
artifact is rewritten.

Evidence:

- Valid profile metadata reports `public-export-v1`, `v1`, `public`, and
  `poker_lite`.
- `validate_with` returns the existing yield-terminal observer export stable
  hash `12011531955662310238`.
- Wrong profile id, wrong validator owner, wrong visibility class, and illegal
  profile field are rejected.
- The yield observer export still omits private crests, loser crest label, and
  seed material.
- `cargo test -p poker_lite` passed.
- `cargo run -p replay-check -- --game poker_lite --all` passed; all Poker
  Lite traces passed.

### UNI8CR2TWOSEA-039 - Masked Claims public-export-v1 profile driver

Selected surface: `games/masked_claims/tests/replay.rs` profile-driver test
over the existing observer public replay export, plus the existing claimed-tile
redaction surface in `games/masked_claims/tests/visibility.rs`.

Before state: Masked Claims had observer public export round-trip and
claimed-tile redaction tests, but no `PublicExportV1Driver` receipt for the
public export profile.

After state:
`public_export_v1_profile_driver_wraps_observer_export_validator` validates the
`public-export-v1` metadata (`v1`, `public`,
`validator_owner = masked_claims`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing observer export JSON bytes.

ADR-0009 classification: `unchanged`. This adds typed public-export profile
evidence only. The observer export path remains unchanged, existing export
bytes remain authoritative, no canonical byte claim is made, and no export
artifact is rewritten.

Evidence:

- Valid profile metadata reports `public-export-v1`, `v1`, `public`, and
  `masked_claims`.
- `validate_with` returns a stable hash over the existing observer export JSON
  bytes produced by `replay_run(31)`.
- Wrong profile id, wrong validator owner, wrong visibility class, and illegal
  profile field are rejected.
- The observer export still omits claim tile ids and seed material.
- `cargo test -p masked_claims` passed.
- `cargo run -p replay-check -- --game masked_claims --all` passed; all
  Masked Claims traces were accepted under the current not-applicable baseline.

### UNI8CR2TWOSEA-040 - Secret Draft seat-private-export-v1 profile driver

Selected surface: `games/secret_draft/tests/replay.rs` profile-driver test
over existing viewer-scoped `export_public_replay` calls for `seat_0` and
`seat_1`, plus the pre-reveal redaction surface in
`games/secret_draft/tests/visibility.rs`.

Before state: Secret Draft had viewer-scoped export support and redaction
tests, but no `SeatPrivateExportV1Driver` receipt for seat-private export
profile metadata.

After state:
`seat_private_export_v1_profile_driver_wraps_viewer_scoped_exports` validates
the `seat-private-export-v1` metadata (`v1`, `seat-private`,
`validator_owner = secret_draft`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing viewer-scoped export bytes
for both seats.

ADR-0009 classification: `unchanged`. This adds typed seat-private export
profile evidence only. The existing export path remains unchanged, no new
exporter is introduced, no canonical byte claim is made, and no export artifact
is rewritten.

Evidence:

- Valid profile metadata reports `seat-private-export-v1`, `v1`,
  `seat-private`, and `secret_draft`.
- Viewer labels are explicit as `seat_0` and `seat_1`.
- The pre-reveal committed item id/path and seed material remain absent even
  for the owner export.
- Wrong profile id, wrong validator owner, wrong visibility class, and illegal
  profile field are rejected.
- `cargo test -p secret_draft` passed.
- `cargo run -p replay-check -- --game secret_draft --all` passed; all Secret
  Draft traces passed.

### UNI8CR2TWOSEA-041 - Poker Lite seat-private-export-v1 profile driver

Selected surface: `games/poker_lite/tests/replay.rs` profile-driver test over
existing viewer-scoped `export_public_replay` calls for `seat_0` and `seat_1`,
plus the own-private-view surface in `games/poker_lite/tests/visibility.rs`.

Before state: Poker Lite had viewer-scoped export support and own-private
view/no-leak tests, but no `SeatPrivateExportV1Driver` receipt for
seat-private export profile metadata.

After state:
`seat_private_export_v1_profile_driver_wraps_viewer_scoped_exports` validates
the `seat-private-export-v1` metadata (`v1`, `seat-private`,
`validator_owner = poker_lite`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing viewer-scoped export bytes
for both seats.

ADR-0009 classification: `unchanged`. This adds typed seat-private export
profile evidence only. The existing export path remains unchanged, no new
exporter is introduced, no canonical byte claim is made, and no export artifact
is rewritten.

Evidence:

- Valid profile metadata reports `seat-private-export-v1`, `v1`,
  `seat-private`, and `poker_lite`.
- Viewer labels are explicit as `seat_0` and `seat_1`.
- Each seat-private export contains the viewer-owned crest and omits the
  opponent crest plus seed material.
- Wrong profile id, wrong validator owner, wrong visibility class, and illegal
  profile field are rejected.
- `cargo test -p poker_lite` passed.
- `cargo run -p replay-check -- --game poker_lite --all` passed; all Poker
  Lite traces passed.

### UNI8CR2TWOSEA-042 - High Card Duel unbiased bounded-index adoption

Selected surface: `games/high_card_duel/src/setup.rs::shuffle_deck`.

Before state: High Card Duel duplicated the v1 unbiased rejection sampler in a
local `next_bounded_index_unbiased` helper and called that helper from
`shuffle_deck`.

After state: `shuffle_deck` calls
`DeterministicRng::next_index_unbiased_v1(index + 1)` directly. The duplicated
local helper and public re-export were removed, and existing setup/rules tests
now exercise the shared engine-core sampler.

ADR-0009 classification: `unchanged`. This is a byte-neutral migration to the
already-shipped generic sampler. Shuffle bounds, draw order, seed meaning, deal
order, game policy, and hidden-information surfaces are unchanged.

Evidence:

- Fixed high-residue vector still returns index `1` for bound `3`.
- Rejection draw count remains `2`; zero-bound draw count remains `0`.
- HCD setup/rules grep now shows `next_index_unbiased_v1` and no local
  `next_bounded_index_unbiased` helper.
- `cargo test -p high_card_duel` passed.
- `cargo run -p replay-check -- --game high_card_duel --all` passed; all HCD
  traces passed.

### UNI8CR2TWOSEA-015 - Poker Lite exact-two-seat structural validation

Selected surface: `games/poker_lite/src/setup.rs::setup_match` and the normal
`game-stdlib` dependency edge.

Before state: local predicate compared `seats.len()` directly against
`STANDARD_SEAT_COUNT`; `poker_lite` had no normal `game-stdlib` dependency.

After state: `SeatCount::new(seats.len())` validates nonzero structure and the
resulting count is compared against the game-owned `STANDARD_SEAT_COUNT`.
`games/poker_lite/Cargo.toml` and `Cargo.lock` record the normal
`game-stdlib` dependency.

ADR-0009 classification: `unchanged`. This changes only the structural count
helper used by setup validation and preserves the exact diagnostic
code/message, standard expected-count policy, setup shuffle/deal behavior,
state bytes, and replay hashes. `SeatCount::next_ring_index` remains not
applicable.

Evidence:

- `games/poker_lite/src/setup.rs::setup_accepts_exact_standard_seat_count`
  pins accepted two-seat setup.
- `games/poker_lite/src/setup.rs::setup_rejects_wrong_seat_counts_with_exact_diagnostic`
  pins 0/1/3 rejection with the exact diagnostic.
- `cargo fmt --all --check` passed.
- `cargo test -p poker_lite` passed.
- `cargo run -p replay-check -- --game poker_lite --all` passed; output
  included `deal-private-no-leak.trace.json`, `seat-private-view.trace.json`,
  and `wasm-exported.trace.json: public export fixture accepted`, then
  `replay-check: all traces passed`.

### UNI8CR2TWOSEA-022 - Poker Lite game-test-support dev-only dependency

Selected surface: `games/poker_lite/Cargo.toml` `[dev-dependencies]` and the
corresponding `Cargo.lock` package dependency list.

Before state: Poker Lite had no `game-test-support` dependency.

After state: Poker Lite lists `game-test-support` only under
`[dev-dependencies]`; the lockfile records the package dependency edge.

ADR-0009 classification: `unchanged`. This is a dev-only test-infrastructure
edge for later no-leak/profile harness use. No normal/build/WASM/tool edge was
added, and no production code or runtime behavior changed.

Evidence:

- `cargo tree --workspace -e normal --invert game-test-support` output showed
  only `game-test-support v0.1.0`, with no `poker_lite` normal edge.
- `bash scripts/boundary-check.sh` passed and reported
  `game-test-support dev-only boundary check passed`.
- `cargo test -p poker_lite` passed.

### UNI8CR2TWOSEA-026 - Poker Lite C-07 pairwise no-leak geometry

Selected surface: `games/poker_lite/tests/visibility.rs` pairwise no-leak
matrix using `game_test_support::no_leak`.

Before state: Poker Lite had focused private-view, center reveal, showdown,
yield, bot, and replay no-leak tests, but no shared pairwise matrix over both
private crest source seats and viewer classes.

After state:
`pairwise_no_leak_matrix_covers_private_showdown_and_yield_surfaces`
enumerates both private crest source seats across observer, seat 0, and seat 1
viewers. Covered surfaces include pre-showdown view, action tree, diagnostic,
effect, public export, seat-private export, bot input, center-revealed
pre-showdown view, showdown view/public export, and yield view/public export
for each possible losing seat.

ADR-0009 classification: `unchanged`. This adds only deterministic no-leak
evidence. It keeps hand privacy, showdown reveal, and yield non-reveal policy
in Poker Lite, and adds no golden trace, fixture, or committed canary.

Evidence:

- The matrix preserves the existing owner policy: owner view and seat-private
  export can contain the owner private crest before showdown, setup effects do
  not expose raw crest ids, showdown reveals both crests, and yield public
  surfaces keep the losing crest hidden while the loser owner view may still
  contain its own private crest.
- `games/poker_lite/tests/visibility.rs::pairwise_no_leak_matrix_covers_private_showdown_and_yield_surfaces`
  passed.
- Existing `games/poker_lite/tests/bots.rs::level2_input_whitelist_excludes_forbidden_hidden_material`
  and `games/poker_lite/tests/replay.rs::yield_terminal_public_export_cannot_reconstruct_folded_private_cards`
  remained green.
- `cargo fmt --all --check` passed.
- `cargo test -p poker_lite` passed.
- `cargo run -p replay-check -- --game poker_lite --all` passed; all Poker
  Lite traces passed.

### UNI8CR2TWOSEA-016 - Masked Claims exact-two-seat structural validation

Selected surface: `games/masked_claims/src/setup.rs::setup_match` and the
normal `game-stdlib` dependency edge.

Before state: local predicate compared `seats.len()` directly against
`STANDARD_SEAT_COUNT`; `masked_claims` had no normal `game-stdlib` dependency.

After state: `SeatCount::new(seats.len())` validates nonzero structure and the
resulting count is compared against the game-owned `STANDARD_SEAT_COUNT`.
`games/masked_claims/Cargo.toml` and `Cargo.lock` record the normal
`game-stdlib` dependency.

ADR-0009 classification: `unchanged`. This changes only the structural count
helper used by setup validation and preserves the exact diagnostic
code/message, standard expected-count policy, setup shuffle/deal behavior,
state bytes, and replay hashes. `SeatCount::next_ring_index` remains not
applicable.

Evidence:

- `games/masked_claims/src/setup.rs::setup_accepts_exact_standard_seat_count`
  pins accepted two-seat setup.
- `games/masked_claims/src/setup.rs::setup_rejects_wrong_seat_counts_with_exact_diagnostic`
  pins 0/1/3 rejection with the exact diagnostic.
- `cargo fmt --all --check` passed.
- `cargo test -p masked_claims` passed.
- `cargo run -p replay-check -- --game masked_claims --all` passed; output
  included `public-observer-no-leak.trace.json`, `claim-pending-window.trace.json`,
  and `public-replay-export-import.trace.json`, then all traces passed.

### UNI8CR2TWOSEA-023 - Masked Claims game-test-support dev-only dependency

Selected surface: `games/masked_claims/Cargo.toml` `[dev-dependencies]` and
the corresponding `Cargo.lock` package dependency list.

Before state: Masked Claims had no `game-test-support` dependency.

After state: Masked Claims lists `game-test-support` only under
`[dev-dependencies]`; the lockfile records the package dependency edge.

ADR-0009 classification: `unchanged`. This is a dev-only test-infrastructure
edge for later no-leak/profile harness use. No normal/build/WASM/tool edge was
added, and no production code or runtime behavior changed.

Evidence:

- `cargo tree --workspace -e normal --invert game-test-support` output showed
  only `game-test-support v0.1.0`, with no `masked_claims` normal edge.
- `bash scripts/boundary-check.sh` passed and reported
  `game-test-support dev-only boundary check passed`.
- `cargo test -p masked_claims` passed.

### UNI8CR2TWOSEA-027 - Masked Claims C-07 pairwise no-leak geometry

Selected surface: `games/masked_claims/tests/visibility.rs` pairwise no-leak
matrix using `game_test_support::no_leak`.

Before state: Masked Claims had focused pending-claim, accepted-secret, bot,
and replay no-leak tests, but no shared pairwise matrix over both claimant
source seats and viewer classes.

After state:
`pairwise_no_leak_matrix_covers_pending_accepted_and_challenge_surfaces`
enumerates both claimant source seats across observer, seat 0, and seat 1
viewers. Covered surfaces include pending claim view, responder action tree,
effects, public export, bot rationale, accepted-secret view/public export, and
challenge-reveal view/effects/public export.

ADR-0009 classification: `unchanged`. This adds only deterministic no-leak
evidence. It keeps reaction-window, pending-responder, accepted-secret, and
challenge-reveal policy in Masked Claims, and adds no golden trace, fixture, or
committed canary.

Evidence:

- The matrix advances through a real accepted claim before constructing a seat 1
  source claim, so claimant sequencing remains game-owned.
- `games/masked_claims/tests/visibility.rs::pairwise_no_leak_matrix_covers_pending_accepted_and_challenge_surfaces`
  passed.
- Existing `games/masked_claims/tests/visibility.rs::accepted_masks_remain_hidden_after_resolution_and_bot_rationale_is_safe`
  and `games/masked_claims/tests/replay.rs::challenge_reveal_appears_after_public_claim_effect`
  remained green.
- `cargo fmt --all --check` passed.
- `cargo test -p masked_claims` passed.
- `cargo run -p replay-check -- --game masked_claims --all` passed; all Masked
  Claims traces passed.

### UNI8CR2TWOSEA-017 - High Card Duel parallel action-tree v1 bytes/hash

Selected surface: `games/high_card_duel/src/replay_support.rs` additive
action-tree v1 adapter.

Before state: HCD had `legal_action_tree` and legacy test-local segment-string
hashing in replay tests, but no game-owned version-pinned v1 byte/hash wrapper.

After state: `action_tree_v1_bytes` and `action_tree_v1_hash` expose
`ActionTreeEncodingVersion::V1` bytes/hash for existing legal action trees.
The adapter is additive and independently removable.

ADR-0009 classification: `parallel-new-surface`. This adds only explicit v1
action-tree evidence for representative commit states. Existing legal choices,
metadata, previews, legacy hashes, state/effect/view/export/replay bytes, and
C-07 debug snapshot behavior are unchanged.

Evidence:

- `games/high_card_duel/tests/replay.rs::action_tree_v1_bytes_and_hashes_are_pinned_for_commit_states`
  pins lead-commit v1 length/hash `1104` / `13958272533655564487` and
  reply-commit v1 length/hash `1107` / `10401739316208507941`.
- `cargo fmt --all --check` passed.
- `cargo test -p high_card_duel` passed.
- `cargo run -p replay-check -- --game high_card_duel --all` passed; 10 traces
  checked and `replay-check: all traces passed`.

### UNI8CR2TWOSEA-024 - High Card Duel C-07 no-leak pilot receipt verification

Selected surface: `games/high_card_duel/tests/visibility.rs` residual no-leak
coverage on top of the existing C-07 pilot matrix.

Before state: High Card Duel already had `MSC-8C-007` coverage through
`no_leak_harness_covers_public_seat_replay_effect_and_bot_surfaces` plus
focused reveal-specific tests.

After state: the pilot matrix remains intact, and
`residual_profile_tree_count_effect_and_rng_surfaces_keep_lead_commit_hidden`
adds post-lead-commit pre-reveal checks for public counts/profile labels,
owner/opponent projected views, filtered effects, reply action tree debug/v1
bytes, reply bot input, and deterministic reply bot decision output.

ADR-0009 classification: `unchanged`. This adds only residual evidence. It
does not rebuild the pilot matrix, change legality, change reveal timing, add
committed canaries, or alter any golden trace/fixture bytes.

Evidence:

- Existing
  `games/high_card_duel/tests/visibility.rs::no_leak_harness_covers_public_seat_replay_effect_and_bot_surfaces`
  remained green.
- New
  `games/high_card_duel/tests/visibility.rs::residual_profile_tree_count_effect_and_rng_surfaces_keep_lead_commit_hidden`
  verifies the unrevealed lead commit is absent from observer/opponent surfaces
  and present only on owner/authorized private surfaces.
- `games/high_card_duel/tests/bots.rs::bot_cannot_access_opponent_hand_deck_or_hidden_commitment_via_input_type`
  remained green, so no additional bot-test edit was needed.
- `cargo fmt --all --check` passed.
- `cargo test -p high_card_duel` passed.
- `cargo run -p replay-check -- --game high_card_duel --all` passed; 10 traces
  checked and `replay-check: all traces passed`.

### UNI8CR2TWOSEA-028 - High Card Duel replay-command-v1 profile driver

Selected surface: `games/high_card_duel/tests/replay.rs` profile-driver test
over the existing internal command trace.

Before state: High Card Duel had internal command trace generation and replay
validation, but no `ReplayCommandV1Driver` receipt.

After state:
`replay_command_v1_profile_driver_wraps_internal_trace_validator` validates
the `replay-command-v1` metadata (`v1`, `internal-dev`,
`validator_owner = high_card_duel`, `canonical_byte_authority = none`) and
delegates through `validate_with` to the existing internal trace validator.

ADR-0009 classification: `unchanged`. This adds typed profile evidence only.
The existing internal trace bytes remain authoritative, no canonical byte claim
is made, no artifact is rewritten, and no viewer surface receives internal
commands.

Evidence:

- Valid profile metadata reports `replay-command-v1`, `v1`, `internal-dev`,
  and `high_card_duel`.
- `validate_with` returns the existing trace stable hash from
  `replay_internal_full_trace`.
- Wrong profile id, wrong validator owner, and illegal profile field are
  rejected.
- `cargo fmt --all --check` passed.
- `cargo test -p high_card_duel` passed.
- `cargo run -p replay-check -- --game high_card_duel --all` passed; 10 traces
  checked and `replay-check: all traces passed`.

### UNI8CR2TWOSEA-018 - Secret Draft parallel action-tree v1 bytes/hash

Selected surface: `games/secret_draft/src/replay_support.rs` additive
action-tree v1 adapter alongside the retained legacy `action_tree_hash`.

Before state: Secret Draft had a local legacy string encoder
`action_tree_hash` used by replay evidence, but no version-pinned v1 byte/hash
surface.

After state: `action_tree_v1_bytes` and `action_tree_v1_hash` expose
`ActionTreeEncodingVersion::V1` bytes/hash for existing legal action trees.
The legacy `action_tree_hash` function remains intact.

ADR-0009 classification: `parallel-new-surface` with legacy `exception`. This
adds explicit v1 action-tree evidence for first-commit and pending-second-commit
trees without reinterpreting the legacy hash, changing legal choices, or
altering reveal timing.

Evidence:

- `games/secret_draft/tests/replay.rs::action_tree_v1_bytes_and_hashes_are_pinned_alongside_legacy_hashes`
  pins first-commit legacy/v1 values `11109919055145097380` /
  `7507` / `4430331744477066435` and pending-second legacy/v1 values
  `8995662196078409061` / `7507` / `4781253235714578176`.
- `cargo fmt --all --check` passed.
- `cargo test -p secret_draft` passed.
- `cargo run -p replay-check -- --game secret_draft --all` passed; 14 traces
  checked and `replay-check: all traces passed`.
