use std::{cmp::min, path::Path};

use crate::{
    gaussians::{GaussianBuilder, GaussianFilter},
    sobel::SobelFilter,
    utils::LuminanceFilter,
};
use image::{
    RgbImage,
    imageops::FilterType::{self},
};

mod gaussians;
// mod pixel;
mod sobel;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("jinx_2.jpg");

    let img = image::open(path)?
        // Sizings
        .resize(944, 531, FilterType::Lanczos3)
        // .resize(1280, 720, FilterType::Lanczos3)
        // .resize(2560, 1440, FilterType::Lanczos3)
    //buff
    ;

    let rgb: RgbImage = img.to_rgb8();

    let width = rgb.width() as usize;
    let height = rgb.height() as usize;

    let raw = rgb.as_raw();

    let out = LuminanceFilter::rgb_luminance_u8(raw, width, height)
        // To flip on y axis
        // .flip_y()
        // .to_raw_rgb()
    //check
    ;

    let gaussian_kernel = GaussianBuilder::create(0.5, 2.25)
        .scalar(0.5)
        .cutoff(40.00)
        .build_kernel();

    let out =
        GaussianFilter::gaussian_on_luminance(out, gaussian_kernel)
            // check
            // .to_raw_rgb()
            // Check
            // .boolean_mask_rgb(raw)
            // check
        ;

    //     let out = LuminanceFilter::rgb_luminance_u8(&out, width, height)
    //     // To flip on y axis
    //     // .flip_y()
    //     // check
    // ;

    // let buff_move = LuminanceBuff { buff: out };

    // let out = SobelFilter::to_direction_colour(&out, width, height);
    // let out = SobelFilter::to_direction_colour_overlay_on_dither(&out, raw);

    // -----
    let font_size = 8;
    let res_dir = SobelFilter::to_ascii_direction(&out, font_size, 10, 0.65);
    let res_lum = out.to_ascii(font_size);

    for y in 0..out.height.div_ceil(font_size) {
        let new_width = out.width.div_ceil(font_size);
        let start_idx = y * new_width;
        let end_idx = min(start_idx + new_width, res_dir.len());

        let mut out_buff = String::new();

        for idx in start_idx..end_idx {
            let dir = res_dir[idx];

            if dir == ' ' {
                out_buff.push(res_lum[idx]);
            } else {
                out_buff.push(dir);
            }
            out_buff.push(' ');
        }

        // for c in &res_dir[start_idx..end_idx] {
        // out_buff.push(*c);
        // out_buff.push(' ');
        // }

        println!("{out_buff}");
    }

    // let res =
    //     RgbImage::from_raw(width as u32, height as u32, out).expect("Should be able to do this");

    // let _ = res.save("circle_gauss_ascii.png");

    Ok(())
}
