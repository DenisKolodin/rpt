use rand::{rngs::ThreadRng, Rng};
use rand_distr::UnitSphere;

use super::{HitRecord, Ray, Shape};
use crate::kdtree::{Bounded, BoundingBox};

/// A unit sphere centered at the origin
pub struct Sphere;

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64, record: &mut HitRecord) -> bool {
        // Translated directly from the GLOO source code, assuming radius = 1
        let a = glm::length2(&ray.dir);
        let b = 2.0 * glm::dot(&ray.dir, &ray.origin);
        let c = glm::length2(&ray.origin) - 1.0;

        let d = b * b - 4.0 * a * c;
        if d.is_sign_negative() {
            return false;
        }

        let d = d.sqrt();
        let t_plus = (-b + d) / (2.0 * a);
        let t_minus = (-b - d) / (2.0 * a);
        let t = if t_minus < t_min {
            if t_plus < t_min {
                return false;
            }
            t_plus
        } else {
            t_minus
        };

        if t < record.time {
            record.time = t;
            record.normal = ray.at(t).normalize();
            true
        } else {
            false
        }
    }

    fn sample(&self, rng: &mut ThreadRng) -> (glm::DVec3, glm::DVec3, f64) {
        let [x, y, z] = rng.sample(UnitSphere);
        let p = glm::vec3(x, y, z);
        (p, p, 0.25 * std::f64::consts::FRAC_1_PI)
    }
}

impl Bounded for Sphere {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            p_min: glm::vec3(-1.0, -1.0, -1.0),
            p_max: glm::vec3(1.0, 1.0, 1.0),
        }
    }
}
