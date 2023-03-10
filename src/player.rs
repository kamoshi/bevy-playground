use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::time::FixedTimestep;
use bevy::utils::HashSet;
use crate::{EnemyCount, GameTextures, PLAYER_LASER_SIZE, PLAYER_RESPAWN_DELAY, PLAYER_SIZE, PlayerState, SPRITE_SCALE, WinSize};
use crate::components::{Enemy, ExplosionToSpawn, FromPlayer, Laser, Movable, Player, SpriteSize, Velocity};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerState::default())
            .add_system_set(SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(player_spawn_system))
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system)
            .add_system(player_laser_hit_enemy_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let now = time.elapsed_seconds_f64();
    let last_shot = player_state.last_shot;
    if !player_state.on && (last_shot == -1. || now > last_shot + PLAYER_RESPAWN_DELAY) {
        let bottom = -win_size.h / 2.;
        commands.spawn(SpriteBundle {
            texture: textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
            .insert(Player)
            .insert(SpriteSize::from(PLAYER_SIZE))
            .insert(Movable { auto_despawn: false })
            .insert(Velocity { x: 0., y: 0. });

        player_state.spawned();
    };
}


fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else { 0. }
    }
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>
) {
    if kb.just_pressed(KeyCode::Space) {
        if let Ok(plr_tf) = query.get_single() {
            let (x, y) = (plr_tf.translation.x, plr_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;

            let mut spawn_laser = |x_offset: f32| {
                commands.spawn(SpriteBundle {
                    texture: textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x + x_offset, y + 20., 0.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity { x: 0., y: 1. });
            };
            spawn_laser(x_offset);
            spawn_laser(-x_offset)
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    let mut despawned = HashSet::<Entity>::new();
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        if despawned.contains(&laser_entity) {continue};
        let laser_scale = Vec2::from(laser_tf.scale.xy());
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            if despawned.contains(&enemy_entity) || despawned.contains(&laser_entity) {continue};
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();
                commands.entity(laser_entity).despawn();
                despawned.insert(enemy_entity);
                despawned.insert(laser_entity);
                commands.spawn_empty().insert(ExplosionToSpawn(enemy_tf.translation.clone()));
                enemy_count.0 -= 1;
            }
        }
    }
}
