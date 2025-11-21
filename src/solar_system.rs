// solar_system.rs
// Solar system scene with multiple celestial objects

use raylib::prelude::*;
use crate::shader_system::ShaderType;
use std::f32::consts::PI;

/// Celestial object types
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CelestialType {
    Star,
    Planet,
    Moon,
}

/// Represents a celestial object
#[derive(Clone, Debug)]
pub struct CelestialObject {
    pub object_type: CelestialType,
    pub shader_type: ShaderType,
    pub position: Vector3,
    pub scale: f32,
    pub rotation: Vector3,
    pub rotation_speed: Vector3,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub orbit_angle: f32,
    pub parent_index: Option<usize>,
}

impl CelestialObject {
    /// Create a new star
    pub fn star(scale: f32) -> Self {
        CelestialObject {
            object_type: CelestialType::Star,
            shader_type: ShaderType::Star,
            position: Vector3::zero(),
            scale,
            rotation: Vector3::zero(),
            rotation_speed: Vector3::new(0.0, 0.02, 0.0),
            orbit_radius: 0.0,
            orbit_speed: 0.0,
            orbit_angle: 0.0,
            parent_index: None,
        }
    }
    
    /// Create a new planet
    pub fn planet(
        parent_idx: usize,
        orbit_radius: f32,
        orbit_speed: f32,
        scale: f32,
        shader: ShaderType,
    ) -> Self {
        CelestialObject {
            object_type: CelestialType::Planet,
            shader_type: shader,
            position: Vector3::new(orbit_radius, 0.0, 0.0),
            scale,
            rotation: Vector3::zero(),
            rotation_speed: Vector3::new(0.0, 0.03, 0.0),
            orbit_radius,
            orbit_speed,
            orbit_angle: (rand::random::<f32>() * PI * 2.0), // Ángulo inicial aleatorio para desincronizar
            parent_index: Some(parent_idx),
        }
    }
    
    /// Create a new moon
    pub fn moon(
        parent_idx: usize,
        orbit_radius: f32,
        orbit_speed: f32,
        scale: f32,
        shader: ShaderType,
    ) -> Self {
        let mut moon = Self::planet(parent_idx, orbit_radius, orbit_speed, scale, shader);
        moon.object_type = CelestialType::Moon;
        moon.rotation_speed = Vector3::new(0.0, 0.05, 0.0);
        moon.orbit_angle = 0.0;
        moon.orbit_speed *= 2.0; // Las lunas orbitan más rápido que los planetas
        moon
    }
    
    /// Update object state
    pub fn update(&mut self, delta_time: f32, parent_pos: Option<Vector3>) {
        // Update rotation
        self.rotation.x += self.rotation_speed.x * delta_time;
        self.rotation.y += self.rotation_speed.y * delta_time;
        self.rotation.z += self.rotation_speed.z * delta_time;
        
        // Update orbital position if has parent
        if let Some(center) = parent_pos {
            self.orbit_angle += self.orbit_speed * delta_time;
            self.position.x = center.x + self.orbit_radius * self.orbit_angle.cos();
            self.position.z = center.z + self.orbit_radius * self.orbit_angle.sin();
            self.position.y = center.y;
        }
    }
}

/// Solar system container
pub struct SolarSystem {
    pub objects: Vec<CelestialObject>,
}

impl SolarSystem {
    /// Create empty solar system
    pub fn new() -> Self {
        SolarSystem {
            objects: Vec::new(),
        }
    }
    
    /// Add an object and return its index
    pub fn add(&mut self, object: CelestialObject) -> usize {
        self.objects.push(object);
        self.objects.len() - 1
    }
    
    /// Update all objects
    pub fn update(&mut self, delta_time: f32) {
        let len = self.objects.len();
        // Update rotations
        for i in 0..len {
            let obj = &mut self.objects[i];
            // Si es planeta, rota sobre su eje Y
            if obj.object_type == CelestialType::Planet {
                obj.rotation.y += 1.0 * delta_time; // Puedes ajustar la velocidad aquí
            }
            obj.rotation.x += obj.rotation_speed.x * delta_time;
            obj.rotation.y += obj.rotation_speed.y * delta_time;
            obj.rotation.z += obj.rotation_speed.z * delta_time;
        }
        // Update orbital positions (hierarchical)
        for i in 0..len {
            if let Some(parent_idx) = self.objects[i].parent_index {
                let parent_pos = self.objects[parent_idx].position;
                let obj = &mut self.objects[i];
                
                // Aplicar ley de Kepler: velocidad más lenta mientras más lejos
                // v = orbit_speed / sqrt(orbit_radius)
                let adjusted_speed = obj.orbit_speed / obj.orbit_radius.sqrt();
                
                obj.orbit_angle += adjusted_speed * 1.0 * delta_time;
                obj.position.x = parent_pos.x + obj.orbit_radius * obj.orbit_angle.cos();
                obj.position.z = parent_pos.z + obj.orbit_radius * obj.orbit_angle.sin();
                obj.position.y = parent_pos.y;
            }
        }
    }
    
    /// Create a basic solar system preset
    pub fn create_basic_system() -> Self {
        let mut system = SolarSystem::new();

        system.add(CelestialObject::star(1.0));
        
        // Central star (Sun)
        let sun_idx = system.add(CelestialObject::star(3.0));
        
        // Inner rocky planet (Mercury-like)
        system.add(CelestialObject::planet(
            sun_idx,
            8.0,
            0.08,
            0.4,
            ShaderType::Rocky
        ));
        
        // Second planet (Venus-like - lava world)
        system.add(CelestialObject::planet(
            sun_idx,
            12.0,
            0.06,
            0.65,
            ShaderType::Lava
        ));
        
        // Earth-like planet with moon
        let earth_idx = system.add(CelestialObject::planet(
            sun_idx,
            17.0,
            0.05,
            1.0,
            ShaderType::CloudPlanet
        ));
        
        // Moon orbiting Earth (planeta pequeño orbitando otro planeta)
        // Solo necesitas crear un planeta con radio de órbita pequeño y agregarlo como hijo de Earth
        system.add(CelestialObject::moon(
            earth_idx,
            0.8,      // radio de órbita pequeño (distancia desde Earth)
            0.15,     // velocidad de órbita más rápida que los planetas grandes
            0.12,     // escala pequeña (es una luna)
            ShaderType::Rocky
        ));
        
        // Gas giant (Jupiter-like)
        let jupiter_idx = system.add(CelestialObject::planet(
            sun_idx,
        24.0,
            0.03,
            1.5,
            ShaderType::GasGiant
        ));
        
        // Moons of gas giant
        system.add(CelestialObject::moon(
            jupiter_idx,
            1.3,
            0.12,
            0.15,
            ShaderType::IceWorld
        ));
        
        system.add(CelestialObject::moon(
            jupiter_idx,
            1.8,
            0.09,
            0.18,
            ShaderType::Rocky
        ));
        
        // Outer ice world
        system.add(CelestialObject::planet(
            sun_idx,
            30.0,
            0.02,
            0.8,
            ShaderType::IceWorld
        ));
        
        system
    }
    
    /// Create an alien system with exotic planets
    pub fn create_alien_system() -> Self {
        let mut system = SolarSystem::new();
        
        // Binary star system (two stars)
        let star1_idx = system.add(CelestialObject::star(1.2));
        
        let mut star2 = CelestialObject::star(0.8);
        star2.shader_type = ShaderType::Lava; // Orange dwarf
        star2.scale = 0.8;
        star2.parent_index = Some(star1_idx);
        star2.orbit_radius = 3.0;
        star2.orbit_speed = 0.1;
        system.add(star2);
        
        // Lava world close to stars
        system.add(CelestialObject::planet(
            star1_idx,
            6.0,
            0.12,
            0.5,
            ShaderType::Lava
        ));
        
        // Large gas giant
        let giant_idx = system.add(CelestialObject::planet(
            star1_idx,
            10.0,
            0.04,
            1.2,
            ShaderType::GasGiant
        ));
        
        // Multiple moons around giant
        system.add(CelestialObject::moon(
            giant_idx,
            1.8,
            0.15,
            0.2,
            ShaderType::IceWorld
        ));
        
        system.add(CelestialObject::moon(
            giant_idx,
            2.3,
            0.11,
            0.25,
            ShaderType::Lava
        ));
        
        system.add(CelestialObject::moon(
            giant_idx,
            2.9,
            0.08,
            0.18,
            ShaderType::CloudPlanet
        ));
        
        // Distant frozen world
        system.add(CelestialObject::planet(
            star1_idx,
            16.0,
            0.02,
            0.6,
            ShaderType::IceWorld
        ));
        
        system
    }
}