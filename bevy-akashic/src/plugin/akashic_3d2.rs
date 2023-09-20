use bevy::app::App;
use bevy::asset::{Handle, load_internal_asset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::{render_graph, RenderApp};
use bevy::render::extract_resource::ExtractResource;
use bevy::render::render_graph::RenderGraph;
use bevy::render::render_resource::{BindGroup, BlendComponent, BlendState, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, PipelineCache, PrimitiveState, RenderPipelineDescriptor, VertexState};
use bevy::render::renderer::{RenderAdapter, RenderContext, RenderDevice, RenderInstance, RenderQueue};
use wgpu::CommandEncoderDescriptor;

use akashic_rs::console_log;
use akashic_rs::game::GAME;

use crate::plugin::akashic_3d::AkashicSurface;

pub struct Akashic3D2Plugin;

pub const UI_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13312847047162779583);

impl Plugin for Akashic3D2Plugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, UI_SHADER_HANDLE, "shader.wgsl", Shader::from_wgsl);
        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.

        let akashic_surface = app.world.non_send_resource::<AkashicSurface>();
        let akashic_surface = akashic_surface.clone();

        let render_app = app.sub_app_mut(RenderApp);
        render_app.world.insert_non_send_resource(akashic_surface);
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

        render_graph.add_node("game_of_life", GameOfLifeNode::default());
        //
        // render_app.add_systems(Render, queue_bind_group.in_set(RenderSet::Queue));


        console_log!("build");
    }

    fn finish(&self, app: &mut App) {
        console_log!("before finish");
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<GameOfLifePipeline>();
        console_log!("finish");
    }
}

#[derive(Resource, Clone, Deref, ExtractResource)]
struct GameOfLifeImage(Handle<Image>);

#[derive(Resource)]
struct GameOfLifeImageBindGroup(BindGroup);


#[derive(Resource)]
pub struct GameOfLifePipeline {
    pipeline_id: CachedRenderPipelineId,
    surface: wgpu::Surface,
}


impl FromWorld for GameOfLifePipeline {
    fn from_world(world: &mut World) -> Self {
        let akashic_surface = world.non_send_resource::<AkashicSurface>();
        let src = akashic_surface.0.clone();
        let canvas = src.canvas();

        let instance = world.resource::<RenderInstance>();

        let adapter = world.resource::<RenderAdapter>();
        let device = world.resource::<RenderDevice>();

        let surface = instance.create_surface_from_canvas(canvas).unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: GAME.width() as u32,
            height: GAME.height() as u32,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(device.wgpu_device(), &config);
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline_id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some("Render Pipeline".into()),
            layout: vec![],
            push_constant_ranges: vec![],
            vertex: VertexState {
                shader: UI_SHADER_HANDLE.typed(),
                shader_defs: vec![],
                entry_point: "vs_main".into(),
                buffers: vec![],
            },
            fragment: Some(FragmentState {
                shader: UI_SHADER_HANDLE.typed(),
                entry_point: "fs_main".into(),
                shader_defs: vec![],
                targets: vec![Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState {
                        color: BlendComponent::REPLACE,
                        alpha: BlendComponent::REPLACE,
                    }),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
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
        });


        GameOfLifePipeline {
            pipeline_id,
            surface,
        }
    }
}


#[derive(Default)]
struct GameOfLifeNode {
    // query: QueryState<&'static ViewTarget, With<ExtractedView>>,
}

// impl FromWorld for GameOfLifeNode {
//     fn from_world(world: &mut World) -> Self {
//         Self {
//             query: QueryState::new(world),
//         }
//     }
// }


impl render_graph::Node for GameOfLifeNode {
    fn update(&mut self, world: &mut World) {
        // console_log!("update");
        // self.query.update_archetypes(world);
    }

    fn run(
        &self,
        graph_context: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        console_log!("run");
        let pipeline = world.resource::<GameOfLifePipeline>();
        let device = world.resource::<RenderDevice>();
        let queue = world.resource::<RenderQueue>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let output = pipeline.surface.get_current_texture().expect("Failed current texture");
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: GAME.random().generate() as f64,
                            g: GAME.random().generate() as f64,
                            b: GAME.random().generate() as f64,
                            a: 1.,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            let Some(pipeline) = pipeline_cache.get_render_pipeline(pipeline.pipeline_id) else {
                console_log!("pipeline nothing");
                return Ok(());
            };
            console_log!("pipeline");
            render_pass.set_pipeline(pipeline);
            render_pass.draw(0..3, 0..1);
            console_log!(" pipeline dada");
        }
        queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

