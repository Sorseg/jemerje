use bevy::{color::palettes::css::GOLDENROD, prelude::*};
use bevy_tween::{interpolate::translation, prelude::*};
use rand::random;
use std::f32::consts::TAU;

static ICONS: &str = include_str!("../assets/icons.txt");

const BOARD_WIDTH: i32 = 16;
const BOARD_HEIGHT: i32 = 9;
const SPRITE_SIZE_PX: i32 = 32;

const PADDING: i32 = 4;

const CELL_SIZE: i32 = SPRITE_SIZE_PX + PADDING;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, DefaultTweenPlugins)).add_systems(Startup, setup)
        // .add_systems(Update, shake_cam.run_if(on_timer(Duration::from_secs(1))))
        .run();
}

#[derive(Component)]
struct MainCam;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((MainCam, Camera2dBundle::default()));

    // spawn width by height board of sprites
    for x in -BOARD_WIDTH/2..BOARD_WIDTH/2 {
        for y in -BOARD_HEIGHT/2..BOARD_HEIGHT/2 {
            commands.spawn(SpriteBundle {
                texture: asset_server.load(select_sprite_to_spawn()),
                transform: Transform::from_xyz((x * CELL_SIZE) as f32,
                                               (y * CELL_SIZE) as f32,
                                               0f32),
                ..default()
            });
        }
    }
}

fn select_sprite_to_spawn() -> String {
    format!("icons/{}", ICONS.lines().nth(15).unwrap())
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
