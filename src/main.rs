mod merge_lines;

use crate::merge_lines::{salty_merge_line, sweet_merge_line, MergeItemDescriptor};
use bevy::prelude::*;
use bevy_tween::{interpolate::translation, prelude::*};
use rand::{random, thread_rng, Rng};
use std::f32::consts::TAU;
use std::iter::Iterator;

#[derive(Clone)]
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
    let board = vec![vec![None; BOARD_WIDTH]; BOARD_HEIGHT];

    app.add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .insert_resource(Board(board))
        .insert_resource(MergeLines {
            sweet: sweet_merge_line(),
            salty: salty_merge_line(),
        })
        .add_systems(Startup, setup)

        .observe(spawn_sprites_for_merge_items)
        .run();
}

#[derive(Resource)]
struct MergeLines {
    sweet: Vec<MergeItemDescriptor>,
    salty: Vec<MergeItemDescriptor>,
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

fn setup(mut commands: Commands,
         asset_server: Res<AssetServer>,
         mut board: ResMut<Board>,
         merge_lines: Res<MergeLines>,
) {
    commands.spawn((MainCam, Camera2dBundle::default()));

    // spawn board background
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            commands.spawn(SpriteBundle {
                texture: asset_server.load("cells/empty.png"),
                transform: Transform::from_translation(idx_to_world_coordinates(x, y).with_z(-1.0)),
                ..default()
            });
        }
    }

    // spawn 5 random items
    for i in 0..5 {
        let merge_line = if random::<f32>() < 0.5 {
            MergeLine::SWEET
        } else {
            MergeLine::SALTY
        };

        // select random position
        let x = thread_rng().gen_range(0..BOARD_WIDTH);
        let y = thread_rng().gen_range(0..BOARD_HEIGHT);


        // todo: check if the cell is empty

        board.0[y][x] = Some(commands.spawn(MergableItem {
            x,
            y,
            tier: 0,
            merge_line,
        }).id());
    }
}

fn idx_to_world_coordinates(x: usize, y: usize) -> Vec3 {
    Vec3 {
        x: (x * CELL_SIZE as usize) as f32 - (CELL_SIZE * BOARD_WIDTH as i32) as f32 / 2.0,
        y: (y * CELL_SIZE as usize) as f32 - (CELL_SIZE * BOARD_HEIGHT as i32) as f32 / 2.0,
        z: 0.0,
    }
}

fn select_sprite_to_spawn(item: &MergableItem, lines: &MergeLines) -> String {
    // merge_line -> tier -> image path
    let path = match item.merge_line {
        MergeLine::SWEET => {
            lines.sweet[item.tier].path.to_string()
        }
        MergeLine::SALTY => {
            lines.salty[item.tier].path.to_string()
        }
    };
    format!("icons/{path}")
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

fn spawn_sprites_for_merge_items(
    e: Trigger<OnAdd, MergableItem>,
    item: Query<&MergableItem>,
    mut commands: Commands, asset_server: Res<AssetServer>,
    merge_lines: Res<MergeLines>
) {
    let item = item.get(e.entity()).unwrap();
    commands.entity(e.entity()).insert(
        SpriteBundle {
            texture: asset_server.load(select_sprite_to_spawn(item, &merge_lines)),
            transform: Transform::from_translation(idx_to_world_coordinates(item.x, item.y)),
            ..default()
        }
    );
}
