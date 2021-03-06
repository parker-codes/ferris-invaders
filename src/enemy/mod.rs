use self::formation::{Formation, FormationMaker};
use crate::{
    components::{Enemy, FromEnemy, Laser, Movable, SpriteSize, Velocity},
    constants::{ENEMY_LASER_SIZE, ENEMY_SIZE, TIME_STEP},
    sprites::{enemy_laser_sprite, enemy_sprite},
    EnemyCount, GameTextures, WindowSize,
};
use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun, prelude::*};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

mod formation;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyCount::default())
            .insert_resource(FormationMaker::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(enemy_spawn_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(enemy_fire_criteria)
                    .with_system(enemy_fire_system),
            )
            .add_system(enemy_movement_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    window_size: Res<WindowSize>,
    mut formation_maker: ResMut<FormationMaker>,
    mut enemy_count: ResMut<EnemyCount>,
) {
    if enemy_count.has_availability() {
        // get formation and start x/y
        let formation = formation_maker.make(&window_size);
        let (x, y) = formation.start;

        let texture = if thread_rng().gen_bool(0.5) {
            game_textures.enemy_1.clone()
        } else {
            game_textures.enemy_2.clone()
        };

        commands
            .spawn_bundle(enemy_sprite(texture, (x, y)))
            .insert(Enemy)
            .insert(formation)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.increment();
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
            .spawn_bundle(enemy_laser_sprite(
                game_textures.enemy_laser.clone(),
                (enemy_x, y),
            ))
            .insert(Laser)
            .insert(FromEnemy)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(Movable::with_auto_despawn(true))
            .insert(Velocity::y(-0.6));
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1.0 / 60.0) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_movement_system(mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>) {
    for (mut enemy_tf, mut formation) in query.iter_mut() {
        // current position
        let (x_origin, y_origin) = (enemy_tf.translation.x, enemy_tf.translation.y);

        let max_distance = TIME_STEP * formation.speed;

        // 1 for clockwise, -1 for counter-clockwise
        let direction = if formation.start.0 < 0.0 { 1.0 } else { -1.0 };
        let (x_pivot, y_pivot) = formation.pivot;
        let (x_radius, y_radius) = formation.radius;

        let angle = formation.angle
            + direction * formation.speed * TIME_STEP / (x_radius.min(y_radius) * PI / 2.0);

        // compute target x/y
        let x_dst = x_radius * angle.cos() + x_pivot;
        let y_dst = y_radius * angle.sin() + y_pivot;

        // compute distance
        let dx = x_origin - x_dst;
        let dy = y_origin - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance != 0.0 {
            max_distance / distance
        } else {
            0.0
        };

        // compute final x/y
        let x = x_origin - dx * distance_ratio;
        let x = if dx > 0.0 { x.max(x_dst) } else { x.min(x_dst) };
        let y = y_origin - dy * distance_ratio;
        let y = if dy > 0.0 { y.max(y_dst) } else { y.min(y_dst) };

        // start rotating the formation angle only when sprite is on or close to ellipse
        if distance < max_distance * formation.speed / 20.0 {
            formation.angle = angle;
        }

        let translation = &mut enemy_tf.translation;
        (translation.x, translation.y) = (x, y);
    }
}
