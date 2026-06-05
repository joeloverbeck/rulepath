# ADR-0006: Static local-first app and multiplayer deferral

## Status

Accepted.

## Context

Rulepath's first priority is a polished public playable portfolio site. Accounts, databases, matchmaking, persistence, moderation, hosting, reconnects, and anti-cheat would delay the product while adding infrastructure that does not prove the deterministic rules engine. At the same time, future multiplayer should not require a rewrite.

## Decision

The initial public Rulepath app is static and local-first.

Initial modes:

- human vs bot;
- local hotseat;
- bot vs bot replay;
- replay viewer;
- game picker;
- local replay export/import where practical.

Initial non-goals:

- accounts;
- database;
- hosted multiplayer;
- matchmaking;
- authoritative server deployment;
- persistence beyond local replay/snapshot artifacts.

The engine still preserves multiplayer-ready foundations:

- deterministic command logs;
- replay from seed and command stream;
- player/seat identity;
- action validation;
- action trees/action paths;
- serializable state;
- public/private views;
- viewer-filtered effects;
- versioning;
- hashes.

Future hosted multiplayer MUST use an authoritative Rust server. Browser clients MUST NOT own authoritative state.

## Consequences

Positive:

- public site can ship sooner and be easier to host;
- architecture focuses on rules, replay, visibility, UI polish, and bots;
- future multiplayer foundations are still preserved.

Negative:

- online play is not an early marketing feature;
- server-side persistence and reconnect design are deferred;
- some multiplayer UX decisions remain open.

Migration consequences:

- future server design requires ADR;
- network payloads must use existing public/private view and effect-filtering contracts;
- browser clients may preview locally but server validation remains final.

## Alternatives considered

### Hosted multiplayer in v1

Rejected. It adds too much infrastructure before local play proves the product.

### Peer-to-peer deterministic lockstep

Rejected as the future default. Deterministic logs are required, but hidden-information games and trust boundaries favor an authoritative server.

### Browser-authoritative multiplayer

Rejected. It is unsafe and incompatible with fair hidden-information play.
