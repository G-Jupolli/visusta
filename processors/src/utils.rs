use rayon::prelude::*;

use crate::composer::{ProcessorPage, ProcessorPageSignal};

pub struct LuminanceBuff {
    pub buff: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl LuminanceBuff {
    pub fn boolean_mask_rgb(self, raw_rgb: &[u8]) -> Vec<u8> {
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
