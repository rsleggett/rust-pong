use bevy::{math::bounding::{
    Aabb2d,
    BoundingCircle,
    BoundingVolume,
    IntersectsVolume
}, prelude::*};

use crate::components::{Ball, Paddle, Position, Shape, Velocity};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top, 
    Bottom,
}

fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) { return None; }

    let closest_point = wall.closest_point(ball.center());
    let offset = ball.center() - closest_point;

    let side = 
        if offset.x.abs() > offset.y.abs() {
            if offset.x < 0. {
                Collision::Left
            }
            else {
                Collision::Right
            }
        } else if offset.y > 0. {
            Collision::Top
        } else {
            Collision::Bottom
        };

    Some(side)
}

pub fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape, Option<&Velocity>, Option<&Paddle>), Without<Ball>>,
) {
    if let Ok((
        mut ball_velocity,
        ball_position,
        ball_shape
    )) = ball.get_single_mut() {
        for (position, shape, velocity, paddle) in &other_things {
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x), 
                Aabb2d::new(position.0, shape.0 / 2.)
            ) {
                //println!("{:?} {:?} {:?}", ball_position.0, position.0, shape.0);
                let mut paddle_modifier = 1.;

                //if let Some(_paddle) = paddle  {
                //    paddle_modifier = velocity.unwrap().0.y * 2.; 
                //};

                match collision {
                    Collision::Left => {
                        ball_velocity.0.x *= -1.1 * paddle_modifier;
                    },
                    Collision::Right => {
                        ball_velocity.0.x *= -1.1 * paddle_modifier;
                    },
                    Collision::Top => {
                        ball_velocity.0.y *= -1.1 * paddle_modifier;
                    },
                    Collision::Bottom => {
                        ball_velocity.0.y *= -1.1 * paddle_modifier;
                    }
                }
            }
        }
    }
}