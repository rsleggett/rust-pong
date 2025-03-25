use bevy::prelude::*;
use crate::components::Position;

pub fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
        // println!("{}", transform.translation.to_string());
    }
}