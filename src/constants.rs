/**
 * Asset Constants
 */
pub const PLAYER_SPRITE: &str = "player.png";
pub const PLAYER_SIZE: (f32, f32) = (144.0, 75.0);
pub const PLAYER_LASER_SPRITE: &str = "player-laser.png";
pub const PLAYER_LASER_SIZE: (f32, f32) = (9.0, 54.0);

pub const ENEMY_1_SPRITE: &str = "enemy-1.png";
pub const ENEMY_2_SPRITE: &str = "enemy-2.png";
pub const ENEMY_SIZE: (f32, f32) = (144.0, 75.0);
pub const ENEMY_LASER_SPRITE: &str = "enemy-laser.png";
pub const ENEMY_LASER_SIZE: (f32, f32) = (17.0, 55.0);
pub const EXPLOSION_SHEET: &str = "explosion-sheet.png";
pub const EXPLOSION_LENGTH: usize = 16;

pub const SPRITE_SCALE: f32 = 0.5;

/**
 * Game Constants
 */
pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const BASE_SPEED: f32 = 500.0;
pub const WINDOW_MARGIN: f32 = 200.0;
pub const MAX_ENEMIES: u32 = 4;
pub const FORMATION_MEMBERS_MAX: u32 = 2;
pub const PLAYER_RESPAWN_DELAY: f64 = 2.0;
