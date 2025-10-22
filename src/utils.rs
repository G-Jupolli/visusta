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
    pub fn _rgb_luminance(rgb_buf: &[u8], width: usize, height: usize) -> Vec<i32> {
        let mut luminance_buff = vec![0i32; width * height];

        luminance_buff
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, luminance)| {
                let base = idx * 3;

                let r = rgb_buf[base] as i32;
                let g = rgb_buf[base + 1] as i32;
                let b = rgb_buf[base + 2] as i32;

                *luminance = (77 * r + 150 * g + 29 * b) / 256;
            });

        luminance_buff
    }

    pub fn _rgb_luminance_i32(rgb_buf: &[u8], width: usize, height: usize) -> Vec<i32> {
        let mut luminance_buff = vec![0i32; width * height];

        luminance_buff
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, luminance)| {
                let base = idx * 3;

                let r = rgb_buf[base] as i32;
                let g = rgb_buf[base + 1] as i32;
                let b = rgb_buf[base + 2] as i32;

                *luminance = (77 * r + 150 * g + 29 * b) / 256;
            });

        luminance_buff
    }

    pub fn rgb_luminance_u8(rgb_buf: &[u8], width: usize, height: usize) -> LuminanceBuff {
        let mut luminance_buff = vec![0u8; width * height];

        let chunk_size = 512;

        luminance_buff
            .par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(chunk_idx, lum_slice)| {
                let start = chunk_idx * chunk_size;
                let end = start + lum_slice.len();

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

    pub fn _rgb_luminance_u8_c64(rgb_buf: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut luminance_buff = vec![0u8; width * height];

        let chunk_size = 256;

        luminance_buff
            .par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(chunk_idx, lum_slice)| {
                let start = chunk_idx * chunk_size;
                let end = start + lum_slice.len();

                for (out_idx, pix) in (start..end).enumerate() {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    lum_slice[out_idx] = ((77 * r + 150 * g + 29 * b) / 256) as u8;
                }
            });

        luminance_buff
    }

    pub fn _rgb_luminance_u8_c256(rgb_buf: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut luminance_buff = vec![0u8; width * height];

        let chunk_size = 256;

        luminance_buff
            .par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(chunk_idx, lum_slice)| {
                let start = chunk_idx * chunk_size;
                let end = start + lum_slice.len();

                for (out_idx, pix) in (start..end).enumerate() {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    lum_slice[out_idx] = ((77 * r + 150 * g + 29 * b) / 256) as u8;
                }
            });

        luminance_buff
    }

    pub fn _rgb_luminance_u8_c512(rgb_buf: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut luminance_buff = vec![0u8; width * height];

        let chunk_size = 512;

        luminance_buff
            .par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(chunk_idx, lum_slice)| {
                let start = chunk_idx * chunk_size;
                let end = start + lum_slice.len();

                for (out_idx, pix) in (start..end).enumerate() {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    lum_slice[out_idx] = ((77 * r + 150 * g + 29 * b) / 256) as u8;
                }
            });

        luminance_buff
    }

    pub fn _rgb_luminance_u8_c1024(rgb_buf: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut luminance_buff = vec![0u8; width * height];

        let chunk_size = 1024;

        luminance_buff
            .par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(chunk_idx, lum_slice)| {
                let start = chunk_idx * chunk_size;
                let end = start + lum_slice.len();

                for (out_idx, pix) in (start..end).enumerate() {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    lum_slice[out_idx] = ((77 * r + 150 * g + 29 * b) / 256) as u8;
                }
            });

        luminance_buff
    }

    pub fn _rgb_luminance_u8_c4096(rgb_buf: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut luminance_buff = vec![0u8; width * height];

        let chunk_size = 4096;

        luminance_buff
            .par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(chunk_idx, lum_slice)| {
                let start = chunk_idx * chunk_size;
                let end = start + lum_slice.len();

                for (out_idx, pix) in (start..end).enumerate() {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    lum_slice[out_idx] = ((77 * r + 150 * g + 29 * b) / 256) as u8;
                }
            });

        luminance_buff
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

    pub fn to_raw_rgb(self) -> Vec<u8> {
        let mut out = vec![0u8; self.width * self.height * 3];

        out.par_chunks_mut(self.width * 3)
            .enumerate()
            .for_each(|(y, row)| {
                let start_x = y * self.width;
                let end_x = start_x + self.width;

                for (ix, x) in (start_x..end_x).enumerate() {
                    let ix = ix * 3;
                    let val = self.buff[x];

                    row[ix] = val;
                    row[ix + 1] = val;
                    row[ix + 2] = val;
                }
            });

        out
    }

    pub fn boolean_mask_rgb(self, raw_rgb: &Vec<u8>) -> Vec<u8> {
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
}
