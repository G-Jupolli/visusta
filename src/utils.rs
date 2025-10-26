use rayon::prelude::*;

/// Extracts luminance from rgb vec to single pixel i32 vec
///
/// e.g. 3 pixel input to 3 pixel output
/// buf -> [r1, g1, b1, r2, g2, b2, r3, g3, b3]
/// out -> [l1, l2, l3]
/// Where l = Luminance
///
/// This, by itself, works as a grey scale filter
pub struct LuminanceFilter;

pub struct LuminanceBuff {
    pub buff: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl LuminanceFilter {
    pub fn rgb_luminance_u8(rgb_buf: &[u8], width: usize, height: usize) -> LuminanceBuff {
        let mut luminance_buff = vec![0u8; width * height];

        luminance_buff
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(chunk_idx, lum_slice)| {
                let start = chunk_idx * width;
                let end = start + width;

                for (out_idx, pix) in (start..end).enumerate() {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    lum_slice[out_idx] = ((77 * r + 150 * g + 29 * b) / 256) as u8;
                }
            });

        LuminanceBuff {
            buff: luminance_buff,
            width,
            height,
        }
    }

    pub fn rgb_luminance_u8_scaled(
        rgb_buf: &[u8],
        width: usize,
        height: usize,
        scale: f32,
    ) -> LuminanceBuff {
        let mut luminance_buff = vec![0u8; width * height];

        luminance_buff
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(y, lum_slice)| {
                let start = y * width;
                let end = start + lum_slice.len();

                for (out_idx, pix) in (start..end).enumerate() {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    let luminance = (77 * r + 150 * g + 29 * b) as f32 * scale;

                    lum_slice[out_idx] = (luminance / 255.0) as u8;
                }
            });

        LuminanceBuff {
            buff: luminance_buff,
            width,
            height,
        }
    }
}

impl LuminanceBuff {
    pub fn flip_y(&self) -> LuminanceBuff {
        let mut out_buff = vec![0u8; self.buff.len()];

        out_buff
            .par_chunks_mut(self.width)
            .enumerate()
            .for_each(|(y, row)| {
                let mut left = y * self.width;
                let mut right = left + self.width - 1;

                let mut step = 0;

                while right >= left {
                    row[step] = self.buff[right];
                    row[(self.width - step) - 1] =
                        //check
                        self.buff[left];

                    left += 1;
                    right -= 1;
                    step += 1;
                }
            });

        LuminanceBuff {
            buff: out_buff,
            width: self.width,
            height: self.height,
        }
    }

    pub fn to_raw_rgb(&self) -> Vec<u8> {
        let mut out = vec![0u8; self.width * self.height * 3];

        out.par_chunks_mut(self.width * 3)
            .enumerate()
            .for_each(|(y, row)| {
                let start_x = y * self.width;
                let end_x = start_x + self.width;

                for (ix, x) in (start_x..end_x).enumerate() {
                    let ix = ix * 3;
                    let val = self.buff[x];

                    // if val == 0 {
                    //     row[ix] = 0x2E;
                    //     row[ix + 1] = 0x1A;
                    //     row[ix + 2] = 0x0F;
                    // } else {
                    //     row[ix] = 0xEB;
                    //     row[ix + 1] = 0xE3;
                    //     row[ix + 2] = 0xC5;
                    // }

                    row[ix + 0] = val;
                    row[ix + 1] = val;
                    row[ix + 2] = val;
                }
            });

        out
    }

    pub fn boolean_mask_rgb(self, raw_rgb: &[u8]) -> Vec<u8> {
        // assert!(
        //     self.width == mask.width && self.height == mask.height,
        //     "LuminanceBuffs must have the same dimensions"
        // );

        let mut out = vec![0u8; raw_rgb.len()];

        out.par_chunks_mut(self.width * 3)
            .enumerate()
            .for_each(|(y, row)| {
                let start_x = y * self.width;
                let end_x = start_x + (self.width);

                for (ix, x) in (start_x..end_x).enumerate() {
                    if self.buff[x] > 0 {
                        let out_idx = ix * 3;

                        row[out_idx] = raw_rgb[start_x * 3 + out_idx];
                        row[out_idx + 1] = raw_rgb[start_x * 3 + out_idx + 1];
                        row[out_idx + 2] = raw_rgb[start_x * 3 + out_idx + 2];
                    }
                }
            });

        out
    }

    pub fn to_ascii(&self, font_size: usize) -> Vec<char> {
        let new_width = self.width.div_ceil(font_size);
        let new_height = self.height.div_ceil(font_size);

        let mut scaled_buff = vec![' '; new_width * new_height];

        scaled_buff
            .par_chunks_mut(new_width)
            .enumerate()
            .for_each(|(y, row)| {
                // This maps to the to left pixel at the start of this
                //  parallelized row
                let start_idx = (y * font_size) * self.width;

                for (x, x_item) in row.iter_mut().enumerate() {
                    // bring cursor to the top left pixel of the group
                    let mut x_idx = start_idx + (font_size * x);

                    let mut sum_luminance: usize = 0;
                    let mut count: usize = 0;

                    // From the top left corner, we scan ( font_size - 1 ) pixels
                    //  down and ( font_size - 1 ) pixels to the right
                    for _ in 0..font_size {
                        for x in 0..font_size {
                            if x_idx + x >= self.buff.len() {
                                break;
                            }
                            sum_luminance += self.buff[x_idx + x] as usize;
                            count += 1;
                        }
                        // Bring x_idx down to the leftmost pixel of the next row
                        x_idx += self.width;
                    }

                    let luminance_avg = (sum_luminance / count) as u8;

                    let levels = 10;
                    let step = 255 / levels;

                    let mut discrete = luminance_avg / step;
                    if discrete >= levels {
                        discrete = levels - 1;
                    }

                    *x_item = match discrete {
                        0 => ' ',
                        1 => '.',
                        2 => ';',
                        3 => 'c',
                        4 => 'o',
                        5 => 'P',
                        6 => '0',
                        7 => '?',
                        8 => '@',
                        9 => '#',
                        _ => {
                            panic!("Illegal discrete state");
                        }
                    };
                }
            });

        scaled_buff
    }
}
