use super::ray::Ray;
use super::vec::Vec3;
use super::hitable::Hitable;
use super::hit_record::HitRecord;

pub struct Sphere {
    pub radius: f64,
    pub center: Vec3
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;

            if temp < tmax && temp > tmin {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - center) / radius;
                return true;
            }

            temp = (-b + (b * b - a * c).sqrt())/a;
            if temp < tmax && temp > tmin {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - center) / radius;
                return true;
            }
        }
        return false;
    }
}