use bevy::app::App;
use bevy::asset::{Handle, load_internal_asset};
use bevy::core_pipeline::core_3d::Transparent3d;
use bevy::ecs::query::ROQueryItem;
use bevy::ecs::system::lifetimeless::{Read, SRes};
use bevy::ecs::system::SystemParamItem;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::extract_resource::ExtractResource;
use bevy::render::render_graph::RenderGraph;
use bevy::render::render_phase::{AddRenderCommand, DrawFunctionId, DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult, RenderPhase, TrackedRenderPass};
use bevy::render::render_resource::{BindGroup, BlendComponent, BlendState, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, PipelineCache, PrimitiveState, RawRenderPipelineDescriptor, RenderPipeline, RenderPipelineDescriptor, VertexState};
use bevy::render::{Extract, Render, RenderApp, RenderSet};
use bevy::render::renderer::{RenderAdapter, RenderDevice, RenderInstance};
use bevy::render::view::{ExtractedView, VisibleEntities};
use wgpu::include_wgsl;

use akashic_rs::console_log;
use akashic_rs::game::GAME;

use crate::plugin::akashic_3d::{AkashicSurface, EntitySurface};

pub struct Akashic3D2Plugin;

pub const UI_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13312847047162779583);

impl Plugin for Akashic3D2Plugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, UI_SHADER_HANDLE, "shader.wgsl", Shader::from_wgsl);
        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.

        let akashic_surface = app.world.non_send_resource::<AkashicSurface>().clone();


        let render_app = app.sub_app_mut(RenderApp)
              .add_systems(ExtractSchedule, extract_core_2d_camera_phases)
            .init_resource::<DrawFunctions<PhaseAkashic>>()
            .add_render_command::<PhaseAkashic, AkashicRenderCommand>()
            .add_systems(Render, queue_colored_mesh2d.in_set(RenderSet::Queue));

        render_app.world.insert_non_send_resource(akashic_surface);
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
        let src = akashic_surface.clone();
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
            view_formats: vec![surface_format],
        };
        surface.configure(device.wgpu_device(), &config);
        let pipeline_cache = world.resource::<PipelineCache>();
        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
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


pub struct PhaseAkashic {
    pub entity: Entity,
    pub pipeline: CachedRenderPipelineId,
    pub draw_function: DrawFunctionId,
}

impl PhaseItem for PhaseAkashic {
    type SortKey = usize;

    #[inline]
    fn entity(&self) -> Entity {
        self.entity
    }

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        self.pipeline.id()
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }

    #[inline]
    fn sort(items: &mut [Self]) {
        items.sort_by_key(|item| item.sort_key());
    }
}

#[derive(Default)]
struct AkashicRenderCommand {
    // query: QueryState<&'static ViewTarget, With<ExtractedView>>,
}


impl<P: PhaseItem> RenderCommand<P> for AkashicRenderCommand {
    type Param = (SRes<GameOfLifePipeline>, SRes<PipelineCache>);
    type ViewWorldQuery = ();
    type ItemWorldQuery = Read<EntitySurface>;

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: (),
        _: ROQueryItem<'w, Self::ItemWorldQuery>,
        (pipeline, cache): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        console_log!("RENDER");

        pass.set_render_pipeline(cache.into_inner().get_render_pipeline(pipeline.into_inner().pipeline_id).unwrap());
        pass.draw(0..3, 0..1);

        RenderCommandResult::Success
    }
}

#[allow(clippy::too_many_arguments)]
pub fn queue_colored_mesh2d(
    transparent_draw_functions: Res<DrawFunctions<PhaseAkashic>>,
    colored_mesh2d_pipeline: Res<GameOfLifePipeline>,
    mut views: Query<(
        Entity,
        &mut RenderPhase<PhaseAkashic>,
    )>,
) {
  console_log!("queue_colored_mesh2d");
    // Iterate each view (a camera is a view)
    for (visible_entities, mut transparent_phase) in &mut views {

        let draw_colored_mesh2d = transparent_draw_functions.read().id::<AkashicRenderCommand>();

        // Queue all entities visible to that view
      transparent_phase.add(PhaseAkashic {
                    entity: visible_entities,
                    draw_function: draw_colored_mesh2d,
                    pipeline: colored_mesh2d_pipeline.pipeline_id,
                });
    }
}

pub fn extract_core_2d_camera_phases(
    mut commands: Commands,
    cameras_2d: Extract<Query<(Entity), With<Camera2d>>>,
) {
    for (entity) in &cameras_2d {
        commands
                .get_or_spawn(entity)
                .insert(RenderPhase::<PhaseAkashic>::default());
    }
}
