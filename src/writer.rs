use std::{io::BufWriter, iter::FromIterator};
use glam::Vec3;
use crate::target::ImageTarget;

pub fn write_target(target: &ImageTarget<Vec3>, path: &std::path::Path) -> std::io::Result<()>{
  let dims = target.dims();
  let vec = Vec::from_iter(target.iter().flat_map(|(_, vec)| vec![vec.x as u8, vec.y as u8, vec.z as u8]));

  let file = std::fs::File::create(path)?;
  let ref mut writer = BufWriter::new(file);
  let mut encoder = png::Encoder::new(writer, dims.x, dims.y);
  encoder.set_color(png::ColorType::RGB);
  encoder.set_depth(png::BitDepth::Eight);
  let mut writer = encoder.write_header()?;
  writer.write_image_data(&vec)?;
  Ok(())
}