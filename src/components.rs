#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Component, Debug, Clone)]
pub struct VitalSigns {
    pub energy: f32,
    pub health: f32,
    pub age: u32,
    pub lifetime_stress: f32,
    pub speed_modifier: f32,
    pub health_regen_modifier: f32,
}

#[derive(Component, Debug, Clone)]
pub struct PhysicalGenome {
    pub size: f32,
    pub speed: f32,
    pub sensory_radius: f32,
    pub thermal_tolerance: f32,
    pub aquatic_adaptation: f32,
}

#[derive(Bundle)]
pub struct EntityBundle {
    pub position: Position,
    pub vital_signs: VitalSigns,
    pub genome: PhysicalGenome,
}

impl EntityBundle {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            position: Position { x, y },
            vital_signs: VitalSigns {
                energy: crate::constants::BASE_MAX_ENERGY * crate::constants::INITIAL_ENTITY_ENERGY_FRAC,
                health: 1.0,
                age: 0,
                lifetime_stress: 0.0,
                speed_modifier: 1.0,
                health_regen_modifier: 1.0,
            },
            genome: PhysicalGenome {
                size: 0.5,
                speed: 0.5,
                sensory_radius: 0.5,
                thermal_tolerance: 0.5,
                aquatic_adaptation: 0.5,
            },
        }
    }
}

pub fn primordial_bundle(x: u32, y: u32) -> EntityBundle {
    EntityBundle::new(x, y)
}
