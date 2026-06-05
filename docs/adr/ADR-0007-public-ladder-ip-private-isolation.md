# ADR-0007: Public mechanic ladder, IP safety, and private stress-test isolation

## Status

Accepted.

## Context

Rulepath has long-term interest in complex tabletop implementations, including private stress tests, but the public product must not look or behave like a disguised licensed adaptation project. Public portfolio quality, neutral mechanics, original presentation, and safe source notes come first.

## Decision

Rulepath's public ladder uses public-domain/classic, original, or permissioned games. Trademark-risk classics use neutral names and original presentation.

The ladder is mechanics-first:

- tiny smoke game;
- flat grid placement;
- gravity/grid alignment;
- directional multi-piece effects;
- movement/capture/mandatory continuation;
- decks/chance/private views;
- resources and score economy;
- simultaneous hidden choice/drafting;
- betting/private-card state;
- trick-taking and variants;
- bluffing/reaction windows;
- cooperative event pressure;
- asymmetric area control;
- event-driven asymmetric scenario systems.

Private licensed or monster-game red-team experiments MAY occur only after the public ladder has matured, and only in private repositories, private submodules, or local-only folders excluded from public CI and public builds.

Public files MUST NOT contain proprietary names, rule prose, card text, assets, icons, screenshots, scenarios, licensed data, trade dress, or hidden private modules.

## Consequences

Positive:

- public identity remains clean and portfolio-safe;
- mechanics are proven incrementally;
- engine abstractions are earned from public games;
- private stress tests cannot contaminate public naming, docs, builds, or architecture.

Negative:

- famous complex games cannot be used as public milestones;
- original microgames require design work;
- private stress tests may reveal needs that cannot immediately enter public code without abstraction and IP review.

Migration consequences:

- any public file mentioning private licensed content must be removed or generalized;
- if private stress testing reveals a reusable primitive, it must be revalidated through public/original examples before public extraction unless an ADR justifies otherwise.

## Alternatives considered

### Publicly target a private monster game

Rejected. It creates IP risk and distorts architecture around one secret target.

### Use famous branded classics in public naming/presentation

Rejected where neutral naming is safer. Mechanics can be implemented under neutral names with original visuals and original rules summaries.

### Avoid private stress tests forever

Rejected. Late private red-team work can be useful, but only after public foundations are strong and isolation is strict.
