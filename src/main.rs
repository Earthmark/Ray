mod logdur;
mod range;
pub mod target;
mod writer;

use std::path::Path;

use glam::{UVec2, Vec3};
use log::info;
use logdur::ChroMark;
use ray::target::ImageTarget;

fn main() {
    log4rs::init_file("log_config.yml", Default::default()).unwrap();

    let dur = ChroMark::new();

    let dims = UVec2::new(1024, 1024);
    let mut target = ImageTarget::new(dims, Vec3::new(0.0, 0.0, 0.0));

    info!("Frame created in {}", dur);
    let dur = ChroMark::new();

    for index in range::range(target.dims()) {
        if let Some(pixel) = target.get_mut(index) {
            let pos = Vec3::new(index.x as f32 / 1024.0 - 0.5, index.y as f32 / 1024.0 - 0.5, 0.0);

            *pixel = if pos.length() < 0.1 { Vec3::new(1.0, 1.0, 1.0) } else { Vec3::ZERO };
        }
    }

    info!("Frame modified in {}", dur);
    let dur = ChroMark::new();

    writer::write_target(&target, Path::new("out.png")).unwrap();

    info!("Frame saved in {}", dur);
}
