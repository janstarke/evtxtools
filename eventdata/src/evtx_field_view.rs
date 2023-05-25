use std::fmt::Display;

pub trait EvtxFieldView: Display {
    fn maximum_display_length(&self) -> usize;
    fn value_with_padding(&self) -> String;
}