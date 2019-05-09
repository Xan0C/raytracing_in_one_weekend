use super::vec;
use super::color;

pub fn run(width: u32, height: u32) -> Vec<u8> {
    let nx = width;
    let ny = height;

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend_from_slice(format!("P3\n{} {}\n255\n", nx, ny).as_bytes());

    for j in (0..ny).rev()  {
        for i in 0..nx {
            let col = vec::Vec3 {
                x: i as f64 / nx as f64,
                y: j as f64 / ny as f64,
                z: 0.2 as f64          
            };

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