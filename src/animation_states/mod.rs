use bevy::{prelude::*, utils::HashMap};
use rand::prelude::*;

use crate::window::{RequestWindowRelativeMove, RequestWindowResize};

use self::state_machine::AnimationStateMachine;

mod cave_chaos;
pub mod state_machine;

pub struct AnimationStatesPlugin;

impl Plugin for AnimationStatesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(state_machine::AnimationStateMachine::new())
            .add_event::<RequestNextState>()
            .add_startup_system(load_sprites)
            .add_system(advance_state);
    }
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct AnimationTimer(Timer);

#[derive(Component, Debug)]
pub struct AnimationHandlesContainer(HashMap<String, Handle<TextureAtlas>>);

fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    animation_state_machine: Res<state_machine::AnimationStateMachine>,
    mut request_window_resize: EventWriter<RequestWindowResize>,
    mut request_window_move: EventWriter<RequestWindowRelativeMove>,
) {
    let mut handles = HashMap::new();

    for (state_name, animation_state) in animation_state_machine.get_states().iter() {
        let texture_handle = asset_server.load(animation_state.sprite_sheet.file_name.to_string());
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(animation_state.size[0], animation_state.size[1]),
            animation_state.sprite_sheet.columns,
            animation_state.sprite_sheet.rows,
            Some(Vec2::new(2.0, 1.0)),
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        handles.insert(state_name.clone(), texture_atlas_handle);
    }

    let initial_state = animation_state_machine.get_current_state();
    let initial_handle = handles.get(&animation_state_machine.current_state).unwrap();

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: initial_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(
            initial_state.sprite_sheet.frame_duration,
            TimerMode::Repeating,
        )),
    ));

    commands.spawn(AnimationHandlesContainer(handles));

    // request the initial window move and resize
    request_window_resize.send(RequestWindowResize);
    request_window_move.send(RequestWindowRelativeMove);
}

/// event to request the next animation state
pub struct RequestNextState;

/// advance the animation state machine
fn advance_state(
    mut animation_state_machine: ResMut<state_machine::AnimationStateMachine>,
    mut next_state_request: EventReader<RequestNextState>,
    mut request_window_resize: EventWriter<RequestWindowResize>,
    mut request_window_move: EventWriter<RequestWindowRelativeMove>,
    query: Query<&mut Handle<TextureAtlas>>,
    animation_handles_container: Query<&AnimationHandlesContainer>,
) {
    if next_state_request.iter().count() > 0 {
        animation_state_machine.advance_state();

        request_window_resize.send(RequestWindowResize);
        request_window_move.send(RequestWindowRelativeMove);

        swap_animation_handle(animation_state_machine, query, animation_handles_container);
    }
}

fn swap_animation_handle(
    mut animation_state_machine: ResMut<AnimationStateMachine>,
    mut query: Query<&mut Handle<TextureAtlas>>,
    animation_handles_container: Query<&AnimationHandlesContainer>,
) {
    let current_state_name = &animation_state_machine.current_state;
    let current_state = animation_state_machine.get_current_state();

    let current_animation_handle = animation_handles_container
        .single()
        .0
        .get(current_state_name)
        .unwrap();

    if current_state.translate.is_some() {
        // if the current state requires a translation, then randomly decide if it should be flipped
        let mut rng = rand::thread_rng();
        // the flip is biased towards keeping the same direction
        let flip_direction = rng.gen_range(0..100) > 60;

        animation_state_machine.is_heading_right = match animation_state_machine.is_heading_right {
            true => !flip_direction,
            false => flip_direction,
        };
    }
    let mut handle = query.single_mut();

    *handle = current_animation_handle.clone();
}
