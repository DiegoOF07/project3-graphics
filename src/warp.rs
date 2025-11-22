// warp.rs
// Instant warping and animated transition system

use raylib::prelude::*;

/// Warp animation state
#[derive(Clone)]
pub enum WarpState {
    Idle,
    Warping {
        start_pos: Vector3,
        start_target: Vector3,
        end_pos: Vector3,
        end_target: Vector3,
        progress: f32,
        duration: f32,
    },
}

/// Manages camera warping to different locations
pub struct WarpSystem {
    pub state: WarpState,
    pub warp_targets: Vec<(String, Vector3, f32)>, // (name, position, view_distance)
    pub current_target: usize,
}

impl WarpSystem {
    pub fn new() -> Self {
        WarpSystem {
            state: WarpState::Idle,
            warp_targets: Vec::new(),
            current_target: 0,
        }
    }
    
    /// Add a warp target
    pub fn add_target(&mut self, name: &str, position: Vector3, view_distance: f32) {
        self.warp_targets.push((name.to_string(), position, view_distance));
    }
    
    /// Start warping to a specific target index
    pub fn warp_to(
        &mut self,
        target_idx: usize,
        current_eye: Vector3,
        current_target: Vector3,
    ) {
        if target_idx >= self.warp_targets.len() {
            return;
        }
        
        let (_, target_pos, view_dist) = &self.warp_targets[target_idx];
        
        // Calculate end position (offset from target)
        let end_target = *target_pos;
        let end_pos = Vector3::new(
            target_pos.x + *view_dist,
            target_pos.y + *view_dist * 0.5,
            target_pos.z + *view_dist,
        );
        
        self.state = WarpState::Warping {
            start_pos: current_eye,
            start_target: current_target,
            end_pos,
            end_target,
            progress: 0.0,
            duration: 0.8, // Reduced from 1.5 to 0.8 seconds for faster transition
        };
        
        self.current_target = target_idx;
    }
    
    /// Warp to next target in list
    pub fn warp_next(&mut self, current_eye: Vector3, current_target: Vector3) {
        if self.warp_targets.is_empty() {
            return;
        }
        let next = (self.current_target + 1) % self.warp_targets.len();
        self.warp_to(next, current_eye, current_target);
    }
    
    /// Warp to previous target
    pub fn warp_prev(&mut self, current_eye: Vector3, current_target: Vector3) {
        if self.warp_targets.is_empty() {
            return;
        }
        let prev = if self.current_target == 0 {
            self.warp_targets.len() - 1
        } else {
            self.current_target - 1
        };
        self.warp_to(prev, current_eye, current_target);
    }
    
    /// Update warp animation
    pub fn update(&mut self, delta_time: f32) -> Option<(Vector3, Vector3)> {
        match &mut self.state {
            WarpState::Idle => None,
            WarpState::Warping {
                start_pos,
                start_target,
                end_pos,
                end_target,
                progress,
                duration,
            } => {
                *progress += delta_time / *duration;
                
                if *progress >= 1.0 {
                    let final_pos = *end_pos;
                    let final_target = *end_target;
                    self.state = WarpState::Idle;
                    return Some((final_pos, final_target));
                }
                
                // Smooth easing function (ease in-out cubic)
                let t = *progress;
                let ease = if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                };
                
                // Interpolate position and target
                let pos = Vector3::new(
                    start_pos.x + (end_pos.x - start_pos.x) * ease,
                    start_pos.y + (end_pos.y - start_pos.y) * ease,
                    start_pos.z + (end_pos.z - start_pos.z) * ease,
                );
                
                let target = Vector3::new(
                    start_target.x + (end_target.x - start_target.x) * ease,
                    start_target.y + (end_target.y - start_target.y) * ease,
                    start_target.z + (end_target.z - start_target.z) * ease,
                );
                
                Some((pos, target))
            }
        }
    }
    
    /// Check if currently warping
    pub fn is_warping(&self) -> bool {
        matches!(self.state, WarpState::Warping { .. })
    }
    
    /// Get current target name
    pub fn current_target_name(&self) -> Option<&str> {
        self.warp_targets.get(self.current_target).map(|(n, _, _)| n.as_str())
    }
}