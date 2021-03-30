use glam::{UVec2, Vec2, Vec3};

use crate::ray::Ray;
use crate::range::range;

struct PerspectiveGenerator {
    dims: UVec2,
    size: Vec2,
    distance: f32,
}

impl PerspectiveGenerator {
    pub fn new(dims: UVec2, size: Vec2,distance: f32) -> PerspectiveGenerator {
        PerspectiveGenerator {
            dims, size, distance
        }
    }

    // Returns a set of rays, from a center point through a grid n units away in the negative z direction-ish.
    pub fn iter(&self) -> impl Iterator<Item = Ray> {
        let walk_per_step = self.size / self.dims.as_f32();
        let root_shift = self.size - (self.size / 2.0);
        let distance = self.distance;
        range(self.dims)
        .map(move |index| root_shift + index.as_f32() * walk_per_step)
        .map(move |point| point.extend(-distance))
        .map(|target| Ray {
            point: Vec3::ZERO,
            direction: target.normalize(),
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
        let generator = PerspectiveGenerator::new(dims, size, 2.0);
        assert_eq!(10000, generator.iter().count());
        for ray in generator.iter() {
            assert_ne!(ray.direction, Vec3::ZERO);
        }
    }
}
