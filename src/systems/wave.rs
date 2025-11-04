//! Wave generation system.

use crate::entities::Enemy;

/// Generate enemies for a given wave number.
///
/// Each wave generates a grid of enemies with progressively more rows.
/// The formula is: rows = 2 + wave_number, with a constant 10 columns.
///
/// # Arguments
///
/// * `wave` - The wave number (1-based)
///
/// # Returns
///
/// A vector of enemies positioned in a grid formation
///
/// # Examples
///
/// ```
/// # use ten::systems::wave::generate_wave;
/// let wave_1 = generate_wave(1);  // 30 enemies (3 rows x 10 columns)
/// let wave_2 = generate_wave(2);  // 40 enemies (4 rows x 10 columns)
/// ```
#[must_use]
pub fn generate_wave(wave: u32) -> Vec<Enemy> {
    let rows = 2 + wave as usize;
    let columns = 10;
    let enemy_count = rows * columns;

    log::info!("Generating wave {} with {} enemies ({} rows x {} columns)",
               wave, enemy_count, rows, columns);

    let mut enemies = Vec::with_capacity(enemy_count);

    for i in 0..columns {
        for j in 0..rows {
            enemies.push(Enemy::new(
                50.0 + i as f32 * 60.0,
                100.0 + j as f32 * 50.0,
            ));
        }
    }

    enemies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_enemies_wave_1() {
        let enemies = generate_wave(1);
        // Wave 1 should have 3 rows (2 + 1) and 10 columns
        assert_eq!(enemies.len(), 30);
    }

    #[test]
    fn test_generate_enemies_wave_2() {
        let enemies = generate_wave(2);
        // Wave 2 should have 4 rows (2 + 2) and 10 columns
        assert_eq!(enemies.len(), 40);
    }

    #[test]
    fn test_generate_enemies_positions() {
        let enemies = generate_wave(1);

        // Check first enemy position
        assert_eq!(enemies[0].x, 50.0);
        assert_eq!(enemies[0].y, 100.0);

        // Check that enemies are spaced properly vertically (same column, next row)
        assert_eq!(enemies[1].x, 50.0);
        assert_eq!(enemies[1].y, 150.0); // 100.0 + 50.0

        // Check enemies are spaced horizontally (next column, first row)
        // Wave 1 has 3 rows, so enemies[3] is first enemy of second column
        assert_eq!(enemies[3].x, 110.0); // 50.0 + 60.0
        assert_eq!(enemies[3].y, 100.0);
    }
}
