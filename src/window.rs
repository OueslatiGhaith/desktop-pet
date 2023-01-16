use bevy::{prelude::*, window::WindowResizeConstraints};
use bevy_assets_bundler::BundledAssetIoPlugin;

use crate::{animation_states::state_machine::AnimationStateMachine, r#mod::BUNDLE_OPTIONS};

const CLEAR_COLOR: ClearColor = ClearColor(Color::NONE);

pub struct CustomWindowPlugin;

impl Plugin for CustomWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            // CLEAR_COLOR must have 0 alpha, otherwise some color will bleed through
            .insert_resource(CLEAR_COLOR)
            .add_plugins(
                DefaultPlugins
                    // prevent blurry sprites
                    .set(ImagePlugin::default_nearest())
                    // transparent window
                    .set(WindowPlugin {
                        window: WindowDescriptor {
                            title: "Desktop-Pet".to_string(),
                            transparent: true,
                            decorations: false,
                            // override minimum window size to allow for small sprites
                            resize_constraints: WindowResizeConstraints {
                                min_width: 10.0,
                                min_height: 10.0,
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .build()
                    // configure asset bundler
                    .add_before::<bevy::asset::AssetPlugin, _>(BundledAssetIoPlugin::from(
                        BUNDLE_OPTIONS.clone(),
                    )),
            )
            .add_event::<RequestWindowResize>()
            .add_event::<RequestWindowRelativeMove>()
            .add_startup_system(setup)
            .add_system(resize_window)
            .add_system(move_window);
    }
}

// sets up the initial window configuration in accordance with the state machine
fn setup(state_machine: Res<AnimationStateMachine>, mut windows: ResMut<Windows>) {
    let position = &state_machine.position;
    let size = state_machine.get_current_state().size;

    let window = windows.get_primary_mut().unwrap();
    window.set_position(MonitorSelection::Primary, position.clone());
    window.set_resolution(size[0], size[1]);
}

/// event to request a window resize
pub struct RequestWindowResize;
/// event to request a window move relative to its current position
pub struct RequestWindowRelativeMove;

fn resize_window(
    mut windows: ResMut<Windows>,
    mut event: EventReader<RequestWindowResize>,
    animation_state_machine: ResMut<AnimationStateMachine>,
) {
    for _ in event.iter() {
        let current_state = animation_state_machine.get_current_state();
        let size = current_state.size;
        let window = windows.get_primary_mut().unwrap();
        window.set_resolution(size[0], size[1]);
    }
}

fn move_window(
    mut windows: ResMut<Windows>,
    mut event: EventReader<RequestWindowRelativeMove>,
    mut animation_state_machine: ResMut<AnimationStateMachine>,
) {
    for _ in event.iter() {
        let window = windows.get_primary_mut().unwrap();

        let current_state = animation_state_machine.get_current_state();
        let is_heading_right = &animation_state_machine.is_heading_right;
        let current_position = &animation_state_machine.position;
        let translation = current_state.translate.unwrap_or([0, 0]);

        let new_position = match is_heading_right {
            false => IVec2::new(
                current_position.x - translation[0],
                current_position.y + translation[1],
            ),
            true => IVec2::new(
                current_position.x + translation[0],
                current_position.y + translation[1],
            ),
        };

        let new_window_position = {
            let offset = current_state.offset;

            match is_heading_right {
                false => IVec2::new(
                    new_position.x + offset[0] / 2,
                    new_position.y + offset[1] / 2,
                ),
                true => IVec2::new(
                    new_position.x + offset[0] / 2,
                    new_position.y + offset[1] / 2,
                ),
            }
        };

        animation_state_machine.position = new_position;
        window.set_position(MonitorSelection::Primary, new_window_position);
    }
}
