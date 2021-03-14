use crate::range::range;
use glam::Vec3;
use ray::target::ImageTarget;
use std::{io::BufWriter, iter::FromIterator};

fn f_to_b(f: f32) -> u8 {
    (f * 255.0) as u8
}

pub fn write_target(target: &ImageTarget<Vec3>, path: &std::path::Path) -> std::io::Result<()> {
    let dims = target.dims();
    let vec = Vec::from_iter(
        range(dims)
            .flat_map(|coord| {
                target
                    .get(coord)
                    .map(|vec| vec![f_to_b(vec.x), f_to_b(vec.y), f_to_b(vec.z)])
            })
            .flatten(),
    );

    let file = std::fs::File::create(path)?;
    let ref mut writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, dims.x, dims.y);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&vec)?;
    Ok(())
}
