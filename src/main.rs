pub(crate) mod camera;
pub(crate) mod config;
pub(crate) mod env;
pub(crate) mod input;
pub(crate) mod render;
pub(crate) mod simulation;
pub(crate) mod state;
pub(crate) mod vertex;

use camera::Camera;
use config::{Config, DEFAULT_DELTA_TIME};
use env::Environment;
use pollster::FutureExt;
use render::RenderState;
use simulation::SimulationState;
use state::State;
use winit::event_loop::EventLoop;

fn main() {
    let config = Config::default();

    let event_loop = EventLoop::new();

    let env = Environment::new(&event_loop).block_on();

    let (camera, camera_bind_group_layout) = Camera::create_camera(&env.device, &env.config);

    let render_state = RenderState::new(&env, camera_bind_group_layout, &config);

    let state = State {
        env,
        render_state,
        camera,
        config,
        delta_time: DEFAULT_DELTA_TIME,
        paused: true,
        simulation_state: SimulationState::new(vec![]),
    };

    state.run(event_loop);
}
