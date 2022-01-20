use explore::*;
use bevy::{
    core::FixedTimestep, ecs::schedule::SystemSet, prelude::*, render::camera::CameraPlugin,
};
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Playing)
        .add_startup_system(setup_cameras)
        .add_system_set(SystemSet::on_enter(GameState::Playing)
            .with_system(setup))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player)
                .with_system(focus_camera)
                .with_system(rotate_bonus)
                .with_system(scoreboard_system),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(teardown))
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(display_score))
        .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(gameover_keyboard))
        .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(teardown))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(5.0))
                .with_system(spawn_bonus),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
