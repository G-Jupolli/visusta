use std::cmp::min;

use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

pub struct PixelFilter;

impl PixelFilter {
    /// This is going to be a bit odd;
    ///
    /// The maths here works out that 1 pixel is the average right and down
    ///   of the block_size at n-1
    ///
    /// e.g. if block_size == 3
    ///  ╭          ╮
    ///  |  O  1  2 |
    ///  |  1  1  2 |
    ///  |  2  2  2 |
    ///  ╰          ╯
    ///
    /// This results in the final size being size ( accounting for rgb channels )
    /// ( original_width / block_size) * (original_height / block_size) * 3
    ///
    /// Referencing in row.
    ///
    /// start_y = row * block_size
    ///
    /// Need some nested loops:
    /// start_y -> start_y + block_size
    /// Then:
    /// x       -> x       + block_size
    ///
    /// Of these block_size ^ 2 pixels, we sum all the channels and then store the avg
    ///
    /// We then make the resultant vec which is back to the original size.
    /// This will just pull in the data to force the pixelation
    ///
    pub fn pixelate_raw_rgb(
        rgb_raw: &Vec<u8>,
        width: usize,
        height: usize,
        block_size: usize,
    ) -> Vec<u8> {
        let new_width = (width + block_size - 1) / block_size;
        let new_height = (height + block_size - 1) / block_size;

        let mut buff = vec![0u8; new_width * new_height * 3];

        todo!();

        buff.par_chunks_mut(new_width * 3)
            .enumerate()
            .for_each(|(y, row)| {
                // Per pixel in res
                for x in 0..new_width {
                    let start_y = (y * block_size) + x * block_size;

                    // Here start y is the index in rgb_raw that is the start of the row
                    // let mut start_y = (y * block_size) * 1;
                    let end_y = min(start_y + block_size, height);

                    // Update start y so we're always init on top left
                    while start_y < end_y {
                        let mut sum_r = 0u32;
                        let mut sum_g = 0u32;
                        let mut sum_b = 0u32;

                        let mut count = 0u32;

                        for y in start_y..(start_y + block_size) {
                            let start_x = y * width;
                        }

                        // start_y += new_width * 3;
                    }
                }
            });

        todo!()
    }
}
