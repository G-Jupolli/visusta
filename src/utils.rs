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

    pub fn rgb_luminance_u8(rgb_buf: &[u8], width: usize, height: usize) -> LuminanceBuff {
        let mut luminance_buff = vec![0u8; width * height];

        let chunk_size = 512;

        luminance_buff
            .par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(chunk_idx, lum_slice)| {
                let start = chunk_idx * chunk_size;
                let end = start + lum_slice.len();

                let mut out_idx = 0;

                for pix in start..end {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    lum_slice[out_idx] = ((77 * r + 150 * g + 29 * b) / 256) as u8;
                    out_idx += 1;
                }
            });

        LuminanceBuff {
            buff: luminance_buff,
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

                let mut out_idx = 0;

                for pix in start..end {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    lum_slice[out_idx] = ((77 * r + 150 * g + 29 * b) / 256) as u8;
                    out_idx += 1;
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

                let mut out_idx = 0;

                for pix in start..end {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    lum_slice[out_idx] = ((77 * r + 150 * g + 29 * b) / 256) as u8;
                    out_idx += 1;
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

                let mut out_idx = 0;

                for pix in start..end {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    lum_slice[out_idx] = ((77 * r + 150 * g + 29 * b) / 256) as u8;
                    out_idx += 1;
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

                let mut out_idx = 0;

                for pix in start..end {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    lum_slice[out_idx] = ((77 * r + 150 * g + 29 * b) / 256) as u8;
                    out_idx += 1;
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

                let mut out_idx = 0;

                for pix in start..end {
                    let base = pix * 3;

                    let r = rgb_buf[base] as i32;
                    let g = rgb_buf[base + 1] as i32;
                    let b = rgb_buf[base + 2] as i32;

                    lum_slice[out_idx] = ((77 * r + 150 * g + 29 * b) / 256) as u8;
                    out_idx += 1;
                }
            });

        luminance_buff
    }
}
