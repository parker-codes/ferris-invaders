use crate::constants::{MAX_ENEMIES, WINDOW_MARGIN};
use bevy::{math::Vec3, prelude::*};

pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

impl WindowSize {
    pub fn new(width: f32, height: f32) -> Self {
        WindowSize { width, height }
    }

    pub fn doesnt_contain(&self, translation: &mut Vec3) -> bool {
        translation.y > self.top_bound()
            || translation.y < self.bottom_bound()
            || translation.x > self.right_bound()
            || translation.x < self.left_bound()
    }

    fn top_bound(&self) -> f32 {
        self.height / 2.0 + WINDOW_MARGIN
    }

    fn bottom_bound(&self) -> f32 {
        -self.height / 2.0 - WINDOW_MARGIN
    }

    fn right_bound(&self) -> f32 {
        self.width / 2.0 + WINDOW_MARGIN
    }

    fn left_bound(&self) -> f32 {
        -self.width / 2.0 - WINDOW_MARGIN
    }
}

pub struct GameTextures {
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>,
    pub enemy_1: Handle<Image>,
    pub enemy_2: Handle<Image>,
    pub enemy_laser: Handle<Image>,
    pub explosion: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct EnemyCount(u32);

impl EnemyCount {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn decrement(&mut self) {
        self.0 -= 1;
    }

    pub fn has_availability(&self) -> bool {
        self.0 < MAX_ENEMIES
    }
}

pub struct PlayerState {
    pub alive: bool,
    pub last_shot: Option<f64>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            alive: false,
            last_shot: None,
        }
    }
}

impl PlayerState {
    pub fn mark_shot(&mut self, time: f64) {
        self.alive = false;
        self.last_shot = Some(time);
    }

    pub fn mark_spawned(&mut self) {
        self.alive = true;
        self.last_shot = None;
    }
}
