use bevy::prelude::*;
use bevy::math::Vec2;

#[derive(Component)]
pub struct PlayerScore;

#[derive(Component)]
pub struct AiScore;

pub enum Scorer { 
    Player, 
    Ai
}

#[derive(Component)]
pub struct Ball;


#[derive(Component)]
pub struct Paddle;


#[derive(Component)]
pub struct Boundary;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Shape(pub Vec2);

#[derive(Event)]
pub struct Scored(pub Scorer);

#[derive(Resource, Default)]
pub struct Score {
    pub player: u32,
    pub ai: u32
}

#[derive(Component)]
pub struct Gutter;

#[derive(Component)]
pub struct Ai;

#[derive(Component)]
pub struct Player;

