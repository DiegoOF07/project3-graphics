// camera.rs
// Orbital camera implementation with keyboard controls

use raylib::prelude::*;
use crate::matrix::create_view_matrix;
use std::f32::consts::PI;

/// Orbital camera that rotates around a target point
pub struct Camera {
    // Camera vectors
    pub eye: Vector3,
    pub target: Vector3,
    pub up: Vector3,

    // Spherical coordinates for orbital control
    pub yaw: f32,      // Horizontal rotation (around Y axis)
    pub pitch: f32,    // Vertical rotation (up/down)
    pub distance: f32, // Distance from target

    // Control sensitivity
    pub rotation_speed: f32,
    pub zoom_speed: f32,
    pub pan_speed: f32,
}

impl Camera {
    /// Creates a new orbital camera
    pub fn new(eye: Vector3, target: Vector3, up: Vector3) -> Self {
        // Calculate initial spherical coordinates
        let direction = Vector3::new(
            eye.x - target.x,
            eye.y - target.y,
            eye.z - target.z,
        );

        let distance = (direction.x * direction.x 
            + direction.y * direction.y 
            + direction.z * direction.z).sqrt();
        let pitch = (direction.y / distance).asin();
        let yaw = direction.z.atan2(direction.x);

        Camera {
            eye,
            target,
            up,
            yaw,
            pitch,
            distance,
            rotation_speed: 0.05,
            zoom_speed: 0.5,
            pan_speed: 0.1,
        }
    }

    /// Updates camera eye position based on spherical coordinates
    fn update_eye_position(&mut self) {
        // Clamp pitch to avoid gimbal lock
        self.pitch = self.pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        // Convert spherical to Cartesian coordinates
        self.eye.x = self.target.x + self.distance * self.pitch.cos() * self.yaw.cos();
        self.eye.y = self.target.y + self.distance * self.pitch.sin();
        self.eye.z = self.target.z + self.distance * self.pitch.cos() * self.yaw.sin();
    }

    /// Returns the view matrix for this camera
    pub fn get_view_matrix(&self) -> Matrix {
        create_view_matrix(self.eye, self.target, self.up)
    }

    /// Processes keyboard input for camera control
    /// 
    /// Controls:
    /// - W/S: Pitch (up/down)
    /// - A/D: Yaw (left/right)
    /// - Up/Down arrows: Zoom in/out
    /// - Q/E or Left/Right arrows: Pan horizontally
    /// - R/F: Pan vertically
    pub fn process_input(&mut self, window: &RaylibHandle) {
        // Yaw rotation (horizontal)
        if window.is_key_down(KeyboardKey::KEY_A) {
            self.yaw += self.rotation_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            self.yaw -= self.rotation_speed;
            self.update_eye_position();
        }

        // Pitch rotation (vertical)
        if window.is_key_down(KeyboardKey::KEY_W) {
            self.pitch += self.rotation_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            self.pitch -= self.rotation_speed;
            self.update_eye_position();
        }

        // Zoom (distance control)
        if window.is_key_down(KeyboardKey::KEY_UP) {
            self.distance = (self.distance - self.zoom_speed).max(0.5);
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            self.distance += self.zoom_speed;
            self.update_eye_position();
        }

        // Calculate movement vectors for panning
        let forward = Vector3::new(
            self.target.x - self.eye.x,
            0.0,
            self.target.z - self.eye.z,
        );
        let forward_len = (forward.x * forward.x + forward.z * forward.z).sqrt();
        let forward_normalized = if forward_len > 0.0 {
            Vector3::new(forward.x / forward_len, 0.0, forward.z / forward_len)
        } else {
            Vector3::new(0.0, 0.0, 1.0)
        };

        let right = Vector3::new(forward_normalized.z, 0.0, -forward_normalized.x);

        // Horizontal panning
        if window.is_key_down(KeyboardKey::KEY_Q) || window.is_key_down(KeyboardKey::KEY_LEFT) {
            self.target.x += right.x * self.pan_speed;
            self.target.z += right.z * self.pan_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_E) || window.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.target.x -= right.x * self.pan_speed;
            self.target.z -= right.z * self.pan_speed;
            self.update_eye_position();
        }

        // Vertical panning
        if window.is_key_down(KeyboardKey::KEY_R) {
            self.target.y += self.pan_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_F) {
            self.target.y -= self.pan_speed;
            self.update_eye_position();
        }
    }
}