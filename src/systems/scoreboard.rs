use bevy::prelude::*;

use crate::components::{AiScore, Ball, PlayerScore, Position, Score, Scored, Scorer};

pub fn detect_scoring(
    mut ball: Query<&mut Position, With<Ball>>,
    window: Query<&Window>,
    mut events: EventWriter<Scored>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();

        if let Ok(ball) = ball.get_single_mut() {
            if ball.0.x > window_width / 2. {
                events.send(Scored(Scorer::Ai));
            } else if ball.0.x < -window_width / 2. {
                events.send(Scored(Scorer::Player));
            }
        }
    }
}

pub fn update_score(
    mut score: ResMut<Score>,
    mut events: EventReader<Scored>
) {
    for event in events.read() {
        match event.0 {
            Scorer::Ai => score.ai += 1,
            Scorer::Player => score.player += 1
        }
    }

    //println!("Score: {} - {}", score.player, score.ai);
}

pub fn update_score_board(
    mut player_score: Query<&mut Text, With<PlayerScore>>,
    mut ai_score: Query<&mut Text, (With<AiScore>, Without<PlayerScore>)>,
    score: Res<Score>
) {
    if (score.is_changed()) {
        if let Ok(mut player_score) = player_score.get_single_mut() {
            player_score.sections[0].value = score.player.to_string();
        }

        if let Ok(mut ai_score) = ai_score.get_single_mut() {
            ai_score.sections[0].value = score.ai.to_string();
        }
    }
}

pub fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 72.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            right: Val::Px(15.),
            ..default()
        }),
        PlayerScore
    ));

    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 72.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(15.),
            ..default()
        }),
        AiScore
    ));
}