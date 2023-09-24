use std::sync::atomic::{AtomicBool, Ordering};

use bevy::app::App;
use bevy::ecs::event::ManualEventReader;
use bevy::ecs::system::{SystemParam, SystemState};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseButtonInput, MouseWheel};
use bevy::input::touchpad::{TouchpadMagnify, TouchpadRotate};
use bevy::prelude::{Deref, DetectChanges, Entity, Event, EventWriter, FromWorld, Ime, Plugin, Query, ReceivedCharacter, SystemParamFunction, TouchInput, WindowMoved, With};
use bevy::window::{PrimaryWindow, RawHandleWrapper, RequestRedraw, Window, WindowBackendScaleFactorChanged, WindowCloseRequested, WindowDestroyed, WindowFocused, WindowResized, WindowScaleFactorChanged, WindowThemeChanged};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, WebDisplayHandle, WebWindowHandle};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;
use winit::event;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::platform::web::WindowBuilderExtWebSys;

use akashic::console_log;
use akashic::game::GAME;
use akashic::prelude::UpdateHandler;
use bevy_akashic::event::AkashicEventQueue;

use crate::winit::converters::{convert_element_state, convert_mouse_button, convert_touch_input};

mod converters;

#[derive(Event, Debug)]
struct SceneUpdateEvent;

#[derive(Deref)]
pub(crate) struct AkashicSurface(akashic::asset::surface::Surface);


pub struct AkashicWinitPlugin;


impl Plugin for AkashicWinitPlugin {
    fn build(&self, app: &mut App) {
        let akashic_surface = create_screen_surface();

        let mut state: SystemState<
            Query<(Entity, &mut Window), With<Window>>
        > = SystemState::from_world(&mut app.world);

        {
            let canvas= akashic_surface.canvas();
            let mut query = state.get_mut(&mut app.world);
            let (primary, mut window) = query.single_mut();
            window.resolution.set_physical_resolution(canvas.width(), canvas.height());

            let mut window_raw_handle = WebWindowHandle::empty();
            window_raw_handle.id = 1;

            app.world
                .entity_mut(primary)
                .insert(RawHandleWrapper {
                    window_handle: RawWindowHandle::Web(window_raw_handle),
                    display_handle: RawDisplayHandle::Web(WebDisplayHandle::empty()),
                });
        }
        // let canvas = window().unwrap().document().unwrap().create_element("canvas")
        //     .unwrap()
        //     .dyn_into::<HtmlCanvasElement>()
        //     .unwrap();
        // canvas.set_width(GAME.width() as u32);
        // canvas.set_height(GAME.height() as u32);
        // window().unwrap().document().unwrap().body().unwrap().append_child(&canvas).unwrap();

        let mut window_handle = WebWindowHandle::empty();
        window_handle.id = 1;
        // let event_loop = EventLoopBuilder::new().build();
        // let window = WindowBuilder::new()
        //     .with_canvas(Some(akashic_surface.canvas()))
        //     .with_inner_size(Size::Logical(LogicalSize {
        //         width: GAME.width() as f64,
        //         height: GAME.height() as f64,
        //     }))
        //     .with_enabled_buttons(WindowButtons::all())
        //     .with_resizable(false)
        //     .with_focusable(true)
        //     .with_visible(true)
        //     .with_position(LogicalPosition::new(0., 0.))
        //     .build(&event_loop)
        //     .unwrap()
        //     ;
        //
        // window.set_cursor_hittest(true);

        app
            // .insert_non_send_resource(window)
            // .insert_non_send_resource(event_loop)
            .add_event::<SceneUpdateEvent>()
            .insert_non_send_resource(AkashicSurface(akashic_surface))
            .insert_resource(AkashicEventQueue::<SceneUpdateEvent>::default());
        // .set_runner(runner);
        // .add_plugins(AkashicScheduleRunnerPlugin);
    }
}

fn runner(app: App) {
    subscribe_scene_update(&app);
    event_loop_runner(app);
}


#[derive(Default)]
struct WinitState {
    redraw: bool,
}

fn event_loop_runner(
    mut app: App
) {
    let event_loop = app.world.remove_non_send_resource::<EventLoop<()>>().unwrap();
    let mut redraw_event_reader = ManualEventReader::<RequestRedraw>::default();

    let mut winit_state = WinitState::default();
    app.world
        .insert_non_send_resource(event_loop.create_proxy());
    let event_handler = move |event: winit::event::Event<()>,
                              event_loop: &EventLoopWindowTarget<()>,
                              control_flow: &mut ControlFlow| {
        let mut system_state: SystemState<(
            Query<(Entity, &mut Window), With<PrimaryWindow>>,
            WindowEvents,
            InputEvents,
        )> = SystemState::from_world(&mut app.world);
        let (
            mut window,
            mut window_events,
            mut input_events,
        ) = system_state.get_mut(&mut app.world);

        let (window_entity, mut window) = window.single_mut();

        match event {
            event::Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Resized(size) => {
                        window
                            .resolution
                            .set_physical_resolution(size.width, size.height);

                        window_events.window_resized.send(WindowResized {
                            window: window_entity,
                            width: window.width(),
                            height: window.height(),
                        });
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        input_events.mouse_button_input.send(MouseButtonInput {
                            state: convert_element_state(state),
                            button: convert_mouse_button(button),
                            window: window_entity,
                        })
                    }
                    WindowEvent::Touch(touch) => {
                        let location = touch.location.to_logical(window.resolution.scale_factor());
                        input_events.touch_input.send(convert_touch_input(touch, location));
                    }
                    WindowEvent::TouchpadMagnify { delta, .. } => {
                        input_events
                            .touchpad_magnify_input
                            .send(TouchpadMagnify(delta as f32));
                    }
                    WindowEvent::TouchpadRotate { delta, .. } => {
                        input_events
                            .touchpad_rotate_input
                            .send(TouchpadRotate(delta));
                    }
                    _ => {}
                }
            }
            event::Event::MainEventsCleared => update_handler(&mut app, &winit_state),
            winit::event::Event::RedrawEventsCleared => {
                *control_flow = ControlFlow::Wait;
                winit_state.redraw = true;
            }
            _ => {}
        }
    };


    event_loop.run(event_handler);
}


#[derive(SystemParam)]
struct InputEvents<'w> {
    keyboard_input: EventWriter<'w, KeyboardInput>,
    character_input: EventWriter<'w, ReceivedCharacter>,
    mouse_button_input: EventWriter<'w, MouseButtonInput>,
    touchpad_magnify_input: EventWriter<'w, TouchpadMagnify>,
    touchpad_rotate_input: EventWriter<'w, TouchpadRotate>,
    mouse_wheel_input: EventWriter<'w, MouseWheel>,
    touch_input: EventWriter<'w, TouchInput>,
    ime_input: EventWriter<'w, Ime>,
}


fn update_handler(app: &mut App, winit_state: &WinitState) {
    static SETUP_DOWN: AtomicBool = AtomicBool::new(false);
    if !SETUP_DOWN.load(Ordering::Relaxed) {
        if app.ready() {
            app.finish();
            app.cleanup();
            SETUP_DOWN.store(true, Ordering::Relaxed);
        } else {
            return;
        }
    }

    let update_queue = app.world.resource::<AkashicEventQueue<SceneUpdateEvent>>().clone();
    while let Some(_) = update_queue.pop_front() {
        update_queue.clear();
        app.update();
    }
}


fn subscribe_scene_update(app: &App) {
    let update_queue = app
        .world
        .resource::<AkashicEventQueue::<SceneUpdateEvent>>()
        .clone();

    GAME
        .scene()
        .on_update()
        .add(move || {
            update_queue.push(SceneUpdateEvent);
        });
}


#[derive(SystemParam)]
struct WindowEvents<'w> {
    window_resized: EventWriter<'w, WindowResized>,
    window_close_requested: EventWriter<'w, WindowCloseRequested>,
    window_scale_factor_changed: EventWriter<'w, WindowScaleFactorChanged>,
    window_backend_scale_factor_changed: EventWriter<'w, WindowBackendScaleFactorChanged>,
    window_focused: EventWriter<'w, WindowFocused>,
    window_moved: EventWriter<'w, WindowMoved>,
    window_theme_changed: EventWriter<'w, WindowThemeChanged>,
    window_destroyed: EventWriter<'w, WindowDestroyed>,
}


#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = g)]
    fn create_screen_surface() -> akashic::asset::surface::Surface;
}