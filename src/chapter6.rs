use super::vec::Vec3;
use super::ray::Ray;
use super::color;
use super::hit_record::HitRecord;
use super::hitable::HitableList;
use super::sphere::Sphere;

fn colorRay(r: Ray, world: &Hitable) -> Vec3 {
    let mut rec: HitRecord;

    if world.hit(r, 0.0, f32::MAX, &rec) {
        return Vec3 { x: rec.normal.x + 1.0, y: rec.normal.y + 1.0, z: rec.normal.z + 1.0 };
    } 

    let unit_direction = r.direction().normalize();
    let t = 0.5 * unit_direction.y + 1.0;
    return Vec3 { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t) + Vec3 { x: 0.5, y: 0.7, z: 1.0 } * t;
}

pub fn run(width:u32, height: u32) -> Vec<u8> {
    let nx = width;
    let ny = height;

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend_from_slice(format!("P3\n{} {}\n255\n", nx, ny).as_bytes());

    let lower_left_corner: Vec3 = Vec3 { x: -2.0, y: -1.0, z: -1.0 };
    let horizontal: Vec3 = Vec3 { x: 4.0, y: 0.0, z: 0.0 };
    let vertical: Vec3 = Vec3 { x: 0.0, y: 2.0, z: 0.0 };
    let origin: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    let world = HitableList { list:   Vec::with_capacity(2) };
    world.list.push( Sphere { center: Vec3 { x: 0.0, y: -1.0, z: -1.0 }, radius: 0.5 } );
    world.list.push( Sphere { center: Vec3 { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0 } );

    for j in (0..ny).rev()  {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;

            let r = Ray { a: origin, b: lower_left_corner + horizontal * u + vertical * v };
            let p = r.point_at_parameter(2.0);
            let col = colorRay(r, &world);

            let icolor = color::Color (
                (255.99 * col.x) as u8,
                (255.99 * col.y) as u8,
                (255.99 * col.z) as u8
            );

            buffer.extend_from_slice(format!("{} {} {}\n", icolor.0, icolor.1, icolor.2).as_bytes());
        }
    }

    return buffer;
}