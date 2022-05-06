use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Debug, Component)]
pub struct CameraTag(pub Vec3);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(move_camera);
    }
}

fn move_camera(input: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &CameraTag)>) {
    // if input.
}
