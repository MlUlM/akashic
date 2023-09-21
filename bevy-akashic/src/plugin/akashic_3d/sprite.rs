use bevy::app::App;
use bevy::prelude::{Plugin, Resource};
use bevy::render::render_resource::{BufferUsages, BufferVec};
use bevy::sprite::SpriteMeta;
use bytemuck::Pod;
use wgpu::{BindGroup, Buffer};

pub struct AkashicSpritePlugin;


impl Plugin for AkashicSpritePlugin {
    fn build(&self, app: &mut App) {

    }
}


#[derive(Resource)]
pub struct AkashicSpriteMeta {
    view_bind_group: Option<BindGroup>,
    sprite_index_buffer: BufferVec<u32>,
    // sprite_instance_buffer: BufferVec<AkashicSpritePlugin>,
}

impl Default for AkashicSpriteMeta {
    fn default() -> Self {
        Self {
            view_bind_group: None,
            sprite_index_buffer: BufferVec::<u32>::new(BufferUsages::INDEX),
            // sprite_instance_buffer: BufferVec::<AkashicSpriteInstance>::new(BufferUsages::VERTEX),
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AkashicSpriteInstance {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub tex_coords: [f32; 2],
}
