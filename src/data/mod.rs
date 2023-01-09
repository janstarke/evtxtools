mod event_id;
use std::fmt::Display;

pub use event_id::*;

mod event_record_id;
pub use event_record_id::*;

mod activity_id;
mod related_activity_id;
pub use activity_id::*;
pub use related_activity_id::*;

pub(crate) trait EvtxFieldView: Display {
    fn maximum_display_length(&self) -> usize;
    fn value_with_padding(&self) -> String;
}