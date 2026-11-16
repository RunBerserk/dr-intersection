# Done

## Scaffold: Rust + Git

- [x] 1. Cargo binary `drl-intersection` + macroquad Hello World
- [x] 2. `.gitignore`, README, MIT LICENSE
- [x] 3. Docs Kanban: `docs/status/status.md` + `done.md`
- [x] 4. GitHub `main` (`RunBerserk/dr-intersection`)

## Viz: intersection map `[viz-intersection-map]`

- [x] 1. Top-down 4-way roads (N–S / E–W asphalt)
- [x] 2. Dashed center lines + edge lane marks on approaches
- [x] 3. White stop lines on all four approaches
- [x] 4. Esc quit via `input::quit_pressed` (HUD hint)

## Env: intersection physics `[env-intersection-physics]`

- [x] 1. Lib `intersection`: `Action` / `Phase` / cars / `step` / `observation`
- [x] 2. NS↔EW phases with min-green; reward `-waiting`; episode cap `MAX_STEPS`
- [x] 3. Demo: fixed timestep, 1/A & 2/D phases, R reset; viz cars + lights
- [x] 4. Unit tests (switch lock, red stop, green go, reward, done)

## RL: Burn policy + training `[rl-burn-policy]`

- [x] 1. Lib layout: `intersection` / `policy` / `train`; `train` binary
- [x] 2. Burn 0.21 policy MLP (7→64→2) on `Autodiff<NdArray>`
- [x] 3. REINFORCE loop (Adam, return norm, early-stop avg≥-200)
- [x] 4. Saves `artifacts/policy.mpk`; README training section
