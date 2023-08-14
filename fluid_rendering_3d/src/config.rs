use glam::Vec3;
use sph::{fluid::Fluid, sph::SPH};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, Buffer,
    BufferUsages, Device, ShaderStages,
};

pub const DEFAULT_DELTA_TIME: f32 = 0.01;

pub struct Config {
    pub sph: SPH,
    pub rest_density: f32,
    pub kinematic_viscosity: f32,
    pub stiffness: f32,
    pub gravity: Vec3,
}

impl Default for Config {
    fn default() -> Self {
        let Fluid {
            particles: _,
            sph,
            rest_density,
            kinematic_viscosity,
            stiffness,
            gravity,
        } = Fluid::default();
        Self {
            sph,
            rest_density,
            kinematic_viscosity,
            stiffness,
            gravity: gravity.into(),
        }
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct DrawShaderConfig {
    h: f32,
    alpha: f32,
}
impl From<&Config> for DrawShaderConfig {
    fn from(cfg: &Config) -> Self {
        let SPH { h, alpha } = cfg.sph;
        Self { h, alpha }
    }
}
impl DrawShaderConfig {
    pub fn as_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Config Buffer"),
            contents: bytemuck::bytes_of(self),
            usage: BufferUsages::UNIFORM,
        })
    }
    pub fn create_bind_group(&self, device: &Device) -> (BindGroupLayout, BindGroup) {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Render Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Render Bind Group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: self.as_buffer(device).as_entire_binding(),
            }],
        });
        (bind_group_layout, bind_group)
    }
}
