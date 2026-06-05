# ADR-0005: Bot policy architecture and no omniscient bots

## Status

Accepted.

## Context

Rulepath needs competent, explainable, non-superhuman bots for public demos. Hidden-information games make cheating bots especially dangerous because they can leak private state into decisions, logs, previews, or debug output. Research AI is attractive but premature for a public portfolio site whose first requirement is enjoyable play.

## Decision

Bots MUST consume the same legal action API as humans and MUST choose only legal actions.

Bots MUST NOT choose actions using information unavailable to their seat.

There is no `DiagnosticOmniscientBot` category. Testing tools MAY inspect internal state and may generate diagnostics, fixtures, or assertions, but they MUST NOT implement the public bot trait or choose production/play actions from hidden state.

Public bot ladder:

1. Level 0: random legal bot for every game.
2. Level 1: rule-informed baseline bot for public demos.
3. Level 2: authored policy bot as the default polished bot.
4. Level 3: shallow deterministic search only for small perfect-information games where benchmarks prove it fits.

MCTS, ISMCTS, Monte Carlo-style bots, ML, and RL are out of public v1/v2 unless an ADR overturns this with benchmarks and a clear reason.

Preferred architecture:

- game-specific policy modules in `games/*`;
- reusable bot traits, deterministic RNG, instrumentation, policy composition helpers, and lexicographic priority helpers in `ai-core`;
- ordered tactical priorities;
- phase-aware decision trees;
- behavior-tree-like policy nodes where useful;
- small scoped scoring functions only as tie-breakers or bounded evaluators;
- explanation reasons;
- deterministic tie-breaking from bot seed;
- decision latency benchmarks.

## Consequences

Positive:

- public bots are fair and explainable;
- hidden-information safety is testable;
- early demos avoid impressive but brittle research AI;
- style profiles can exist without cheating.

Negative:

- non-random bots require game-specific work;
- bots may be weaker than search/ML systems;
- hidden-information bots need documented belief/sample models if they reason beyond public facts.

Migration consequences:

- any existing omniscient bot concept must be renamed as a test oracle/tool and prohibited from choosing gameplay actions;
- hidden-info games need no-leak tests for bot views and decision traces.

## Alternatives considered

### Diagnostic omniscient bots

Rejected. The term normalizes cheating and risks accidental public use. Test oracles can inspect state; bots cannot.

### MCTS/ISMCTS by default

Rejected. It requires benchmarks, playout speed, action abstraction, hidden-information discipline, and careful UX explanation that the early public ladder does not yet have.

### ML/RL in v1/v2

Rejected. It adds training, reproducibility, model-storage, inference, explainability, and maintenance burdens before the public product proves itself.
