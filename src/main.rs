use bevy::math::Vec3Swizzles;
use bevy::utils::HashSet;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use components::{
    Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromEnemy, FromPlayer, Laser, Movable,
    Player, SpriteSize, Velocity,
};
use constants::{
    BASE_SPEED, ENEMY_1_SPRITE, ENEMY_2_SPRITE, ENEMY_LASER_SPRITE, EXPLOSION_LENGTH,
    EXPLOSION_SHEET, PLAYER_LASER_SPRITE, PLAYER_SPRITE, TIME_STEP,
};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use resources::{EnemyCount, GameTextures, PlayerState, WindowSize};
use sprites::explosion_sprite;

mod components;
mod constants;
mod enemy;
mod player;
mod resources;
mod sprites;

fn main() {
    App::new()
        .add_startup_system(setup_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
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
    commands.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)));

    commands.insert_resource(WindowDescriptor {
        title: "Ferris Invaders!".to_string(),
        width: 598.0,
        height: 676.0,
        ..Default::default()
    });

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
        enemy_1: asset_server.load(ENEMY_1_SPRITE),
        enemy_2: asset_server.load(ENEMY_2_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion,
    };
    commands.insert_resource(game_textures);
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

        if movable.should_auto_despawn() && window_size.doesnt_contain(translation) {
            // remove because it's off-screen
            commands.entity(entity).despawn();
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
                enemy_count.decrement();

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
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
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

                // remove player
                commands.entity(player_entity).despawn();
                player_state.mark_shot(time.seconds_since_startup());

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
        commands
            .spawn_bundle(explosion_sprite(
                game_textures.explosion.clone(),
                explosion_to_spawn.0,
            ))
            .insert(Explosion)
            .insert(ExplosionTimer::default());

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
                commands.entity(entity).despawn();
            }
        }
    }
}
