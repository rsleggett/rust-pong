use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::components::{Ai, Paddle, Player, Position, Velocity};
use crate::constants::{GUTTER_HEIGHT, PADDLE_HEIGHT, PADDLE_SPEED, PADDLE_WIDTH};
use crate::bundles::*;

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.height();
        let padding = 100.;
        let right_paddle_x = window_width - padding;
        let left_paddle_x = -(window_width) + padding;

        let player_paddle = PaddleBundle::new(right_paddle_x, 0.);
        let ai_paddle = PaddleBundle::new(left_paddle_x, 0.);

        let shape = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);
        let player_colour = Color::srgb(1., 0., 0.);
        let ai_colour = Color::srgb(0., 0., 1.);
        let mesh = meshes.add(shape);
    
        commands.spawn((
            Player,
            player_paddle,
            MaterialMesh2dBundle {
                mesh: mesh.clone().into(),
                material: materials.add(player_colour),
                ..default()
            },
        ));

        commands.spawn((
            Ai,
            ai_paddle,
            MaterialMesh2dBundle {
                mesh: mesh.into(),
                material: materials.add(ai_colour),
                ..default()
            },
        ));
    }
}

pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = paddle.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y = 1.;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -1.;
        } else {
            velocity.0.y = 0.;
        }
    }
}

pub fn move_paddles(
    mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
    window: Query<&Window>
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();
        let max_y = window_height / 2. - GUTTER_HEIGHT - PADDLE_HEIGHT / 2.;

        for (mut position, velocity) in &mut paddle {
            let new_position = position.0 + velocity.0 * PADDLE_SPEED;
            if (new_position.y.abs() < max_y) {
                position.0 = new_position;
            } 
        }
    }
}