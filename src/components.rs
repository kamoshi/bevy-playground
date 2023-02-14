use bevy::prelude::*;


#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(value: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(value.0, value.1))
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;
