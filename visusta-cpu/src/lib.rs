use async_trait::async_trait;
use image::{ImageBuffer, LumaA, Rgba, RgbaImage};
use rayon::prelude::*;
use visusta_core::{SobelColorData, SobelColorItem, VisustaProcessor};

pub struct VisustaCPU;

#[async_trait]
impl VisustaProcessor for VisustaCPU {
    async fn sobel_to_colour(&self, img: &RgbaImage, filter: SobelColorData) -> RgbaImage {
        let luminance = rgb_luminance_u8(img);

        page_to_direction_colour(&luminance, filter)
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

pub fn rgb_luminance_u8(img: &RgbaImage) -> ImageBuffer<LumaA<u8>, Vec<u8>> {
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

                let out_idx = col * 2;

                row_slice[out_idx] = luminance;
                row_slice[out_idx + 1] = a;
            }
        });

    ImageBuffer::from_raw(img.width(), img.height(), buf)
        .expect("Luminance buffer should be sized correctly")
}

pub fn page_to_direction_colour(
    img: &ImageBuffer<LumaA<u8>, Vec<u8>>,
    filter: SobelColorData,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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
