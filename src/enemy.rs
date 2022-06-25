use crate::{
    components::{Enemy, FromEnemy, Laser, Movable, SpriteSize, Velocity},
    EnemyCount, GameTextures, WindowSize, ENEMY_LASER_SIZE, ENEMY_SIZE, MAX_ENEMIES, SPRITE_SCALE,
};
use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun, prelude::*};
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(enemy_spawn_system),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(enemy_fire_criteria)
                .with_system(enemy_fire_system),
        );
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    window_size: Res<WindowSize>,
    mut enemy_count: ResMut<EnemyCount>,
) {
    if enemy_count.0 < MAX_ENEMIES {
        // random placement
        let mut rng = thread_rng();
        let width_span = window_size.width / 2.0 - 100.0;
        let height_span = window_size.height / 2.0 - 100.0;
        let x = rng.gen_range(-width_span..width_span);
        let y = rng.gen_range(-height_span..height_span);

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.0),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for &enemy_tf in enemy_query.iter() {
        let (enemy_x, enemy_y) = (enemy_tf.translation.x, enemy_tf.translation.y);
        let y = enemy_y - 15.0;

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(enemy_x, y, 10.0),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(FromEnemy)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(Movable { auto_despawn: true })
            .insert(Velocity { x: 0.0, y: -1.0 });
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1.0 / 60.0) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
