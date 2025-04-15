use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, Pixel};

pub fn to_8bit(img: &DynamicImage, pixel_size: u32, color_levels: u8) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut img_8bit = ImageBuffer::new(width, height);

    for y in (0..height).step_by(pixel_size as usize) {
        for x in (0..width).step_by(pixel_size as usize) {
            let avg_color = get_average_color(&img, x, y, pixel_size, height, width);
            let quant_color = quantize_color(avg_color, color_levels);

            for dy in 0..pixel_size {
                for dx in 0..pixel_size {
                    if x + dx < width && y + dy < height {
                        img_8bit.put_pixel(x + dx, y + dy, quant_color);
                    }
                }
            }
        }
    }

    img_8bit
}

fn get_average_color(img: &DynamicImage, x: u32, y: u32, block_size: u32, height: u32, width: u32) -> Rgb<u8> {
    let mut r_total = 0;
    let mut g_total = 0;
    let mut b_total = 0;
    let mut count = 0;


    for dy in 0..block_size {
        for dx in 0..block_size {
            if x + dx < width && y + dy < height {
                let pixel = img.get_pixel(x + dx, y + dy);
                let rgb = pixel.to_rgba();
                r_total += rgb[0] as u32;
                g_total += rgb[1] as u32;
                b_total += rgb[2] as u32;
                count += 1;
            }
        }
    }

    if count == 0 {
        return Rgb([0, 0, 0]);
    }

    Rgb([
        (r_total / count) as u8,
        (g_total / count) as u8,
        (b_total / count) as u8,
    ])
}


fn quantize_color(color: Rgb<u8>, levels: u8) -> Rgb<u8> {
    let step = 255 / (levels - 1) as u8;
    let r = (color[0] / step) * step;
    let g = (color[1] / step) * step;
    let b = (color[2] / step) * step;

    Rgb([r, g, b])
}
