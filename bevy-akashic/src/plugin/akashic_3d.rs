use std::iter;
use std::sync::{Arc, Mutex};

use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{Commands, Component, Deref, DerefMut, IntoSystemConfigs, NonSend, Query, Res, Resource, Transform, Vec3, With};
use bevy::render::renderer::{RenderAdapter, RenderDevice, RenderQueue};
use bevy::sprite::Sprite;
use bevy::tasks::IoTaskPool;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlCanvasElement;
use wgpu::{Adapter, Device, Instance, PowerPreference, Queue, Surface};
use akashic_rs::console_log;


use akashic_rs::game::GAME;
use akashic_rs::prelude::SpriteBuilder;
use akashic_rs::resource_factory::ResourceFactory;

use crate::command::IntoBundle;
use crate::plugin::system_set::AkashicSystemSet;

#[derive(Default, Deref, DerefMut)]
struct FutureDevice(Arc<Mutex<Option<(
    Device,
    Adapter,
    Queue,
    Instance,
    AkashicSurface
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
            .add_systems(Startup, update.in_set(AkashicSystemSet::Despawn))
            .add_systems(Update, move_system)
        ;
        IoTaskPool::get()
            .spawn_local(async move {
                let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

                let akshic_surface = canvas_only(100, 100);
                let surface = instance
                    .create_surface_from_canvas(akshic_surface.canvas())
                    .unwrap();

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
                let surface_caps = surface.get_capabilities(&adapter);
                // Shader code in this tutorial assumes an sRGB surface texture. Using a different
                // one will result all the colors coming out darker. If you want to support non
                // sRGB surfaces, you'll need to account for that when drawing to the frame.
                let surface_format = surface_caps.formats.iter()
                    .copied()
                    .find(|f| f.is_srgb())
                    .unwrap_or(surface_caps.formats[0]);
                let config = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: surface_format,
                    width: 100 as u32,
                    height: 100 as u32,
                    present_mode: surface_caps.present_modes[0],
                    alpha_mode: surface_caps.alpha_modes[0],
                    view_formats: vec![],
                };
                surface.configure(&device, &config);

                let mut future_device = future_device.lock().unwrap();
                *future_device = Some((device, adapter, queue, instance, AkashicSurface(akshic_surface)));
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
        let (device, adapter, queue, instance, akashic_surface) = futures.lock().unwrap().take().unwrap();

        app.insert_non_send_resource(RenderDevice::from(device));
        app.insert_non_send_resource(RenderAdapter(Arc::new(adapter)));
        app.insert_non_send_resource(RenderQueue(Arc::new(queue)));
        app.insert_non_send_resource(instance);
        app.insert_non_send_resource(akashic_surface);
    }
}

#[derive(Component)]
struct DADAD;

fn update(
    mut commands: Commands,
    instance: NonSend<Instance>,
    device: NonSend<RenderDevice>,
    queue: NonSend<RenderQueue>,
    adapter: NonSend<RenderAdapter>,
    factory: NonSend<AkashicResourceFactory>,
    akashic_surface: NonSend<AkashicSurface>
) {
    let size = 300.;
    let src = akashic_surface.0.clone();
    let canvas = src.canvas();

    let surface: Surface = instance.create_surface_from_canvas(canvas).unwrap();
    let surface_caps = surface.get_capabilities(&adapter);
    // Shader code in this tutorial assumes an sRGB surface texture. Using a different
    // one will result all the colors coming out darker. If you want to support non
    // sRGB surfaces, you'll need to account for that when drawing to the frame.
    let surface_format = surface_caps.formats.iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: 1000,
        height: 1000,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(device.wgpu_device(), &config);
    let surface = Arc::new(surface);
    commands.spawn(SpriteBuilder::new(src)
        .width(size)
        .height(size)
        .build().into_bundle())
        .insert(DADAD);
    commands.insert_resource(SURFACE(surface));
}

#[derive(Resource, Deref)]
struct SURFACE(Arc<Surface>);


#[derive(Deref)]
struct AkashicSurface(akashic_rs::asset::surface::Surface);

unsafe impl Send for SURFACE {}

unsafe impl Sync for SURFACE {}

fn move_system(
    device: NonSend<RenderDevice>,
    queue: NonSend<RenderQueue>,
    mut sprite: Query<&mut Transform, With<DADAD>>,
    surface: Res<SURFACE>,
) {
    let mut sprite = sprite.single_mut();
    sprite.translation += Vec3::NEG_X * 1.2;

    let output = surface.get_current_texture().expect("Failed current texture");
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
    }

    queue.submit(iter::once(encoder.finish()));
    output.present();
}

#[wasm_bindgen(getter_with_clone)]
pub struct Param {
    pub src: akashic_rs::asset::surface::Surface,
    pub drawer: Drawer,
}


#[wasm_bindgen]
#[derive(Clone)]
pub struct Drawer {
    device: Arc<Device>,
    queue: Arc<Queue>,
    surface: Arc<Surface>,
}

#[wasm_bindgen]
impl Drawer {
    pub fn render(&self) {
        console_log!("RENDERER");

        let surface = &self.surface;
        let device = &self.device;
        let queue = &self.queue;

        let output = surface.get_current_texture().expect("Failed current texture");
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

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

    #[wasm_bindgen(js_namespace = g)]
    pub fn create_3d(param: Param) -> akashic_rs::object2d::entity::sprite::Sprite;
}