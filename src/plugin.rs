use bevy::prelude::*;
use bevy_tilemap::prelude::TilemapDefaultPlugins;

use crate::{loaders::TileMapLoader, tilemap::TileMap};

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        println!("Adding tilemap plugin");
        app.add_asset::<TileMap>()
            .init_asset_loader::<TileMapLoader>()
            .add_plugins(TilemapDefaultPlugins);
    }
}
