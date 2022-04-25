use crate::prelude::*;

/// All components.
/// Components have data, but have no behaviors

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

/// Player is a tag component used for identify rule of entity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player;

/// Enemy
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Enemy;

/// Indicate what character used for rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityGlyph(FontCharType);
