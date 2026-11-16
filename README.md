# DRL Intersection

Small learning/demo repo: **deep reinforcement learning** for a traffic **intersection** in Rust with [Burn](https://burn.dev/). Sister project to [drl-balancer](https://github.com/RunBerserk/drl-balancer) (CartPole). Live demo uses [macroquad](https://github.com/not-fl3/macroquad); Versus-lang may join later for agent/env scripting.

The agent (for now: you) controls **traffic lights**; cars are rule-based.

## Stack

| Component | Technology |
|-----------|------------|
| Language | Rust (edition 2024) |
| Env | Custom 4-way intersection (NS/EW phases, min-green, queues) |
| ML | [Burn](https://burn.dev/) _(planned)_ |
| Demo / render | [macroquad](https://github.com/not-fl3/macroquad) 0.4 |
| Scripting _(optional)_ | Versus-lang _(maybe)_ |
| Lint | `warnings = deny`, `clippy::all = deny` |

Binaries:

| Command | Binary | Role |
|---------|--------|------|
| `cargo run` | `drl-intersection` (default) | macroquad demo |

## Requirements

- Rust toolchain (edition 2024 / recent stable)
- Display for the demo window

## Run: demo

```bash
cargo run
# equivalent: cargo run --bin drl-intersection
```

Top-down 4-way intersection with spawning cars and signal lights. Manual mode:

| Key | Action |
|-----|--------|
| **1** / **A** | NS green |
| **2** / **D** | EW green |
| **R** | Reset episode |
| **Esc** | Quit |

Phases respect a **min-green** lock (~2 s). Reward is `-waiting_cars` each step; episode ends at 1000 steps.

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
  main.rs           # macroquad entry
  demo.rs           # fixed-timestep demo loop
  input.rs          # keys
  viz.rs            # map + cars + lights
docs/status/        # Kanban (status.md / done.md)
artifacts/          # training weights (gitignored, later)
```

## Project status

Kanban: [docs/status/status.md](docs/status/status.md)

Env + manual demo are up; Burn policy / train binary come next.

## License

[MIT](LICENSE)
