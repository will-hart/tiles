use std::collections::HashMap;

use bevy::{prelude::*, sprite::TextureAtlasBuilder};
use bevy_tilemap::{
    prelude::{LayerKind, TilemapBundle},
    Tile, Tilemap as BevyTileMap, TilemapLayer,
};
use tilemap::TileMap;

pub mod loaders;
pub mod plugin;
pub mod tilemap;

pub fn build_tilemap(
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    textures: ResMut<Assets<Texture>>,
    mut tile_data: TileMap,
) -> TilemapBundle {
    // create the texture atlas from the tilemap
    let (atlas, tile_mapping) =
        build_texture_atlas(asset_server, textures, texture_atlases, &tile_data);

    // create the tilemap
    let mut tilemap = BevyTileMap::builder()
        .texture_atlas(atlas)
        .dimensions(tile_data.width as u32, tile_data.height as u32)
        .chunk_dimensions(16, 16, 1)
        .auto_chunk()
        .texture_dimensions(16, 16)
        .auto_spawn(200, 200)
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Dense,
            },
            0,
        )
        .finish()
        .unwrap();

    // build the map from the tilemap
    let widths = 0..tile_data.width;
    let heights = 0..tile_data.height;

    // cartesian product of heights and widths to fill the entire grid
    let tiles = heights
        .flat_map(|y| widths.clone().map(move |x| (x, y)))
        .map(|(x, y)| {
            let definition = tile_data.get_sprite_index(x, y);
            let sprite_index = tile_mapping.get(&definition).unwrap().clone();

            Tile {
                point: (x as u32, y as u32),
                sprite_index,
                ..Default::default()
            }
        })
        .collect::<Vec<_>>();

    tilemap
        .insert_tiles(tiles)
        .expect("Unable to generate tilemap from tilemap data");

    // create the Bevy Component Bundle
    let components = TilemapBundle {
        tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Default::default(),
        global_transform: Default::default(),
    };

    components
}

// Builds and returns a texture atlas as well as a mapping between TileSprite index and
// texture atlas index
fn build_texture_atlas(
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    data: &TileMap,
) -> (Handle<TextureAtlas>, HashMap<usize, usize>) {
    let mut builder = TextureAtlasBuilder::default();
    let mut mapping: HashMap<usize, usize> = HashMap::new();

    for definition in data.sprites.iter() {
        // these should be preloaded
        let texture_handle = asset_server.load(definition.path.as_str());
        let texture = textures.get(texture_handle.clone_weak()).unwrap();

        builder.add_texture(texture_handle.clone_weak(), texture);
    }

    let texture_atlas = builder.finish(&mut textures).unwrap();

    // build up indices - key is the index in the TileMap::sprites vec, value is the texture_atlas index
    for (idx, definition) in data.sprites.iter().enumerate() {
        let sprite: Handle<Texture> = asset_server.get_handle(definition.path.as_str());
        let sprite_idx = texture_atlas.get_texture_index(&sprite).unwrap();
        mapping.insert(idx, sprite_idx);
    }

    let atlas_handle = texture_atlases.add(texture_atlas);

    (atlas_handle, mapping)
}
