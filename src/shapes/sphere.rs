use glam::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Ray> {
        let e : Vec3 = ray.point();
        let d : Vec3 = ray.direction();
        let c : Vec3 = self.center;
        let r : f32 = self.radius;

        println!("e = {} d = {} c = {} r = {}", e, d, c, r);

        let b : Vec3 = 2.0 * d * (e - c);
        let a : Vec3 = d * d;
        let c_ : Vec3 = (e - c) * (e - c) - Vec3::splat(r * r);

        println!("b = {} a = {} c_ = {}", b, a, c_);

        let root : Vec3 = b * b - 4.0 * a * c_;

        println!("root = {}", root);

        Some(Ray::new((-b + root) / (a * Vec3::splat(2.0)), (-b - root) / (a * Vec3::splat(2.0))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{vec2, uvec2};

    #[test]
    fn test_walks_all_indexes() {
        let sphere = Sphere::new(Vec3::new(10.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Vec3::ZERO, Vec3::X);

        let option = sphere.intersect(&ray);
        assert_eq!(option, Some(Ray::new(Vec3::new(9.0, 0.0, 0.0), -Vec3::X)));
    }
}
