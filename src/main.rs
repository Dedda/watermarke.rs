extern crate image;

use image::{DynamicImage, GenericImageView, GenericImage, Pixel};

fn main() {
    let watermark_bytes = include_bytes!("watermark.png");
    let watermark = image::load_from_memory(watermark_bytes).unwrap();
    println!("Watermark original size: {} x {}", watermark.width(), watermark.height());
    let args = std::env::args();
    let mut args = args.into_iter();
    args.next();
    args.map(|filename| (filename.clone(), image::open(&filename).unwrap()))
        .map(|(filename, pic)| (format!("watermarked_{}", filename), watermark_pic(pic, &watermark)))
        .for_each(|(filename, pic)| pic.save(filename).unwrap());
}

fn watermark_pic(mut pic: DynamicImage, watermark: &DynamicImage) -> DynamicImage {
    let watermark_w = watermark.width();
    let watermark_h = watermark.height();
    let watermark_ratio = watermark_w as f32 / watermark_h as f32;
    let pic_w = pic.width();
    let pic_h = pic.height();
    let watermark_max_w = pic_w / 7;
    let watermark_max_h = pic_h / 12;
    let offset_x = (pic.width() as f32 * 0.05) as u32;
    let offset_y = (pic.height() as f32 * 0.05) as u32;
    let (watermark_target_w, watermark_target_h) = if (watermark_max_w as f32 / watermark_max_h as f32) > watermark_ratio {
        ((watermark_max_h as f32 * watermark_ratio) as u32,
         watermark_max_h)
    } else {
        (watermark_max_w,
         (watermark_max_w as f32 / watermark_ratio) as u32)
    };
    let watermark_scaled = watermark.clone().resize_exact(watermark_target_w, watermark_target_h, image::imageops::FilterType::Gaussian);
    for x in 0..watermark_scaled.width() {
        for watermark_y in 0..watermark_scaled.height() {
            let pic_y = pic_h - offset_y - watermark_target_h + watermark_y;
            let mut watermark_pixel = watermark_scaled.get_pixel(x, watermark_y);
            watermark_pixel.0[3] = (watermark_pixel.0[3] as f32 * 0.7) as u8;
            let mut pic_pixel = pic.get_pixel(x + offset_x, pic_y);
            pic_pixel.blend(&watermark_pixel);
            pic.put_pixel(x + offset_x, pic_y, pic_pixel);

        }
    }
    pic
}