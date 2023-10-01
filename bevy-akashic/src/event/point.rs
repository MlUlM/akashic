use bevy::math::Vec2;

use akashic::player::Player;
use akashic::trigger::PointEventBase;

use crate::resource::game::GameInfo;

pub mod down;
pub mod up;
pub mod r#move;
pub(crate) mod event_inner;


pub trait AkashicPointEventBase {
    fn source_player(&self) -> Player;


    fn pointer_location(&self) -> Vec2;


    #[inline(always)]
    fn source_player_id(&self) -> String {
        self.source_player().id().unwrap()
    }

    #[inline(always)]
    fn this_event_is_mine(&self, game_info: &GameInfo) -> bool {
        self.source_player_id() == game_info.self_id()
    }
}


impl<E: PointEventBase> AkashicPointEventBase for E {
    #[inline]
    fn source_player(&self) -> Player {
        self.player().unwrap()
    }


    #[inline]
    fn pointer_location(&self) -> Vec2 {
        let pos = self.point();
        Vec2::new(pos.x(), pos.y())
    }
}



