# DRL Intersection

Small learning/demo repo: **deep reinforcement learning** for a traffic **intersection** in Rust with [Burn](https://burn.dev/). Sister project to [drl-balancer](https://github.com/RunBerserk/drl-balancer) (CartPole). Live demo uses [macroquad](https://github.com/not-fl3/macroquad); Versus-lang may join later for agent/env scripting.

The agent controls **traffic lights**; cars are rule-based.

## Stack

| Component | Technology |
|-----------|------------|
| Language | Rust (edition 2024) |
| Env | Custom 4-way intersection (NS/EW phases, min-green, queues) |
| ML | [Burn](https://burn.dev/) 0.21 — `ndarray` + `autodiff` backends |
| Policy | MLP `7 → 64 → 2` (REINFORCE) |
| Demo / render | [macroquad](https://github.com/not-fl3/macroquad) 0.4 |
| Scripting _(optional)_ | Versus-lang _(maybe)_ |
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

Phases respect a **min-green** lock (~2 s). Reward mixes throughput, waiting penalty, and “green the busier axis” shaping; episode ends at 1000 steps. In agent mode: auto-reset ~1 s after episode end. HUD shows `want:NS/EW` for the policy’s requested phase.

## Run: training

```bash
cargo run --release --bin train
```

- Algorithm: **REINFORCE** (Adam, return norm, grad clip, entropy bonus)
- 300 episodes × 300 steps; early stop when moving-average return ≥ **150**
- Writes weights to **`artifacts/policy.mpk`** (`artifacts/` is gitignored)

Typical (release, CPU): several minutes depending on early stop. Re-train after pulling training changes.

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
  viz.rs            # map + cars + lights
docs/status/        # Kanban (status.md / done.md)
artifacts/          # training weights (gitignored)
```

## Project status

Kanban: [docs/status/status.md](docs/status/status.md)

Demo pipeline complete: env → train → live eval.

## License

[MIT](LICENSE)
