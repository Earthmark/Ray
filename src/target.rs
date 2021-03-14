use glam::UVec2;
use log::info;
use std::convert::TryInto;

pub struct ImageTarget<PixelFormat> {
    dims: UVec2,
    pixels: Vec<PixelFormat>,
}

fn to_index(pos: UVec2, dims: UVec2) -> Option<usize> {
    if pos.cmplt(dims).all() {
        Some((pos.x + pos.y * dims.x).try_into().unwrap())
    } else {
        None
    }
}

impl<PixelFormat: Clone> ImageTarget<PixelFormat> {
    pub fn new(dims: UVec2, color: PixelFormat) -> ImageTarget<PixelFormat> {
        info!(
            "Frame created with {}x{} pixels, {} bytes",
            dims.x,
            dims.y,
            std::mem::size_of::<PixelFormat>() * (dims.x * dims.y) as usize
        );
        ImageTarget {
            dims,
            pixels: vec![color; (dims.x * dims.y).try_into().unwrap()],
        }
    }

    pub fn dims(&self) -> UVec2 {
        self.dims
    }

    pub fn get(&self, pos: UVec2) -> Option<&PixelFormat> {
        to_index(pos, self.dims).and_then(move |index| self.pixels.get(index))
    }

    pub fn get_mut(&mut self, pos: UVec2) -> Option<&mut PixelFormat> {
        to_index(pos, self.dims).and_then(move |index| self.pixels.get_mut(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;
    use crate::range::range;

    #[test]
    fn test_allocates_fixed() {
        let dims = UVec2::new(1024, 1024);
        let mut target = ImageTarget::new(dims, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(target.dims(), dims);
        for x in 0..1024 {
            for y in 0..1024 {
                assert_eq!(
                    target.get(UVec2::new(x, y)),
                    Some(&Vec3::new(0.0, 0.0, 0.0))
                );
                assert_eq!(
                    target.get_mut(UVec2::new(x, y)),
                    Some(&mut Vec3::new(0.0, 0.0, 0.0))
                );
            }
        }
    }

    #[test]
    fn test_index_setter() {
        let dims = UVec2::new(1024, 1024);
        let mut target = ImageTarget::new(dims, Vec3::new(0.0, 0.0, 0.0));
        for index in range(UVec2::new(1024, 1024)) {
            if let Some(pixel) = target.get_mut(index) {
                *pixel = Vec3::new(index.x as f32, index.y as f32, 0.0);
            }
        }
        for x in 0..1024 {
            for y in 0..1024 {
                assert_eq!(
                    target.get(UVec2::new(x, y)),
                    Some(&Vec3::new(x as f32, y as f32, 0.0))
                );
                assert_eq!(
                    target.get_mut(UVec2::new(x, y)),
                    Some(&mut Vec3::new(x as f32, y as f32, 0.0))
                );
            }
        }
    }
}
