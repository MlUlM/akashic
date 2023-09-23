use crate::asset::image::ImageAsset;

pub mod surface;
pub mod src;
pub mod image;
pub mod audio;
pub mod text;
pub mod accessor;


pub mod prelude{
    pub use crate::asset::{
        accessor::AssetAccessor,
        audio::*,
        image::ImageAsset,
        src::Src,
        surface::Surface,
        text::TextAsset
    };
}

pub trait Asset {
    fn id(&self) -> String;


    fn original_path(&self) -> String;


    fn path(&self) -> String;


    fn asset_type(&self) -> String;


    /// このアセットのリソースの破棄を行う。
    fn destroy(&self);


    /// このアセットのリソースが破棄済みであるかどうかを判定する。
    fn destroyed(&self) -> bool;


    /// 現在利用中で解放出来ない Asset かどうかを返す。 戻り値は、利用中である場合真、でなければ偽である。
    ///
    /// 本メソッドは通常 false が返るべきである。 例えば Sprite の元画像として使われているケース等では、その Sprite によって Asset は Surface に変換されているべきで、 Asset が利用中で解放出来ない状態になっていない事を各プラットフォームで保障する必要がある。
    ///
    /// 唯一、例外的に本メソッドが true を返すことがあるのは音楽を表す Asset である。 BGM等はシーンをまたいで演奏することもありえる上、 演奏中のリソースのコピーを常に各プラットフォームに強制するにはコストがかかりすぎるため、 本メソッドは true を返し、適切なタイミングで Asset が解放されるよう制御する必要がある。
    fn in_use(&self) -> bool;
}

