use super::vec::Vec3;
use super::ray::Ray;
use super::hit_record::HitRecord;

pub trait Hitable {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &HitRecord) -> bool;
}

pub struct HitableList {
  pub list: Vec<Hitable>,
}

impl HitableList {
  pub fn new() -> HitableList {
        let xs = Vec::new();
        HitableList { list: xs }
    }
}

impl Hitable for HitableList {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &HitRecord) -> bool {
    let mut temp_rec: HitRecord;
    let mut hit_anything = false;

    let mut closest_so_far = tmax;

    for i in (0..self.size).rev()  {
      if self.list[i].hit(r, tmin, closest_so_far, temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        rec = temp_rec;
      }
    }
    return hit_anything;
}

/*
impl fmt::Debug for Hitable {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Hitable {{ aabb: {:?} }}", self.bounding_box())
  }
}
*/