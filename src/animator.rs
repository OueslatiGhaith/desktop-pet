use bevy::prelude::*;

use crate::animation_states::{AnimationTimer, RequestNextState};

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
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            if sprite.index + 1 == texture_atlas.textures.len() {
                sprite.index = 0;
                println!("requesting next state");
                request_next_state.send(RequestNextState);
            } else {
                sprite.index += 1
            }
        }
    }
}
