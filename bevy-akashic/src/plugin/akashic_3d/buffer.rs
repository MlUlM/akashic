use bevy::prelude::{FromWorld, Resource, World};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use wgpu::{BindGroup, BindGroupLayout, Buffer, include_wgsl, RenderPipeline, TextureFormat};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::plugin::akashic_3d::texture;

#[derive(Resource)]
pub struct BufferPipeline {
    pub bind_group: BindGroup,
    pub renderer_pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub num_vertices: u32,
}


impl FromWorld for BufferPipeline {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>().wgpu_device();
        let shader = device.create_shader_module(include_wgsl!("buffer.wgsl"));
        let queue = world.resource::<RenderQueue>();

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let diffuse_bytes = include_bytes!("happy-tree.png");
        let diffuse_texture = texture::Texture::from_bytes(device, queue, diffuse_bytes, "happy-tree.png").unwrap();
        let bind_group_layout = create_bind_group_layout(device);
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Buffer Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
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
                    format: *world.non_send_resource::<TextureFormat>(),
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
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let bind_group = create_bind_group(&diffuse_texture, device, &bind_group_layout);
        Self {
            renderer_pipeline,
            bind_group,
            vertex_buffer,
            index_buffer,
            num_vertices: INDICES.len() as u32,
        }
    }
}


fn create_bind_group_layout(device: &wgpu::Device) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("texture_bind_group_layout"),
    })
}


fn create_bind_group(
    diffuse_texture: &texture::Texture,
    device: &wgpu::Device,
    texture_bind_group_layout: &BindGroupLayout,
) -> BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
            },
        ],
        label: Some("diffuse_bind_group"),
    })
}


const INDICES: &[u16] = &[0, 2, 3, 0, 1, 2];


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BufferVertex {
    pub position: [f32; 3],
    tex_coords: [f32; 2],
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

pub const VERTICES: &[BufferVertex] = &[
    BufferVertex {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.4131759, 0.00759614],
    }, // A
    BufferVertex {
        position: [0.5, -0.5, 0.0],
        tex_coords: [0.0048659444, 0.43041354],
    }, // B
    BufferVertex {
        position: [0.5, 0.5, 0.0],
        tex_coords: [0.28081453, 0.949397],
    }, // C
    BufferVertex {
        position: [-0.5, 0.5, 0.0],
        tex_coords: [0.85967, 0.84732914],
    }, // D
];


pub fn vertex(z: f32) -> Vec<BufferVertex> {
    vec![
        BufferVertex {
            position: [-z, -0.5,  0.0],
            tex_coords: [0.4131759, 0.00759614],
        }, // A
        BufferVertex {
            position: [z, -0.5, 0.0],
            tex_coords: [0.0048659444, 0.43041354],
        }, // B
        BufferVertex {
            position: [z, 0.5, 0.0],
            tex_coords: [0.28081453, 0.949397],
        }, // C
        BufferVertex {
            position: [-z, 0.5, 0.0],
            tex_coords: [0.85967, 0.84732914],
        },
    ]
}