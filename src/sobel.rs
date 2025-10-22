use crate::utils::LuminanceBuff;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

/// The Sobel ( Sobel-Feldman ) filter is going to be used for edge detection
///
///      ╭          ╮
///      | -1  0  1 |
/// Gx = | -2  0  2 | * A
///      | -1  0  1 |
///      ╰          ╯
///      ╭          ╮
///      | -1 -2 -1 |
/// Gy = |  0  0  0 | * A
///      |  1  2  1 |
///      ╰          ╯
///
/// To Avoid doing a full 3x3 convolve we can do 2 1D convolves:
///
///      ╭    ╮
///      |  1 |     ╭          ╮
/// Gx = |  2 | * ( |  1  0 -1 | * A )
///      |  1 |     ╰          ╯
///      ╰    ╯
///      ╭    ╮
///      |  1 |     ╭          ╮
/// Gy = |  0 | * ( |  1  2  1 | * A )
///      | -1 |     ╰          ╯
///      ╰    ╯
///
/// If we consider these co ordinates around center c
///  ╭          ╮
///  | nw  n ne |
///  |  w  c  e |
///  | sw  s se |
///  ╰          ╯
///
/// We can calculate Gx and Gy as:
///
/// Gx = ne - nw + ( 2 * (e - w) ) + se - sw
/// Gy = sw + ( s * 2 ) + se - nw + ( n * 2 ) + ne
///
/// Gx and Gy are directional, we can use this to find the magnitude and direction
///
/// magnitude^2 = Gx^2 + Gy^2
/// direction   = atan2(Gx, Gy)
///
pub struct SobelFilter;

const MAX_SOBEL_SQ: f32 = 255.0 * 255.0;

impl SobelFilter {
    pub fn _to_normal(luminance_buff: LuminanceBuff, width: usize, height: usize) -> Vec<u8> {
        // I'm mostly just doing this to enforce the type, not sure how much it matters
        let luminance_buff = luminance_buff.buff;

        let mut sobel_buff = vec![0u8; width * height * 3];
        sobel_buff
            .par_chunks_mut(width * 3)
            .enumerate()
            .for_each(|(y, row)| {
                if y == 0 || y == height - 1 {
                    return;
                }

                for x in 1..(width - 1) {
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

                    let out_idx = x * 3;
                    row[out_idx] = normal;
                    row[out_idx + 1] = normal;
                    row[out_idx + 2] = normal;
                }
            });

        sobel_buff
    }

    pub fn to_direction_colour(
        luminance_buff: &LuminanceBuff,
        width: usize,
        height: usize,
    ) -> Vec<u8> {
        let luminance_buff = &luminance_buff.buff;

        let mut sobel_buff = vec![0u8; width * height * 3];
        sobel_buff
            .par_chunks_mut(width * 3)
            .enumerate()
            .for_each(|(y, row)| {
                if y == 0 || y == height - 1 {
                    return;
                }

                for x in 1..(width - 1) {
                    let nw = luminance_buff[(y - 1) * width + (x - 1)] as i32;
                    let n = luminance_buff[(y - 1) * width + x] as i32;
                    let ne = luminance_buff[(y - 1) * width + (x + 1)] as i32;

                    let w = luminance_buff[y * width + (x - 1)] as i32;
                    let e = luminance_buff[y * width + (x + 1)] as i32;

                    let sw = luminance_buff[(y + 1) * width + (x - 1)] as i32;
                    let s = luminance_buff[(y + 1) * width + x] as i32;
                    let se = luminance_buff[(y + 1) * width + (x + 1)] as i32;

                    let gx = (ne - nw) + 2 * (e - w) + (se - sw);
                    let gy = (sw + s * 2 + se) - (nw + n * 2 + ne);

                    let mag_sq = (gx * gx + gy * gy) as f32;
                    let normal = ((mag_sq / MAX_SOBEL_SQ) * 255.0).min(255.0) as u8;

                    if normal < 20 {
                        continue;
                    }

                    // if normal > 32 {
                    let out_idx = x * 3;

                    row[out_idx + 0] = (gx * 2).abs().min(255) as u8;
                    // row[out_idx + 2] = gy.abs().min(255) as u8;
                    // row[out_idx + 1] = gy.abs().min(255) as u8;
                    row[out_idx + 1] = normal.checked_mul(2).unwrap_or(255);
                    row[out_idx + 2] = (gy * 2).abs().min(255) as u8;
                    // row[out_idx + 2] = normal

                    // row[out_idx] = (gx * 4).abs().min(255) as u8;
                    // row[out_idx + 2] = gy.abs().min(255) as u8;

                    // let dir = atan2f(g_y, g_x);

                    // row[out_idx + 1] = (normal as f32 * 0.8) as u8;
                    // row[out_idx + 2] = normal.checked_mul(3) / 2;

                    // if let Some(fin) = normal.checked_mul(3) {
                    //     row[out_idx + 2] = fin / 2;
                    // } else {
                    // }
                    // }
                }
            });

        sobel_buff
    }

    pub fn _to_direction_colour_i32(
        luminance_buff: &[i32],
        width: usize,
        height: usize,
    ) -> Vec<u8> {
        // let luminance_buff = luminance_buff.buff;

        let mut sobel_buff = vec![0u8; width * height * 3];
        sobel_buff
            .par_chunks_mut(width * 3)
            .enumerate()
            .for_each(|(y, row)| {
                if y == 0 || y == height - 1 {
                    return;
                }

                for x in 1..(width - 1) {
                    let nw = luminance_buff[(y - 1) * width + (x - 1)] as i32;
                    let n = luminance_buff[(y - 1) * width + x] as i32;
                    let ne = luminance_buff[(y - 1) * width + (x + 1)] as i32;

                    let w = luminance_buff[y * width + (x - 1)] as i32;
                    let e = luminance_buff[y * width + (x + 1)] as i32;

                    let sw = luminance_buff[(y + 1) * width + (x - 1)] as i32;
                    let s = luminance_buff[(y + 1) * width + x] as i32;
                    let se = luminance_buff[(y + 1) * width + (x + 1)] as i32;

                    let gx = (ne - nw) + 2 * (e - w) + (se - sw);
                    let gy = (sw + s * 2 + se) - (nw + n * 2 + ne);

                    let mag_sq = (gx * gx + gy * gy) as f32;
                    let normal = ((mag_sq / MAX_SOBEL_SQ) * 255.0 * 4.0).min(255.0) as u8;

                    // if normal > 32 {
                    let out_idx = x * 3;

                    row[out_idx + 0] = gx.abs().min(255) as u8;
                    row[out_idx + 2] = gy.abs().min(255) as u8;

                    // row[out_idx] = (gx * 4).abs().min(255) as u8;
                    // row[out_idx + 2] = gy.abs().min(255) as u8;

                    // let dir = atan2f(g_y, g_x);

                    // row[out_idx + 1] = (normal as f32 * 0.8) as u8;
                    row[out_idx + 1] = normal;
                    // }
                }
            });

        sobel_buff
    }

    // pub fn a() {

    //     let mut sobel_buff = vec![0u8; width * height * 3];
    //     sobel_buff
    //         .par_chunks_mut(width * 3) // each row has width*3 bytes
    //         .enumerate()
    //         .for_each(|(y, row)| {
    //             if y == 0 || y == height - 1 {
    //                 return;
    //             } // skip borders

    //             for x in 1..(width - 1) {
    //                 // let idx = y * width + x;

    //                 // Compute neighbors in luminance_buff
    //                 let nw = luminance_buff[(y - 1) * width + (x - 1)];
    //                 let n = luminance_buff[(y - 1) * width + x];
    //                 let ne = luminance_buff[(y - 1) * width + (x + 1)];

    //                 let w = luminance_buff[y * width + (x - 1)];
    //                 let e = luminance_buff[y * width + (x + 1)];

    //                 let sw = luminance_buff[(y + 1) * width + (x - 1)];
    //                 let s = luminance_buff[(y + 1) * width + x];
    //                 let se = luminance_buff[(y + 1) * width + (x + 1)];

    //                 let g_x = (ne - nw) + 2 * (e - w) + (se - sw);
    //                 let g_y = (sw + s * 2 + se) - (nw + n * 2 + ne);

    //                 let mag_sq = (g_x * g_x + g_y * g_y) as f32;
    //                 let normal = ((mag_sq / MAX_SOBEL_SQ) * 255.0).min(255.0) as u8;

    //                 // Each row slice is independent, so this is safe
    //                 let out_idx = x * 3;
    //                 row[out_idx + 0] = normal;
    //                 row[out_idx + 1] = normal;
    //                 row[out_idx + 2] = normal;
    //             }
    //         });
    // }
}
