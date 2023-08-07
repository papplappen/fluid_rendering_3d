use glam::Vec3;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, Buffer,
    BufferUsages, Device, ShaderStages,
};

pub struct SimulationState {
    pub positions: Vec<Vec3>,
    pub buffer: Buffer,
    pub bind_group: BindGroup,
}

impl SimulationState {
    pub fn create_simulation(positions: Vec<Vec3>, device: &Device) -> (Self, BindGroupLayout) {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Simulation Posititons"),
            contents: bytemuck::cast_slice(&Self::to_raw(&positions)),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Simulation Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Simulation Bind Group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        (
            Self {
                positions,
                buffer,
                bind_group,
            },
            bind_group_layout,
        )
    }
    fn to_raw(positions: &[Vec3]) -> Vec<[f32; 4]> {
        positions.iter().map(|p| [p.x, p.y, p.z, 0.]).collect()
    }
}
