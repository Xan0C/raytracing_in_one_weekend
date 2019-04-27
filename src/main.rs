mod utils { pub mod image_util; }
mod math { pub mod vec3; }

use std::fs::File;
use std::io::prelude::*;

fn main() {

    let buffer = utils::image_util::generate_ppm(200, 100);
    let mut output = File::create("output.ppm").expect("Failed to create file output.ppm");
    output.write_all(&buffer[..]).expect("Failed to write PPM to output.ppm");

}
