use std::cmp::min;

use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

pub struct PixelFilter;

// fn downsample_avg_parallel(
//     rgb_raw: &Vec<u8>,
//     width: usize,
//     height: usize,
//     block_size: usize,
// ) -> Vec<u8> {
//     let new_width = (width + block_size - 1) / block_size;
//     let new_height = (height + block_size - 1) / block_size;

//     let mut buff = vec![0u8; rgb_raw.len()];

//     buff.par_chunks_mut(width * 3)

//     buff.par_chunks_mut(width * 3)
//         .enumerate()
//         .for_each(|(y, row)| {
//             for x in 0..new_width {
//                 let start_x = x * block_size;
//                 let start_y = y * block_size;

//                 let end_x = min(start_x + block_size, width);
//                 let end_y = min(start_y + block_size, height);
//             }
//         });

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

// fn downsample_avg_parallel(
//     rgb_raw: &Vec<u8>,
//     width: usize,
//     height: usize,
//     block_size: usize,
// ) -> Vec<u8> {
//     let new_width = (width + block_size - 1) / block_size;
//     let new_height = (height + block_size - 1) / block_size;

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
