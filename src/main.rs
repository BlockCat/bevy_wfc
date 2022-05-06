use bevy::{prelude::*, window::{WindowMode, PresentMode}};
use camera::{CameraPlugin, CameraTag};
use ui::UIPlugin;
use wfc_asset::WfcAssetPlugin;
use world::WorldPlugin;

mod camera;
mod ui;
mod wfc_asset;
mod world;

#[derive(Debug, Component)]
struct TileTag;

/// This example shows how to configure Multi-Sample Anti-Aliasing. Setting the sample count higher
/// will result in smoother edges, but it will also increase the cost to render those edges. The
/// range should generally be somewhere between 1 (no multi sampling, but cheap) to 8 (crisp but
/// expensive).
/// Note that WGPU currently only supports 1 or 4 samples.
/// Ultimately we plan on supporting whatever is natively supported on a given device.
/// Check out [this issue](https://github.com/gfx-rs/wgpu/issues/1832) for more info.
fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(WfcAssetPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(WorldPlugin)
        .add_system(cycle_msaa)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "WFC".into(),
            width: 1080.0,
            height: 1920.0,
            mode: WindowMode::BorderlessFullscreen,
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands) {
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-10.0, 15.0, -10.0)
                .looking_at(Vec3::new(7.0, 0.0, 7.0), Vec3::Y),

            ..default()
        })
        .insert(CameraTag(Vec3::ZERO));
}

fn cycle_msaa(input: Res<Input<KeyCode>>, mut msaa: ResMut<Msaa>) {
    if input.just_pressed(KeyCode::M) {
        if msaa.samples == 4 {
            info!("Not using MSAA");
            msaa.samples = 1;
        } else {
            info!("Using 4x MSAA");
            msaa.samples = 4;
        }
    }
}
