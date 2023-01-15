use bevy::prelude::*;

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
pub struct RequestWindowResize(pub f32, pub f32);
/// event to request a window move relative to its current position
pub struct RequestWindowRelativeMove(pub IVec2);

fn resize_window(mut windows: ResMut<Windows>, mut event: EventReader<RequestWindowResize>) {
    for event in event.iter() {
        let window = windows.get_primary_mut().unwrap();
        window.set_resolution(event.0, event.1);
    }
}

fn move_window(mut windows: ResMut<Windows>, mut event: EventReader<RequestWindowRelativeMove>) {
    for event in event.iter() {
        let window = windows.get_primary_mut().unwrap();
        let position = window.position().unwrap();
        window.set_position(
            MonitorSelection::Primary,
            IVec2::new(position.x + event.0.x, position.y + event.0.y),
        );
    }
}
