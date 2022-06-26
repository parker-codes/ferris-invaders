use crate::{
    components::{FromPlayer, Laser, Movable, Player, SpriteSize, Velocity},
    constants::{PLAYER_LASER_SIZE, PLAYER_RESPAWN_DELAY, PLAYER_SIZE, SPRITE_SCALE},
    resources::{GameTextures, PlayerState, WindowSize},
};
use bevy::{core::FixedTimestep, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(player_spawn_system),
            )
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    window_size: Res<WindowSize>,
) {
    if !player_state.alive
        && time.seconds_since_startup()
            > player_state.last_shot.unwrap_or(-1.0) + PLAYER_RESPAWN_DELAY
    {
        let bottom = -window_size.height / 2.0;
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        0.0,
                        bottom + PLAYER_SIZE.1 / 2.0 * SPRITE_SCALE + 5.0,
                        10.0,
                    ),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player)
            .insert(SpriteSize::from(PLAYER_SIZE))
            .insert(Movable {
                auto_despawn: false,
            })
            .insert(Velocity { x: 0.0, y: 0.0 });

        player_state.mark_spawned();
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.0
        } else if kb.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        }
    }
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (player_x, player_y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2.0 * SPRITE_SCALE - 5.0;

            let mut spawn_laser = |x_offset: f32| {
                let x = player_x + x_offset;
                let y = player_y + 15.0;

                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.0),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity { x: 0.0, y: 1.3 });
            };

            spawn_laser(x_offset); // right claw
            spawn_laser(-x_offset); // left claw
        }
    }
}
