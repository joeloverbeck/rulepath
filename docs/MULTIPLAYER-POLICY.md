# MULTIPLAYER POLICY

Status: multiplayer deferral and future architecture law.

Online multiplayer is not part of the initial build. The architecture MUST preserve future multiplayer options without dragging accounts, matchmaking, persistence, networking, or server deployment into v1.

## 1. Initial scope

Initial public app MUST be local-first:

- static site;
- no accounts;
- no database;
- no hosted multiplayer;
- no matchmaking;
- no public server deployment;
- human vs bot;
- local hotseat;
- bot vs bot replay;
- deterministic replay viewer.

Bots do not require multiplayer infrastructure.

## 2. Multiplayer-ready foundations required now

Even without online play, the engine MUST support:

- deterministic command logs;
- replay from seed and command stream;
- player identity;
- seat identity;
- action validation;
- action trees/action paths;
- serializable state;
- public/private views;
- viewer-filtered effect logs;
- rules versioning;
- state/effect/checkpoint hashes;
- batched API boundaries.

These are useful for local play and debugging immediately, and they prevent future multiplayer from requiring a rewrite.

## 3. Future hosted multiplayer law

When hosted multiplayer eventually exists, it MUST use an authoritative Rust server running the same engine natively.

Browser clients MUST NOT be trusted as authoritative state owners.

Future architecture SHOULD look like:

```text
client sends proposed action path
  -> server validates against authoritative state
  -> server applies command through Rust engine
  -> server appends command/effects to log
  -> server sends each client its filtered public view/effects
  -> clients render and optionally reconcile previews
```

Clients MAY preview legal actions locally using WASM. Server validation remains authoritative.

## 4. Deterministic logs vs online lockstep

Deterministic command logs are required.

Do not confuse this with committing to peer-to-peer deterministic lockstep online multiplayer.

For hidden-information board/card games, peer lockstep can be awkward because clients must not receive hidden state. Authoritative server architecture is the safer future default.

Use deterministic logs for:

- replay;
- debugging;
- desync diagnosis;
- audit trails;
- reconnect recovery;
- server validation;
- bot simulation;
- reproducible bug reports.

## 5. Public/private information over network

A future server MUST send only what each client is allowed to see.

Forbidden:

- sending full hidden state to the browser and hiding it in UI;
- bundling private card identities in client-visible logs;
- exposing hidden bot/private decisions in network payloads;
- making admin credentials the only barrier to shipped private licensed content.

Allowed:

- local developer builds with full-state inspector;
- server-side full state;
- viewer-filtered public views;
- private hand views for the owning player;
- redacted replay exports.

## 6. Reconnect and persistence, later

Do not build persistence in v1.

When multiplayer becomes real, persistence SHOULD store:

- match metadata;
- rules version;
- seed;
- command stream;
- periodic snapshots/checkpoints;
- player/seat mapping;
- server-side private state;
- audit data.

Persistence design REQUIRES ADR.

## 7. Latency policy, later

Turn-based games can tolerate more latency than twitch games. Optimize correctness, trust, and clarity first.

Future clients MAY use optimistic local previews for responsiveness, but final state MUST reconcile to the server-authoritative result.

## 8. Security and abuse, later

Accounts, auth, anti-cheat, moderation, rate limits, and abuse handling are explicitly deferred.

Do not add them until:

- local engine is stable;
- public static demo is good;
- multiple ladder games are implemented;
- command/replay infrastructure is proven;
- multiplayer ADR is accepted.

## 9. Multiplayer anti-patterns

MUST NOT:

- add online multiplayer to the first implementation order;
- trust browser clients as source of truth;
- let networking concerns invade `engine-core`;
- ship hidden full state to clients;
- add accounts before local play is excellent;
- build matchmaking before the engine has games worth matching;
- make bots depend on online multiplayer.

## Source notes

See `SOURCES.md`, especially deterministic lockstep, client-server architecture, command-log replay, Rust/WASM, and IP/public-private module sources.
