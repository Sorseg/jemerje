mod merge_lines;

use bevy::prelude::*;
use bevy_tween::{interpolate::translation, prelude::*};
use rand::{random, thread_rng, Rng};
use std::f32::consts::TAU;
use std::iter::Iterator;


pub enum MergeLine {
    SWEET,
    SALTY,
}

static MERGE_ITEM_ICON_PATHS: &str = include_str!("../assets/icons.txt");
static ITEM_ICONS_COUNT: usize = 101;

static EMPTY_ICON: &str = "cells/empty.png";

const BOARD_WIDTH: usize = 16;
const BOARD_HEIGHT: usize = 9;
const SPRITE_SIZE_PX: i32 = 32;

const PADDING: i32 = 4;

const CELL_SIZE: i32 = SPRITE_SIZE_PX + PADDING;

fn main() {
    let mut app = App::new();
    // define 2d array:
    let board = vec![vec![None; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize];

    app.add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .insert_resource(Board(board))
        .add_systems(Startup, setup)

        .observe(spawn_sprites_for_merge_items)
        .run();
}

/// Board is a 2d Array of cells. Cells can be empty or contain a merge item.
/// board -> cell -> item or null
#[derive(Resource)]
struct Board(Vec<Vec<Cell>>);

/// a piece of board, either empty or with a merge item
type Cell = Option<Entity>;


#[derive(Component)]
struct MergableItem {
    x: usize,
    y: usize,
    tier: usize,
    merge_line: MergeLine,
}

#[derive(Component)]
struct MainCam;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut board: ResMut<Board>) {
    commands.spawn((MainCam, Camera2dBundle::default()));

    // spawn board background
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            commands.spawn(SpriteBundle {
                texture: asset_server.load("cells/empty.png"),
                transform: Transform::from_translation(idx_to_world_coordinates(x, y)),
                ..default()
            });
        }
    }

    board.0[2][2] = Some(commands.spawn(MergableItem {
        x: 2,
        y: 2,
        tier: 0,
        merge_line: MergeLine::SWEET,
    }).id());
}

fn idx_to_world_coordinates(x: usize, y: usize) -> Vec3 {
    Vec3 {
        x: (x * CELL_SIZE as usize) as f32 - (CELL_SIZE * BOARD_WIDTH as i32) as f32 / 2.0,
        y: (y * CELL_SIZE as usize) as f32 - (CELL_SIZE * BOARD_HEIGHT as i32) as f32 / 2.0,
        z: 0.0,
    }
}

fn spawn_merge_item(x: usize, y: usize, commands: &mut Commands, asset_server: &AssetServer) -> Entity {
    commands.spawn(SpriteBundle {
        texture: asset_server.load(select_sprite_to_spawn()),
        transform: Transform::from_xyz(
            (x * CELL_SIZE as usize) as f32,
            (y * CELL_SIZE as usize) as f32,
            0f32,
        ),
        ..default()
    }).id()
}

fn select_sprite_to_spawn() -> String {
    let random_index = thread_rng().gen_range(0..ITEM_ICONS_COUNT);
    format!(
        "icons/{}",
        MERGE_ITEM_ICON_PATHS.lines().nth(random_index).unwrap()
    )
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

// /// synchronise LOGICAL board with the ECS systems
// fn sync_sprites_with_board(mut board: ResMut<Board>, mut commands: Commands, asset_server: Res<AssetServer>) {
//     // iterate over board rows
//     for (y, line) in board.0.iter_mut().enumerate() {
//         // iterate over board columns
//         for (x, cell) in line.iter_mut().enumerate() {
//             // if item in cell
//             if let Some(item) = cell {
//                 // if item does not have a spawned entity:
//                 if item.entity.is_none() {
//                     item.entity = Some(spawn_merge_item_sprite(x, y, &mut commands, &asset_server));
//                 }
//             }
//         }
//     }
// }

fn spawn_sprites_for_merge_items(e: Trigger<OnAdd, MergableItem>, item: Query<&MergableItem>, mut commands: Commands, asset_server: Res<AssetServer>) {
    let item = item.get(e.entity()).unwrap();
    commands.entity(e.entity()).insert(
        SpriteBundle {
            texture: asset_server.load(select_sprite_to_spawn()),
            transform: Transform::from_translation(idx_to_world_coordinates(item.x, item.y)),
            ..default()
        }
    );
}
