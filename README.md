# DRL Intersection

Small learning/demo repo: **deep reinforcement learning** for a traffic **intersection** in Rust with [Burn](https://burn.dev/). Sister project to [drl-balancer](https://github.com/RunBerserk/drl-balancer) (CartPole). An agent controls **traffic lights**; cars are rule-based. The live demo uses [macroquad](https://github.com/not-fl3/macroquad).

## Stack

| Component | Technology |
|-----------|------------|
| Language | Rust (edition 2024) |
| Env | Custom 4-way intersection (τ=0.05, min-green 2 s, right-hand traffic, episode cap 1000 steps) |
| Obs / actions | Queues N/S/E/W + phase one-hot + time-in-phase → `NsGreen` / `EwGreen` |
| ML | [Burn](https://burn.dev/) 0.21 — `ndarray` + `autodiff` backends |
| Policy | MLP `7 → 64 → 2` (REINFORCE) |
| Demo / render | [macroquad](https://github.com/not-fl3/macroquad) 0.4 |
| Lint | `warnings = deny`, `clippy::all = deny` |

Binaries:

| Command | Binary | Role |
|---------|--------|------|
| `cargo run` | `drl-intersection` (default) | macroquad demo |
| `cargo run --release --bin train` | `train` | headless training |

## Requirements

- Rust toolchain (edition 2024 / recent stable)
- Display for the demo window
- Training runs headless on CPU (`ndarray`)

## Run: demo

Without weights, train first (writes `artifacts/policy.mpk`):

```bash
cargo run --release --bin train
```

Then start the demo:

```bash
cargo run
# equivalent: cargo run --bin drl-intersection
```

If `artifacts/policy.mpk` exists, the demo starts in **agent mode** (greedy argmax). Otherwise it stays manual (console hint: train as above).

| Key | Action |
|-----|--------|
| **P** | Toggle agent ↔ manual (only if a policy is loaded) |
| **1** / **A** | NS green; override while in agent mode |
| **2** / **D** | EW green; override while in agent mode |
| **R** | Reset episode |
| **Esc** | Quit |

Phases respect a **min-green** lock (~2 s). Cars roll up to the stop line on red, then wait. Reward mixes throughput, waiting penalty, and “green the busier axis” shaping. Demo episodes end at 1000 steps; in agent mode auto-reset ~1 s after that. HUD shows mode, phase, queues, and `want:NS/EW`.

## Run: training

```bash
cargo run --release --bin train
```

- Algorithm: **REINFORCE** (Adam, return normalization, grad clip, entropy bonus)
- Horizon: **300** steps/episode (shorter than demo cap for denser credit assignment)
- Early stop when moving-average return ≥ **150** (else max 300 episodes)
- Writes weights to **`artifacts/policy.mpk`** (`artifacts/` is gitignored)
- Logs greedy phase-switch count on eval after training

Typical (release, CPU): solved within a few minutes when pressure shaping works; re-train after reward/env changes.

## Tests

```bash
cargo test
cargo clippy --all-targets
```

## Layout

```
src/
  lib.rs            # crate root
  intersection.rs   # env: lights, cars, step/obs
  policy.rs         # Burn PolicyNet
  train.rs          # REINFORCE loop
  agent.rs          # inference: load policy.mpk
  bin/train.rs      # training entry
  main.rs           # demo entry
  demo.rs           # fixed-timestep demo loop
  input.rs          # keys
  viz.rs            # map + cars + lights (render-only)
docs/status/        # Kanban (status.md / done.md / notes.md)
artifacts/          # training weights (gitignored)
```

## Project status

Kanban: [docs/status/status.md](docs/status/status.md)

Demo pipeline complete: env → train → live eval. Versus scripting was evaluated and dropped — [notes.md](docs/status/notes.md).

## License

[MIT](LICENSE)
