use crate::prelude::EntityObject2D;

pub mod label;


pub trait CacheableEntityObject2D: EntityObject2D {
    /// このエンティティの描画キャッシュ無効化をエンジンに通知する。
    /// このメソッドを呼び出し後、描画キャッシュの再構築が行われ、各 Renderer に描画内容の変更が反映される。
    fn invalidate(&self);
}

