# dfs
[![CI](https://github.com/determinism-from-scratch/dfs/actions/workflows/ci.yaml/badge.svg)](https://github.com/determinism-from-scratch/dfs/actions/workflows/ci.yaml)
![Rust](https://img.shields.io/badge/rust-stable-orange?logo=rust)
![Edition](https://img.shields.io/badge/edition-2024-blue)
![Crate](https://img.shields.io/badge/crate-v0.1.0-blue)

A distributed filesystem built around **deterministic simulation testing** (DST). The whole system runs inside a single-stepped, fully reproducible simulation runtime where faults can be injected at any I/O boundary — and where an injected error is indistinguishable from one the real kernel would have returned.

The approach follows the lineage of FoundationDB and TigerBeetle: rather than chase nondeterministic failures in production, drive the entire system from a seeded simulation so any bug replays exactly from its seed.

> **Status:** early-stage, under active development. The runtime, scheduler, fault-injection, environment, and abstraction layers are scaffolded and evolving — expect stubs and rough edges.

## Why

Distributed-systems bugs are hard because they are rare and non-reproducible: timing, interleaving, and partial failures rarely line up the same way twice. DST removes the nondeterminism. Everything that would normally be a source of entropy — scheduling, randomness, fault timing — is derived from a single seed, so a failing run is a value you can replay bit-for-bit.

Two invariants make that worth doing:

- **Determinism end to end.** Events are ordered in a min-heap by fire-time, then priority, then insertion sequence. `BTreeMap` is used over `HashMap` for stable iteration. Per-subsystem RNGs are derived from one master seed via stable keys (not init-order draws), so changing one subsystem doesn't shift another's random stream. Counters use checked arithmetic so overflow is a panic, not a silent wrap. Same seed ⇒ same run ⇒ exact replay.
- **Fault fidelity.** Injected I/O errors reproduce both the `ErrorKind` *and* the `raw_os_error()` of genuine kernel errors, so a replica cannot tell an injected failure from an organic one. Each errno has exactly one producer — either state-derived (organic) or environment-caused (injectable), never both.

## Architecture

```
src/
├── lib.rs                   # crate roots: abstraction, database, simulation
├── main.rs                  # demo: spawn a replica, run the sim
├── abstraction/             # filesystem traits + backends (real / stub / memory) — WIP
├── database/
│   └── btree.rs             # order-5 B-tree (insert / find) + tests
└── simulation/
    └── runtime/
        ├── runtime.rs        # the simulation loop
        ├── event_queue.rs    # deterministic min-heap of events
        ├── scheduler/        # Scheduler trait + FIFO impl
        ├── fault_injector/   # FaultInjector trait + "perfect" (identity) impl
        ├── environment/      # request → response against a FS backend
        └── replica.rs        # replica handle + trap() rendezvous
```

The runtime is the core. Each tick of `Runtime::run` does the following:

1. pop the next `Event` from the queue (deterministic order),
2. pass it through the fault injector,
3. hand it to the environment, which either runs it (yielding a `Response`), requeues it (rescheduled), or drops it,
4. deliver the `Response` to the target replica via `trap`, and receive its next `Request`,
5. schedule that `Request` as a new event.

The loop runs until the queue drains. Replicas execute on their own OS threads but are driven lock-step through mpsc channels and a thread-local `HANDLE`: only one replica makes progress at a time, and the interleaving is fully determined by the scheduler. The `Scheduler` (FIFO impl) assigns timing and sequence; the `FaultInjector` (perfect/identity impl) decides what, if anything, goes wrong; the `Environment` serves requests against a pluggable `FileSystem` backend.

## Development

The repo ships a Nix flake and `.envrc`, so the toolchain is pinned and reproducible — this is exactly what CI uses.

### Quick start (Nix + direnv)

1. Install Nix with flakes enabled (CI uses the Determinate Systems installer).
2. *(Optional)* with direnv + nix-direnv, run `direnv allow` once; thereafter `cd` into the repo auto-loads the dev shell.
3. Otherwise enter the shell manually:

   ```sh
   nix develop
   ```

   You get `cargo`, `clippy`, `rustc`, `rustfmt`, and `rust-analyzer` (pinned to stable via fenix), plus `pkg-config`, `gcc`, and `lld`.

4. Build, test, lint, format:

   ```sh
   cargo build --all-targets
   cargo test
   cargo clippy --all-targets -- -D warnings
   cargo fmt
   ```

The Rust channel is set in `flake.nix` (`channelName = "stable"`); flip it to `beta` / `complete` there if you need nightly or `build-std`. Nix files are formatted with `nix fmt` (alejandra).

### Without Nix

Any stable Rust toolchain works — edition 2024 needs ≥ 1.85; tested on 1.91. Install via rustup or your distro package, then run the same `cargo` commands. CI additionally passes `--locked`; add it locally for reproducible dependency resolution.

### Run the demo

```sh
cargo run
```

Spawns a single replica that prints and drives one pass of the simulation loop.

## Continuous integration

`.github/workflows/ci.yaml` runs three jobs on every push / PR to `main`, each inside `nix develop` so local and CI toolchains match:

- **compile** — `cargo build --locked --all-targets`
- **test** — `cargo test --locked`
- **clippy** — `cargo clippy --locked --all-targets -- -D warnings` (warnings are errors)

The CI badge above goes green only when all three pass, so it doubles as the "it compiles" and "clippy is clean" indicator. For separate per-job badges, split the workflow or use a per-job badge service.

## Roadmap

- Clean replica shutdown via stored `JoinHandle`s (the `Shutdown` request/response path; threads are currently detached).
- Additional fault injectors beyond the perfect/identity one, with per-op fault types so invalid op/fault pairings are unrepresentable.
- Fuller in-memory filesystem semantics — POSIX-faithful errno mapping at every operation.

## License

Not yet specified — add a `LICENSE` file before publishing.
