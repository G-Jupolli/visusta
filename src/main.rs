use image::{
    GenericImageView, ImageReader, RgbImage,
    imageops::FilterType::{self},
};
use std::path::Path;

use crate::sobel::SobelFilter;
use crate::utils::LuminanceFilter;

mod sobel;
mod utils;

fn _try_img() -> anyhow::Result<()> {
    let img = ImageReader::open("test_img_1.jpg")?.decode()?;

    println!("Has Some img");

    let (width, height) = img.dimensions();

    let down = img.resize(width / 8, height / 8, FilterType::Lanczos3);

    println!("Down scaled");

    let up = down.resize(width, height, FilterType::Lanczos3);

    println!("Up scaled");
    println!("Now save as scaled");

    let _ = up.save("scaled.jpg")?;

    Ok(())
}

// fn downsample_avg_parallel(img: &DynamicImage, block_size: usize) -> RgbaImage {
//     let rgba: RgbaImage = img.to_rgba8();

//     let width = rgba.width() as usize;
//     let height = rgba.height() as usize;

//     let new_width = (width + block_size - 1) / block_size;
//     let new_height = (height + block_size - 1) / block_size;

//     let raw = rgba.as_raw();

//     // Need all RGBA channels
//     // Idk if this is the best way to do this
//     // just to have a buffer, maybe need the RBG crate?
//     let mut buff = vec![0u8; new_height * new_width * 4];

//     buff.par_chunks_mut(4).enumerate().for_each(|(i, chunk)| {
//         let start_x = (i % new_width) * block_size;
//         let start_y = (i / new_width) * block_size;

//         let end_x = min(start_x + block_size, width);
//         let end_y = min(start_y + block_size, height);

//         let mut sum_r: u32 = 0;
//         let mut sum_g: u32 = 0;
//         let mut sum_b: u32 = 0;
//         let mut sum_a: u32 = 0;
//         let mut count: u32 = 0;

//         for y in start_y..end_y {
//             let row_base = y * width * 4;
//             for x in start_x..end_x {
//                 let pix_base = row_base + x * 4;
//                 let r = raw[pix_base] as u32;
//                 let g = raw[pix_base + 1] as u32;
//                 let b = raw[pix_base + 2] as u32;
//                 let a = raw[pix_base + 3] as u32;

//                 sum_r += r;
//                 sum_g += g;
//                 sum_b += b;
//                 sum_a += a;
//                 count += 1;
//             }
//         }

//         if count > 0 {
//             // chunk[0] = ((sum_r * 59) / (count * 100)) as u8;
//             // chunk[1] = ((sum_g * 3) / (count * 10)) as u8;
//             // chunk[2] = ((sum_b * 11) / (count * 100)) as u8;
//             //
//             // chunk[0] = ((sum_r * 3) / (count * 10)) as u8;
//             // chunk[1] = ((sum_g * 59) / (count * 100)) as u8;
//             // chunk[2] = ((sum_b * 11) / (count * 100)) as u8;
//             //
//             // let a_fac =

//             let luminance_sum =
//                 2126 * (sum_r / count) + 7152 * (sum_g / count) + 0722 * (sum_b / count);

//             let mut luminance = (luminance_sum / 10000) as u8;

//             // need to get the luminance value as 0 => 10

//             // chunk[0] = ((sum_r * 3) / (count * 10)) as u8;
//             // chunk[1] = ((sum_g * 59) / (count * 100)) as u8;
//             // chunk[2] = ((sum_b * 11) / (count * 100)) as u8;

//             let levels = 10;
//             let step = 255 / levels; // 256/10 = 25

//             // Compute discrete level
//             let mut discrete = luminance / step;
//             if discrete >= levels {
//                 discrete = levels - 1;
//             }

//             // Map back to 0–255 scale
//             luminance = discrete * (255 / (levels - 1));

//             chunk[0] = luminance;
//             chunk[1] = luminance;
//             chunk[2] = luminance;

//             chunk[3] = (sum_a / count) as u8;
//         }
//     });

//     RgbaImage::from_raw(new_width as u32, new_height as u32, buff)
//         .expect("from_raw should succeed with correctly sized buffer")

//     // let x = DynamicImage::ImageRgba8(
//     //     .expect("fuck"),
//     // );

//     // // Construct an ImageBuffer from raw bytes
//     // ImageBuffer::from_raw(new_width as u32, new_height as u32, buff)
//     //     .expect("from_raw should succeed with correctly sized buffer")
// }

// fn as_text(img: &DynamicImage, block_x: usize, block_y: usize) -> Vec<char> {
//     let rgba: RgbaImage = img.to_rgba8();

//     let width = rgba.width() as usize;
//     let height = rgba.height() as usize;

//     let new_width = (width + block_x - 1) / block_x;
//     let new_height = (height + block_y - 1) / block_y;

//     let raw = rgba.as_raw();

//     // Need all RGBA channels
//     // Idk if this is the best way to do this
//     // just to have a buffer, maybe need the RBG crate?
//     let mut buff = vec![' '; new_height * new_width];

//     buff.par_iter_mut().enumerate().for_each(|(i, c)| {
//         let start_x = (i % new_width) * block_x;
//         let start_y = (i / new_width) * block_y;

//         let end_x = min(start_x + block_x, width);
//         let end_y = min(start_y + block_y, height);

//         let mut sum_r: u32 = 0;
//         let mut sum_g: u32 = 0;
//         let mut sum_b: u32 = 0;
//         let mut sum_a: u32 = 0;
//         let mut count: u32 = 0;

//         for y in start_y..end_y {
//             let row_base = y * width * 4;
//             for x in start_x..end_x {
//                 let pix_base = row_base + x * 4;
//                 let r = raw[pix_base] as u32;
//                 let g = raw[pix_base + 1] as u32;
//                 let b = raw[pix_base + 2] as u32;
//                 let a = raw[pix_base + 3] as u32;

//                 sum_r += r;
//                 sum_g += g;
//                 sum_b += b;
//                 sum_a += a;
//                 count += 1;
//             }
//         }

//         if sum_a > 0 && count > 0 {
//             let luminance_sum =
//                 2126 * (sum_r / count) + 7152 * (sum_g / count) + 0722 * (sum_b / count);

//             let luminance = (luminance_sum / 10000) as u8;

//             let levels = 10;
//             let step = 255 / levels; // 256/10 = 25

//             // Compute discrete level
//             let mut discrete = luminance / step;
//             if discrete >= levels {
//                 discrete = levels - 1;
//             }

//             *c = match discrete {
//                 0 => '#',
//                 1 => '@',
//                 2 => '?',
//                 3 => '0',
//                 4 => 'P',
//                 5 => 'o',
//                 6 => 'c',
//                 7 => ' ',
//                 8 => ' ',
//                 9 => ' ',
//                 _ => {
//                     panic!("Illegal discrete state");
//                 }
//             };
//         }
//     });

//     buff
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("pagani.jpg");
    let img = image::open(path)?
        // Sizings
        // .resize(1280, 720, FilterType::Lanczos3)
    // .resize(2560, 1440, FilterType::Lanczos3)
;

    let rgb: RgbImage = img.to_rgb8();

    let width = rgb.width() as usize;
    let height = rgb.height() as usize;

    let raw = rgb.as_raw();

    let luminance = LuminanceFilter::rgb_luminance_u8(raw, width, height);

    let sobel_out = SobelFilter::to_direction_colour(luminance, width, height);

    let res = RgbImage::from_raw(width as u32, height as u32, sobel_out)
        .expect("Should be able to do this");

    let _ = res.save("pagani_sobel_dir_mix.png");

    Ok(())
}

// fn main() {
//     let _ = try_img().inspect_err(|err| {
//         println!("Oopsie {err:?}");
//     });
// }
