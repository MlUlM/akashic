use std::iter;
use std::sync::{Arc, Mutex};

use bevy::app::{App, Plugin, Startup, Update};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, Component, Deref, DerefMut, NonSend, Query, Res, Resource, Transform, With};
use bevy::render::render_resource::TextureUsages;
use bevy::render::renderer::{RenderAdapter, RenderDevice, RenderQueue};
use bevy::tasks::IoTaskPool;
use bevy::time::Time;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlCanvasElement;
use wgpu::{Adapter, Device, Instance, PowerPreference, Queue, Surface, SurfaceConfiguration, TextureFormat};

use akashic_rs::game::GAME;
use akashic_rs::prelude::SpriteBuilder;
use akashic_rs::resource_factory::ResourceFactory;

use crate::command::IntoBundle;
use crate::plugin::akashic_3d::buffer::{BufferPipeline, vertex};

mod buffer;

#[derive(Default, Deref, DerefMut)]
struct FutureDevice(Arc<Mutex<Option<(
    Device,
    Adapter,
    Queue,
    Instance,
    AkashicSurface,
    TextureFormat
)>>>);


#[derive(Deref)]
struct AkashicResourceFactory(ResourceFactory);

#[derive(Deref)]
pub struct SurfaceConfig(pub SurfaceConfiguration);

#[derive(Deref, Resource)]
pub struct WgpuInstance(Instance);

impl WgpuInstance {
    pub fn create_surface(
        &self,
        size: Vec2,
        akashic_surface: &AkashicSurface,
        adapter: &RenderAdapter,
        device: &RenderDevice,
    ) -> EntitySurface {
        let surface = self.create_surface_from_canvas(akashic_surface.canvas()).unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.x as u32,
            height: size.y as u32,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(device.wgpu_device(), &config);

        EntitySurface(surface)
    }
}

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

            ))
            .add_systems(Update, move_system);

        IoTaskPool::get()
            .spawn_local(async move {
                let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
                let akashic_surface = canvas_only(GAME.width() as u32, GAME.height() as u32);
                let surface = instance.create_surface_from_canvas(akashic_surface.canvas()).unwrap();
                let request_adapter_options = wgpu::RequestAdapterOptions {
                    power_preference: PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
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

                let mut future_device = future_device.lock().unwrap();
                let surface_caps = surface.get_capabilities(&adapter);
                let texture_format = surface_caps.formats.iter()
                    .copied()
                    .find(|f| f.is_srgb())
                    .unwrap_or(surface_caps.formats[0]);
                let config = wgpu::SurfaceConfiguration {
                    usage: TextureUsages::RENDER_ATTACHMENT,
                    format: texture_format,
                    width: GAME.width() as u32,
                    height: GAME.height() as u32,
                    present_mode: surface_caps.present_modes[0],
                    alpha_mode: surface_caps.alpha_modes[0],
                    view_formats: vec![],
                };
                surface.configure(&device, &config);

                *future_device = Some((device, adapter, queue, instance, AkashicSurface(akashic_surface), texture_format));
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
        let (device, adapter, queue, instance, akashic_surface, texture_format) = futures.lock().unwrap().take().unwrap();
        let device = RenderDevice::from(device);
        let adapter = RenderAdapter(Arc::new(adapter));
        let queue = RenderQueue(Arc::new(queue));
        //
        // app.world.spawn(SpriteBuilder::new(akashic_surface.0.clone())
        //     .width(GAME.width())
        //     .height(GAME.height())
        //     .build()
        //     .into_bundle()
        // );

        app.insert_non_send_resource(texture_format);
        app.insert_resource(device);
        app.insert_resource(adapter);
        app.insert_resource(queue);
        app.insert_resource(WgpuInstance(instance));
        app.insert_non_send_resource(akashic_surface);
        app.init_resource::<BufferPipeline>();
    }
}


#[derive(Resource, Deref)]
pub struct AkashicPipeline(Arc<wgpu::RenderPipeline>);

fn setup(
    mut commands: Commands,
    akashic_surface: NonSend<AkashicSurface>,
    instance: Res<WgpuInstance>,
    adapter: Res<RenderAdapter>,
    device: Res<RenderDevice>,
) {
    let size = 300.;
    let src = akashic_surface.clone();

    commands.spawn(SpriteBuilder::new(src.0)
        .width(size)
        .height(size)
        .x(GAME.random().generate() * 100.)
        .y(GAME.random().generate() * 100.)
        .build().into_bundle()
    )
        .insert(Cube)
        .insert(instance.create_surface(Vec2::new(size, size), &akashic_surface, &adapter, &device));
}

#[derive(Clone, Deref)]
pub struct AkashicSurface(pub akashic_rs::asset::surface::Surface);

#[derive(Component, Deref)]
pub struct EntitySurface(pub Surface);

#[derive(Component)]
struct Cube;

fn move_system(
    mut sprite: Query<(&mut Transform, &EntitySurface), With<Cube>>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    pipe_line: Res<BufferPipeline>,
) {
    for (mut t, surface) in sprite.iter_mut() {
        t.rotate_z(1. / 20.);
        t.translation += Vec3::X;

        let output = surface.get_current_texture().expect("Failed current texture");
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
                            r: 0.1,
                            g: 0.2,
                            b: 1.,
                            a: 0.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&pipe_line.renderer_pipeline);
            render_pass.set_vertex_buffer(0, pipe_line.vertex_buffer.slice(..));
            render_pass.draw(0..pipe_line.num_vertices, 0..1);
        }

        queue.write_buffer(&pipe_line.vertex_buffer, 0, bytemuck::cast_slice(&vertex(t.rotation.z)));
        queue.submit(iter::once(encoder.finish()));
        output.present();
    }
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