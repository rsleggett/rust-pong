use bevy::prelude::*;
mod systems;
pub mod constants;
pub mod bundles;
pub mod components;
use components::{Score, Scored};
use systems::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(create_window()))
    .init_resource::<Score>()
    .add_event::<Scored>()
    .add_systems(Startup, (
        spawn_ball, 
        spawn_dotted_line, 
        spawn_camera, 
        spawn_paddles,
        spawn_gutters,
        spawn_scoreboard))
    .add_systems(Update, (
        move_ball,
        handle_player_input,
        detect_scoring,
        reset_ball.after(detect_scoring),
        update_score.after(detect_scoring),
        update_score_board.after(update_score),
        handle_collisions.after(move_ball),
        project_positions.after(move_ball),
        move_paddles.after(handle_player_input),
        move_ai,
    ))
    .run();
}

