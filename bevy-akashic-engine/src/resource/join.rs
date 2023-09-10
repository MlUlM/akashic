use std::fmt::Debug;
use std::ops::Deref;

use bevy::prelude::{Deref, DerefMut, Resource};

use crate::component::player_id::PlayerId;
use crate::SharedObject;

#[derive(Debug, Eq, PartialEq, Hash, Deref, DerefMut, Clone, Resource)]
pub struct JoinedAsStreamer(pub(crate) PlayerId);


#[derive(Debug, Eq, PartialEq, Hash, Deref, DerefMut, Clone, Resource)]
pub struct JoinedAsListener(pub(crate) PlayerId);

macro_rules! impl_joined {
    ($ident: ident) => {
        impl $ident{
            #[inline(always)]
            pub fn player_id(&self) -> &PlayerId{
                self.deref()
            }

            #[inline(always)]
            pub fn player_id_as_str(&self) -> &str{
                self.player_id().deref()
            }
        }
    };
}

impl_joined!(JoinedAsStreamer);
impl_joined!(JoinedAsListener);


#[derive(Resource, Debug, Default, Deref, Clone)]
pub(crate) struct JoinStatusResource(pub(crate) SharedObject<JoinStatus>);


#[derive(Default, Debug, Clone)]
pub(crate) enum JoinStatus {
    #[default]
    Undefined,
    Streamer(String),
    Listener(String),
    /// マルチプレイ時に裏で動いているサーバ
    /// g.game.selfIdがundefinedの場合node側と判断できると思われる
    NodeServer
}


impl JoinStatus {
    pub fn not_undefined(&self) -> bool {
        !matches!(self, JoinStatus::Undefined)
    }
}
