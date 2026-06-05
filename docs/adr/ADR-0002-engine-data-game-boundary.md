# ADR-0002: Engine, game-stdlib, game module, and data boundary

## Status

Accepted.

## Context

Rulepath needs a game-agnostic engine without pretending that game rules are data-driven. The engine must stay small enough to remain reusable, testable, and uncontaminated by early games. Static data is useful for content, but it must not become an untyped programming language.

## Decision

`engine-core` owns only generic contracts and infrastructure:

- identities;
- seats;
- deterministic RNG contracts;
- action trees and paths;
- commands;
- effect logs;
- replay;
- visibility/public-view contracts;
- serialization contracts;
- diagnostics;
- versioning;
- hashes.

`engine-core` MUST NOT contain game nouns, board/card/faction/scenario terms, rule-specific helpers, bot strategy, UI layout, networking, accounts, persistence, or licensed data.

`game-stdlib` owns reusable mechanics only after repeated pressure, preferably two implemented games or an ADR. It may eventually contain helpers for grids, coordinates, line detection, zones/piles, decks, tracks/resources, graph maps, simultaneous-choice helpers, and reaction-window helpers.

`games/*` owns game nouns, setup, legal action generation, validation, state transitions, scoring, terminal detection, visibility projection, semantic effect emission, game-specific bots, game-specific UI metadata, tests, and rules docs.

Static data MAY contain content and typed parameters. Static data MUST NOT contain procedural behavior.

## Consequences

Positive:

- the engine remains portable across games;
- game-specific code can be explicit and readable;
- reusable primitives are earned, not imagined;
- agents have clearer boundaries.

Negative:

- early games may duplicate some local mechanics;
- extracting helpers requires back-port work;
- some content authoring is more verbose than a hypothetical DSL.

Migration consequences:

- a helper may move from `games/*` to `game-stdlib` only with tests, docs, and back-port proof;
- a concept may enter `engine-core` only through the kernel-change protocol and, usually, ADR.

## Alternatives considered

### Put grids/cards/resources directly in `engine-core`

Rejected. Those are reusable mechanics at best, not universal engine contracts.

### Make rules data-driven from the start

Rejected. It creates an accidental untyped language and hides behavior from Rust tests, type checking, benchmarks, and source review.

### Keep everything inside each game forever

Rejected as a permanent rule. Local duplication is acceptable early, but repeated proven shapes should move to `game-stdlib`.
