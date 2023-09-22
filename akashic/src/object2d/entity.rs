use derive_builder::Builder;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::{EntityObject2D, object_e_parameter};

use crate::game::Game;
use crate::object2d::Object2D;
use crate::parent::Parent;
use crate::prelude::{PointDownHandler, UpdateHandler};
use crate::scene::Scene;
use crate::trigger::point::point_move::PointMoveHandler;

pub mod filled_rect;
pub mod sprite;

pub mod cacheable;


pub mod prelude {
    pub use crate::object2d::entity::{
        AkashicEntity,
        EntityObject2D,
        filled_rect::*,
        sprite::*,
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
    /// このエンティティに割り振られる Game 単位で一意のID。(ただし local が真である場合を除く)
    fn id(&self) -> isize;


    /// このエンティティが属する[`Scene`](crate::scene::Scene)を取得します。
    fn scene(&self) -> Scene;


    /// このエンティティが属する[`Game`](crate::game::Game)を返します。
    fn game(&self) -> Game;


    /// 自身の子となるエンティティをすべて取得します。
    fn children(&self) -> Box<[AkashicEntity]>;


    /// 自身の親を返します。
    ///
    /// 親が存在しない場合、Noneが返されます。
    fn parent(&self) -> Option<Parent>;


    /// 指定されたエンティティが自身の子に属す場合、そのエンティティを削除します。
    ///
    /// ## Panics
    ///
    /// 指定されたエンティティが自身の子ではない場合
    fn remove_child(&self, child_entity: impl Into<AkashicEntity>);


    /// 自身を親から削除します。
    ///
    /// ## Panics
    ///
    /// 属する親がいない場合
    fn remove_self(&self);


    /// TODO: shader_programメソッドを定義する
    // fn shader_program(&self);

    /// プレイヤーにとって触れられるオブジェクトであるかを表します。
    ///
    /// 値がfalseである場合、ポインティングイベントの対象になりません。
    /// デフォルトはfalseです。
    ///
    /// [`EntityObject2D`](EntityObject2D)の他のプロパティと異なり、この値の変更後に this.modified() を呼び出す必要はありません。
    fn touchable(&self) -> bool;


    /// 子を追加します。
    fn append(&self, child: impl Into<AkashicEntity>);


    /// 子をtargetの直前に挿入します。
    ///
    /// targetが自身の子でない場合、append(e) と同じ動作となります。
    fn insert_before(&self, child: impl Into<AkashicEntity>, target: Option<AkashicEntity>);


    /// このエンティティを破棄します。
    fn destroy(&self);


    /// このエンティティが破棄済みであるかを返します。
    fn destroyed(&self) -> bool;


    /// このエンティティが表示状態であるかを返します。
    fn visible(&self) -> bool;


    /// 自身を表示状態にします。
    fn show(&self);


    /// 自身を非表示状態にします。
    ///
    /// [`show`](EntityObject2D::show) が呼ばれるまでの間、このエンティティは各 Renderer によって描画されず、Game#findPointSource() で返されることもなくなります。
    ///
    /// this#pointDown, pointMove, pointUp なども通常の方法ではfireされなくなります。
    fn hide(&self);


    /// このエンティティに対する変更をエンジンに通知します。
    ///
    /// このメソッドの呼び出し後、 自身に対する変更が各Rendererの描画に反映されます。
    ///
    /// ## Notes
    ///
    /// - このオブジェクトの Object2D 由来のプロパティ (x, y, angle など) を変更した場合にも呼びだす必要があります。
    ///
    /// - このメソッドは描画キャッシュの無効化処理を含みません。描画キャッシュを持つエンティティは、このメソッドとは別に[`invalidate`](cacheable::CacheableEntityObject2D::invalidate) が提供されており、
    /// そちらを呼び出す必要があります。
    fn modified(&self);


    /// 自身を表すJsValueをクローンして返します。
    fn as_js_value(&self) -> JsValue;


    /// 自身を表すJsValueの不変参照を返します。
    fn js_value_ref(&self) -> &JsValue;
}


#[wasm_bindgen(js_namespace = g)]
extern {
    /// アカシックエンティティの基底クラスを表します。
    ///
    /// これはアカシックの[`E`](https://akashic-games.github.io/akashic-engine/v3/classes/E.html)と同等のものです。
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
#[object_e_parameter]
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