# <game_id> Benchmarks

Game ID: `<game_id>`

Rules version: <version>

Data version: <version>

Engine version: <version>

Last updated: YYYY-MM-DD

## Environment

| Field | Value |
|---|---|
| Hardware | <CPU/RAM/device> |
| OS | <OS/version> |
| Rust version | <version> |
| Build profile | debug / release / bench |
| Target | native / wasm smoke |
| Engine version | <version/hash> |
| Rules version | <version> |
| Data version | <version> |

## Commands

| Benchmark | Command | Notes |
|---|---|---|
| <benchmark> | <command> | <notes> |

## Baseline numbers

| Operation | Target | Baseline | Regression threshold | Notes |
|---|---:|---:|---:|---|
| setup | <target> | <baseline> | <threshold> | <notes> |
| legal action generation | <target> | <baseline> | <threshold> | <notes> |
| preview | <target> | <baseline> | <threshold> | <notes> |
| apply action | <target> | <baseline> | <threshold> | <notes> |
| public/private view generation | <target> | <baseline> | <threshold> | <notes> |
| effect filtering | <target> | <baseline> | <threshold> | <notes> |
| serialization/deserialization | <target> | <baseline> | <threshold> | <notes> |
| replay throughput | <target> | <baseline> | <threshold> | <notes> |
| random playout throughput | <target> | <baseline> | <threshold> | <notes> |
| bot decision latency | <target> | <baseline> | <threshold> | <notes> |

## Bottlenecks

| Bottleneck | Evidence | Planned response |
|---|---|---|
| <bottleneck> | <benchmark/profile> | <response> |

## Trace and data versions

- Golden traces included:
- Data/schema versions:
- Hash compatibility notes:

## Comparison to previous release

| Operation | Previous | Current | Change | Accept? |
|---|---:|---:|---:|---:|
| <operation> | <previous> | <current> | <change> | yes/no |

## Notes

Document anomalous results, hardware differences, disabled benchmarks, or accepted regressions with rationale.
