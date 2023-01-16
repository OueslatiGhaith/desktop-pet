use bevy::prelude::*;

use crate::{
    animation_states::{state_machine::AnimationStateMachine, AnimationTimer, RequestNextState},
    window::RequestWindowRelativeMove,
};

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate);
    }
}

fn animate(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut request_next_state: EventWriter<RequestNextState>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    animation_state_machine: Res<AnimationStateMachine>,
    mut request_window_move: EventWriter<RequestWindowRelativeMove>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            if sprite.index + 1 == texture_atlas.textures.len() {
                // swapping to the next state
                sprite.index = 0;
                request_next_state.send(RequestNextState);
            } else {
                // advancing to the next frame in the current state
                sprite.index += 1;
                sprite.flip_x = !animation_state_machine.is_heading_right;

                let current_state = animation_state_machine.get_current_state();
                if current_state.translate.is_some() {
                    // current frame requires a window move
                    request_window_move.send(RequestWindowRelativeMove)
                }
            }
        }
    }
}
