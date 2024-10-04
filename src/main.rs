use bevy::{color::palettes::css::GOLDENROD, prelude::*, time::common_conditions::on_timer};
use bevy_tween::{interpolate::translation, prelude::*};
use rand::random;
use std::f32::consts::TAU;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, shake_cam.run_if(on_timer(Duration::from_secs(1))))
        .run();
}

#[derive(Component)]
struct MainCam;

fn setup(mut commands: Commands) {
    commands.spawn((MainCam, Camera2dBundle::default()));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            color: GOLDENROD.into(),
            ..default()
        },
        ..default()
    });
}

fn shake_cam(mut commands: Commands, cam: Query<Entity, With<MainCam>>) {
    let cam = cam.single();

    let mut cam_commands = commands.entity(cam);
    let cam_target = cam.into_target();
    let shake_strength = 20.0;
    let shake_dir = Vec2::from_angle(random::<f32>() * TAU) * shake_strength;
    cam_commands.animation().insert_tween_here(
        Duration::from_millis(300),
        EaseFunction::BackOut,
        cam_target.with(translation(shake_dir.extend(0.0), Vec3::ZERO)),
    );
}
