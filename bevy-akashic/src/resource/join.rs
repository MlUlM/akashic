use std::fmt::Debug;
use std::ops::Deref;

use bevy::prelude::{Deref, DerefMut, Resource};

use crate::component::player_id::PlayerId;

#[derive(Debug, Eq, PartialEq, Hash, Deref, DerefMut, Clone, Resource)]
pub struct JoinedAsStreamer(pub(crate) PlayerId);

#[derive(Debug, Eq, PartialEq, Hash, Deref, DerefMut, Clone, Resource)]
pub struct JoinedAsListener(pub(crate) PlayerId);

macro_rules! impl_joined {
    ($ident: ident) => {
        impl $ident {
            #[inline(always)]
            pub fn player_id(&self) -> &PlayerId {
                self.deref()
            }

            #[inline(always)]
            pub fn player_id_as_str(&self) -> &str {
                self.player_id().deref()
            }
        }
    };
}

impl_joined!(JoinedAsStreamer);
impl_joined!(JoinedAsListener);
