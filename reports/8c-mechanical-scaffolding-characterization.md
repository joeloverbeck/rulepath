# Unit 8C Mechanical-Scaffolding Characterization

Status: baseline for UNI8CMECSCA-003.

Date: 2026-06-22

Purpose: record current pilot byte/hash/visibility/RNG surfaces before Unit 8C
helper extraction. Every value below is produced or pinned by the
characterization tests added with this ticket. Expected classification is
`unchanged` unless a later ticket records a parallel-new-surface or an
intentional ADR-0009 migration.

No private card identities, hidden canary values, or seat-private payload
contents are recorded in this report. The committed tests pin hashes and
viewer/profile metadata through the Rust SUT.

## RACE-FLAT-ACTION-TREE

- owning path/symbol: `games/race_to_n/src/replay_support.rs::action_tree_hash`
  and `games/race_to_n/tests/serialization_tests.rs`.
- artifact profile: legacy `replay-command-v1` action-tree hash surface.
- visibility class: public perfect-information action tree.
- versions: `race_to_n`, rules version 1, seed 9, seats `seat-0`, `seat-1`.
- canonical-byte authority: legacy local encoder, not ADR-0009 v1 writer.
- input vector: setup seed 9, actor `seat-0`, initial legal tree.
- legacy bytes: escaped `add-1|add-2|add-3`.
- legacy hash: `8451402319224114161`.
- seat spellings and alias route: legacy hyphen seats in Race setup/tests;
  no alias normalization in this ticket.
- legacy RNG output and draw count: not applicable.
- new surface/version proposed: `action-tree-encoding-v1` in later tickets.
- expected classification: `unchanged`.
- migration_update_note: not applicable until a later migration ticket.
- compatibility window: legacy action-tree hash remains authoritative through
  this characterization ticket.
- rollback boundary: remove the test added to
  `games/race_to_n/tests/serialization_tests.rs`.
- validator commands: `cargo test -p race_to_n characterization`.

## DRAUGHTS-COMPOUND-ACTION-TREE

- owning path/symbol:
  `games/draughts_lite/src/replay_support.rs::action_tree_hash` and
  `games/draughts_lite/tests/replay.rs`.
- artifact profile: legacy `replay-command-v1` action-tree hash surface.
- visibility class: public perfect-information action tree.
- versions: `draughts_lite`, `draughts_lite-rules-v1`, seed 7 fixture state.
- canonical-byte authority: legacy private encoder inside replay support.
- input vector: `multi_jump` fixture state, actor `seat-0`, root segment
  `from/r3c2`, recursive child choices present.
- legacy bytes: private to the current local encoder; the SUT-derived hash is
  pinned as the byte-surface guard until C-04/C-05 expose a framed v1 surface.
- legacy hash: `7788678278305142813`.
- seat spellings and alias route: legacy hyphen seats in Draughts replay state.
- legacy RNG output and draw count: not applicable.
- new surface/version proposed: `action-tree-encoding-v1` in later tickets.
- expected classification: `unchanged`.
- migration_update_note: not applicable until a later migration ticket.
- compatibility window: legacy action-tree hash remains authoritative through
  this characterization ticket.
- rollback boundary: remove the characterization test from
  `games/draughts_lite/tests/replay.rs`.
- validator commands: `cargo test -p draughts_lite characterization`.

## RIVER-SETUP-EVIDENCE

- owning path/symbol:
  `games/river_ledger/data/fixtures/river_ledger_3p_standard.fixture.json` and
  `games/river_ledger/tests/replay.rs`.
- artifact profile: legacy setup fixture, future `setup-evidence-v1`.
- visibility class: public setup evidence.
- versions: `river_ledger`, `river-ledger-rules-v2`, 3 seats.
- canonical-byte authority: current fixture bytes; no new canonical-byte claim.
- input vector: committed 3-seat standard fixture.
- legacy bytes: fixture text bytes.
- legacy hash: `2633580370171550625`.
- seat spellings and alias route: canonical underscore seats in River fixtures.
- legacy RNG output and draw count: not applicable for this fixture hash.
- new surface/version proposed: `setup-evidence-v1` metadata in later tickets.
- expected classification: `unchanged`.
- migration_update_note: not applicable until a later metadata migration.
- compatibility window: current fixture remains readable through C-11.
- rollback boundary: remove the River characterization test from
  `games/river_ledger/tests/replay.rs`.
- validator commands: `cargo test -p river_ledger characterization`;
  `cargo run -p fixture-check -- --game river_ledger`.

## RIVER-PUBLIC-AND-SEAT-PRIVATE-SURFACES

- owning path/symbol:
  `games/river_ledger/tests/golden_traces/public-replay-export-import.trace.json`,
  `games/river_ledger/tests/golden_traces/seat-private-view.trace.json`, and
  `games/river_ledger/src/replay_support.rs::export_public_replay`.
- artifact profile: legacy public/seat-private visibility and export fixtures,
  future `public-export-v1` and `seat-private-export-v1` surfaces.
- visibility class: public observer and seat-private viewer.
- versions: `river_ledger`, `river-ledger-rules-v2`, seed 21, 3 seats.
- canonical-byte authority: current fixture/export stable bytes.
- input vector: setup-only trace from `trace_from_commands(21, 3, &[])`, viewer
  `observer` and viewer `seat_0`.
- legacy bytes: fixture/export stable bytes.
- legacy hashes:
  - public replay fixture: `11946834064931283956`.
  - seat-private fixture: `6382002720248622821`.
  - observer export: `2482097568303728278`.
  - seat_0 export: `7443748736294317283`.
- seat spellings and alias route: canonical underscore `seat_0`.
- legacy RNG output and draw count: not applicable for export hashes.
- new surface/version proposed: profile drivers in later tickets.
- expected classification: `unchanged`.
- migration_update_note: not applicable until a later profile migration.
- compatibility window: current fixtures/exports remain readable through C-11.
- rollback boundary: remove the River characterization test from
  `games/river_ledger/tests/replay.rs`.
- validator commands: `cargo test -p river_ledger characterization`;
  `cargo run -p replay-check -- --game river_ledger --all`.

## RIVER-RNG-UNBIASED-LOCAL

- owning path/symbol:
  `games/river_ledger/src/setup.rs::next_bounded_index_unbiased`.
- artifact profile: deterministic RNG characterization.
- visibility class: internal-dev.
- versions: deterministic RNG contract over `u64` words; upper bound 3.
- canonical-byte authority: no byte surface; output and draw count are pinned.
- input vector: upper bound 0, then upper bound 3 with words
  `18446744073709551615`, `4`, `9`.
- legacy bytes: not applicable.
- legacy hash: not applicable.
- seat spellings and alias route: not applicable.
- legacy RNG output and draw count: zero-bound returns `None` with 0 draws;
  bounded call rejects the first word, accepts `4`, returns index `1`, and
  consumes 2 words.
- new surface/version proposed: `next_index_unbiased_v1` in later tickets.
- expected classification: `unchanged`.
- migration_update_note: not applicable until the River helper replacement.
- compatibility window: local helper remains authoritative until C-09 adoption.
- rollback boundary: remove the test added to `games/river_ledger/src/setup.rs`.
- validator commands: `cargo test -p river_ledger characterization`.

## HIGH-CARD-DUEL-PUBLIC-AND-SEAT-PRIVATE

- owning path/symbol:
  `games/high_card_duel/tests/golden_traces/public-replay-export-import.trace.json`,
  `games/high_card_duel/tests/golden_traces/seat-private-view.trace.json`, and
  `games/high_card_duel/src/replay_support.rs::export_public_observer_replay`.
- artifact profile: legacy public export and seat-private view surfaces.
- visibility class: public observer and seat-private viewer.
- versions: `high_card_duel`, `high-card-duel-rules-v1`, seed 9.
- canonical-byte authority: current fixture/export stable bytes.
- input vector: generated internal trace seed 9; public observer export.
- legacy bytes: fixture/export stable bytes.
- legacy hashes:
  - public replay fixture: `3518406067041173473`.
  - seat-private fixture: `15303656505157591945`.
  - generated observer export: `11079559833511455730`.
- seat spellings and alias route: current committed traces include legacy
  hyphen spellings where applicable; no alias normalization in this ticket.
- legacy RNG output and draw count: not applicable.
- new surface/version proposed: no-leak harness and export profile drivers in
  later tickets.
- expected classification: `unchanged`.
- migration_update_note: not applicable until a later profile migration.
- compatibility window: current fixtures/exports remain readable through C-11.
- rollback boundary: remove the High Card characterization test from
  `games/high_card_duel/tests/replay.rs`.
- validator commands: `cargo test -p high_card_duel characterization`.

## VOW-TIDE-PUBLIC-AND-SEAT-PRIVATE-EXPORTS

- owning path/symbol:
  `games/vow_tide/tests/golden_traces/public-replay-export-import.trace.json`,
  `games/vow_tide/tests/golden_traces/seat-private-replay-export-import-all-viewers.trace.json`,
  and `games/vow_tide/src/replay_support.rs::export_for_viewer`.
- artifact profile: legacy public and seat-private viewer export surfaces.
- visibility class: public observer and seat-private viewer.
- versions: `vow_tide`, `vow-tide-rules-v1`, seed 20260621, 4 seats.
- canonical-byte authority: current fixture/export stable bytes.
- input vector: setup seed 20260621, viewers `observer` and `seat_0`.
- legacy bytes: fixture/export stable bytes.
- legacy hashes:
  - public export fixture: `9606057229737834804`.
  - all-viewers seat-private fixture: `16909558442784598481`.
  - observer export: `14136592432406028852`.
  - seat_0 export: `12688236753872554050`.
- seat spellings and alias route: canonical underscore seats.
- legacy RNG output and draw count: not applicable.
- new surface/version proposed: `public-export-v1` and
  `seat-private-export-v1` drivers in later tickets.
- expected classification: `unchanged`.
- migration_update_note: not applicable until a later profile migration.
- compatibility window: current fixtures/exports remain readable through C-11.
- rollback boundary: remove the Vow characterization test from
  `games/vow_tide/tests/replay.rs`.
- validator commands: `cargo test -p vow_tide characterization`.

## BRIAR-CIRCUIT-DOMAIN-EVIDENCE

- owning path/symbol:
  `games/briar_circuit/data/fixtures/briar_circuit_moon.fixture.json` and
  `games/briar_circuit/data/fixtures/briar_circuit_first_trick_exception.fixture.json`.
- artifact profile: legacy domain/setup fixtures, future `domain-evidence-v1`.
- visibility class: public fixture metadata; game-local scoring/legality remains
  Rust-owned.
- versions: `briar_circuit`, `briar-circuit-rules-v1`, seeds 1602 and 1601.
- canonical-byte authority: current fixture bytes; no new canonical-byte claim.
- input vector: committed moon and first-trick-exception fixture files.
- legacy bytes: fixture text bytes.
- legacy hashes:
  - moon fixture: `12129920730792203110`.
  - first-trick-exception fixture: `16932830783837267987`.
- seat spellings and alias route: canonical underscore seats.
- legacy RNG output and draw count: not applicable.
- new surface/version proposed: `domain-evidence-v1` driver in later tickets.
- expected classification: `unchanged`.
- migration_update_note: not applicable until a later profile migration.
- compatibility window: current fixtures remain readable through C-11.
- rollback boundary: remove the Briar characterization test from
  `games/briar_circuit/tests/replay.rs`.
- validator commands: `cargo test -p briar_circuit characterization`.
