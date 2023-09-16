use bevy::prelude::{Component, Deref, DerefMut};

/// プレイヤーにとって触れられるオブジェクトであるかを表す。
///
/// この値が偽である場合、ポインティングイベントの対象にならない。
#[derive(Component, Debug, Copy, Clone, Deref, DerefMut, Eq, PartialEq, Hash)]
pub struct Touchable(pub(crate) bool);


impl Touchable{
    /// このコンポーネントに関連付けられているアカシックエンティティがポインティングイベントの対象であるかを返します。
    #[inline]
    pub fn get(&self) -> bool{
        self.0
    }

    /// このコンポーネントに関連付けられているアカシックエンティティをポインティングイベントの対象から外します。
    #[inline(always)]
    pub fn off(&mut self){
        self.0 = false;
    }


    /// このコンポーネントに関連付けられているアカシックエンティティをポインティングイベントの対象にします。
    #[inline(always)]
    pub fn on(&mut self){
        self.0 = true;
    }


    /// 現在ポインティングイベントの対象を表す真偽値を反転させます。
    #[inline(always)]
    pub fn toggle(&mut self){
        self.0 = !self.0;
    }
}