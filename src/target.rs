use glam::UVec2;
use std::convert::TryInto;

pub struct ImageTarget<PixelFormat> {
  dims: UVec2,
  pixels: Vec<PixelFormat>,
}

fn to_index(pos: UVec2, dims: UVec2) -> Option<usize> {
  if within_bounds(pos, dims) {
    Some((pos.x + pos.y * dims.x).try_into().unwrap())
  } else {
    None
  }
}

fn to_pos(index: usize, dims: UVec2) -> Option<UVec2> {
  let c_index: u32 = index.try_into().unwrap();
  let y = c_index / dims.x;
  let x = c_index - y * dims.x;
  let pos = UVec2::new(x, y);
  if pos.cmplt(dims).all() { Some(pos) } else { None }
}

fn within_bounds(pos:UVec2, dims: UVec2) -> bool {
  pos.cmplt(dims).all()
}

impl<PixelFormat: Clone> ImageTarget<PixelFormat> {
  pub fn new(dims: UVec2, color: PixelFormat) -> ImageTarget<PixelFormat> {
    ImageTarget {
      dims,
      pixels: vec![color; (dims.x * dims.y).try_into().unwrap()],
    }
  }

  pub fn dims(&self) -> UVec2 {
    self.dims
  }

  pub fn get(&self, pos: UVec2) -> Option<&PixelFormat> {
    to_index(pos, self.dims).and_then(|index|self.pixels.get(index))
  }

  pub fn get_mut(&mut self, pos: UVec2) -> Option<&mut PixelFormat> {
    to_index(pos,  self.dims).and_then(move |index| self.pixels.get_mut(index))
  }

  pub fn iter(&self) -> impl Iterator<Item = (UVec2, &PixelFormat)> {
    let dims = self.dims;
    self.pixels.iter().enumerate().filter_map(move |(index, value)| to_pos(index, dims).map(|index| (index, value)))
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = (UVec2, &mut PixelFormat)> {
    let dims = self.dims;
    self.pixels.iter_mut().enumerate().filter_map(move |(index, value)| to_pos(index, dims).map(|index| (index, value)))
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use glam::Vec3;

  #[test]
  fn test_allocates_fixed() {
    let mut target = ImageTarget::new(UVec2::new(1024, 1024), Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(target.dims(), UVec2::new(1024, 1024));
    assert_eq!(target.iter().count(), 1024 * 1024);
    assert_eq!(target.iter_mut().count(), 1024 * 1024);
    for x in 0..1024 {
      for y in 0..1024 {
        assert_eq!(target.get(UVec2::new(x, y)), Some(&Vec3::new(0.0, 0.0, 0.0)));
        assert_eq!(target.get_mut(UVec2::new(x, y)), Some(&mut Vec3::new(0.0, 0.0, 0.0)));
      }
    }
  }

  #[test]
  fn test_index_setter() {
    let mut target = ImageTarget::new(UVec2::new(1024, 1024), Vec3::new(0.0, 0.0, 0.0));
    for (cord, pixel) in target.iter_mut() {
      *pixel = Vec3::new(cord.x as f32, cord.y as f32, 0.0);
    }
    for x in 0..1024 {
      for y in 0..1024 {
        assert_eq!(target.get(UVec2::new(x, y)), Some(&Vec3::new(x as f32, y as f32, 0.0)));
        assert_eq!(target.get_mut(UVec2::new(x, y)), Some(&mut Vec3::new(x as f32, y as f32, 0.0)));
      }
    }
  }
}
