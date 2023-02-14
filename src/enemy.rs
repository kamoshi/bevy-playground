use std::f32::consts::PI;
use bevy::ecs::schedule::ShouldRun;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::time::FixedTimestep;
use rand::{Rng, thread_rng};
use crate::{ENEMY_LASER_SIZE, ENEMY_MAX, ENEMY_SIZE, EnemyCount, GameTextures, PlayerState, SPRITE_SCALE, WinSize};
use crate::components::{Enemy, ExplosionToSpawn, FromEnemy, Laser, Movable, Player, SpriteSize, Velocity};


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(enemy_spawn_system))
            .add_system_set(SystemSet::new()
                .with_run_criteria(enemy_fire_criteria)
                .with_system(enemy_fire_system))
            .add_system(enemy_laser_hit_player);
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

fn enemy_fire_criteria() -> ShouldRun {
    match thread_rng().gen_bool(1. / 60.) {
        true => ShouldRun::Yes,
        false => ShouldRun::No,
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>
) {
    for &tf in enemy_query.iter() {
        let (x, y) = (tf.translation.x, tf.translation.y);
        commands.spawn(SpriteBundle {
            texture: textures.enemy_laser.clone(),
            transform: Transform {
                translation: Vec3::new(x, y - 15., 0.),
                rotation: Quat::from_rotation_x(PI),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 0.),
                ..Default::default()
            },
            ..Default::default()
        })
            .insert(Laser)
            .insert(FromEnemy)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(Velocity { x: 0., y: -1. })
            .insert(Movable { auto_despawn: true });
    }
}

fn enemy_laser_hit_player(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>
) {
    if let Ok((player_entity, player_tf, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_tf.scale.xy());

        for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
            let laser_scale = Vec2::from(laser_tf.scale.xy());
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                player_tf.translation,
                player_size.0 * player_scale,
            );
            if let Some(_) = collision {
                commands.entity(player_entity).despawn();
                commands.entity(laser_entity).despawn();
                commands.spawn_empty().insert(ExplosionToSpawn(player_tf.translation.clone()));
                player_state.shot(time.elapsed_seconds_f64());
                break;
            }
        }
    }
}
