# ADR-0001: Rust core with WebAssembly web shell

## Status

Accepted.

## Context

Rulepath is first a public playable web app, but its rules must be deterministic, replayable, testable, and benchmarkable. A browser-only rules engine would make native simulations, deterministic replay, and future authoritative server reuse weaker. A server-first product would delay the static public portfolio site.

## Decision

Rulepath uses a Rust workspace as the rule and simulation foundation, exposed to the browser through a thin WebAssembly API.

Rust owns:

- setup;
- legal action trees and paths;
- validation;
- state transitions;
- deterministic RNG;
- semantic effects;
- replay;
- serialization contracts;
- public/private views;
- bot decisions and bot hot loops;
- native simulation and benchmarks.

React/TypeScript owns:

- app shell;
- game picker;
- match setup;
- layout and panels;
- renderer integration;
- input orchestration;
- replay controls;
- settings and accessibility wrappers;
- WASM package integration.

The TypeScript client MUST NOT implement rule legality.

## Consequences

Positive:

- one authoritative rule implementation can serve native tests, simulations, WASM local play, and future server play;
- public static deployment remains possible;
- native benchmarks can run before browser profiling;
- bots can share the same legal-action API as humans.

Negative:

- the WASM boundary must be designed carefully;
- browser integration requires serialization and binding discipline;
- UI developers cannot quickly patch rule behavior in TypeScript.

Migration consequences:

- any temporary TypeScript mock rules must be clearly marked disposable and removed before a game is considered implemented.

## Alternatives considered

### TypeScript-only rules

Rejected. It weakens native simulation, replay, benchmarking, and future authoritative server reuse.

### Rust server first

Rejected for v1. It delays the public static playable site and drags in accounts, hosting, persistence, and multiplayer concerns too early.

### Existing web framework as the core

Rejected for the foundation. Existing systems are useful precedents, but Rulepath's contract centers deterministic Rust rule enforcement, visibility safety, and engine cleanliness.
