use bevy::prelude::*;
use crate::components::{Movable, Velocity};
use crate::player::PlayerPlugin;

mod player;
mod components;

const HEIGHT: f32 = 676.0;
const WIDTH: f32 = 598.0;
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);
const SPRITE_SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;


#[derive(Resource)]
pub struct WinSize {
    w: f32,
    h: f32,
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
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
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .run();
}


fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.height(), window.width());

    commands.insert_resource(WinSize {
        w: win_w,
        h: win_h,
    });

    commands.insert_resource(GameTextures {
        player: assets.load(PLAYER_SPRITE),
        player_laser: assets.load(PLAYER_LASER_SPRITE),
    });
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


