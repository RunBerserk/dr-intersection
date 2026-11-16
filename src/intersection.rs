//! Minimal 4-way intersection: agent controls traffic lights; cars are rule-based.

use rand::Rng;

/// Discrete signal choice (NS green vs EW green).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    NsGreen,
    EwGreen,
}

impl Action {
    /// Map policy index: `0` → NS, `1` → EW.
    pub fn from_index(index: usize) -> Self {
        if index == 0 {
            Self::NsGreen
        } else {
            Self::EwGreen
        }
    }
}

/// Current illuminated phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    NsGreen,
    EwGreen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Approach {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Debug)]
pub struct Car {
    pub approach: Approach,
    /// Position along the travel axis (see [`Car::xy`]).
    pub along: f32,
    pub speed: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct StepResult {
    pub reward: f32,
    pub done: bool,
}

/// Half-width of the junction box (world units).
pub const JUNCTION_HALF: f32 = 1.2;
/// Half-width of a road corridor.
pub const ROAD_HALF: f32 = 1.4;
/// Distance from center to stop line.
pub const STOP_LINE: f32 = 1.35;
/// Spawn / despawn distance from center.
pub const SPAWN_DIST: f32 = 9.0;
/// Lateral offset from centerline into the inbound lane.
pub const LANE_OFFSET: f32 = 0.55;
pub const CAR_LENGTH: f32 = 0.7;
pub const CAR_WIDTH: f32 = 0.4;
const CAR_GAP: f32 = 0.35;
const CAR_SPEED: f32 = 4.0;

/// Fixed integration step (seconds).
pub const TAU: f32 = 0.05;
/// Minimum time a phase must stay before a switch is accepted.
pub const MIN_GREEN: f32 = 2.0;
/// Episode length cap (steps).
pub const MAX_STEPS: usize = 1000;

/// Observation: queues N/S/E/W, phase one-hot (NS, EW), time-in-phase / MIN_GREEN.
pub const OBS_DIM: usize = 7;
pub const N_ACTIONS: usize = 2;

const DEFAULT_SPAWN_PROB: f32 = 0.025;

#[derive(Clone, Debug)]
pub struct Intersection {
    pub phase: Phase,
    pub time_in_phase: f32,
    pub cars: Vec<Car>,
    pub steps: u32,
    /// Per-step spawn probability per approach (set `0.0` in unit tests).
    pub spawn_prob: f32,
}

impl Intersection {
    pub fn new() -> Self {
        Self {
            phase: Phase::NsGreen,
            time_in_phase: MIN_GREEN,
            cars: Vec::new(),
            steps: 0,
            spawn_prob: DEFAULT_SPAWN_PROB,
        }
    }

    pub fn reset(&mut self) {
        self.phase = Phase::NsGreen;
        self.time_in_phase = MIN_GREEN;
        self.cars.clear();
        self.steps = 0;
    }

    pub fn observation(&self) -> [f32; OBS_DIM] {
        let (qn, qs, qe, qw) = self.queue_counts();
        let (ns, ew) = match self.phase {
            Phase::NsGreen => (1.0, 0.0),
            Phase::EwGreen => (0.0, 1.0),
        };
        [
            qn as f32,
            qs as f32,
            qe as f32,
            qw as f32,
            ns,
            ew,
            (self.time_in_phase / MIN_GREEN).min(2.0),
        ]
    }

    pub fn queue_counts(&self) -> (u32, u32, u32, u32) {
        let mut q = [0_u32; 4];
        for car in &self.cars {
            if car_is_queued(car) {
                q[approach_index(car.approach)] += 1;
            }
        }
        (q[0], q[1], q[2], q[3])
    }

    pub fn waiting_count(&self) -> u32 {
        self.cars.iter().filter(|c| c.speed < 1e-3 && car_before_stop(c)).count() as u32
    }

    pub fn is_done(&self) -> bool {
        self.steps as usize >= MAX_STEPS
    }

    /// Place a stopped car on an approach just before the stop line (tests / seeding).
    pub fn spawn_car_at_stop(&mut self, approach: Approach) {
        let along = match approach {
            Approach::North | Approach::East => STOP_LINE + CAR_LENGTH * 0.5 + 0.05,
            Approach::South | Approach::West => -(STOP_LINE + CAR_LENGTH * 0.5 + 0.05),
        };
        self.cars.push(Car {
            approach,
            along,
            speed: 0.0,
        });
    }

    pub fn step(&mut self, action: Action, rng: &mut impl Rng) -> StepResult {
        self.apply_action(action);
        self.update_cars();
        self.try_spawn(rng);
        self.steps = self.steps.saturating_add(1);

        let waiting = self.waiting_count();
        let done = self.is_done();
        StepResult {
            reward: -(waiting as f32),
            done,
        }
    }

    fn apply_action(&mut self, action: Action) {
        let desired = match action {
            Action::NsGreen => Phase::NsGreen,
            Action::EwGreen => Phase::EwGreen,
        };
        if desired != self.phase && self.time_in_phase >= MIN_GREEN {
            self.phase = desired;
            self.time_in_phase = 0.0;
        } else {
            self.time_in_phase += TAU;
        }
    }

    fn update_cars(&mut self) {
        let phase = self.phase;
        // Snapshot alongs for gap checks.
        let snapshot: Vec<(Approach, f32)> =
            self.cars.iter().map(|c| (c.approach, c.along)).collect();

        for car in &mut self.cars {
            let green = phase_allows(phase, car.approach);
            let past_stop = !car_before_stop(car);
            let clear = gap_ahead(car, &snapshot);
            let can_go = past_stop || (green && clear);
            car.speed = if can_go { CAR_SPEED } else { 0.0 };
            car.along += travel_dir(car.approach) * car.speed * TAU;
        }

        self.cars.retain(|c| !car_finished(c));
    }

    fn try_spawn(&mut self, rng: &mut impl Rng) {
        if self.spawn_prob <= 0.0 {
            return;
        }
        for approach in [
            Approach::North,
            Approach::South,
            Approach::East,
            Approach::West,
        ] {
            if !rng.random_bool(self.spawn_prob as f64) {
                continue;
            }
            if spawn_blocked(approach, &self.cars) {
                continue;
            }
            self.cars.push(Car {
                approach,
                along: spawn_along(approach),
                speed: 0.0,
            });
        }
    }
}

impl Default for Intersection {
    fn default() -> Self {
        Self::new()
    }
}

impl Car {
    /// World `(x, y)` with +x east, +y north.
    pub fn xy(&self) -> (f32, f32) {
        match self.approach {
            Approach::North => (-LANE_OFFSET, self.along),
            Approach::South => (LANE_OFFSET, self.along),
            Approach::East => (self.along, -LANE_OFFSET),
            Approach::West => (self.along, LANE_OFFSET),
        }
    }
}

fn approach_index(a: Approach) -> usize {
    match a {
        Approach::North => 0,
        Approach::South => 1,
        Approach::East => 2,
        Approach::West => 3,
    }
}

fn phase_allows(phase: Phase, approach: Approach) -> bool {
    matches!(
        (phase, approach),
        (Phase::NsGreen, Approach::North | Approach::South)
            | (Phase::EwGreen, Approach::East | Approach::West)
    )
}

fn travel_dir(approach: Approach) -> f32 {
    match approach {
        Approach::North | Approach::East => -1.0,
        Approach::South | Approach::West => 1.0,
    }
}

fn spawn_along(approach: Approach) -> f32 {
    match approach {
        Approach::North | Approach::East => SPAWN_DIST,
        Approach::South | Approach::West => -SPAWN_DIST,
    }
}

fn car_before_stop(car: &Car) -> bool {
    match car.approach {
        Approach::North | Approach::East => car.along > STOP_LINE,
        Approach::South | Approach::West => car.along < -STOP_LINE,
    }
}

fn car_is_queued(car: &Car) -> bool {
    car_before_stop(car)
}

fn car_finished(car: &Car) -> bool {
    match car.approach {
        Approach::North | Approach::East => car.along < -SPAWN_DIST,
        Approach::South | Approach::West => car.along > SPAWN_DIST,
    }
}

fn gap_ahead(car: &Car, others: &[(Approach, f32)]) -> bool {
    let dir = travel_dir(car.approach);
    let min_gap = CAR_LENGTH + CAR_GAP;
    for &(approach, along) in others {
        if approach != car.approach {
            continue;
        }
        // Skip self (same along).
        if (along - car.along).abs() < 1e-4 {
            continue;
        }
        let ahead_delta = (along - car.along) * dir;
        if ahead_delta > 0.0 && ahead_delta < min_gap {
            return false;
        }
    }
    true
}

fn spawn_blocked(approach: Approach, cars: &[Car]) -> bool {
    let spawn = spawn_along(approach);
    let min_gap = CAR_LENGTH + CAR_GAP;
    cars.iter().any(|c| {
        c.approach == approach && (c.along - spawn).abs() < min_gap
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn rng() -> StdRng {
        StdRng::seed_from_u64(1)
    }

    #[test]
    fn min_green_blocks_immediate_switch() {
        let mut env = Intersection::new();
        env.spawn_prob = 0.0;
        env.phase = Phase::NsGreen;
        env.time_in_phase = 0.0;
        let mut rng = rng();
        env.step(Action::EwGreen, &mut rng);
        assert_eq!(env.phase, Phase::NsGreen);
        assert!(env.time_in_phase > 0.0);
    }

    #[test]
    fn switch_after_min_green() {
        let mut env = Intersection::new();
        env.spawn_prob = 0.0;
        env.phase = Phase::NsGreen;
        env.time_in_phase = MIN_GREEN;
        let mut rng = rng();
        env.step(Action::EwGreen, &mut rng);
        assert_eq!(env.phase, Phase::EwGreen);
        assert_eq!(env.time_in_phase, 0.0);
    }

    #[test]
    fn car_stops_on_red() {
        let mut env = Intersection::new();
        env.spawn_prob = 0.0;
        env.phase = Phase::EwGreen;
        env.time_in_phase = MIN_GREEN;
        env.spawn_car_at_stop(Approach::North);
        let start = env.cars[0].along;
        let mut rng = rng();
        for _ in 0..20 {
            env.step(Action::EwGreen, &mut rng);
        }
        assert!(
            (env.cars[0].along - start).abs() < CAR_SPEED * TAU * 2.0,
            "north car should stay near stop on EW green"
        );
        assert!(env.cars[0].along > STOP_LINE - 0.2);
    }

    #[test]
    fn car_moves_on_green() {
        let mut env = Intersection::new();
        env.spawn_prob = 0.0;
        env.phase = Phase::NsGreen;
        env.time_in_phase = MIN_GREEN;
        env.spawn_car_at_stop(Approach::North);
        let start = env.cars[0].along;
        let mut rng = rng();
        for _ in 0..30 {
            env.step(Action::NsGreen, &mut rng);
        }
        assert!(
            env.cars.is_empty() || env.cars[0].along < start - 1.0,
            "north car should proceed through junction on NS green"
        );
    }

    #[test]
    fn reward_non_positive_when_queued() {
        let mut env = Intersection::new();
        env.spawn_prob = 0.0;
        env.phase = Phase::EwGreen;
        env.time_in_phase = MIN_GREEN;
        env.spawn_car_at_stop(Approach::North);
        let mut rng = rng();
        let result = env.step(Action::EwGreen, &mut rng);
        assert!(result.reward <= 0.0);
        assert!(result.reward < 0.0);
    }

    #[test]
    fn episode_done_at_max_steps() {
        let mut env = Intersection::new();
        env.spawn_prob = 0.0;
        env.steps = (MAX_STEPS as u32).saturating_sub(1);
        let mut rng = rng();
        let result = env.step(Action::NsGreen, &mut rng);
        assert!(result.done);
        assert!(env.is_done());
    }
}
