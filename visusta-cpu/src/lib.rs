use std::f32::consts::PI;

use async_trait::async_trait;
use image::{ImageBuffer, RgbaImage};
use libm::atan2f;
use rayon::prelude::*;
use visusta_core::{
    CharImage, LumaAImage, LuminanceAsciiFilter, LuminanceFilter, SobelAscii, SobelColorData,
    SobelColorItem, VisustaProcessor,
    gaussians::{GaussianBuilder, GaussianColorData, GaussianColorItem, GaussianKernelData},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DirectionAscii {
    None,
    X,
    Y,
    LR,
    RL,
}

pub struct VisustaCPU;

#[async_trait]
impl VisustaProcessor for VisustaCPU {
    async fn rgba_to_luma_a(&self, img: &RgbaImage, filter: LuminanceFilter) -> LumaAImage {
        rgb_luminance_u8(img, filter)
    }

    async fn sobel_to_colour(&self, img: &LumaAImage, filter: SobelColorData) -> RgbaImage {
        page_to_direction_colour(&img, filter)
    }

    async fn gaussian_on_luma(&self, img: &LumaAImage, builder: GaussianBuilder) -> LumaAImage {
        let kernel_data = builder.build_kernel();

        gaussian_on_luminance(img, kernel_data)
    }

    async fn gaussian_to_coloured(
        &self,
        img: &LumaAImage,
        builder: GaussianBuilder,
        filter: GaussianColorData,
    ) -> RgbaImage {
        let kernel_data = builder.build_kernel();

        gaussian_to_coloured(img, kernel_data, filter)
    }

    async fn luminance_to_ascii(
        &self,
        img: &LumaAImage,
        filter: LuminanceAsciiFilter,
    ) -> CharImage {
        luminance_to_ascii(img, filter)
    }

    async fn sobel_ascii_directional(&self, img: &LumaAImage, filter: SobelAscii) -> CharImage {
        sobel_ascii_directional(img, filter)
    }

    async fn overlay_image(&self, img_bg: &RgbaImage, img_fg: &RgbaImage) -> RgbaImage {
        assert!(
            img_bg.width() == img_fg.width() && img_bg.height() == img_fg.height(),
            "Images must be same size"
        );

        let width = img_bg.width();
        let height = img_bg.height();

        let mut out = vec![0u8; (width * height * 4) as usize];

        out.par_chunks_mut((width * 4) as usize)
            .enumerate()
            .for_each(|(y, row)| {
                let y = y as u32;

                for x in 0..width {
                    let pixel_bg = img_bg.get_pixel(x, y);
                    let pixel_fg = img_fg.get_pixel(x, y);

                    let alpha_bg = pixel_bg[3] as f32 / 255.0;
                    let alpha_fg = pixel_fg[3] as f32 / 255.0;

                    let alpha_out = alpha_fg + alpha_bg * (1.0 - alpha_fg);

                    let out_idx = (x * 4) as usize;

                    if alpha_out == 0.0 {
                        row[out_idx] = 0;
                        row[out_idx + 1] = 0;
                        row[out_idx + 2] = 0;
                        row[out_idx + 3] = 0;
                    } else {
                        // Simple alpha blend: result = fg * alpha_fg + bg * (1 - alpha_fg)
                        for i in 0..3 {
                            let fg = pixel_fg[i] as f32 * alpha_fg;
                            let bg = pixel_bg[i] as f32 * (1.0 - alpha_fg);
                            row[out_idx + i] = (fg + bg) as u8;
                        }
                        row[out_idx + 3] = (alpha_out * 255.0) as u8;
                    }
                }
            });

        ImageBuffer::from_raw(width, height, out).expect("Buffer should be sized correctly")
    }
}

const MAX_SOBEL_SQ: f32 = 255.0 * 255.0;

pub fn rgb_luminance_u8(img: &RgbaImage, filter: LuminanceFilter) -> LumaAImage {
    let width = img.width() as usize;
    let height = img.height() as usize;

    let mut buf: Vec<u8> = vec![0u8; width * height * 2];

    buf.par_chunks_mut(width * 2)
        .enumerate()
        .for_each(|(row_idx, row_slice)| {
            for col in 0..width {
                let rgba_pixel = img.get_pixel(col as u32, row_idx as u32);

                let r = rgba_pixel[0] as i32;
                let g = rgba_pixel[1] as i32;
                let b = rgba_pixel[2] as i32;
                let a = rgba_pixel[3];

                if a == 0 {
                    continue;
                }

                let luminance = ((77 * r + 150 * g + 29 * b) / 256) as u8;

                let filtered = if filter.multiplier == 0.0 {
                    luminance
                } else {
                    ((luminance as f32) * filter.multiplier).min(255.0) as u8
                };

                if filtered < filter.min {
                    continue;
                }

                let out_idx = col * 2;

                row_slice[out_idx] = filtered;
                row_slice[out_idx + 1] = a;
            }
        });

    ImageBuffer::from_raw(img.width(), img.height(), buf)
        .expect("Luminance buffer should be sized correctly")
}

pub fn page_to_direction_colour(img: &LumaAImage, filter: SobelColorData) -> RgbaImage {
    assert!(
        !matches!(filter.a, SobelColorItem::None),
        "Should not allow none on a channel sobel filter"
    );
    let width = img.width() as usize;
    let height = img.height() as usize;

    let mut sobel_buff = vec![0u8; width * height * 4];

    sobel_buff
        .par_chunks_mut(width * 4)
        .enumerate()
        .for_each(|(y, row)| {
            if y == 0 || y == height - 1 {
                return;
            }

            for x in 1..(width - 1) {
                let nw = img.get_pixel((x - 1) as u32, (y - 1) as u32).0[0] as i32;
                let n = img.get_pixel(x as u32, (y - 1) as u32).0[0] as i32;
                let ne = img.get_pixel((x + 1) as u32, (y - 1) as u32).0[0] as i32;

                let w = img.get_pixel((x - 1) as u32, y as u32).0[0] as i32;
                let e = img.get_pixel((x + 1) as u32, y as u32).0[0] as i32;

                let sw = img.get_pixel((x - 1) as u32, (y + 1) as u32).0[0] as i32;
                let s = img.get_pixel(x as u32, (y + 1) as u32).0[0] as i32;
                let se = img.get_pixel((x + 1) as u32, (y + 1) as u32).0[0] as i32;

                let gx = (ne - nw) + 2 * (e - w) + (se - sw);
                let gy = (sw + s * 2 + se) - (nw + n * 2 + ne);

                let mag_sq = (gx * gx + gy * gy) as f32;
                let normal = ((mag_sq / MAX_SOBEL_SQ) * 255.0).min(255.0) as u8;

                if normal < filter.magnitude_min {
                    continue;
                }

                let out_idx = x * 4;

                row[out_idx + 0] = match filter.r {
                    SobelColorItem::NormalScale(s) => ((normal as f32) * s) as u8,
                    SobelColorItem::GxScale(s) => ((gx as f32) * s) as u8,
                    SobelColorItem::GyScale(s) => ((gy as f32) * s) as u8,
                    SobelColorItem::Absolute(v) => v,
                    SobelColorItem::None => 0,
                };

                row[out_idx + 1] = match filter.g {
                    SobelColorItem::NormalScale(s) => ((normal as f32) * s) as u8,
                    SobelColorItem::GxScale(s) => ((gx as f32) * s) as u8,
                    SobelColorItem::GyScale(s) => ((gy as f32) * s) as u8,
                    SobelColorItem::Absolute(v) => v,
                    SobelColorItem::None => 0,
                };

                row[out_idx + 2] = match filter.b {
                    SobelColorItem::NormalScale(s) => ((normal as f32) * s) as u8,
                    SobelColorItem::GxScale(s) => ((gx as f32) * s) as u8,
                    SobelColorItem::GyScale(s) => ((gy as f32) * s) as u8,
                    SobelColorItem::Absolute(v) => v,
                    SobelColorItem::None => 0,
                };

                row[out_idx + 3] = match filter.a {
                    SobelColorItem::NormalScale(s) => ((normal as f32) * s) as u8,
                    SobelColorItem::GxScale(s) => ((gx as f32) * s) as u8,
                    SobelColorItem::GyScale(s) => ((gy as f32) * s) as u8,
                    SobelColorItem::Absolute(v) => v,
                    SobelColorItem::None => 0,
                }
            }
        });

    ImageBuffer::from_raw(width as u32, height as u32, sobel_buff)
        .expect("Sobel buff should be sized correctly")
}

pub fn gaussian_on_luminance(img: &LumaAImage, kernel_data: GaussianKernelData) -> LumaAImage {
    let width = img.width();
    let height = img.height();

    let kernel = kernel_data.kernel;

    let mut gaussian_buff = vec![0u8; (width * height * 2) as usize];

    gaussian_buff
        .par_chunks_mut(width as usize * 2)
        .enumerate()
        .for_each(|(y, row)| {
            let y = y as u32;
            if y == 0 || y == height - 1 {
                return;
            }

            for x in 1..(width - 1) {
                let mut acc = 0f32;

                acc += img.get_pixel(x - 1, y - 1).0[0] as f32 * kernel[0];
                acc += img.get_pixel(x, y - 1).0[0] as f32 * kernel[1];
                acc += img.get_pixel(x + 1, y - 1).0[0] as f32 * kernel[2];

                acc += img.get_pixel(x - 1, y).0[0] as f32 * kernel[3];
                acc += img.get_pixel(x, y).0[0] as f32 * kernel[4];
                acc += img.get_pixel(x + 1, y).0[0] as f32 * kernel[5];

                acc += img.get_pixel(x - 1, y + 1).0[0] as f32 * kernel[6];
                acc += img.get_pixel(x, y + 1).0[0] as f32 * kernel[7];
                acc += img.get_pixel(x + 1, y + 1).0[0] as f32 * kernel[8];

                // Just using this as a binary cutoff right now
                if kernel_data.cutoff.is_some_and(|cutoff| acc > cutoff) {
                    row[(x * 2) as usize] = 255u8;
                    row[(x * 2) as usize + 1] = 255u8;
                    // continue;
                }

                // row[x] = min((acc * 12.0) as u32, 255u32) as u8;
            }
        });

    ImageBuffer::from_raw(img.width(), img.height(), gaussian_buff)
        .expect("Luminance buffer should be sized correctly")
}

fn gaussian_to_coloured(
    img: &LumaAImage,
    kernel_data: GaussianKernelData,
    filter: GaussianColorData,
) -> RgbaImage {
    let width = img.width();
    let height = img.height();

    let kernel = kernel_data.kernel;

    let mut gaussian_buff = vec![0u8; (width * height * 4) as usize];

    gaussian_buff
        .par_chunks_mut(width as usize * 4)
        .enumerate()
        .for_each(|(y, row)| {
            let y = y as u32;
            if y == 0 || y == height - 1 {
                return;
            }

            for x in 1..(width - 1) {
                let mut acc = 0f32;

                acc += img.get_pixel(x - 1, y - 1).0[0] as f32 * kernel[0];
                acc += img.get_pixel(x, y - 1).0[0] as f32 * kernel[1];
                acc += img.get_pixel(x + 1, y - 1).0[0] as f32 * kernel[2];

                acc += img.get_pixel(x - 1, y).0[0] as f32 * kernel[3];
                acc += img.get_pixel(x, y).0[0] as f32 * kernel[4];
                acc += img.get_pixel(x + 1, y).0[0] as f32 * kernel[5];

                acc += img.get_pixel(x - 1, y + 1).0[0] as f32 * kernel[6];
                acc += img.get_pixel(x, y + 1).0[0] as f32 * kernel[7];
                acc += img.get_pixel(x + 1, y + 1).0[0] as f32 * kernel[8];

                if kernel_data.cutoff.is_some_and(|cutoff| acc > cutoff) {
                    let out_idx = (x * 4) as usize;

                    row[out_idx] = match filter.r {
                        GaussianColorItem::NormalScale(scalar) => (acc * scalar) as u8,
                        GaussianColorItem::Absolute(val) => val,
                        GaussianColorItem::None => 0,
                    };

                    row[out_idx + 1] = match filter.g {
                        GaussianColorItem::NormalScale(scalar) => (acc * scalar) as u8,
                        GaussianColorItem::Absolute(val) => val,
                        GaussianColorItem::None => 0,
                    };

                    row[out_idx + 2] = match filter.b {
                        GaussianColorItem::NormalScale(scalar) => (acc * scalar) as u8,
                        GaussianColorItem::Absolute(val) => val,
                        GaussianColorItem::None => 0,
                    };

                    row[out_idx + 3] = match filter.a {
                        GaussianColorItem::NormalScale(scalar) => (acc * scalar) as u8,
                        GaussianColorItem::Absolute(val) => val,
                        GaussianColorItem::None => 0,
                    };
                }
            }
        });

    ImageBuffer::from_raw(img.width(), img.height(), gaussian_buff)
        .expect("Luminance buffer should be sized correctly")
}

fn luminance_to_ascii(img: &LumaAImage, filter: LuminanceAsciiFilter) -> CharImage {
    let width = (img.width() as usize).div_ceil(filter.font_size) * 2;
    let height = (img.height() as usize).div_ceil(filter.font_size);

    let mut char_buff = vec![' '; (width * height) as usize];

    char_buff
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(char_y, row)| {
            for char_x in 0..width {
                if char_x % 2 != 0 {
                    match filter.space_type {
                        visusta_core::AsciiSpaceType::Duplicate => row[char_x] = row[char_x - 1],
                        visusta_core::AsciiSpaceType::Space => continue,
                        visusta_core::AsciiSpaceType::Raw(v) => row[char_x] = v,
                    }
                    continue;
                }

                // Convert char coordinate to pixel coordinate
                // char_x is doubled (0, 2, 4...), so divide by 2 to get actual char index
                let pixel_x_start = (char_x / 2) * filter.font_size;
                let pixel_y_start = char_y * filter.font_size;

                let mut sum_luminance: usize = 0;
                let mut count: usize = 0;

                for px in pixel_x_start..(pixel_x_start + filter.font_size) {
                    for py in pixel_y_start..(pixel_y_start + filter.font_size) {
                        if let Some(pix) = img.get_pixel_checked(px as u32, py as u32)
                            && pix.0[1] > 0
                        {
                            count += 1;
                            sum_luminance += pix.0[0] as usize;
                        }
                    }
                }

                if count == 0 {
                    continue;
                }

                let luminance_avg = (sum_luminance / count) as u8;

                let levels = 10;
                let step = 255 / levels;

                let mut discrete = luminance_avg / step;
                if discrete >= levels {
                    discrete = levels - 1;
                }

                row[char_x] = filter.chars[discrete as usize];
            }
        });

    CharImage {
        width,
        height,
        data: char_buff,
    }
}

fn sobel_ascii_directional(img: &LumaAImage, filter: SobelAscii) -> CharImage {
    let width = img.width();
    let height = img.height() as usize;

    // We're creating the initial buff to know the direction
    //  at the pixel level.
    // We will use this to compute the char at the expanded pixel level
    let mut direction_buff = vec![DirectionAscii::None; width as usize * height];

    direction_buff
        .par_chunks_mut(width as usize)
        .enumerate()
        .for_each(|(y, row)| {
            if y == 0 || y == height - 1 {
                return;
            }

            for x in 1..(width - 1) {
                let nw = img.get_pixel((x - 1) as u32, (y - 1) as u32).0[0] as i32;
                let n = img.get_pixel(x as u32, (y - 1) as u32).0[0] as i32;
                let ne = img.get_pixel((x + 1) as u32, (y - 1) as u32).0[0] as i32;

                let w = img.get_pixel((x - 1) as u32, y as u32).0[0] as i32;
                let e = img.get_pixel((x + 1) as u32, y as u32).0[0] as i32;

                let sw = img.get_pixel((x - 1) as u32, (y + 1) as u32).0[0] as i32;
                let s = img.get_pixel(x as u32, (y + 1) as u32).0[0] as i32;
                let se = img.get_pixel((x + 1) as u32, (y + 1) as u32).0[0] as i32;

                let gx = (ne - nw) + 2 * (e - w) + (se - sw);
                let gy = (sw + s * 2 + se) - (nw + n * 2 + ne);

                let mag_sq = (gx * gx + gy * gy) as f32;
                let normal = ((mag_sq / MAX_SOBEL_SQ) * 255.0).min(255.0) as u8;

                if normal >= filter.magnitude_min {
                    row[x as usize] = sobel_dir_gx_gy(gx, gy);
                }
            }
        });

    let new_width = (width as usize).div_ceil(filter.font_size) * 2;
    let new_height = height.div_ceil(filter.font_size);

    let mut scaled_buff = vec![' '; new_width * new_height];

    let width = width as usize;

    scaled_buff
        .par_chunks_mut(new_width)
        .enumerate()
        .for_each(|(char_y, row)| {
            for char_x in 0..new_width {
                if char_x % 2 != 0 {
                    match filter.space_type {
                        visusta_core::AsciiSpaceType::Duplicate => row[char_x] = row[char_x - 1],
                        visusta_core::AsciiSpaceType::Space => continue,
                        visusta_core::AsciiSpaceType::Raw(v) => row[char_x] = v,
                    }
                    continue;
                }

                // Convert char coordinate to pixel coordinate
                // char_x is doubled (0, 2, 4...), so divide by 2 to get actual char index
                let pixel_x_start = (char_x / 2) * filter.font_size;
                let pixel_y_start = char_y * filter.font_size;

                let mut sum_emp: f32 = 0.0;
                let mut sum_x: u32 = 0;
                let mut sum_lr: u32 = 0;
                let mut sum_y: u32 = 0;
                let mut sum_rl: u32 = 0;

                let mut sum_total: f32 = 0.0;

                for px in pixel_x_start..(pixel_x_start + filter.font_size) {
                    if px >= width {
                        continue;
                    }

                    for py in pixel_y_start..(pixel_y_start + filter.font_size) {
                        if py >= height {
                            continue;
                        }
                        sum_total += 1.0;
                        match direction_buff[py * width + px] {
                            DirectionAscii::None => {
                                sum_emp += 1.0;
                            }
                            DirectionAscii::X => {
                                sum_x += 1;
                            }
                            DirectionAscii::LR => {
                                sum_lr += 1;
                            }
                            DirectionAscii::Y => {
                                sum_y += 1;
                            }
                            DirectionAscii::RL => {
                                sum_rl += 1;
                            }
                        }
                    }
                }

                if sum_total == 0.0 || (sum_emp / sum_total) > filter.ascii_max {
                    continue;
                }

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

                row[char_x] = match res {
                    DirectionAscii::None => ' ',
                    DirectionAscii::X => filter.chars[0],
                    DirectionAscii::LR => filter.chars[1],
                    DirectionAscii::Y => filter.chars[2],
                    DirectionAscii::RL => filter.chars[3],
                }
            }
        });

    CharImage {
        width: new_width,
        height: new_height,
        data: scaled_buff,
    }
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
