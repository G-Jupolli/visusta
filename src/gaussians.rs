use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::f32::consts::PI;

use crate::utils::LuminanceBuff;

pub type GaussianKernel = [f32; 9];

/// Currently calculating a dog ( difference of gaussians ) with r = 1 for a
///  3x3 kernel
fn create_gausian_kernel(sigma_1: f32, sigma_2: f32) -> GaussianKernel {
    assert!(sigma_1 > 0.0 && sigma_2 > 0.0, "Sigmas must be positive");
    assert!(sigma_1 < sigma_2, "Sigma 2 must be greater than sigma 1");

    let (gaussian_a, sum_a, gaussian_b, sum_b) = calculate_continuous_gaussians(sigma_1, sigma_2);

    return calculate_normalised_difference(gaussian_a, sum_a, gaussian_b, sum_b);
}

/// Currently calculating a gaussian kernel of r = 1
///
/// The resulting kernels will be for these co ordinates around center c 0,0
/// ╭                   ╮
/// | -1,-1  0,-1  1,-1 |
/// | -1, 0  0, 0  1, 0 |
/// | -1, 1  0, 1  1, 1 |
/// ╰                   ╯
///
/// Possible optimisations:
/// Looks like the resulting kernel has 1 + 2r possible values as the x,y values are squared
///
/// ╭          ╮
/// | Aσ Bσ Aσ |
/// | Bσ Cσ Bσ |
/// | Aσ Bσ Aσ |
/// ╰          ╯
/// There could be time saved by just expressing the resulting kernel as [3; f32]
/// I'll stick to the simpler solution for now and check if it is worth it
/// to minimise this down the line
///
fn calculate_continuous_gaussians(
    sigma_1: f32,
    sigma_2: f32,
) -> (GaussianKernel, f32, GaussianKernel, f32) {
    let mut gaussian_a = GaussianKernel::default();
    let mut gaussian_b = GaussianKernel::default();

    let mut sum_a = 0.0f32;
    let mut sum_b = 0.0f32;

    for (iy, y) in (-1..=1).enumerate() {
        for (ix, x) in (-1..=1).enumerate() {
            let val_a = calculate_continuous(sigma_1, x as f32, y as f32);
            gaussian_a[ix + (3 * iy)] = val_a;
            sum_a += val_a;

            let val_b = calculate_continuous(sigma_2, x as f32, y as f32);
            gaussian_b[ix + (3 * iy)] = val_b;
            sum_b += val_b;
        }
    }

    (gaussian_a, sum_a, gaussian_b, sum_b)
}

/// Normalising a gaussian in this case would be to divide the matrix points
///  by the sum of the gaussian.
///
/// The difference can then be calculated by the difference of the normalised gaussians.
///
/// So for point xy, can be calculated by:
///
/// Gaxy = point xy on the first Gaussian
/// Gbxy = point xy on the second Gaussian
///
/// Sa   = sum of the first Gaussian
/// Sb   = sum of the second Gaussian
///
/// kxy  = resulting kernel value at xy
///
///        Gaxy     Gbxy
/// kxy = ────── - ──────
///         Sa       Sb
///
fn calculate_normalised_difference(
    gaussian_a: GaussianKernel,
    sum_a: f32,
    gaussian_b: GaussianKernel,
    sum_b: f32,
) -> GaussianKernel {
    let mut res = GaussianKernel::default();

    for idx in 0..9usize {
        res[idx] = (gaussian_a[idx] / sum_a) - (gaussian_b[idx] / sum_b);
    }

    res
}

/// To calculate the Gaussian at an index around center c ( 0 , 0 )
/// we can apply this formula for:
/// G = gaussian
/// π = PI
/// σ = sigma value for the gaussian
/// x = x co ordinate relative to c
/// y = y co ordinate relative to c
///
///       1       ╭  x^2 + y^2 ╮
/// G = ───── exp |- ───────── |
///     2πσ^2     ╰     2σ^2   ╯
///
/// Simplified to:
///
/// Ga = 2πσ^2
/// Gb = - ( x^2 + y^2 ) / 2σ^2
///
/// G = Gb / Ga
///
/// This will give us the continuous gaussian that we will need
///  to normalise later
fn calculate_continuous(sigma: f32, x: f32, y: f32) -> f32 {
    let base = 2.0 * PI * sigma * sigma;
    let exp_power = 0.0 - ((x * x + y * y) / (2.0 * sigma * sigma));

    exp_power.exp() / base
}

pub struct GaussianFilter;

impl GaussianFilter {
    pub fn create_kernel(sigma_1: f32, sigma_2: f32) -> GaussianKernel {
        create_gausian_kernel(sigma_1, sigma_2)
    }

    pub fn filter_luminance(
        luminance_buff: LuminanceBuff,
        width: usize,
        height: usize,
        sigma_1: f32,
        sigma_2: f32,
    ) -> Vec<f32> {
        let luminance_buff = luminance_buff.buff;
        let kernel = Self::create_kernel(sigma_1, sigma_2);

        let mut gaussian_buff = vec![0f32; luminance_buff.len()];
        gaussian_buff
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(y, row)| {
                if y == 0 || y == height - 1 {
                    return;
                }

                for x in 1..(width - 1) {
                    let mut acc = 0f32;

                    acc += luminance_buff[(y - 1) * width + (x - 1)] as f32 * kernel[0];
                    acc += luminance_buff[(y - 1) * width + x] as f32 * kernel[1];
                    acc += luminance_buff[(y - 1) * width + (x + 1)] as f32 * kernel[2];

                    acc += luminance_buff[y * width + (x - 1)] as f32 * kernel[3];
                    acc += luminance_buff[y * width + x] as f32 * kernel[4];
                    acc += luminance_buff[y * width + (x + 1)] as f32 * kernel[5];

                    acc += luminance_buff[(y + 1) * width + (x - 1)] as f32 * kernel[6];
                    acc += luminance_buff[(y + 1) * width + x] as f32 * kernel[7];
                    acc += luminance_buff[(y + 1) * width + (x + 1)] as f32 * kernel[8];

                    row[x] = acc
                }
            });

        gaussian_buff
    }

    pub fn filter_luminance_to_lum_buff(
        luminance_buff: LuminanceBuff,
        width: usize,
        height: usize,
        sigma_1: f32,
        sigma_2: f32,
    ) -> LuminanceBuff {
        let luminance_buff = luminance_buff.buff;
        let kernel = Self::create_kernel(sigma_1, sigma_2);

        let mut gaussian_buff = vec![0u8; luminance_buff.len()];
        gaussian_buff
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(y, row)| {
                if y == 0 || y == height - 1 {
                    return;
                }

                for x in 1..(width - 1) {
                    let mut acc = 0f32;

                    acc += luminance_buff[(y - 1) * width + (x - 1)] as f32 * kernel[0];
                    acc += luminance_buff[(y - 1) * width + x] as f32 * kernel[1];
                    acc += luminance_buff[(y - 1) * width + (x + 1)] as f32 * kernel[2];

                    acc += luminance_buff[y * width + (x - 1)] as f32 * kernel[3];
                    acc += luminance_buff[y * width + x] as f32 * kernel[4];
                    acc += luminance_buff[y * width + (x + 1)] as f32 * kernel[5];

                    acc += luminance_buff[(y + 1) * width + (x - 1)] as f32 * kernel[6];
                    acc += luminance_buff[(y + 1) * width + x] as f32 * kernel[7];
                    acc += luminance_buff[(y + 1) * width + (x + 1)] as f32 * kernel[8];

                    // acc *= 32f32;

                    row[x] = acc as u8;
                }
            });

        LuminanceBuff {
            buff: gaussian_buff,
        }
    }

    pub fn apply_gaussian(
        luminance_buff: LuminanceBuff,
        width: usize,
        height: usize,
        sigma_1: f32,
        sigma_2: f32,
    ) -> Vec<u8> {
        let luminance_buff = luminance_buff.buff;
        let kernel = Self::create_kernel(sigma_1, sigma_2);

        let mut gaussian_buff = vec![0u8; luminance_buff.len() * 3];
        gaussian_buff
            .par_chunks_mut(width * 3)
            .enumerate()
            .for_each(|(y, row)| {
                if y == 0 || y == height - 1 {
                    return;
                }

                for x in 1..(width - 1) {
                    let mut acc = 0f32;

                    acc += luminance_buff[(y - 1) * width + (x - 1)] as f32 * kernel[0];
                    acc += luminance_buff[(y - 1) * width + x] as f32 * kernel[1];
                    acc += luminance_buff[(y - 1) * width + (x + 1)] as f32 * kernel[2];

                    acc += luminance_buff[y * width + (x - 1)] as f32 * kernel[3];
                    acc += luminance_buff[y * width + x] as f32 * kernel[4];
                    acc += luminance_buff[y * width + (x + 1)] as f32 * kernel[5];

                    acc += luminance_buff[(y + 1) * width + (x - 1)] as f32 * kernel[6];
                    acc += luminance_buff[(y + 1) * width + x] as f32 * kernel[7];
                    acc += luminance_buff[(y + 1) * width + (x + 1)] as f32 * kernel[8];

                    // let acc = acc as u8;

                    // if acc <  {
                    //     continue;
                    // }

                    let out_idx = x * 3;

                    // acc *= 128f32;

                    // row[x] = acc
                    //

                    row[out_idx] = acc as u8;
                    row[out_idx + 1] = acc as u8;
                    row[out_idx + 2] = acc as u8;
                    // row[out_idx] = 255u8;
                    // row[out_idx + 1] = 255u8;
                    // row[out_idx + 2] = 255u8;
                }
            });

        gaussian_buff
    }
}

#[cfg(test)]
mod tests {}
