mod perspective;
mod projection;

use glam::Vec3;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Ray {
    point: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(point: Vec3, direction: Vec3) -> Ray {
        Ray {
            point, direction,
        }
    }
    pub fn point(&self) -> Vec3 {
        self.point
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
