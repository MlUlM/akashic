use derive_builder::Builder;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::{EntityObject2D, entity_params};

use crate::game::Game;
use crate::object2d::Object2D;
use parent::Parent;
use crate::prelude::{PointDownHandler, UpdateHandler};
use crate::scene::Scene;
use crate::trigger::point::r#move::PointMoveHandler;

pub mod filled_rect;
pub mod sprite;

pub mod cacheable;
pub mod parent;


pub mod prelude {
    pub use crate::object2d::entity::{
        AkashicEntity,
        parent::Parent,
        EntityObject2D,
        filled_rect::*,
        sprite::*,
        cacheable::{
            CacheableEntityObject2D,
            label::*
        }
    };
}


/// アカシックエンティティオブジェクトが実装するプロパティやメソッド群を提供します。
///
/// アカシックエンジンにおける[E](https://akashic-games.github.io/akashic-engine/v3/classes/E.html)を表し、
/// [`E`]を継承する全てのエンティティ([`FilledRect`](filled_rect::FilledRect)や[`Sprite`](sprite::Sprite)など)もこのトレイトが実装されます。
///
///
/// より詳細な情報は[akashic-engineのリファレンス](https://akashic-games.github.io/akashic-engine/v3/classes/E.html)を参照してください。
pub trait EntityObject2D: Object2D + PointDownHandler + PointMoveHandler + PointMoveHandler + UpdateHandler + Into<AkashicEntity> {
    /// Returns the unique-id for each game assigned to this entity. (unless local is true).
    fn id(&self) -> isize;


    /// Returns the [`Scene`] to which this entity belongs.
    fn scene(&self) -> Scene;


    /// Returns the [`Game`] to which this entity belongs.
    fn game(&self) -> Game;


    /// Returns the children of this entity.
    fn children(&self) -> Box<[AkashicEntity]>;


    /// Returns the parent.
    ///
    /// If the parent does not exists, returns `None`.
    fn parent(&self) -> Option<Parent>;


    /// Remove the target entity from this entity.
    ///
    /// ## Panics
    ///
    /// If passed entity is not a child of this entity.
    fn remove_child(&self, child_entity: impl Into<AkashicEntity>);


    /// Remove this entity from parent.
    ///
    /// ## Panics
    ///
    /// If there is no parent to belong to.
    fn remove_self(&self);


    /// TODO: shader_programメソッドを定義する
    // fn shader_program(&self);

    /// Returns whether this object is a pointing target.
    fn touchable(&self) -> bool;


    fn append(&self, child: impl Into<AkashicEntity>);


    fn insert_before(&self, child: impl Into<AkashicEntity>, target: Option<AkashicEntity>);


    fn destroy(&self);


    fn destroyed(&self) -> bool;


    fn visible(&self) -> bool;


    fn show(&self);


    fn hide(&self);


    /// Notifies the engine of changes to this entity.
    ///
    /// After calling this method, changes made to itself will be reflected in each Renderer's drawing.
    fn modified(&self);


    fn as_js_value(&self) -> JsValue;


    fn js_value_ref(&self) -> &JsValue;
}


#[wasm_bindgen(js_namespace = g)]
extern {
    /// Represents the base class for Akashic entities.
    ///
    /// This is equivalent to [`E`](https:akashic-games.github.ioakashic-enginev 3 classes E.html).
    #[derive(Clone, EntityObject2D, Debug)]
    #[wasm_bindgen(js_name = "E")]
    pub type AkashicEntity;

    #[wasm_bindgen(constructor, js_class = "E")]
    pub fn new(param: AkashicEntityParam) -> AkashicEntity;
}


impl Default for AkashicEntity {
    #[inline(always)]
    fn default() -> Self {
        AkashicEntityBuilder::default().build()
    }
}


#[non_exhaustive]
#[entity_params]
#[wasm_bindgen(getter_with_clone)]
#[derive(Default, Debug, Builder)]
#[builder(
name = "AkashicEntityBuilder",
build_fn(private, name = "fallible_build")
)]
pub struct AkashicEntityParam {}


impl AkashicEntityBuilder {
    #[inline]
    pub fn build(&self) -> AkashicEntity {
        AkashicEntity::new(self.fallible_build().unwrap())
    }
}
