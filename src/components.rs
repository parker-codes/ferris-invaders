use bevy::{
    core::Timer,
    math::{Vec2, Vec3},
    prelude::Component,
};

/**
 * Common
 */

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn none() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn y(value: f32) -> Self {
        Self { x: 0.0, y: value }
    }
}

#[derive(Component)]
pub struct Movable {
    auto_despawn: bool,
}

impl Movable {
    pub fn with_auto_despawn(set: bool) -> Self {
        Self { auto_despawn: set }
    }

    pub fn should_auto_despawn(&self) -> bool {
        self.auto_despawn
    }
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);
impl From<(f32, f32)> for SpriteSize {
    fn from(value: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(value.0, value.1))
    }
}

/**
 * Player
 */

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;

/**
 * Enemy
 */

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;

/**
 * Explosion
 */

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        ExplosionTimer(Timer::from_seconds(0.05, true))
    }
}
