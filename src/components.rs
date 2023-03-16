use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}
// Tag component denotes an entity is a player
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;
