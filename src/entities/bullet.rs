//! Bullet entity implementation.

use crate::constants::BULLET_SPEED;

/// Represents a bullet fired by the player.
///
/// Bullets move upward at a constant speed until they either
/// hit an enemy or move off the top of the screen.
#[derive(Debug, Clone)]
pub struct Bullet {
    /// X position in pixels
    pub x: f32,
    /// Y position in pixels
    pub y: f32,
}

impl Bullet {
    /// Create a new bullet at the specified position.
    ///
    /// # Arguments
    ///
    /// * `x` - Initial X coordinate
    /// * `y` - Initial Y coordinate
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        log::debug!("Creating bullet at ({}, {})", x, y);
        Self { x, y }
    }

    /// Update bullet position based on delta time.
    ///
    /// # Arguments
    ///
    /// * `dt` - Delta time in seconds
    pub fn update(&mut self, dt: f32) {
        self.y -= BULLET_SPEED * dt;
    }

    /// Check if bullet has moved off the top of the screen.
    #[must_use]
    pub fn is_out_of_bounds(&self) -> bool {
        self.y < 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bullet_out_of_bounds() {
        let bullet = Bullet::new(100.0, -10.0);
        assert!(bullet.is_out_of_bounds());
    }

    #[test]
    fn test_bullet_in_bounds() {
        let bullet = Bullet::new(100.0, 100.0);
        assert!(!bullet.is_out_of_bounds());
    }
}
