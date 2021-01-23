use std::path::PathBuf;

use bevy::{prelude::*, render::camera::Camera};
use bevy_ldtk::{LdtkMapBundle, LdtkMapConfig, LdtkPlugin, MapScale};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup.system())
        .add_system(camera_movement.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(LdtkMapBundle {
            map: asset_server.load(PathBuf::from(
                &std::env::args()
                    .skip(1)
                    .next()
                    .unwrap_or("map1.ldtk".into()),
            )),
            scale: MapScale(0.),
            transform: Transform::default(),
            config: LdtkMapConfig {
                set_clear_color: true,
            },
        })
        // .spawn(SpriteBundle {
        //     material: materials.add(
        //         asset_server
        //             .load("map1/png/0000-00-IntGrid_layer.png")
        //             .into(),
        //     ),
        //     ..Default::default()
        // })
        .spawn(Camera2dBundle::default());
}

const SPEED: f32 = 1.0;

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::zero();
        let scale = transform.scale.x;

        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-SPEED, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(SPEED, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, SPEED, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -SPEED, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Q) {
            let scale = scale + 0.005;
            transform.scale = Vec3::new(scale, scale, 1.);
        }

        if keyboard_input.pressed(KeyCode::E) {
            let scale = scale - 0.005;
            transform.scale = Vec3::new(scale, scale, 1.);
        }

        transform.translation += time.delta_seconds() * direction * 1000.;
    }
}
