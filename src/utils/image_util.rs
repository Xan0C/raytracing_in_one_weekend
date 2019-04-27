mod math;

pub fn generate_ppm(width: u32, height: u32) -> Vec<u8> {
    let nx = width;
    let ny = height;

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend_from_slice(format!("P3\n{} {}\n255\n", nx, ny).as_bytes());

    for j in (0..ny).rev()  {
        for i in 0..nx {
            let color = vec3 {
                x: i as f64 / nx as f64,
                y: j as f64 / ny as f64,
                z: 0.2           
            };
            let icolor = vec3 {
                x: (255.99 * color.x),
                y: (255.99 * color.y),
                z: (255.99 * color.z) 
            };

            buffer.extend_from_slice(format!("{} {} {}\n", icolor.x, icolor.y, icolor.z).as_bytes());
        }
    }

    return buffer;
}