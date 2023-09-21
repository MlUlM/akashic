use bevy::prelude::{FromWorld, Resource, World};
use bevy::render::renderer::RenderDevice;
use wgpu::{Buffer, FragmentState, include_wgsl, RenderPipeline, SurfaceConfiguration, VertexState};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::plugin::akashic_3d::SurfaceConfig;

#[derive(Resource)]
pub struct BufferPipeline {
    pub renderer_pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,
    pub num_vertices: u32,
}


impl FromWorld for BufferPipeline {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>().wgpu_device();
        let shader = device.create_shader_module(include_wgsl!("buffer.wgsl"));
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Buffer Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let renderer_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Buffer Pipeline"),
            layout: Some(&pipeline_layout),
          vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[BufferVertex::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: world.non_send_resource::<SurfaceConfig>().format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        Self {
            renderer_pipeline,
            vertex_buffer,
            num_vertices: VERTICES.len() as u32,
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct BufferVertex {
    position: [f32; 3],
    color: [f32; 3],
}


impl BufferVertex {
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBS: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<BufferVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                 wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

const VERTICES: &[BufferVertex] = &[
    BufferVertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    BufferVertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    BufferVertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];
