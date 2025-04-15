pub mod image_tools;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_image>", args[0]);
        std::process::exit(1);
    }

    let input_image = &args[1];
    let img = image::open(input_image).expect("Fail to open image");
    let img_as_8bit = image_tools::to_8bit(&img, 4, 16);
    let output_path = format!("8bit_{}", input_image);

    img_as_8bit.save(&output_path)
        .expect("Failed to save image");

    println!("Saved 8-bit image to {}", output_path);
}
