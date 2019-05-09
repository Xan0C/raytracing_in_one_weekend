mod ray;
mod vec;
mod color;
mod hit_record;
//mod hitable;
//mod sphere;

mod chapter2;
mod chapter3;
mod chapter4;
mod chapter5;
//mod chapter6;

use std::fs::File;
use std::io::prelude::*;

fn main() {

    // Chapter 1
    /*
    let buffer = chapter2::run(200, 100);
    let mut output = File::create("output.ppm").expect("Failed to create file output.ppm");
    output.write_all(&buffer[..]).expect("Failed to write PPM to output.ppm");
    */

    // Chapter 3
    /*
    let buffer = chapter3::run(200, 100);
    let mut output = File::create("output.ppm").expect("Failed to create file output.ppm");
    output.write_all(&buffer[..]).expect("Failed to write PPM to output.ppm");
    */

    // Chapter 4
    /*
    let buffer = chapter4::run(200, 100);
    let mut output = File::create("output.ppm").expect("Failed to create file output.ppm");
    output.write_all(&buffer[..]).expect("Failed to write PPM to output.ppm");
    */
    // Chapter 5
    
    let buffer = chapter5::run(200, 100);
    let mut output = File::create("output.ppm").expect("Failed to create file output.ppm");
    output.write_all(&buffer[..]).expect("Failed to write PPM to output.ppm");
    

    // Chapter 6
    /*
    let buffer = chapter6::run(200, 100);
    let mut output = File::create("output.ppm").expect("Failed to create file output.ppm");
    output.write_all(&buffer[..]).expect("Failed to write PPM to output.ppm");
    */
}