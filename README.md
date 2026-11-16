# DRL Intersection

Small learning/demo repo: **deep reinforcement learning** for a traffic **intersection** in Rust with [Burn](https://burn.dev/). Sister project to [drl-balancer](https://github.com/RunBerserk/drl-balancer) (CartPole). Live demo uses [macroquad](https://github.com/not-fl3/macroquad); Versus-lang may join later for agent/env scripting.

## Stack

| Component | Technology |
|-----------|------------|
| Language | Rust (edition 2024) |
| Env | Intersection traffic sim _(planned)_ |
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

Opens a window with a Hello World placeholder. Esc quits.

## Tests

```bash
cargo test
cargo clippy --all-targets
```

## Layout

```
src/
  main.rs       # macroquad entry
  viz.rs        # Hello World draw loop
docs/status/    # Kanban (status.md / done.md)
artifacts/      # training weights (gitignored, later)
```

## Project status

Kanban: [docs/status/status.md](docs/status/status.md)

Scaffold only: Hello World window is up; intersection env + Burn training come next.

## License

[MIT](LICENSE)
