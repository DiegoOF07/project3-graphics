// camera.rs
// Orbital camera with keyboard controls and 3D movement

use raylib::prelude::*;
use crate::matrix::create_view_matrix;
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub rotation_speed: f32,
    pub zoom_speed: f32,
    pub pan_speed: f32,
}

impl Camera {
    pub fn new(eye: Vector3, target: Vector3, up: Vector3) -> Self {
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
            eye, target, up,
            yaw, pitch, distance,
            rotation_speed: 0.05,
            zoom_speed: 0.5,
            pan_speed: 0.1,
        }
    }

    fn update_eye_position(&mut self) {
        self.pitch = self.pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
        self.eye.x = self.target.x + self.distance * self.pitch.cos() * self.yaw.cos();
        self.eye.y = self.target.y + self.distance * self.pitch.sin();
        self.eye.z = self.target.z + self.distance * self.pitch.cos() * self.yaw.sin();
    }

    pub fn get_view_matrix(&self) -> Matrix {
        create_view_matrix(self.eye, self.target, self.up)
    }
    
    /// Set camera position directly (used by warp system)
    pub fn set_position(&mut self, eye: Vector3, target: Vector3) {
        self.eye = eye;
        self.target = target;
        
        // Recalculate spherical coordinates
        let direction = Vector3::new(
            eye.x - target.x,
            eye.y - target.y,
            eye.z - target.z,
        );
        
        self.distance = (direction.x * direction.x 
            + direction.y * direction.y 
            + direction.z * direction.z).sqrt();
        
        if self.distance > 0.001 {
            self.pitch = (direction.y / self.distance).asin();
            self.yaw = direction.z.atan2(direction.x);
        }
    }

    pub fn process_input(&mut self, window: &RaylibHandle) {
        // Yaw (horizontal rotation)
        if window.is_key_down(KeyboardKey::KEY_A) {
            self.yaw += self.rotation_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            self.yaw -= self.rotation_speed;
            self.update_eye_position();
        }

        // Pitch (vertical rotation) - Full 3D movement
        if window.is_key_down(KeyboardKey::KEY_W) {
            self.pitch += self.rotation_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            self.pitch -= self.rotation_speed;
            self.update_eye_position();
        }

        // Zoom
        if window.is_key_down(KeyboardKey::KEY_UP) {
            self.distance = (self.distance - self.zoom_speed).max(0.5);
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            self.distance += self.zoom_speed;
            self.update_eye_position();
        }

        // Panning vectors
        let forward = Vector3::new(
            self.target.x - self.eye.x,
            0.0,
            self.target.z - self.eye.z,
        );
        let forward_len = (forward.x * forward.x + forward.z * forward.z).sqrt();
        let forward_norm = if forward_len > 0.0 {
            Vector3::new(forward.x / forward_len, 0.0, forward.z / forward_len)
        } else {
            Vector3::new(0.0, 0.0, 1.0)
        };

        let right = Vector3::new(forward_norm.z, 0.0, -forward_norm.x);

        // Horizontal pan
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

        // Vertical pan (full 3D movement)
        if window.is_key_down(KeyboardKey::KEY_R) {
            self.target.y += self.pan_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_F) {
            self.target.y -= self.pan_speed;
            self.update_eye_position();
        }
        
        // Forward/backward movement along view direction
        if window.is_key_down(KeyboardKey::KEY_Z) {
            let move_speed = self.pan_speed * 2.0;
            self.target.x += forward_norm.x * move_speed;
            self.target.z += forward_norm.z * move_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_X) {
            let move_speed = self.pan_speed * 2.0;
            self.target.x -= forward_norm.x * move_speed;
            self.target.z -= forward_norm.z * move_speed;
            self.update_eye_position();
        }
    }
}