mod target;
mod writer;

use std::path::Path;

use glam::{UVec2, Vec3};

fn main() {
    let dims = UVec2::new(1024, 1024);
    let target = target::ImageTarget::new(dims, Vec3::new(0.0, 0.0, 0.0));

    writer::write_target(&target, Path::new("out.png")).unwrap();
}
