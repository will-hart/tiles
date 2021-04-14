use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlas};

use conquest_tiles::{plugin::TileMapPlugin, tilemap::TileMap};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum EditorState {
    Loading,
    Editing,
}

#[derive(Default, Clone)]
pub struct LoadingStatus {
    pub loaded: bool,
    pub tilemap: Handle<TileMap>,
    pub floor: Handle<Texture>,
    pub wall: Handle<Texture>,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(EditorState::Loading)
        .add_plugin(TileMapPlugin)
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::on_update(EditorState::Loading).with_system(build_tilemap.system()),
        )
        .init_resource::<LoadingStatus>()
        .run();
}

fn setup(
    mut commands: Commands,
    mut loading: ResMut<LoadingStatus>,
    asset_server: Res<AssetServer>,
) {
    loading.tilemap = asset_server.load("basic.tilemap");
    loading.floor = asset_server.load("floor.png");
    loading.wall = asset_server.load("wall.png");

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}

fn build_tilemap(
    mut commands: Commands,
    mut loading: ResMut<LoadingStatus>,
    asset_server: Res<AssetServer>,
    tilemaps: Res<Assets<TileMap>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    textures: ResMut<Assets<Texture>>,
    mut app_state: ResMut<State<EditorState>>,
) {
    let loaded = asset_server.get_load_state(loading.tilemap.clone_weak()) == LoadState::Loaded
        && asset_server.get_load_state(loading.floor.clone_weak()) == LoadState::Loaded
        && asset_server.get_load_state(loading.wall.clone_weak()) == LoadState::Loaded;

    if !loaded {
        return;
    }

    let tilemap = tilemaps.get(loading.tilemap.clone_weak()).unwrap();
    println!("Loaded tilemap, {:?}", tilemap);

    // create the bundle and spawn it
    let bundle =
        conquest_tiles::build_tilemap(asset_server, texture_atlases, textures, tilemap.clone());
    commands
        .spawn_bundle(bundle)
        .insert(Timer::from_seconds(0.075, true));

    // go to editing state
    println!("Finished loading, moving to editing state");
    loading.loaded = true;
    app_state.set(EditorState::Editing).unwrap();
}
