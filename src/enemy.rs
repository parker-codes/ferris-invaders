use crate::{
    components::{Enemy, SpriteSize},
    EnemyCount, GameTextures, WindowSize, ENEMY_SIZE, MAX_ENEMIES, SPRITE_SCALE,
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_spawn_system);
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
