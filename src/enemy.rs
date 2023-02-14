use bevy::prelude::*;
use rand::{Rng, thread_rng};
use crate::{ENEMY_MAX, ENEMY_SIZE, EnemyCount, GameTextures, SPRITE_SCALE, WinSize};
use crate::components::{Enemy, SpriteSize};


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_spawn_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {
    if enemy_count.0 >= ENEMY_MAX { return };
    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - 100.;
    let h_span = win_size.h / 2. - 100.;
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);

    commands.spawn(SpriteBundle {
        texture: textures.enemy.clone(),
        transform: Transform {
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            translation: Vec3::new(x, y, 0.),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Enemy)
        .insert(SpriteSize::from(ENEMY_SIZE));

    enemy_count.0 += 1;
}
