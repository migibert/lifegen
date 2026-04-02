#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct Tile {
    pub biomass: f32,
    pub corpse_meat: f32,
    pub soil_nutrients: f32,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            biomass: 0.0,
            corpse_meat: 0.0,
            soil_nutrients: crate::constants::INITIAL_SOIL_NUTRIENTS,
        }
    }
}

#[derive(Debug, Resource)]
pub struct TileGrid {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
}

impl TileGrid {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tile::default(); (width * height) as usize],
        }
    }

    pub fn new_empty() -> Self {
        Self::new(0, 0)
    }

    pub fn index(&self, x: u32, y: u32) -> Option<usize> {
        if x < self.width && y < self.height {
            Some((y * self.width + x) as usize)
        } else {
            None
        }
    }

    pub fn get(&self, x: u32, y: u32) -> &Tile {
        let idx = self.index(x, y).unwrap_or(0);
        &self.tiles[idx]
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut Tile {
        let idx = self.index(x, y).unwrap_or(0);
        &mut self.tiles[idx]
    }

    pub fn generate_biomes(&mut self) {
        // stub: actual implementation comes from spec
        for tile in self.tiles.iter_mut() {
            tile.biomass = 1.0;
        }
    }
}

#[derive(Debug, Resource)]
pub struct SpatialHashGrid {
    pub chunks_per_row: u32,
}

impl SpatialHashGrid {
    pub fn new(chunks_per_row: u32) -> Self {
        Self { chunks_per_row }
    }

    pub fn insert(&mut self, _x: u32, _y: u32, _entity: Entity) {
        // stub: no-op for now
    }
}

#[derive(Debug, Resource, Default)]
pub struct GlobalClock {
    pub tick: u32,
}
