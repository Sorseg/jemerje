mod merge_lines;

use crate::merge_lines::{salty_merge_line, sweet_merge_line, MergeItemDescriptor};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_tween::{interpolate::translation, prelude::*};
use rand::{random, thread_rng, Rng};

fn main() {
    let mut app = App::new();
    // define 2d array:
    let board = vec![vec![None; BOARD_WIDTH]; BOARD_HEIGHT];

    app.add_plugins((DefaultPlugins, DefaultTweenPlugins, DefaultPickingPlugins))
        .insert_resource(Board(board))
        .insert_resource(MergeLines {
            sweet: sweet_merge_line(),
            salty: salty_merge_line(),
        })
        .add_systems(Startup, setup)
        .observe(spawn_sprites_for_merge_items)
        .run();
}

#[derive(Clone)]
pub enum MergeLine {
    SWEET,
    SALTY,
}

static EMPTY_ICON: &str = "cells/empty.png";

const BOARD_WIDTH: usize = 16;
const BOARD_HEIGHT: usize = 9;
const SPRITE_SIZE_PX: i32 = 32;

const PADDING: i32 = 4;

const CELL_SIZE: i32 = SPRITE_SIZE_PX + PADDING;

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut board: ResMut<Board>) {
    commands.spawn((MainCam, Camera2dBundle::default()));

    // Load and play background music
    let background_music = asset_server.load("apple_cider.ogg");
    commands.spawn(AudioBundle {
        source: background_music,
        ..default()
    });

    // todo: spawn a background image
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform::from_translation(Vec3::ZERO.with_z(-2f32))
            .with_scale(Vec3::new(3f32, 3f32, 3f32)),

        ..default()
    });

    // spawn board background
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            commands.spawn(SpriteBundle {
                texture: asset_server.load(EMPTY_ICON),
                transform: Transform::from_translation(idx_to_world_coordinates(x, y).with_z(-1.0)),
                ..default()
            });
        }
    }

    // spawn 5 random items
    for _ in 0..5 {
        let merge_line = if random::<f32>() < 0.5 {
            MergeLine::SWEET
        } else {
            MergeLine::SALTY
        };

        // select random position
        let x = thread_rng().gen_range(0..BOARD_WIDTH);
        let y = thread_rng().gen_range(0..BOARD_HEIGHT);

        // todo: check if the cell is empty

        board.0[y][x] = Some(
            commands
                .spawn(MergableItem {
                    x,
                    y,
                    tier: 0,
                    merge_line,
                })
                .id(),
        );
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
        MergeLine::SWEET => lines.sweet[item.tier].path.to_string(),
        MergeLine::SALTY => lines.salty[item.tier].path.to_string(),
    };
    format!("icons/{path}")
}

fn spawn_sprites_for_merge_items(
    e: Trigger<OnAdd, MergableItem>,
    item: Query<&MergableItem>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    merge_lines: Res<MergeLines>,
) {
    let item = item.get(e.entity()).unwrap();
    commands.entity(e.entity()).insert((
        SpriteBundle {
            texture: asset_server.load(select_sprite_to_spawn(item, &merge_lines)),
            transform: Transform::from_translation(idx_to_world_coordinates(item.x, item.y)),
            ..default()
        },
        On::<Pointer<Drag>>::target_component_mut(|event, component: &mut Transform| {
            component.translation.x += event.delta.x;
            component.translation.y -= event.delta.y;
        }),
        On::<Pointer<DragEnd>>::run(
            |event: Listener<Pointer<DragEnd>>,
             mut commands: Commands,
             item: Query<&MergableItem>,
             trans: Query<&Transform>| {
                let trans = trans.get(event.target).unwrap();
                let item = item.get(event.target).unwrap();
                let mut entity_cmd = commands.get_entity(event.target).unwrap();
                let target = entity_cmd.id().into_target();
                entity_cmd.animation().insert_tween_here(
                    Duration::from_millis(500),
                    EaseFunction::BackOut,
                    target.with(translation(
                        trans.translation,
                        idx_to_world_coordinates(item.x, item.y),
                    )),
                );
            },
        ),
    ));
}
