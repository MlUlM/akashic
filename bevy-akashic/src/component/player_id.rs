use bevy::prelude::{Component, Deref, DerefMut, Resource};
use serde::{Deserialize, Serialize};


#[derive(Debug, Eq, PartialEq, Hash, Deref, DerefMut, Clone, Component, Resource, Serialize, Deserialize)]
pub struct SelfPlayerId(pub Option<String>);


#[derive(Debug, Eq, PartialEq, Hash, Deref, DerefMut, Clone, Component, Resource, Serialize, Deserialize)]
pub struct PlayerId(pub String);


