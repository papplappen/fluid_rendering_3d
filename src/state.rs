use std::time::Instant;

use winit::event_loop::EventLoop;

use crate::{
    camera::Camera, config::Config, env::Environment, input, render::RenderState,
    simulation::SimulationState,
};
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};
pub struct State {
    pub env: Environment,
    pub render_state: RenderState,
    pub simulation_state: SimulationState,
    pub camera: Camera,
    pub config: Config,
    pub delta_time: f32,
    pub paused: bool,
}

impl State {
    pub fn run(mut self, event_loop: EventLoop<()>) {
        // * SETUP
        let mut start = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.env.window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,

                    event => {
                        input::input(&mut self, event);
                    }
                },
                Event::MainEventsCleared => {
                    // * UPDATE SIMULATION
                    if !self.paused {
                        self.update_simulation()
                    }
                    // * UPDATE CAMERA
                    if self.env.cursor_grab {
                        self.camera.update(self.delta_time, &self.env.queue);
                    }
                    // * RENDER
                    self.render_state.render_call(
                        &self.simulation_state,
                        &self.env,
                        &self.camera.bind_group,
                    );
                }
                Event::RedrawEventsCleared => {
                    // * UPDATE DELTA TIME
                    self.delta_time = start.elapsed().as_secs_f32();
                    println!("{}", 1. / self.delta_time);
                    start = Instant::now();
                }

                Event::DeviceEvent {
                    device_id: _,
                    event,
                } => {
                    self.camera.controller.handle_mouse_movement(&event);
                }
                _ => {}
            }
        })
    }

    pub fn update_simulation(&mut self) {}
}
