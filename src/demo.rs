use macroquad::prelude::*;
use ::rand::rngs::StdRng;
use ::rand::SeedableRng;

use drl_intersection::agent::Agent;
use drl_intersection::intersection::{Action, Intersection, Phase, TAU};

use crate::input;
use crate::viz;

pub async fn run() {
    let agent = Agent::try_load();
    let mut agent_mode = agent.is_some();
    if agent.is_some() {
        println!("loaded policy — agent mode (P toggles manual)");
    } else {
        println!(
            "no artifacts/policy.mpk — manual mode (train with: cargo run --release --bin train)"
        );
    }

    let mut env = Intersection::new();
    let mut rng = StdRng::seed_from_u64(42);
    env.reset_with_traffic(&mut rng);
    let mut accum = 0.0_f32;
    let mut episode_reward = 0.0_f32;
    let mut done_timer = 0.0_f32;
    let mut last_agent_action: Option<Action> = None;

    loop {
        if input::quit_pressed() {
            break;
        }

        if input::toggle_agent_pressed() && agent.is_some() {
            agent_mode = !agent_mode;
        }

        let want_reset = input::reset_pressed()
            || (agent_mode && env.is_done() && done_timer > 1.0)
            || (!agent_mode && env.is_done());

        if want_reset {
            env.reset_with_traffic(&mut rng);
            episode_reward = 0.0;
            accum = 0.0;
            done_timer = 0.0;
            last_agent_action = None;
        }

        if env.is_done() {
            done_timer += get_frame_time();
        }

        accum += get_frame_time();
        while accum >= TAU {
            accum -= TAU;
            if env.is_done() {
                break;
            }

            let action = if agent_mode {
                if let Some(manual) = input::action() {
                    last_agent_action = None;
                    manual
                } else if let Some(agent) = agent.as_ref() {
                    let a = agent.act(env.observation());
                    last_agent_action = Some(a);
                    a
                } else {
                    last_agent_action = None;
                    hold_phase(env.phase)
                }
            } else {
                last_agent_action = None;
                match input::action() {
                    Some(a) => a,
                    None => hold_phase(env.phase),
                }
            };

            let result = env.step(action, &mut rng);
            episode_reward += result.reward;
        }

        viz::draw(
            &env,
            episode_reward,
            agent_mode,
            agent.is_some(),
            last_agent_action,
        );
        next_frame().await;
    }
}

fn hold_phase(phase: Phase) -> Action {
    match phase {
        Phase::NsGreen => Action::NsGreen,
        Phase::EwGreen => Action::EwGreen,
    }
}
