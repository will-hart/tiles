use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileDefinition {
    pub path: String,
    pub cost: i8,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NavPoint {
    pub id: u16,
    pub x: u16,
    pub y: u16,
    pub edges: Vec<u16>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, TypeUuid)]
#[uuid = "39cadc55-aa9c-4543-8640-a018b74b5052"]
pub struct TileMap {
    pub width: u16,
    pub height: u16,
    pub sprites: Vec<TileDefinition>,

    // maps between map locations (u16) and sprite definition indices usize
    pub tiles: HashMap<(u16, u16), usize>,

    // default is applied if a tile is not specified
    pub default_tile: usize,

    pub nav_points: Vec<NavPoint>,
}

impl TileMap {
    pub fn new(width: u16, height: u16) -> Self {
        TileMap {
            width,
            height,
            tiles: HashMap::new(),
            nav_points: Vec::new(),
            default_tile: 0,
            sprites: vec![],
        }
    }

    // gets the tile definition at a given location
    pub fn get(&mut self, x: u16, y: u16) -> TileDefinition {
        match self.tiles.get(&(x, y)) {
            Some(tile) => self.sprites[*tile].clone(),
            None => self.sprites[self.default_tile].clone(),
        }
    }

    // gets the index at the given location
    pub fn get_sprite_index(&mut self, x: u16, y: u16) -> usize {
        match self.tiles.get(&(x, y)) {
            Some(tile) => tile.clone(),
            None => self.default_tile.clone(),
        }
    }

    pub fn set(&mut self, x: u16, y: u16, sprite_index: usize) {
        self.tiles.insert((x, y), sprite_index);
    }

    pub fn from_str(data: &str) -> TileMap {
        ron::from_str::<TileMap>(data).unwrap()
    }

    pub fn to_string(&self) -> String {
        ron::to_string(self).unwrap()
    }
}
