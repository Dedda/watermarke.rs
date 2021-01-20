extern crate image;

use image::{DynamicImage, GenericImageView, GenericImage, Pixel};

const WATERMARK_MAX_WIDTH_SHARE: u32 = 7;
const WATERMARK_MAX_HEIGHT_SHARE: u32 = 12;
const OFFSET_X_SHARE: f32 = 0.05;
const OFFSET_Y_SHARE: f32 = 0.05;
const ALPHA: usize = 3;
const WATERMARK_ALPHA_INTENSITY: f32 = 0.7;
const WATERMARKED_PREFIX: &'static str = "watermarked_";

fn main() {
    let watermark_bytes = include_bytes!("watermark.png");
    let watermark = image::load_from_memory(watermark_bytes).unwrap();
    let args = std::env::args();
    let mut args = args.into_iter();
    args.next();
    args.map(|filename| {
        println!("Processing file `{}`...", filename);
        (filename.clone(), image::open(&filename).unwrap())
    })
        .map(|(filename, pic)| (format!("{}{}", WATERMARKED_PREFIX, filename), watermark_pic(pic, &watermark)))
        .for_each(|(filename, pic)| pic.save(filename).unwrap());
}

fn watermark_pic(mut pic: DynamicImage, watermark: &DynamicImage) -> DynamicImage {
    let pic_size = pic.dimensions();
    let (offset_x, offset_y) = offset(pic_size);
    let (watermark_target_w, watermark_target_h) = target_watermark_size(pic_size, (watermark.width(), watermark.height()));
    let watermark_scaled = watermark.clone().resize_exact(watermark_target_w, watermark_target_h, image::imageops::FilterType::Gaussian);
    for x in 0..watermark_scaled.width() {
        for watermark_y in 0..watermark_scaled.height() {
            let pic_y = pic_size.1 - offset_y - watermark_target_h + watermark_y;
            let mut watermark_pixel = watermark_scaled.get_pixel(x, watermark_y);
            watermark_pixel.0[ALPHA] = (watermark_pixel.0[ALPHA] as f32 * WATERMARK_ALPHA_INTENSITY) as u8;
            let mut pic_pixel = pic.get_pixel(x + offset_x, pic_y);
            pic_pixel.blend(&watermark_pixel);
            pic.put_pixel(x + offset_x, pic_y, pic_pixel);
        }
    }
    pic
}

fn offset(pic_size: (u32, u32)) -> (u32, u32) {
    ((pic_size.0 as f32 * OFFSET_X_SHARE) as u32,
     (pic_size.1 as f32 * OFFSET_Y_SHARE) as u32)
}

fn target_watermark_size(pic_size: (u32, u32), watermark_size: (u32, u32)) -> (u32, u32) {
    let (watermark_w, watermark_h) = watermark_size;
    let watermark_ratio = watermark_w as f32 / watermark_h as f32;
    let watermark_max_w = pic_size.0 / WATERMARK_MAX_WIDTH_SHARE;
    let watermark_max_h = pic_size.1 / WATERMARK_MAX_HEIGHT_SHARE;
    if (watermark_max_w as f32 / watermark_max_h as f32) > watermark_ratio {
        ((watermark_max_h as f32 * watermark_ratio) as u32,
         watermark_max_h)
    } else {
        (watermark_max_w,
         (watermark_max_w as f32 / watermark_ratio) as u32)
    }
}