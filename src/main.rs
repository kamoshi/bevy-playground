use bevy::prelude::*;
use crate::components::{Explosion, ExplosionTimer, ExplosionToSpawn, Movable, Velocity};
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;

mod components;
mod player;
mod enemy;

const HEIGHT: f32 = 676.0;
const WIDTH: f32 = 598.0;
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (9., 54.);
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const SPRITE_SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const EXPLOSION_LEN: usize = 16;
const ENEMY_MAX: u32 = 3;
const PLAYER_RESPAWN_DELAY: f64 = 1.0;

#[derive(Resource)]
pub struct WinSize {
    w: f32,
    h: f32,
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct EnemyCount(u32);

#[derive(Resource)]
pub struct PlayerState {
    on: bool,
    last_shot: f64,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self { on: false, last_shot: -1., }
    }
}

impl PlayerState {
    pub fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
    }
    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.;
    }
}


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Ferris Shooter".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(explosion_to_spawn_system)
        .add_system(explosion_animation_system)
        .run();
}


fn setup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    assets: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.height(), window.width());

    commands.insert_resource(WinSize { w: win_w, h: win_h, });

    let explosion_handle = assets.load(EXPLOSION_SHEET);
    let explosion_atlas = TextureAtlas::from_grid(
        explosion_handle,
        Vec2::new(64., 64.),
        4, 4, None, None,
    );

    commands.insert_resource(GameTextures {
        player: assets.load(PLAYER_SPRITE),
        player_laser: assets.load(PLAYER_LASER_SPRITE),
        enemy: assets.load(ENEMY_SPRITE),
        enemy_laser: assets.load(ENEMY_LASER_SPRITE),
        explosion: atlases.add(explosion_atlas),
    });

    commands.insert_resource(EnemyCount(0));
}

fn movable_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Velocity, &Movable)>,
    win_size: Res<WinSize>,
) {
    for (entity, mut transform, velocity, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            const MARGIN: f32 = 200.;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN {
                commands.entity(entity).despawn();
            }
        }
    }
}


fn explosion_to_spawn_system(
    mut commands: Commands,
    textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>
) {
    for (spawn_entity, explosion_spawn) in query.iter() {
        commands.spawn(SpriteSheetBundle {
            texture_atlas: textures.explosion.clone(),
            transform: Transform {
                translation: explosion_spawn.0,
                ..Default::default()
            },
            ..Default::default()
        })
            .insert(Explosion)
            .insert(ExplosionTimer::default());
        commands.entity(spawn_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1;

            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}
