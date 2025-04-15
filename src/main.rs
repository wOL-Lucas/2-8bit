pub mod image_tools;
use clap::Parser;

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
    let mut output_image = args.output.clone();
    let pixel_size = args.pixel_size;
    let color_levels = args.color_levels;
   
    let img = image::open(input_image).expect("Fail to open image");
    let img_as_8bit = image_tools::to_8bit(&img, pixel_size, color_levels);
    if output_image.is_empty() {
        output_image = format!("8bit_{}", input_image);
    }

    img_as_8bit.save(&output_image)
        .expect("Failed to save image");

    println!("Saved 8-bit image to {}", output_image);
}
