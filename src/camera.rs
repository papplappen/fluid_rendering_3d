use bytemuck::{Pod, Zeroable};
use glam::{vec3, Mat3, Mat4, Vec3};
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, Buffer,
    BufferUsages, Device, Queue, ShaderStages, SurfaceConfiguration,
};
use winit::event::{DeviceEvent, ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

const SPEED: f32 = 100.;
const SHIFT_SPEED: f32 = 0.1 * SPEED;
const SENS: f32 = 0.1;

const FOV_Y: f32 = 45.; // ! DEGREES

// * +X = Right; +Y = Up; +Z = Back
pub struct Camera {
    pub entity: CameraEntity,
    pub view_matrix: Mat4,
    pub controller: CameraController,
    pub bind_group: BindGroup,
    pub buffer: Buffer,
}

impl Camera {
    pub fn create_camera(
        device: &Device,
        config: &SurfaceConfiguration,
    ) -> (Self, BindGroupLayout) {
        let pos = vec3(50., 50., 200.);
        let dir = vec3(0., 0., -1.);
        let screen_height = config.height as f32;
        let screen_dist = (0.5 * screen_height) / (FOV_Y * 0.5).to_radians().tan();
        let entity = CameraEntity {
            pos,
            dir: dir.normalize(),
            up: Vec3::Y,
            screen_dist,
            screen_width: config.width as f32,
            screen_height,
        };
        let view_matrix = Mat4::look_to_rh(entity.pos, entity.dir, entity.up);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::bytes_of(&CameraUniform {
                entity,
                view_matrix,
                inverse_view_matrix: view_matrix.inverse(),
            }),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Groups"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        let controller = CameraController::new(SPEED, SENS);

        (
            Self {
                entity,
                controller,
                bind_group,
                buffer,
                view_matrix,
            },
            bind_group_layout,
        )
    }

    pub fn update(&mut self, delta: f32, queue: &Queue) {
        self.controller
            .update_camera_entity(&mut self.entity, delta);
        self.view_matrix = Mat4::look_to_rh(self.entity.pos, self.entity.dir, self.entity.up);

        queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::bytes_of(&CameraUniform {
                entity: self.entity,
                view_matrix: self.view_matrix,
                inverse_view_matrix: self.view_matrix.inverse(),
            }),
        );
    }
}
#[repr(C)]
#[derive(Debug, Pod, Clone, Copy, Zeroable)]
pub struct CameraEntity {
    pub pos: Vec3,
    pub screen_width: f32,
    pub dir: Vec3,
    pub screen_height: f32,
    pub up: Vec3,
    pub screen_dist: f32,
}

pub struct CameraController {
    speed: f32,
    sens: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    delta: (f32, f32),
}

impl CameraController {
    pub fn new(speed: f32, sens: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            delta: (0., 0.),
            sens,
        }
    }

    pub fn handle_key_input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }

                    _ => false,
                }
            }
            WindowEvent::ModifiersChanged(modifier_state) => {
                if modifier_state.shift() {
                    self.speed = SHIFT_SPEED;
                } else {
                    self.speed = SPEED
                }
                true
            }
            _ => false,
        }
    }
    pub fn handle_mouse_movement(&mut self, event: &DeviceEvent) -> bool {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.delta = (delta.0 as f32, delta.1 as f32);
            true
        } else {
            false
        }
    }
    pub fn update_camera_entity(&mut self, camera_entity: &mut CameraEntity, dt: f32) {
        camera_entity.dir = camera_entity.dir.normalize();
        let yaw = Mat3::from_rotation_y(-self.delta.0.to_radians() * self.sens);
        camera_entity.dir = yaw * camera_entity.dir;

        let pitch = Mat3::from_axis_angle(
            camera_entity.dir.cross(camera_entity.up).normalize(),
            -self.delta.1.to_radians() * self.sens,
        );
        self.delta = (0., 0.);
        let new_dir = pitch * camera_entity.dir;
        if camera_entity
            .dir
            .cross(camera_entity.up)
            .dot(new_dir.cross(camera_entity.up))
            >= 0.
        {
            camera_entity.dir = new_dir;
        } else {
            camera_entity.dir.y = camera_entity.dir.y.signum();
        }
        camera_entity.dir = camera_entity.dir.normalize();
        let forward = camera_entity.dir * self.speed * dt;
        if self.is_forward_pressed {
            camera_entity.pos += forward;
        }
        if self.is_backward_pressed {
            camera_entity.pos -= forward;
        }

        let right = camera_entity.dir.cross(camera_entity.up).normalize() * self.speed * dt;

        if self.is_right_pressed {
            camera_entity.pos += right;
        }
        if self.is_left_pressed {
            camera_entity.pos -= right;
        }
    }
}
#[repr(C)]
#[derive(Pod, Clone, Copy, Zeroable)]
struct CameraUniform {
    entity: CameraEntity,
    view_matrix: Mat4,
    inverse_view_matrix: Mat4,
}
