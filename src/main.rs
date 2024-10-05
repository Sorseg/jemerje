mod merge_lines;

use bevy::audio::Volume;
use crate::merge_lines::{salty_merge_line, sweet_merge_line};
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
        .insert_resource(ClearColor(Color::srgb(0.53, 0.76, 0.96)))
        .add_systems(Startup, setup)
        .observe(spawn_sprites_for_merge_items)
        .run();
}

#[derive(Clone, PartialEq, Eq)]
pub enum MergeLine {
    SWEET,
    SALTY,
}

static EMPTY_ICON: &str = "cells/empty.png";

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;
const SPRITE_SIZE_PX: u32 = 32;

const PADDING: i32 = 4;

const STARTING_ITEMS: i32 = 16;

const CELL_SIZE: u32 = SPRITE_SIZE_PX.saturating_add_signed(PADDING);
const BOARD_WORLD_WIDTH: f32 = CELL_SIZE as f32 * BOARD_WIDTH as f32;
const BOARD_WORLD_HEIGHT: f32 = CELL_SIZE as f32 * BOARD_HEIGHT as f32;

#[derive(Resource)]
struct MergeLines {
    sweet: Vec<&'static str>,
    salty: Vec<&'static str>,
}

/// Board is a 2d Array of cells. Cells can be empty or contain a merge item.
/// board -> cell -> item or null
#[derive(Resource)]
struct Board(Vec<Vec<Cell>>);

/// a piece of board, either empty or with a merge item
type Cell = Option<Entity>;

#[derive(Component, Clone)]
struct MergableItem {
    x: usize,
    y: usize,
    tier: usize,
    merge_line: MergeLine,
}

impl MergableItem {
    fn can_be_merged_with(&self, other: &Self) -> bool {
        self.tier == other.tier && self.merge_line == other.merge_line && !(self.x == other.x && self.y == other.y)
    }
}

#[derive(Component)]
struct MainCam;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut board: ResMut<Board>) {
    commands.spawn((MainCam, Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(1.0, 1.0, 1.0), // fixme: clicking and drag coordinates break when this is != 1.0
            ..default()
        },
        ..default()
    }));

    // Load and play background music
    let background_music = asset_server.load("apple_cider.ogg");
    commands.spawn(AudioBundle {
        source: background_music,
        ..default()
    });

    let background_scaling = 3f32;
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform::from_translation(Vec3::ZERO.with_z(-2f32))
            .with_scale(Vec3::new(background_scaling, background_scaling, background_scaling)),

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

    let mut spawned = 0;
    
    // spawn random items
    while spawned < STARTING_ITEMS {
        let merge_line = if random::<f32>() < 0.5 {
            MergeLine::SWEET
        } else {
            MergeLine::SALTY
        };

        // select random position
        let x = thread_rng().gen_range(0..BOARD_WIDTH);
        let y = thread_rng().gen_range(0..BOARD_HEIGHT);

        // todo: check if the cell is empty
        if board.0[y][x].is_some() {
            continue
        };
        
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
        spawned += 1
    }
}

fn idx_to_world_coordinates(x: usize, y: usize) -> Vec3 {
    Vec3 {
        x: (x * CELL_SIZE as usize) as f32 - BOARD_WORLD_WIDTH / 2.0,
        y: (y * CELL_SIZE as usize) as f32 - BOARD_WORLD_HEIGHT / 2.0,
        z: 0.0,
    }
}

fn world_coordinates_to_idx(c: Vec3) -> Option<(usize, usize)> {
    // + 0.5 for proper rounding when converting to usize
    let x = (c.x + BOARD_WORLD_WIDTH / 2.0) / CELL_SIZE as f32 + 0.5;
    let y = (c.y + BOARD_WORLD_HEIGHT / 2.0) / CELL_SIZE as f32 + 0.5;
    
    if x < 0.0 || x >= BOARD_WIDTH as f32 {
        return None;
    }
    if y < 0.0 || y >= BOARD_HEIGHT as f32 {
        return None;
    }

    Some((x as usize, y as usize))
}

fn select_sprite_to_spawn(item: &MergableItem, lines: &MergeLines) -> String {
    // merge_line -> tier -> image path
    let line = match item.merge_line {
        MergeLine::SWEET => &lines.sweet,
        MergeLine::SALTY => &lines.salty,
    };
    let path = line[item.tier].to_string();
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
        On::<Pointer<DragEnd>>::run(merge_or_snap_back),
    ));
}

fn merge_or_snap_back(
    event: Listener<Pointer<DragEnd>>,
    mut commands: Commands,
    items: Query<&MergableItem>,
    trans: Query<&Transform>,
    mut board: ResMut<Board>,
    asset_server: Res<AssetServer>,
) {
    let dropped_entity = event.target;
    let dropped_item = items.get(dropped_entity).unwrap();
    let dropped_pos = trans.get(event.target).unwrap().translation;

    if let Some((x, y)) = world_coordinates_to_idx(dropped_pos) {
        if let Some(underlying_entity) = board.0[y][x] {
            if let Ok(underlying_item) = items.get(underlying_entity) {
                if dropped_item.can_be_merged_with(underlying_item) {
                    // Merge!

                    // play merge music
                    commands.spawn(AudioBundle {
                        source: asset_server.load("merge.ogg"),
                        settings: PlaybackSettings {
                            volume: Volume::new(0.5),
                            ..default()
                        },
                        ..default()
                    });

                    commands.entity(underlying_entity).despawn_recursive();
                    commands.entity(dropped_entity).despawn_recursive();
                    board.0[dropped_item.y][dropped_item.x] = None;
                    board.0[underlying_item.y][underlying_item.x] = Some(
                        commands
                            .spawn(MergableItem {
                                tier: underlying_item.tier + 1,
                                ..underlying_item.clone()
                            })
                            .id(),
                    );
                    return;
                }
            } else {
                warn!("entity in board resource has no mergeable item");
                board.0[y][x] = None;
            }
        }
    }

    // snap back

    // play merge music
    commands.spawn(AudioBundle {
        source: asset_server.load("woosh.ogg"),
        settings: PlaybackSettings {
            volume: Volume::new(0.5),
            ..default()
        },
        ..default()
    });


    let mut dropped_entity_cmd = commands.get_entity(dropped_entity).unwrap();
    let target = dropped_entity.into_target();
    dropped_entity_cmd.animation().insert_tween_here(
        Duration::from_millis(500),
        EaseFunction::BackOut,
        target.with(translation(
            dropped_pos,
            idx_to_world_coordinates(dropped_item.x, dropped_item.y),
        )),
    );
}
