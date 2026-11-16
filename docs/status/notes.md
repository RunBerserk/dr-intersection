# Notes

## Versus-lang (2026-11-16) — dropped

Ticket `[versus-probe-max-pressure]` was a go/no-go probe for Versus as an agent/env scripting hook in this repo.

**Result: drop Versus** for `drl-intersection`.

- No usable Rust embed / crates.io host for Versus-lang was found.
- Manual + Burn agent is enough here; a scripting layer adds little for a 2-action traffic-light policy.

**Rhai:** briefly considered as an embeddable stand-in, but **not** adopted — unnecessary complexity (dependency, extra demo modes, hot-reload) for a heuristic that would be trivial in Rust. The probe ticket did not ask for a Rhai experiment.

**Later (optional):** revisit Versus if we ever explore **model quantization** (shrinking the Burn policy for size/speed), and only if Versus has a clear host story by then.
