pub mod image_tools;
use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, help = "Image to be converted to 8-bit")]
    image: String,

    #[arg(short, long, default_value = "", help = "Output image path")]
    output: String,

    #[arg(short, long, default_value_t = 2, help="Pixel size. The higher the value, less clear the image will be.")]
    pixel_size: u32,

    #[arg(short, long, default_value_t = 16, help="Number of color levels to reduce the palette")]
    color_levels: u8
}

fn main() {
    let args = Args::parse();

    let input_image = &args.image;
    let pixel_size = args.pixel_size;
    let color_levels = args.color_levels;
    let output_image = &args.output;
    let img = image::open(input_image).expect("Fail to open image");
    let img_as_8bit = image_tools::to_8bit(&img, pixel_size, color_levels);

    let output_path = if output_image.is_empty() {
        let input_path = Path::new(input_image);
        let file_name = input_path.file_name().unwrap_or_default();
        PathBuf::from("./assets").join(format!("8bit_{}", file_name.to_string_lossy()))
    } else {
        PathBuf::from(output_image)
    };

    img_as_8bit.save(&output_path)
        .expect("Failed to save image");

    println!("Saved 8-bit image to {}", output_path.display());
}
