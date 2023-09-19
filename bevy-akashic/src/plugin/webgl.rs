use bevy::app::App;
use bevy::prelude::Plugin;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use wgpu::include_wgsl;
use crate::plugin::akashic_3d::canvas;

pub struct AkashicWebGlPlugin;


impl Plugin for AkashicWebGlPlugin{
    fn build(&self, app: &mut App) {
        let gl = canvas(100, 100).get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
        gl.clear_color(0., 0., 1., 1.);
        gl.clear_depth(1.);
        gl.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        let shader = include_wgsl!("shader.wgsl");

    }
}