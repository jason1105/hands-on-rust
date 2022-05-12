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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovingRandomly;

/// Indicate what character used for rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityGlyph(FontCharType);

/// A message for movement
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub position: Point,
}

/// Health of player
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
