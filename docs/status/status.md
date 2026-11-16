# Project Status

Last updated: 2026-11-16

**Time invested:** ~1h _(human — agents must not edit)_

<!-- AGENT-README:
  In Progress = human is working here by hand. Do NOT edit; do not take as a task.
  Next Up     = ordered queue. Top entry = next agent step. Top to bottom.
  Later       = backlog. Do not start without asking.
  Ticket format: `[id] (module) Title` — id is stable, for references.
  Done = done.md — status.md stays the Kanban pipeline.
-->

## Current Phase

Demo pipeline complete (env → train → eval). Next: Versus-lang probe (agent hook).

## Done

See [done.md](done.md).

## In Progress _(human — agents must not edit)_

_(none)_

## Next Up

- [ ] `[versus-probe-max-pressure]` (script) Spike: Versus-lang as agent hook
  - Goal: decide keep / drop Versus for this repo (embed + iteration value)
  - Scope (thin): one scripted policy `max_pressure` (green busier NS vs EW axis from obs/queues)
  - Wire into demo as third mode beside neural + manual (e.g. key **V** or cycle with **P**)
  - Document: how Versus is embedded/run, minimal example script path, rebuild needed or hot-reload
  - Out of scope: reward scripting, spawn scenarios, train integration
  - Done when: script drives lights in live demo; short note in Gaps (keep vs Rhai/Lua/TOML vs drop)

## Later (Backlog)

- [ ] `[versus-scripting]` (script) Broader Versus hooks (reward / scenarios) — only if probe keeps it
- [ ] `[docs-readme-demo]` (docs) Full stack / run-demo / run-train README like balancer

## Gaps / Notes

- Sister of `drl-balancer`; keep Cargo/lints/docs/status conventions aligned.
- Versus-lang is optional — probe first; only expand if embed is easy and iteration clearly improves.
- Training uses shaped reward + entropy; old `policy.mpk` from flat `-waiting` may collapse to one phase — re-train.
- Best Versus probe target: heuristic agent script (not viz/train hot path).
