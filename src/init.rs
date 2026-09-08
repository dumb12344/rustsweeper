use bevy::prelude::*;
use rand::Rng;

use crate::{constants::*, pointer_handling::*, tile_states::{Tile, TileState}, GameValues};

pub fn initialize_tiles (tiles: Vec<Vec<TileState>>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>, mut game_values: ResMut<GameValues>) {
    tiles.iter().enumerate().for_each(|(i, tile_row)| {
        tile_row.iter().enumerate().for_each(|(j, tile)| {
            let mut surrounding_mines = 0;
            for dx in -1isize..=1 {
                for dy in -1isize..=1 {
                    let x = i as isize + dx;
                    let y = j as isize + dy;
                    if x < 0 || y < 0 {
                        continue;
                    }
                    let Some(row) = tiles.get(x as usize) else {
                        continue;
                    };
                    let Some(loop_tile) = row.get(y as usize) else {
                        continue;
                    };
                    if loop_tile.is_mine() {
                        surrounding_mines += 1;
                    }
                }
            }
            let tile_size_float = game_values.tile_size as f32;
            let absolute_x = (i as f32) * tile_size_float - (game_values.tile_size * SIZE_X) as f32 / 2.0 + tile_size_float / 2.0;
            let absolute_y = (j as f32) * tile_size_float - (game_values.tile_size * SIZE_Y) as f32 / 2.0 + tile_size_float / 2.0;
            let tile = Tile {
                x: i,
                y: j,
                tile: *tile,
                surrounding_mines
            };
            let entity = commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(tile_size_float, tile_size_float))),
                MeshMaterial2d(materials.add(tile.get_color())),
                Transform::from_xyz(
                    absolute_x,
                    absolute_y,
                    0.0
                ),
                tile
            ))
            .with_children(|parent| {
                parent.spawn((
                    Sprite::from_image(asset_server.load("flag_icon.png")),
                    Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::new(tile_size_float / 128.0, tile_size_float / 128.0, 1.0)),
                    Visibility::Hidden
                ));
                parent.spawn((
                    Text2d::new(surrounding_mines.to_string()),
                    Transform::from_xyz(0.0, 0.0, 1.0),
                    TextFont {
                        font_size: FontSize::Px(45.0 / 50.0 * tile_size_float),
                        font: asset_server.load("google_sans.otf").into(),
                        ..Default::default()
                    },
                    TextColor(Color::from(Srgba::hex(NUMBER_COLORS.get(surrounding_mines as usize).unwrap()).unwrap())),
                    Visibility::Hidden
                ));
                parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new((1.0 + 1.0/7.5) * tile_size_float, (1.0 + 1.0/7.5) * tile_size_float))),
                    MeshMaterial2d(materials.add(Color::from(Srgba::hex("#87af3a").unwrap()))),
                    Transform::from_xyz(0.0, 0.0, -0.5)
                ));
                parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new(tile_size_float, tile_size_float))),
                    MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 0.2))),
                    Transform::from_xyz(0.0, 0.0, 0.5),
                    Visibility::Hidden
                ));
            })
            .observe(click())
            .observe(reveal())
            .observe(flag())
            .observe(hover())
            .observe(unhover())
            .id();
            game_values.tile_entities.get_mut(i).unwrap().push(entity);
        })
    });
}

pub fn generate_mines (tiles: &mut Vec<Vec<TileState>>) {
    let mut i = 0;
    while i < MINE_COUNT as i32 {
        let rand_x = rand::thread_rng().gen_range(0..SIZE_X) as usize;
        let rand_y = rand::thread_rng().gen_range(0..SIZE_Y) as usize;
        // Don't place mine in top left corner
        let tile = tiles.get_mut(rand_x).unwrap().get_mut(rand_y).unwrap();
        if tile == &TileState::HiddenBlank && !(rand_x <= 1 && rand_y >= SIZE_Y as usize - 2) {
            *tile = TileState::HiddenMine;
            i += 1;
        }
    }
}

pub fn setup (mut commands: Commands, meshes: ResMut<Assets<Mesh>>, materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>, mut game_values: ResMut<GameValues>) {
    game_values.remaining_blanks = (SIZE_X as i32 * SIZE_Y as i32) - MINE_COUNT as i32;
    commands.spawn(Camera2d);
    let mut tiles = Vec::new();
    for _ in 0..SIZE_X {
        let mut tile_row = Vec::new();
        for _ in 0..SIZE_Y {
            tile_row.push(TileState::HiddenBlank)
        }
        game_values.tile_entities.push(Vec::new());
        tiles.push(tile_row);
    }
    generate_mines(&mut tiles);
    initialize_tiles(tiles, commands, meshes, materials, asset_server, game_values);
}