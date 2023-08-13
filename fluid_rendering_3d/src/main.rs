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
use glam::{vec3a, Vec3A};
use pollster::FutureExt;
use render::RenderState;
use simulation::SimulationState;
use sph::{fluid::Fluid, sph::Particle};
use state::State;
use winit::event_loop::EventLoop;

fn main() {
    let config = Config::default();

    let event_loop = EventLoop::new();

    let env = Environment::new(&event_loop).block_on();

    let (camera, camera_bind_group_layout) = Camera::create_camera(&env.device, &env.config);

    // let particles = vec![
    //     Particle::new(vec3a(50., 50., 50.), 1., true),
    //     Particle::new(vec3a(75., 75., 75.), 1., true),
    //     Particle::new(vec3a(80., 80., 80.), 1., true),
    //     Particle::new(vec3a(80., 80., 80.), 1., true),
    //     Particle::new(vec3a(80., 80., 80.), 1., true),
    //     Particle::new(vec3a(80., 80., 80.), 1., true),
    //     Particle::new(vec3a(80., 80., 80.), 1., true),
    // ];
    let particles = cube_in_box(2);
    // particles.push(Particle::new(vec3a(0., 0., 0.), 1., true));
    let fluid = Fluid::new(
        particles,
        config.sph.h,
        config.rest_density,
        config.kinematic_viscosity,
        config.stiffness,
        config.gravity.into(),
    );
    let (simulation_state, simulation_bind_group_layout) =
        SimulationState::create_simulation(fluid, &env.device);

    let render_state = RenderState::new(
        &env,
        camera_bind_group_layout,
        simulation_bind_group_layout,
        &config,
    );

    let state = State {
        simulation_state,
        env,
        render_state,
        camera,
        config,
        delta_time: DEFAULT_DELTA_TIME,
        paused: true,
    };

    state.run(event_loop);
}
fn cube_in_box(extend: i32) -> Vec<Particle> {
    let mut particles = vec![];
    for x in -extend..=extend {
        for y in -extend..=extend {
            for z in -extend..=extend {
                if z < 0 || (x.abs() < extend && y.abs() < extend) {
                    let movable = x.abs() < extend && y.abs() < extend && z > -extend;
                    particles.push(Particle::new(
                        vec3a(x as f32, z as f32, y as f32),
                        1.,
                        movable,
                    ));
                }
            }
        }
    }
    particles
}
