use std::path::Path;

use image::{
    GenericImageView, ImageReader, RgbImage,
    imageops::FilterType::{self},
};

use crate::{sobel::SobelFilter, utils::LuminanceFilter};

mod gaussians;
// mod pixel;
mod sobel;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let buff = LuminanceBuff {
    //     buff: vec![100, 100, 100, 100, 200, 100, 100, 100, 100],
    // };

    // let gaussed = GaussianFilter::filter_luminance(buff, 3, 3, 0.5, 1.0);

    // println!("Gauss : {gaussed:?}");

    let path = Path::new("owl.jpg");
    let img = image::open(path)?
        // Sizings
        // .resize(1280, 720, FilterType::Lanczos3)
        // .resize(2560, 1440, FilterType::Lanczos3)
    //buff
    ;

    let rgb: RgbImage = img.to_rgb8();

    let width = rgb.width() as usize;
    let height = rgb.height() as usize;

    let raw = rgb.as_raw();

    let out = LuminanceFilter::rgb_luminance_u8(raw, width, height);

    // let gauss_out = GaussianFilter::apply_gaussian(luminance, width, height, 0.5, 1.0);
    // let out = GaussianFilter::apply_gaussian(luminance, width, height, 1.0, 1.75);

    // let buff_move = LuminanceBuff { buff: out };

    let out = SobelFilter::to_direction_colour(&out, width, height);

    let res =
        RgbImage::from_raw(width as u32, height as u32, out).expect("Should be able to do this");

    let _ = res.save("owl.png");

    Ok(())
}

fn _try_img() -> anyhow::Result<()> {
    let img = ImageReader::open("test_img_1.jpg")?.decode()?;

    println!("Has Some img");

    let (width, height) = img.dimensions();

    let down = img.resize(width / 8, height / 8, FilterType::Lanczos3);

    println!("Down scaled");

    let up = down.resize(width, height, FilterType::Lanczos3);

    println!("Up scaled");
    println!("Now save as scaled");

    up.save("scaled.jpg")?;

    Ok(())
}

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
