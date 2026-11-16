# Project Status

Last updated: 2026-11-16

**Time invested:** ~0h _(human — agents must not edit)_

<!-- AGENT-README:
  In Progress = human is working here by hand. Do NOT edit; do not take as a task.
  Next Up     = ordered queue. Top entry = next agent step. Top to bottom.
  Later       = backlog. Do not start without asking.
  Ticket format: `[id] (module) Title` — id is stable, for references.
  Done = done.md — status.md stays the Kanban pipeline.
-->

## Current Phase

Scaffold: Rust + macroquad Hello World. Intersection env and Burn training are backlog.

## Done

See [done.md](done.md).

## In Progress _(human — agents must not edit)_

_(none)_

## Next Up

- [ ] `[viz-intersection-map]` (viz) Draw a basic intersection (roads, lanes, stop lines)
- [ ] `[env-intersection-physics]` (env) Minimal traffic env: cars, lights/signals, step API

## Later (Backlog)

- [ ] `[rl-burn-policy]` (rl) Burn policy + train binary (parity with drl-balancer)
- [ ] `[demo-eval-loop]` (demo) Load policy, agent ↔ manual toggle
- [ ] `[versus-scripting]` (script) Evaluate Versus-lang for agent/env hooks _(optional)_
- [ ] `[docs-readme-demo]` (docs) Full stack / run-demo / run-train README like balancer

## Gaps / Notes

- Sister of `drl-balancer`; keep Cargo/lints/docs/status conventions aligned.
- Versus-lang is optional — only pick up if it clearly helps iteration.
