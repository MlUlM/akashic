use std::iter;
use std::sync::{Arc, Mutex};

use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{Commands, Component, Deref, DerefMut, NonSend, Query, Res, Resource, Transform, Vec3, With};
use bevy::render::renderer::{RenderAdapter, RenderDevice, RenderQueue};
use bevy::tasks::IoTaskPool;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlCanvasElement;
use wgpu::{Adapter, Device, include_wgsl, Instance, PowerPreference, Queue, Surface, SurfaceConfiguration, TextureUsages};

use akashic_rs::game::GAME;
use akashic_rs::prelude::SpriteBuilder;
use akashic_rs::resource_factory::ResourceFactory;

use crate::command::IntoBundle;
use crate::resource::random::AkashicRandomGenerator;

#[derive(Default, Deref, DerefMut)]
struct FutureDevice(Arc<Mutex<Option<(
    Device,
    Adapter,
    Queue,
    Instance,
    ScreenSurface,
    AkashicPipeline
)>>>);


#[derive(Deref)]
struct AkashicResourceFactory(ResourceFactory);


pub struct Akashic3DPlugin;


impl Plugin for Akashic3DPlugin {
    fn build(&self, app: &mut App) {
        let future_device = Arc::new(Mutex::new(None));
        let resource_factory = GAME.resource_factory();
        app
            .insert_non_send_resource(AkashicResourceFactory(resource_factory.clone()))
            .insert_non_send_resource(FutureDevice(Arc::clone(&future_device)))
            .add_systems(Startup, (
                setup,
                setup,
                setup,
                setup,
                setup,
            ))
            .add_systems(Update, move_system)
        ;
        IoTaskPool::get()
            .spawn_local(async move {
                let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
                let akashic_surface = canvas_only(GAME.width() as u32, GAME.height() as u32);
                let surfaces = create_screen_surface(&instance, akashic_surface);

                let request_adapter_options = wgpu::RequestAdapterOptions {
                    power_preference: PowerPreference::HighPerformance,
                    compatible_surface: Some(&surfaces.wgpu),
                    ..Default::default()
                };

                let adapter = instance.request_adapter(&request_adapter_options)
                    .await
                    .unwrap();

                let (device, queue) = adapter
                    .request_device(
                        &wgpu::DeviceDescriptor {
                            features: wgpu::Features::empty(),
                            limits: wgpu::Limits::downlevel_webgl2_defaults(),
                            label: None,
                        },
                        None,
                    )
                    .await
                    .unwrap();
                let surface_caps = surfaces.wgpu.get_capabilities(&adapter);
                let surface_format = surface_caps.formats.iter()
                    .copied()
                    .find(|f| f.is_srgb())
                    .unwrap_or(surface_caps.formats[0]);
                let config = wgpu::SurfaceConfiguration {
                    usage: TextureUsages::RENDER_ATTACHMENT,
                    format: surface_format,
                    width: GAME.width() as u32,
                    height: GAME.height() as u32,
                    present_mode: surface_caps.present_modes[0],
                    alpha_mode: surface_caps.alpha_modes[0],
                    view_formats: vec![],
                };
                surfaces.wgpu.configure(&device, &config);
                let pipeline = AkashicPipeline(Arc::new(new_pipeline(&device, &config)));
                let mut future_device = future_device.lock().unwrap();
                *future_device = Some((device, adapter, queue, instance, surfaces, pipeline));
            })
            .detach();
    }


    fn ready(&self, app: &App) -> bool {
        let ready = app
            .world
            .get_non_send_resource::<FutureDevice>()
            .map(|future| future.0.lock().unwrap().is_some())
            .unwrap_or(true);

        ready
    }

    fn finish(&self, app: &mut App) {
        let Some(futures) = app.world.remove_non_send_resource::<FutureDevice>() else { return; };
        let (device, adapter, queue, instance, akashic_surface, pipeline) = futures.lock().unwrap().take().unwrap();
        let device = RenderDevice::from(device);
        let adapter = RenderAdapter(Arc::new(adapter));
        let queue = RenderQueue(Arc::new(queue));

        app.insert_resource(device);
        app.insert_resource(adapter);
        app.insert_resource(queue);
        app.insert_non_send_resource(instance);
        app.insert_non_send_resource(akashic_surface);
        app.insert_resource(pipeline);
    }
}


#[derive(Resource, Deref)]
pub struct AkashicPipeline(Arc<wgpu::RenderPipeline>);

fn setup(
    mut commands: Commands,
    akashic_surface: NonSend<ScreenSurface>,
) {
    let size = 100.;
    let src = akashic_surface.akashic.clone();

    commands.spawn(SpriteBuilder::new(src)
        .width(size)
        .height(size)
        .x(GAME.random().generate() * 100.)
        .y(GAME.random().generate() * 100.)
        .build().into_bundle()
    )
        .insert(Cube);
}


fn create_screen_surface(
    instance: &Instance,
    akashic_surface: akashic_rs::asset::surface::Surface,
) -> ScreenSurface {
    let src = akashic_surface.clone();
    let canvas = src.canvas();
    let surface: Surface = instance.create_surface_from_canvas(canvas).unwrap();

    ScreenSurface {
        akashic: akashic_surface,
        wgpu: surface,
    }
}


fn new_pipeline(device: &Device, config: &SurfaceConfiguration) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
    let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
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

    pipeline
}

pub struct ScreenSurface {
    pub akashic: akashic_rs::asset::surface::Surface,
    pub wgpu: Surface,
}

#[derive(Component)]
struct Cube;

fn move_system(
    mut sprite: Query<&mut Transform, With<Cube>>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    surfaces: NonSend<ScreenSurface>,
    pipe_line: Res<AkashicPipeline>,
    random: Res<AkashicRandomGenerator>,
) {
    for mut t in sprite.iter_mut() {
        t.translation += Vec3::X;
    }

    let output = surfaces.wgpu.get_current_texture().expect("Failed current texture");
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1. as f64,
                        g: random.generate() as f64,
                        b: random.generate() as f64,
                        a: 1. as f64,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&pipe_line);
        render_pass.draw(0..3, 0..1);
    }

    queue.submit(iter::once(encoder.finish()));
    output.present();
}


#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = g, js_name = isNode)]
    pub fn is_node() -> bool;

    #[wasm_bindgen(js_namespace = g)]
    pub fn canvas(width: u32, height: u32) -> HtmlCanvasElement;

    #[wasm_bindgen(js_namespace = g)]
    pub fn canvas_only(width: u32, height: u32) -> akashic_rs::asset::surface::Surface;
}