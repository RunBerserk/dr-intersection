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

Demo pipeline complete (env → train → eval). Optional: Versus-lang / README polish.

## Done

See [done.md](done.md).

## In Progress _(human — agents must not edit)_

_(none)_

## Next Up

_(empty)_

## Later (Backlog)

- [ ] `[versus-scripting]` (script) Evaluate Versus-lang for agent/env hooks _(optional)_
- [ ] `[docs-readme-demo]` (docs) Full stack / run-demo / run-train README like balancer

## Gaps / Notes

- Sister of `drl-balancer`; keep Cargo/lints/docs/status conventions aligned.
- Versus-lang is optional — only pick up if it clearly helps iteration.
- Training uses shaped reward + entropy; old `policy.mpk` from flat `-waiting` may collapse to one phase — re-train.
