use bevy::prelude::*;
use std::fmt;

pub enum SpriteDirection {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for SpriteDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpriteDirection::Up => write!(f, "Up"),
            SpriteDirection::Down => write!(f, "Down"),
            SpriteDirection::Left => write!(f, "Left"),
            SpriteDirection::Right => write!(f, "Right"),
        }
    }
}

pub fn update_sprite_direction(sprite_direction: &mut SpriteDirection, x: f32, y: f32) {
    // If a key is held that matches current direction, keep it!
    if (x < 0.0 && matches!(sprite_direction, SpriteDirection::Left)) ||
        (x > 0.0 && matches!(sprite_direction, SpriteDirection::Right)) ||
        (y < 0.0 && matches!(sprite_direction, SpriteDirection::Down)) ||
        (y > 0.0 && matches!(sprite_direction, SpriteDirection::Up)) { 
        return;
    }

    if x < 0.0 {
        *sprite_direction = SpriteDirection::Left;
        return;
    }
    if x > 0.0 {
        *sprite_direction = SpriteDirection::Right;
        return;
    }
    if y < 0.0 {
        *sprite_direction = SpriteDirection::Down;
        return;
    }
    if y > 0.0 {
        *sprite_direction = SpriteDirection::Up;
        return;
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer {
    pub timer: Timer,
}

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    timer: &mut AnimationTimer,
    sprite: &mut TextureAtlasSprite,
    texture_atlas_handle: &Handle<TextureAtlas>
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
    }
}