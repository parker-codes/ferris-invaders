use bevy::math::Vec3Swizzles;
use bevy::utils::HashSet;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use components::{
    Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromEnemy, FromPlayer, Laser, Movable,
    Player, SpriteSize, Velocity,
};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod components;
mod enemy;
mod player;

/**
 * Asset Constants
 */
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144.0, 75.0);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9.0, 54.0);

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144.0, 75.0);
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (17.0, 55.0);

const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const EXPLOSION_LENGTH: usize = 16;

const SPRITE_SCALE: f32 = 0.5;

/**
 * Game Constants
 */
const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 500.0;
const WINDOW_MARGIN: f32 = 200.0;
const MAX_ENEMIES: u32 = 2;

/**
 * Resources
 */
struct WindowSize {
    pub width: f32,
    pub height: f32,
}
impl WindowSize {
    fn new(width: f32, height: f32) -> Self {
        WindowSize { width, height }
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

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

struct EnemyCount(u32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Ferris Invaders!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_system)
        .add_system(movement_system)
        .add_system(player_laser_hit_enemy_system)
        .add_system(enemy_laser_hit_player_system)
        .add_system(explosion_to_spawn_system)
        .add_system(explosion_animation_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let window_size = WindowSize::new(window.width(), window.height());
    commands.insert_resource(window_size);

    // create explosion texture
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 4, 4);
    let explosion = texture_atlases.add(texture_atlas);

    // add textures
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion,
    };
    commands.insert_resource(game_textures);

    commands.insert_resource(EnemyCount(0));
}

fn movement_system(
    mut commands: Commands,
    window_size: Res<WindowSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            // despawn once off-screen
            if translation.y > window_size.top_bound()
                || translation.y < window_size.bottom_bound()
                || translation.x > window_size.right_bound()
                || translation.x < window_size.left_bound()
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
    mut enemy_count: ResMut<EnemyCount>,
) {
    let mut despawned_entities = HashSet::new();

    // iterate through lasers
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }

        let laser_scale = Vec2::from(laser_tf.scale.xy());

        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&laser_entity)
            {
                continue;
            }

            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // determine if collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            // collision effects
            if let Some(_) = collision {
                // remove laser
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);

                // remove enemy
                commands.entity(enemy_entity).despawn();
                despawned_entities.insert(enemy_entity);
                enemy_count.0 -= 1;

                // show explosion
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}

fn enemy_laser_hit_player_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
) {
    // iterate through lasers
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        for (player_entity, player_tf, player_size) in player_query.iter() {
            let player_scale = Vec2::from(player_tf.scale.xy());

            // determine if collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                player_tf.translation,
                player_size.0 * player_scale,
            );

            // collision effects
            if let Some(_) = collision {
                // remove laser
                commands.entity(laser_entity).despawn();

                // remove enemy
                commands.entity(player_entity).despawn();

                // show explosion
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(player_tf.translation.clone()));
            }
        }
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (explosion_entity, explosion_to_spawn) in query.iter() {
        // spawn sprite
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        // despawn
        commands.entity(explosion_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            sprite.index += 1; // move to next sprite cell
            if sprite.index >= EXPLOSION_LENGTH {
                // despawn
                commands.entity(entity).despawn();
            }
        }
    }
}
