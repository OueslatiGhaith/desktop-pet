use bevy::{prelude::*, window::WindowResizeConstraints};

use crate::animation_states::state_machine::AnimationStateMachine;

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
                            transparent: true,
                            decorations: false,
                            height: 50.0,
                            width: 50.0,
                            resize_constraints: WindowResizeConstraints {
                                min_width: 10.0,
                                min_height: 10.0,
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    }),
            )
            .add_event::<RequestWindowResize>()
            .add_event::<RequestWindowRelativeMove>()
            .add_system(resize_window)
            .add_system(move_window);
    }
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
    animation_state_machine: ResMut<AnimationStateMachine>,
) {
    for _ in event.iter() {
        let window = windows.get_primary_mut().unwrap();

        let current_state = animation_state_machine.get_current_state();
        let new_position = {
            let translation = current_state.translate.unwrap_or([0, 0]);
            let offset = current_state.offset;

            let old_position = window.position().unwrap();
            IVec2::new(
                old_position.x + translation[0] + offset[0],
                old_position.y + translation[1] + offset[1],
            )
        };

        window.set_position(MonitorSelection::Primary, new_position);
    }
}
