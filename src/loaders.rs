use bevy::asset::{AssetLoader, LoadedAsset};

use crate::tilemap::TileMap;

#[derive(Default)]
pub struct TileMapLoader;

impl AssetLoader for TileMapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let asset = ron::de::from_bytes::<TileMap>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tilemap"]
    }
}
