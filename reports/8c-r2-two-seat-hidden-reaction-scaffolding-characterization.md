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
