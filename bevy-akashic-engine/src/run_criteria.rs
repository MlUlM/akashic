use bevy::prelude::{Res, resource_added, resource_exists};

use crate::resource::join::{JoinedAsListener, JoinedAsStreamer};

#[inline]
pub fn joined_as_streamer() -> impl FnMut(Option<Res<JoinedAsStreamer>>) -> bool + Clone + Sized {
    resource_exists::<JoinedAsStreamer>()
}


#[inline]
pub fn add_joined_as_streamer() -> impl FnMut(Option<Res<JoinedAsStreamer>>) -> bool + Clone + Sized {
    resource_added::<JoinedAsStreamer>()
}


#[inline]
pub fn joined_as_listener() -> impl FnMut(Option<Res<JoinedAsListener>>) -> bool + Clone + Sized {
    resource_exists::<JoinedAsListener>()
}

#[inline]
pub fn add_joined_as_listener() -> impl FnMut(Option<Res<JoinedAsListener>>) -> bool + Clone + Sized {
    resource_added::<JoinedAsListener>()
}