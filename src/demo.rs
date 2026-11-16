use macroquad::prelude::*;
use ::rand::rngs::StdRng;
use ::rand::SeedableRng;

use drl_intersection::intersection::{Action, Intersection, Phase, TAU};

use crate::input;
use crate::viz;

pub async fn run() {
    let mut env = Intersection::new();
    let mut rng = StdRng::seed_from_u64(42);
    let mut accum = 0.0_f32;
    let mut episode_reward = 0.0_f32;

    loop {
        if input::quit_pressed() {
            break;
        }

        if input::reset_pressed() || env.is_done() {
            env.reset();
            episode_reward = 0.0;
            accum = 0.0;
        }

        accum += get_frame_time();
        while accum >= TAU {
            accum -= TAU;
            if env.is_done() {
                break;
            }

            let action = match input::action() {
                Some(a) => a,
                None => match env.phase {
                    Phase::NsGreen => Action::NsGreen,
                    Phase::EwGreen => Action::EwGreen,
                },
            };
            let result = env.step(action, &mut rng);
            episode_reward += result.reward;
        }

        viz::draw(&env, episode_reward);
        next_frame().await;
    }
}
