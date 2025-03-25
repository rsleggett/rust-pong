use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::bundles::BallBundle;
use crate::components::{Ball, Position, Scored, Scorer, Velocity};
use crate::constants::{self, BALL_SPEED};

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball");

    let ball = BallBundle::new(BALL_SPEED, BALL_SPEED);

    let shape = Circle::new(ball.shape.0.x);
    let color = Color::srgb(0., 1., 0.);

    let mesh = meshes.add(shape);
    let material = materials.add(color);

    commands.spawn((
        ball, 
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material,
            ..default()
        }
    ));
}

pub fn move_ball(mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0
    }
}

pub fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>
) {
    for event in events.read() {
        if let Ok((
            mut position,
            mut velocity
        )) = ball.get_single_mut() {
            match event.0 {
                Scorer::Ai => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(-BALL_SPEED, BALL_SPEED);
                },
                Scorer::Player => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(BALL_SPEED, BALL_SPEED);
                }
            }
        }
    }
}