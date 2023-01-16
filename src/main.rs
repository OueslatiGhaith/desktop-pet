use bevy::prelude::*;

mod animation_states;
mod animator;
mod window;

mod r#mod;

fn main() {
    App::new()
        .add_plugin(window::CustomWindowPlugin)
        .add_plugin(animation_states::AnimationStatesPlugin)
        .add_plugin(animator::AnimatorPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
