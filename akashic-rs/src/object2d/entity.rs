use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use akashic_macro::AkashicEntity;

use crate::game::Game;
use crate::object2d::Object2D;
use crate::parent::Parent;
use crate::prelude::{PointDownHandler, UpdateHandler};
use crate::scene::Scene;
use crate::trigger::point_move::PointMoveHandler;

pub mod filled_rect;
pub mod sprite;

pub mod cacheable;


pub mod prelude {
    pub use crate::object2d::entity::{
        Entity,
        EntityObject2D,
        filled_rect::*,
        sprite::*,
    };
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, AkashicEntity, Debug)]
    pub type Entity;
}


pub trait EntityObject2D: Object2D + PointDownHandler + PointMoveHandler + PointMoveHandler + UpdateHandler + Into<Entity> {
    /// このエンティティに割り振られる Game 単位で一意のID。(ただし local が真である場合を除く)
    fn id(&self) -> isize;


    /// このエンティティが属する[`Scene`]を取得する。
    fn scene(&self) -> Scene;


    /// このエンティティが属する[`Game`](crate::game::Game)を返す。
    fn game(&self) -> Game;


    /// 自身の子となるエンティティをすべて取得する。
    fn children(&self) -> Box<[Entity]>;


    fn parent(&self) -> Option<Parent>;


    /// 指定されたエンティティが自身の子に属す場合、そのエンティティを削除する。
    ///
    /// ## Panics
    ///
    /// 指定されたエンティティが自身の子ではない場合
    fn remove_child(&self, child_entity: impl Into<Entity>);


    /// 自身を親から削除する。
    ///
    /// ## Panics
    ///
    /// 属する親がいない場合
    fn remove_self(&self);


    /// TODO: shader_programメソッドを定義する
    // fn shader_program(&self);

    /// プレイヤーにとって触れられるオブジェクトであるかを表す。
    ///
    /// この値が偽である場合、ポインティングイベントの対象にならない。 初期値は false である。
    ///
    /// E の他のプロパティと異なり、この値の変更後に this.modified() を呼び出す必要はない。
    fn touchable(&self) -> bool;


    /// 子を追加する。
    fn append(&self, child: impl Into<Entity>);


    /// 子を挿入する。
    ///
    /// target がthis の子でない場合、append(e) と同じ動作となる。
    fn insert_before(&self, child: impl Into<Entity>, target: Option<Entity>);


    /// このエンティティを破棄する。
    fn destroy(&self);


    /// このエンティティが破棄済みであるかを返す
    fn destroyed(&self) -> bool;

    /// このEを非表示状態にする。
    ///
    /// this.show() が呼ばれるまでの間、このエンティティは各 Renderer によって描画されない。 また Game#findPointSource() で返されることもなくなる。 this#pointDown, pointMove, pointUp なども通常の方法ではfireされなくなる。
    fn hide(&self);

    /// このエンティティに対する変更をエンジンに通知する。
    ///
    /// このメソッドの呼び出し後、 this に対する変更が各 Renderer の描画に反映される。 ただし逆は真ではない。すなわち、再描画は他の要因によって行われることもある。 ゲーム開発者は、このメソッドを呼び出していないことをもって再描画が行われていないことを仮定してはならない。
    ///
    /// 本メソッドは、このオブジェクトの Object2D 由来のプロパティ (x, y, angle など) を変更した場合にも呼びだす必要がある。 本メソッドは、描画キャッシュの無効化処理を含まない。描画キャッシュを持つエンティティは、このメソッドとは別に invalidate() を提供している。 描画キャッシュの無効化も必要な場合は、このメソッドではなくそちらを呼び出す必要がある。
    fn modified(&self);

    fn as_js_value(&self) -> JsValue;


    fn js_value_ref(&self) -> &JsValue;
}




