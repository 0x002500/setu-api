use image::{ImageBuffer, Rgba};
use std::path::Path;

use crate::Color;

pub fn generate_pictures(color: Color) -> Result<(), Box<dyn std::error::Error>> {
    let width = 256;
    let height = 256;

    let mut img = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let r = color.r;
            let g = color.g;
            let b = color.b;
            let a = color.a;

            let pixel = Rgba([r, g, b, a]);

            img.put_pixel(x, y, pixel);
        }
    }

    let output_path = Path::new("output.png");

    println!("正在保存图片到: {}", output_path.display());
    img.save_with_format(output_path, image::ImageFormat::Png)?;

    println!("图片已成功生成！");

    Ok(())
}
