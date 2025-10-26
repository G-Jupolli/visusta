use std::f32::consts::PI;

use crate::utils::LuminanceBuff;
use libm::atan2f;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DirectionAscii {
    None,
    X,
    Y,
    LR,
    RL,
}

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
                    row[out_idx + 1] = normal.saturating_mul(2);
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

    pub fn to_ascii_direction(
        luminance_buff: &LuminanceBuff,
        font_size: usize,
        sobel_cutoff: u8,
        ascii_cutoff: f32,
    ) -> Vec<char> {
        let width = luminance_buff.width;
        let height = luminance_buff.height;
        let luminance_buff = &luminance_buff.buff;

        // We're creating the initial buff to know the direction
        //  at the pixel level.
        // We will use this to compute the char at the expanded pixel level
        let mut direction_buff = vec![DirectionAscii::None; width * height];

        direction_buff
            .par_chunks_mut(width)
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

                    if normal > sobel_cutoff {
                        row[x] = SobelFilter::sobel_dir_gx_gy(gx, gy);
                    }
                }
            });

        let new_width = width.div_ceil(font_size);
        let new_height = height.div_ceil(font_size);

        let mut scaled_buff = vec![' '; new_width * new_height];

        scaled_buff
            .par_chunks_mut(new_width)
            .enumerate()
            .for_each(|(y, row)| {
                // This maps to the to left pixel at the start of this
                //  parallelized row
                let start_idx = (y * font_size) * width;

                for (x, x_item) in row.iter_mut().enumerate() {
                    // bring cursor to the top left pixel of the group
                    let mut x_idx = start_idx + (font_size * x);

                    let mut sum_emp = 0;
                    let mut sum_x = 0;
                    let mut sum_lr = 0;
                    let mut sum_y = 0;
                    let mut sum_rl = 0;

                    let mut sum_total = 0;

                    // From the top left corner, we scan ( font_size - 1 ) pixels
                    //  down and ( font_size - 1 ) pixels to the right
                    for _ in 0..font_size {
                        for x in 0..font_size {
                            if x_idx + x >= direction_buff.len() {
                                break;
                            }

                            match direction_buff[x_idx + x] {
                                DirectionAscii::None => {
                                    sum_emp += 1;
                                }
                                DirectionAscii::X => {
                                    sum_x += 1;
                                }
                                DirectionAscii::LR => {
                                    sum_lr += 1;
                                }
                                DirectionAscii::Y => {
                                    sum_rl += 1;
                                }
                                DirectionAscii::RL => {
                                    sum_y += 1;
                                }
                            }
                            sum_total += 1;
                        }

                        // Bring x_idx down to the leftmost pixel of the next row
                        x_idx += width;
                    }

                    if (sum_emp as f32 / sum_total as f32) < ascii_cutoff {
                        let (a_dir, a_max) = if sum_x > sum_y {
                            (DirectionAscii::X, sum_x)
                        } else {
                            (DirectionAscii::Y, sum_y)
                        };

                        let (b_dir, b_max) = if sum_rl > sum_lr {
                            (DirectionAscii::RL, sum_rl)
                        } else {
                            (DirectionAscii::LR, sum_lr)
                        };

                        let res = if a_max > b_max { a_dir } else { b_dir };

                        *x_item = match res {
                            DirectionAscii::None => ' ',
                            DirectionAscii::X => '|',
                            DirectionAscii::LR => '/',
                            DirectionAscii::Y => '-',
                            DirectionAscii::RL => '\\',
                        }
                    }
                }
            });

        scaled_buff
    }

    pub fn sobel_dir_gx_gy(gx: i32, gy: i32) -> DirectionAscii {
        let mut dir = atan2f(gy as f32, gx as f32);

        if dir < 0.0 {
            dir += PI;
        }

        let eighth = PI / 8.0;

        if dir < eighth {
            DirectionAscii::X
        } else if dir < eighth * 3.0 {
            DirectionAscii::LR
        } else if dir < eighth * 5.0 {
            DirectionAscii::Y
        } else if dir < eighth * 7.0 {
            DirectionAscii::RL
        } else {
            DirectionAscii::X
        }
    }

    pub fn to_direction_colour_overlay(luminance_buff: &LuminanceBuff, raw_rgb: &[u8]) -> Vec<u8> {
        let width = luminance_buff.width;
        let height = luminance_buff.height;
        let luminance_buff = &luminance_buff.buff;

        let mut sobel_buff = vec![0u8; raw_rgb.len()];
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
                    // let c = luminance_buff[y * width + x] as i32;
                    let e = luminance_buff[y * width + (x + 1)] as i32;

                    let sw = luminance_buff[(y + 1) * width + (x - 1)] as i32;
                    let s = luminance_buff[(y + 1) * width + x] as i32;
                    let se = luminance_buff[(y + 1) * width + (x + 1)] as i32;

                    let gx = (ne - nw) + 2 * (e - w) + (se - sw);
                    let gy = (sw + s * 2 + se) - (nw + n * 2 + ne);

                    let mag_sq = (gx * gx + gy * gy) as f32;
                    let normal = ((mag_sq / MAX_SOBEL_SQ) * 255.0).min(255.0) as u8;

                    let out_idx = x * 3;

                    if normal < 20 {
                        let raw_out_idx = (y * width + x) * 3;

                        row[out_idx + 0] = raw_rgb[raw_out_idx + 0];
                        row[out_idx + 1] = raw_rgb[raw_out_idx + 1];
                        row[out_idx + 2] = raw_rgb[raw_out_idx + 2];

                        continue;
                    }

                    row[out_idx + 0] = (gx * 2).abs().min(255) as u8;
                    row[out_idx + 1] = normal.saturating_mul(2);
                    row[out_idx + 2] = (gy * 2).abs().min(255) as u8;
                }
            });

        sobel_buff
    }

    pub fn to_direction_colour_overlay_on_dither(
        luminance_buff: &LuminanceBuff,
        raw_rgb: &[u8],
    ) -> Vec<u8> {
        let width = luminance_buff.width;
        let height = luminance_buff.height;
        let luminance_buff = &luminance_buff.buff;

        let mut sobel_buff = vec![0u8; raw_rgb.len()];
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
                    // let c = luminance_buff[y * width + x] as i32;
                    let e = luminance_buff[y * width + (x + 1)] as i32;

                    let sw = luminance_buff[(y + 1) * width + (x - 1)] as i32;
                    let s = luminance_buff[(y + 1) * width + x] as i32;
                    let se = luminance_buff[(y + 1) * width + (x + 1)] as i32;

                    let gx = (ne - nw) + 2 * (e - w) + (se - sw);
                    let gy = (sw + s * 2 + se) - (nw + n * 2 + ne);

                    let mag_sq = (gx * gx + gy * gy) as f32;
                    let normal = ((mag_sq / MAX_SOBEL_SQ) * 255.0).min(255.0) as u8;

                    let out_idx = x * 3;

                    if normal < 20 {
                        let raw_out_idx = y * width + x;

                        row[out_idx + 0] = raw_rgb[raw_out_idx + 0];
                        row[out_idx + 1] = raw_rgb[raw_out_idx + 1];
                        row[out_idx + 2] = raw_rgb[raw_out_idx + 2];

                        continue;
                    }

                    // row[out_idx + 1] = (gx * 2).abs().min(255) as u8;
                    // row[out_idx + 0] = normal.checked_mul(2).unwrap_or(255);
                    // row[out_idx + 2] = (gy * 2).abs().min(255) as u8;
                    // row[out_idx + 0] = 0x30;
                    // row[out_idx + 1] = 0x58;
                    // row[out_idx + 2] = 0x8C;
                    row[out_idx + 0] = 0x07;
                    row[out_idx + 1] = 0x62;
                    row[out_idx + 2] = 0xb4;
                }
            });

        sobel_buff
    }
}
