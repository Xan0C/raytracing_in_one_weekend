use super::ray;
use super::vec;
use super::color;

fn colorRay(ray: ray::Ray) -> vec::Vec3 {
    let unit_direction = ray.direction().normalize();
    let t: f64 = 0.5 * unit_direction.y + 1.0;
    return vec::Vec3 { x: 1.0, y: 1.0, z: 1.0 } * (1.0-t) + vec::Vec3 { x: 0.5, y: 0.7, z: 1.0 } * t;
}

pub fn run(width:u32, height: u32) -> Vec<u8> {
    let nx = width;
    let ny = height;

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend_from_slice(format!("P3\n{} {}\n255\n", nx, ny).as_bytes());

    let lower_left_corner: vec::Vec3 = vec::Vec3 { x: -2.0, y: -1.0, z: -1.0 };
    let horizontal: vec::Vec3 = vec::Vec3 { x: 4.0, y: 0.0, z: 0.0 };
    let vertical: vec::Vec3 = vec::Vec3 { x: 0.0, y: 2.0, z: 0.0 };
    let origin: vec::Vec3 = vec::Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    for j in (0..ny).rev()  {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;

            let r = ray::Ray { a: origin, b: lower_left_corner + horizontal * u + vertical * v };
            let col = colorRay(r);

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
