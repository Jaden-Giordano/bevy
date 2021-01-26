use bevy_reflect_derive::impl_reflect_value;
use glyph_brush_layout::{VerticalAlign, HorizontalAlign};

impl_reflect_value!(VerticalAlign(Hash, PartialEq));
impl_reflect_value!(HorizontalAlign(Hash, PartialEq));
