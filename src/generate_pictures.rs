use image::{ImageBuffer, Rgba};
use std::io::Cursor;

use crate::{Color, Size};

async fn generate_pictures(color: Color, size: Size) -> Vec<u8>{
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::<Rgba<u8>, _>::from_fn(size.width, size.height, |_x: u32, _y: u32| Rgba([color.r, color.g, color.b, color.a]));

    let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageFormat::Png)
        .unwrap();

    buf.into_inner()
}
