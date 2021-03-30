use glam::{UVec2, Vec2, Vec3};

use crate::ray::Ray;
use crate::range::range;

struct ProjectionGenerator {
    dims: UVec2,
    size: Vec2,
}

impl ProjectionGenerator {
    pub fn new(dims: UVec2, size: Vec2) -> ProjectionGenerator {
        ProjectionGenerator {
            dims, size
        }
    }

    // Returns a set of rays, pointing towards the negative Z axis.
    pub fn iter(&self) -> impl Iterator<Item = Ray> {
        let walk_per_step = self.size / self.dims.as_f32();
        let root_shift = self.size - (self.size / 2.0);
        range(self.dims).map(move |index| root_shift + index.as_f32() * walk_per_step).map(|point| Ray {
            point: point.extend(0.0),
            direction: -Vec3::Z,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{vec2, uvec2};

    #[test]
    fn test_walks_all_indexes() {
        let dims = uvec2(100, 100);
        let size = vec2(256.0, 256.0);
        let generator = ProjectionGenerator::new(dims, size);
        assert_eq!(10000, generator.iter().count());
    }
}
