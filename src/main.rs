use image::{
    GenericImageView, ImageReader, RgbImage,
    imageops::{
        self,
        FilterType::{self},
    },
};

fn try_img() -> anyhow::Result<()> {
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

use image::{DynamicImage, ImageBuffer, RgbaImage};
use rayon::prelude::*;
use std::{cmp::min, fs::File};
use std::{io::Write, path::Path};

fn downsample_avg_parallel(img: &DynamicImage, block_size: usize) -> RgbaImage {
    let rgba: RgbaImage = img.to_rgba8();

    let width = rgba.width() as usize;
    let height = rgba.height() as usize;

    let new_width = (width + block_size - 1) / block_size;
    let new_height = (height + block_size - 1) / block_size;

    let raw = rgba.as_raw();

    // Need all RGBA channels
    // Idk if this is the best way to do this
    // just to have a buffer, maybe need the RBG crate?
    let mut buff = vec![0u8; new_height * new_width * 4];

    buff.par_chunks_mut(4).enumerate().for_each(|(i, chunk)| {
        let start_x = (i % new_width) * block_size;
        let start_y = (i / new_width) * block_size;

        let end_x = min(start_x + block_size, width);
        let end_y = min(start_y + block_size, height);

        let mut sum_r: u32 = 0;
        let mut sum_g: u32 = 0;
        let mut sum_b: u32 = 0;
        let mut sum_a: u32 = 0;
        let mut count: u32 = 0;

        for y in start_y..end_y {
            let row_base = y * width * 4;
            for x in start_x..end_x {
                let pix_base = row_base + x * 4;
                let r = raw[pix_base] as u32;
                let g = raw[pix_base + 1] as u32;
                let b = raw[pix_base + 2] as u32;
                let a = raw[pix_base + 3] as u32;

                sum_r += r;
                sum_g += g;
                sum_b += b;
                sum_a += a;
                count += 1;
            }
        }

        if count > 0 {
            // chunk[0] = ((sum_r * 59) / (count * 100)) as u8;
            // chunk[1] = ((sum_g * 3) / (count * 10)) as u8;
            // chunk[2] = ((sum_b * 11) / (count * 100)) as u8;
            //
            // chunk[0] = ((sum_r * 3) / (count * 10)) as u8;
            // chunk[1] = ((sum_g * 59) / (count * 100)) as u8;
            // chunk[2] = ((sum_b * 11) / (count * 100)) as u8;
            //
            // let a_fac =

            let luminance_sum =
                2126 * (sum_r / count) + 7152 * (sum_g / count) + 0722 * (sum_b / count);

            let mut luminance = (luminance_sum / 10000) as u8;

            // need to get the luminance value as 0 => 10

            // chunk[0] = ((sum_r * 3) / (count * 10)) as u8;
            // chunk[1] = ((sum_g * 59) / (count * 100)) as u8;
            // chunk[2] = ((sum_b * 11) / (count * 100)) as u8;

            let levels = 10;
            let step = 255 / levels; // 256/10 = 25

            // Compute discrete level
            let mut discrete = luminance / step;
            if discrete >= levels {
                discrete = levels - 1;
            }

            // Map back to 0–255 scale
            luminance = discrete * (255 / (levels - 1));

            chunk[0] = luminance;
            chunk[1] = luminance;
            chunk[2] = luminance;

            chunk[3] = (sum_a / count) as u8;
        }
    });

    RgbaImage::from_raw(new_width as u32, new_height as u32, buff)
        .expect("from_raw should succeed with correctly sized buffer")

    // let x = DynamicImage::ImageRgba8(
    //     .expect("fuck"),
    // );

    // // Construct an ImageBuffer from raw bytes
    // ImageBuffer::from_raw(new_width as u32, new_height as u32, buff)
    //     .expect("from_raw should succeed with correctly sized buffer")
}

fn as_text(img: &DynamicImage, block_x: usize, block_y: usize) -> Vec<char> {
    let rgba: RgbaImage = img.to_rgba8();

    let width = rgba.width() as usize;
    let height = rgba.height() as usize;

    let new_width = (width + block_x - 1) / block_x;
    let new_height = (height + block_y - 1) / block_y;

    let raw = rgba.as_raw();

    // Need all RGBA channels
    // Idk if this is the best way to do this
    // just to have a buffer, maybe need the RBG crate?
    let mut buff = vec![' '; new_height * new_width];

    buff.par_iter_mut().enumerate().for_each(|(i, c)| {
        let start_x = (i % new_width) * block_x;
        let start_y = (i / new_width) * block_y;

        let end_x = min(start_x + block_x, width);
        let end_y = min(start_y + block_y, height);

        let mut sum_r: u32 = 0;
        let mut sum_g: u32 = 0;
        let mut sum_b: u32 = 0;
        let mut sum_a: u32 = 0;
        let mut count: u32 = 0;

        for y in start_y..end_y {
            let row_base = y * width * 4;
            for x in start_x..end_x {
                let pix_base = row_base + x * 4;
                let r = raw[pix_base] as u32;
                let g = raw[pix_base + 1] as u32;
                let b = raw[pix_base + 2] as u32;
                let a = raw[pix_base + 3] as u32;

                sum_r += r;
                sum_g += g;
                sum_b += b;
                sum_a += a;
                count += 1;
            }
        }

        if sum_a > 0 && count > 0 {
            let luminance_sum =
                2126 * (sum_r / count) + 7152 * (sum_g / count) + 0722 * (sum_b / count);

            let luminance = (luminance_sum / 10000) as u8;

            let levels = 10;
            let step = 255 / levels; // 256/10 = 25

            // Compute discrete level
            let mut discrete = luminance / step;
            if discrete >= levels {
                discrete = levels - 1;
            }

            *c = match discrete {
                0 => '#',
                1 => '@',
                2 => '?',
                3 => '0',
                4 => 'P',
                5 => 'o',
                6 => 'c',
                7 => ' ',
                8 => ' ',
                9 => ' ',
                _ => {
                    panic!("Illegal discrete state");
                }
            };
        }
    });

    buff
}

const MAX_SOBEL_SQ: f32 = 255.0 * 255.0;

fn sobel(img: &DynamicImage) -> RgbImage {
    let rgb: RgbImage = img.to_rgb8();

    let width = rgb.width() as usize;
    let height = rgb.height() as usize;

    let raw = rgb.as_raw();

    let mut luminance_buff = vec![0i32; width * height];

    luminance_buff
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, luminance)| {
            let base = idx * 3;

            let r = raw[base] as i32;
            let g = raw[base + 1] as i32;
            let b = raw[base + 2] as i32;

            *luminance = (77 * r + 150 * g + 29 * b) / 256;
        });

    // Going to be trying the config that there is on the wiki:
    //
    //      |  1  |
    // Gx = |  2  | * ( [ 1  0  -1 ] * A)
    //      |  1  |
    //
    //      |  1  |
    // Gy = |  0  | * ( [ 1  2  1 ] * A)
    //      | -1  |
    // let mut sobel_buff = vec![0u8; width * height * 3];

    // sobel_buff
    //     .par_chunks_mut(3)
    //     .enumerate()
    //     .for_each(|(idx, pix)| {
    //         // Maybe I need to find another way to do this
    //         let base_idx = idx * width;
    //         let row = idx / width;

    //         if row != 0 && row < height - 2 && base_idx != 0 && base_idx < width - 1 {
    //             let nw = luminance_buff[(idx - width) - 1];
    //             let n = luminance_buff[idx - width];
    //             let ne = luminance_buff[(idx - width) + 1];

    //             let w = luminance_buff[idx - 1];
    //             // c
    //             let e = luminance_buff[idx + 1];

    //             let sw = luminance_buff[(idx + width) - 1];
    //             let s = luminance_buff[idx + width];
    //             let se = luminance_buff[(idx + width) + 1];

    //             let g_x = (nw - ne) + (2 * (w - e)) + (sw - se);
    //             let g_y = (nw + (2 * n) + ne) - (sw + (2 * s) + se);

    //             let mag = (g_x * g_x + g_y * g_y) as f32;

    //             let normal = ((mag / MAX_SOBEL_SQ) * 255.0).min(255.0) as u8;

    //             pix[0] = normal;
    //             pix[1] = normal;
    //             pix[2] = normal;
    //         };
    //     });

    let mut sobel_buff = vec![0u8; width * height * 3];
    sobel_buff
        .par_chunks_mut(width * 3) // each row has width*3 bytes
        .enumerate()
        .for_each(|(y, row)| {
            if y == 0 || y == height - 1 {
                return;
            } // skip borders

            for x in 1..(width - 1) {
                // let idx = y * width + x;

                // Compute neighbors in luminance_buff
                let nw = luminance_buff[(y - 1) * width + (x - 1)];
                let n = luminance_buff[(y - 1) * width + x];
                let ne = luminance_buff[(y - 1) * width + (x + 1)];

                let w = luminance_buff[y * width + (x - 1)];
                let e = luminance_buff[y * width + (x + 1)];

                let sw = luminance_buff[(y + 1) * width + (x - 1)];
                let s = luminance_buff[(y + 1) * width + x];
                let se = luminance_buff[(y + 1) * width + (x + 1)];

                let g_x = (ne - nw) + 2 * (e - w) + (se - sw);
                let g_y = (sw + s * 2 + se) - (nw + n * 2 + ne);

                let mag_sq = (g_x * g_x + g_y * g_y) as f32;
                let normal = ((mag_sq / MAX_SOBEL_SQ) * 255.0).min(255.0) as u8;

                // Each row slice is independent, so this is safe
                let out_idx = x * 3;
                row[out_idx + 0] = normal;
                row[out_idx + 1] = normal;
                row[out_idx + 2] = normal;
            }
        });

    RgbImage::from_raw(width as u32, height as u32, sobel_buff).expect("Should be able to do this")
}
// const MAX_SOBEL_SQ: f32 = 4.0 * 255.0 * 4.0 * 255.0;

fn sobel_a(img: &DynamicImage) -> RgbaImage {
    let rgb: RgbaImage = img.to_rgba8();

    let width = rgb.width() as usize;
    let height = rgb.height() as usize;

    let raw = rgb.as_raw();

    let mut luminance_buff = vec![0i32; width * height];

    luminance_buff
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, luminance)| {
            let base = idx * 4;

            if raw[base + 3] < 50 {
                *luminance = 255;
                return;
            }

            let r = raw[base] as i32;
            let g = raw[base + 1] as i32;
            let b = raw[base + 2] as i32;

            *luminance = (77 * r + 150 * g + 29 * b) / 256;
        });

    // Going to be trying the config that there is on the wiki:
    //
    //      |  1  |
    // Gx = |  2  | * ( [ 1  0  -1 ] * A)
    //      |  1  |
    //
    //      |  1  |
    // Gy = |  0  | * ( [ 1  2  1 ] * A)
    //      | -1  |
    // let mut sobel_buff = vec![0u8; width * height * 3];

    let mut sobel_buff = vec![0u8; width * height * 4];
    sobel_buff
        .par_chunks_mut(width * 4) // each row has width*3 bytes
        .enumerate()
        .for_each(|(y, row)| {
            if y == 0 || y == height - 1 {
                return;
            } // skip borders

            for x in 1..(width - 1) {
                // let idx = y * width + x;

                // Compute neighbors in luminance_buff
                let nw = luminance_buff[(y - 1) * width + (x - 1)];
                let n = luminance_buff[(y - 1) * width + x];
                let ne = luminance_buff[(y - 1) * width + (x + 1)];

                let w = luminance_buff[y * width + (x - 1)];
                let e = luminance_buff[y * width + (x + 1)];

                let sw = luminance_buff[(y + 1) * width + (x - 1)];
                let s = luminance_buff[(y + 1) * width + x];
                let se = luminance_buff[(y + 1) * width + (x + 1)];

                let g_x = (ne - nw) + 2 * (e - w) + (se - sw);
                let g_y = (sw + s * 2 + se) - (nw + n * 2 + ne);

                let mag_sq = (g_x * g_x + g_y * g_y) as f32;
                let normal = ((mag_sq / MAX_SOBEL_SQ) * 255.0).min(255.0) as u8;

                // Each row slice is independent, so this is safe
                let out_idx = x * 4;
                row[out_idx + 0] = normal;
                row[out_idx + 1] = normal;
                row[out_idx + 2] = normal;
                row[out_idx + 3] = 255;
            }
        });

    RgbaImage::from_raw(width as u32, height as u32, sobel_buff).expect("Should be able to do this")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // for a in -1..=1 {
    //     println!("{a:?}");
    // }
    let path = Path::new("scythe.png");
    let img = image::open(path)?;
    // .resize(1280, 720, FilterType::Lanczos3);

    let x = sobel(&img).save("scythe_sobel_high.png");

    // let scale = 8;

    // // let block_x = (img.width() / 100) as usize;
    // // let block_y = (img.height() / 100) as usize;

    // let res = as_text(&img, scale, scale);

    // let mut file = File::create("owl.txt").expect("Should be able to make file");

    // for buf in res.chunks((img.width() as usize + scale - 1) / scale) {
    //     let str_val: String = buf.iter().collect();
    //     writeln!(file, "{}", str_val).expect("Should be able to write to the file");
    // }

    // let down: RgbaImage = downsample_avg_parallel(&img, 32);
    // down.save("scythe_down_grey.png")?;

    // let dynamic = DynamicImage::ImageRgba8(down)
    //     .resize(img.width(), img.height(), FilterType::Lanczos3)
    //     .save("scythe_pix_grey.png")?;

    // println!(
    //     "Input: {}x{} -> Output: {}x{}",
    //     img.width(),
    //     img.height(),
    //     down.width(),
    //     down.height()
    // );

    // println!("x {x:?}");

    Ok(())
}

// fn main() {
//     let _ = try_img().inspect_err(|err| {
//         println!("Oopsie {err:?}");
//     });
// }
