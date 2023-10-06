use crate::event::join::JoinEvent;
use crate::prelude::Trigger;

pub trait JoinHandler {
    /// Returns the trigger dealing with [`JoinEvent`].
    ///
    /// ### Notes
    ///
    /// JoinEvent is occurred when the player jointed,
    /// but the case of the running on "nico-live" is a bit special.
    ///
    /// In this case, the event will only fire when the streamer joins.
    fn on_join(&self) -> Trigger<JoinEvent>;
}
