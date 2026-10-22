# Phase 0 evidence

This directory holds the throwaway benchmarks and raw output used by `docs/phase-0.md`. They measure the old implementation. They are not production code and do not define the new architecture.

## Quiet benchmark

`quiet-bench` runs `Simulator::step` in a loop without per-step output.

```bash
cd docs/phase-0/evidence/quiet-bench
cargo run --release -- 10000
```

`quiet-5runs.txt` records five runs at 300 objects. `quiet-time-v.txt` records `/usr/bin/time -v` for one run, including CPU and memory.

## Phase benchmark

`phase-bench` times the five public simulation systems around their calls. It prints throughput, system shares, final population, and generations.

```bash
cd docs/phase-0/evidence/phase-bench
cargo run --release -- 3000 300
cargo run --release -- 2000 500
```

`phases-300-5runs.txt` and `phases-500-5runs.txt` record five pinned runs each. The runs used `taskset -c 0` to reduce interference from the browser process.

## Limits

- The measurements use the old 4-8-1 network and old sensor code.
- The machine has 2 cores and 3.8 GB of memory.
- `perf` is blocked by `perf_event_paranoid=4`, so the system split uses timers.
- Population and generation changes add noise to the scaling result.
