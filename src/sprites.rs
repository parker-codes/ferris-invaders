use std::f32::consts::PI;

use crate::constants::{PLAYER_SIZE, SPRITE_SCALE};
use bevy::{
    math::{Quat, Vec3},
    prelude::{Handle, Image, Transform},
    sprite::{SpriteBundle, SpriteSheetBundle, TextureAtlas},
};

pub fn player_sprite(texture: Handle<Image>, bottom: f32) -> SpriteBundle {
    SpriteBundle {
        texture,
        transform: Transform {
            translation: Vec3::new(0.0, bottom + PLAYER_SIZE.1 / 2.0 * SPRITE_SCALE + 5.0, 10.0),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn player_laser_sprite(texture: Handle<Image>, (x, y): (f32, f32)) -> SpriteBundle {
    SpriteBundle {
        texture,
        transform: Transform {
            translation: Vec3::new(x, y, 0.0),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn enemy_sprite(texture: Handle<Image>, (x, y): (f32, f32)) -> SpriteBundle {
    SpriteBundle {
        texture,
        transform: Transform {
            translation: Vec3::new(x, y, 10.0),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn enemy_laser_sprite(texture: Handle<Image>, (x, y): (f32, f32)) -> SpriteBundle {
    SpriteBundle {
        texture,
        transform: Transform {
            translation: Vec3::new(x, y, 10.0),
            rotation: Quat::from_rotation_x(PI),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn explosion_sprite(
    texture_atlas: Handle<TextureAtlas>,
    translation: Vec3,
) -> SpriteSheetBundle {
    SpriteSheetBundle {
        texture_atlas,
        transform: Transform {
            translation,
            ..Default::default()
        },
        ..Default::default()
    }
}
