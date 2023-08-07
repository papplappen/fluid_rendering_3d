use wgpu::{
    include_wgsl, BindGroup, BindGroupLayout, Buffer, Color, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, Device, FragmentState, MultisampleState, PipelineLayoutDescriptor,
    PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    SurfaceConfiguration, TextureViewDescriptor, VertexState,
};

use crate::{
    config::{Config, DrawShaderConfig},
    env::Environment,
    simulation::SimulationState,
    vertex::{Vertex, SQUARE},
};

const BACKGROUND_COLOR: Color = Color {
    r: 0.1,
    g: 0.2,
    b: 0.3,
    a: 1.0,
};
pub struct RenderState {
    pub vertex_buffer: Buffer,
    pub render_pipeline: RenderPipeline,
    pub config_bind_group: BindGroup,
}
impl RenderState {
    pub fn new(
        env: &Environment,
        camera_bind_group_layout: BindGroupLayout,
        config: &Config,
    ) -> Self {
        // * CREATE VERTEX & INSTANCE BUFFERS
        let vertex_buffer = Vertex::create_vertex_buffer(&env.device);

        let (config_bind_group_layout, config_bind_group) =
            DrawShaderConfig::from(config).create_bind_group(&env.device);

        // * CREATE RENDER PIPELINE
        let render_pipeline = Self::create_render_pipeline(
            &env.device,
            &env.config,
            &[&camera_bind_group_layout, &config_bind_group_layout],
        );
        Self {
            vertex_buffer,
            render_pipeline,
            config_bind_group,
        }
    }

    pub fn render_call(
        &self,
        simulation_state: &SimulationState,
        env: &Environment,
        camera_bind_group: &BindGroup,
    ) {
        let output = env.surface.get_current_texture().unwrap();
        let mut encoder = env
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Encoder"),
            });
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BACKGROUND_COLOR),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(0, camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.config_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            render_pass.draw(0..SQUARE.len() as u32, 0..1)
        }
        env.queue.submit(Some(encoder.finish()));
        output.present();
    }

    fn create_render_pipeline(
        device: &Device,
        config: &SurfaceConfiguration,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> RenderPipeline {
        // * LOAD SHADER
        let draw_shader = device.create_shader_module(include_wgsl!("draw.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        });
        // * CREATE RENDER PIPELINE
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &draw_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
                module: &draw_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview: None,
        })
    }
}
