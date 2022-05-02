use bevy::{asset, prelude::*};
use bevy_asset_ron::RonAssetPlugin;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use ui::UIPlugin;
use wfc_asset::{TileAsset, WfcAssetPlugin};

mod wfc_asset;
mod ui;

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
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(WfcAssetPlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(setup)
        .add_system(cycle_msaa)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let rekt: Handle<TileAsset> = asset_server.load("straight.tile");
    let rekt2: Handle<TileAsset> = asset_server.load("corner.tile");

    commands.spawn().insert(rekt);
    commands.spawn().insert(rekt2);

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-3.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FlyCamera::default());
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

