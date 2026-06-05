# Rulepath Multiplayer Policy

Status: multiplayer deferral and future architecture law.

Online multiplayer is not part of the initial build. Rulepath MUST preserve future multiplayer options without dragging accounts, matchmaking, persistence, networking, or server deployment into v1.

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
- replay viewer;
- local replay import/export.

Bots do not require multiplayer infrastructure.

## 2. Multiplayer-ready foundations required now

Even without online play, the engine MUST support:

- deterministic command logs;
- replay from seed and command stream;
- game/rules/data versioning;
- player identity;
- seat identity;
- actor/viewer identity;
- action validation;
- action trees/action paths;
- serializable state;
- public/private views;
- viewer-filtered effect logs;
- state/effect/action-tree/checkpoint hashes;
- batched API boundaries;
- stale-action diagnostics.

These are useful for local play and debugging immediately, and they prevent future multiplayer from requiring a rewrite.

## 3. Future hosted multiplayer law

Future hosted multiplayer MUST use an authoritative Rust server running the same rules natively.

Browser clients MUST NOT be trusted as authoritative state owners.

Future architecture SHOULD look like:

```text
client sends proposed action path
  -> server validates against authoritative Rust state
  -> server applies command through Rust engine
  -> server appends command/effects to log
  -> server sends each client filtered public/private view and effects
  -> clients render and reconcile previews
```

Clients MAY preview legal actions locally using WASM. Server validation remains authoritative.

## 4. Deterministic logs vs peer lockstep

Deterministic command logs are required.

Do not confuse this with committing to peer-to-peer deterministic lockstep online multiplayer.

For hidden-information card/board games, peer lockstep is awkward because clients must not receive hidden state. Authoritative server architecture is the safer future default.

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
- exposing private licensed content through static bundles;
- making admin credentials the only barrier to shipped private content.

Allowed:

- server-side full state;
- viewer-filtered public views;
- private hand views for the owning player;
- redacted replay exports;
- local developer builds with full-state inspector.

## 6. Reconnect and persistence, later

Do not build persistence in v1.

When multiplayer becomes real, persistence SHOULD store:

- match metadata;
- rules version;
- data version;
- seed;
- command stream;
- periodic snapshots/checkpoints;
- player/seat mapping;
- server-side private state;
- audit data;
- migration/version metadata.

Persistence design REQUIRES ADR.

## 7. Latency policy, later

Turn-based games can tolerate more latency than twitch games. Optimize correctness, trust, and clarity first.

Future clients MAY use optimistic local previews for responsiveness, but final state MUST reconcile to the server-authoritative result.

Any optimistic UI MUST handle rejected/stale actions gracefully.

## 8. Security and abuse, later

Accounts, auth, anti-cheat, moderation, rate limits, abuse handling, and matchmaking are explicitly deferred.

Do not add them until:

- local engine is stable;
- public static demo is good;
- multiple ladder games are implemented;
- command/replay infrastructure is proven;
- visibility filtering is proven;
- multiplayer ADR is accepted.

## 9. Local modes

Initial local modes SHOULD include:

| Mode | Required? | Notes |
|---|---|---|
| human vs bot | yes | core public demo mode |
| local hotseat | yes where game supports multiple humans | no accounts needed |
| bot vs bot replay | yes | useful for demos and debugging |
| replay viewer | yes | first-class feature |
| local saved replay import/export | should | safe JSON or file download/upload |
| online multiplayer | no | future ADR only |

## 10. Multiplayer anti-patterns

MUST NOT:

- add online multiplayer to the first implementation order;
- trust browser clients as source of truth;
- let networking concerns invade `engine-core`;
- ship hidden full state to clients;
- add accounts before local play is excellent;
- build matchmaking before the engine has games worth matching;
- make bots depend on online multiplayer;
- bundle private modules/data in public static files;
- design public UI around hypothetical server features.

## Source notes

See `SOURCES.md`, especially deterministic lockstep, client-server architecture, command-log replay, Rust/WASM, boardgame.io, and IP/public-private module sources.
