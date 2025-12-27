use std::f32::consts::PI;

type GaussianKernel = [f32; 9];

#[derive(Debug)]
pub struct GaussianKernelData {
    pub kernel: GaussianKernel,
    pub cutoff: Option<f32>,
}

#[derive(Debug)]
pub struct GaussianColorData {
    pub r: GaussianColorItem,
    pub g: GaussianColorItem,
    pub b: GaussianColorItem,
    pub a: GaussianColorItem,
}

#[derive(Debug)]
pub enum GaussianColorItem {
    NormalScale(f32),
    Absolute(u8),
    None,
}

/// Currently calculating a dog ( difference of gaussians ) with r = 1 for a
///  3x3 kernel
///
/// Additional parameters for kernel building:
/// τ = post normal scale of second gaussian
/// ε = cut off
///
pub struct GaussianBuilder {
    sigma_a: f32,
    sigma_b: f32,
    scalar: Option<f32>,
    cutoff: Option<f32>,
}

impl GaussianBuilder {
    pub fn create(sigma_a: f32, sigma_b: f32) -> GaussianBuilder {
        assert!(sigma_a > 0.0 && sigma_b > 0.0, "Sigmas must be positive");
        assert!(sigma_a < sigma_b, "Sigma 2 must be greater than sigma 1");

        GaussianBuilder {
            sigma_a,
            sigma_b,
            scalar: None,
            cutoff: None,
        }
    }

    pub fn scalar(mut self, scalar: f32) -> GaussianBuilder {
        self.scalar = Some(scalar);
        self
    }

    pub fn cutoff(mut self, cutoff: f32) -> GaussianBuilder {
        self.cutoff = Some(cutoff);
        self
    }

    pub fn build_kernel(&self) -> GaussianKernelData {
        let (gaussian_a, sum_a, gaussian_b, sum_b) = self.calculate_continuous_gaussians();

        let kernel = self.calculate_normalised_difference(gaussian_a, sum_a, gaussian_b, sum_b);

        GaussianKernelData {
            kernel,
            cutoff: self.cutoff,
        }
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
        &self,
        gaussian_a: GaussianKernel,
        sum_a: f32,
        gaussian_b: GaussianKernel,
        sum_b: f32,
    ) -> GaussianKernel {
        let mut res = GaussianKernel::default();

        if let Some(scalar) = self.scalar {
            for idx in 0..9usize {
                // res[idx] = ((1.0 + scalar) * (gaussian_a[idx] / sum_a))
                //     - (scalar * (gaussian_b[idx] / sum_b));
                res[idx] = (gaussian_a[idx] / sum_a) - (scalar * (gaussian_b[idx] / sum_b));
            }
        } else {
            for idx in 0..9usize {
                res[idx] = (gaussian_a[idx] / sum_a) - (gaussian_b[idx] / sum_b);
            }
        }

        res
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
    fn calculate_continuous_gaussians(&self) -> (GaussianKernel, f32, GaussianKernel, f32) {
        let mut gaussian_a = GaussianKernel::default();
        let mut gaussian_b = GaussianKernel::default();

        let mut sum_a = 0.0f32;
        let mut sum_b = 0.0f32;

        for (iy, y) in (-1..=1).enumerate() {
            for (ix, x) in (-1..=1).enumerate() {
                let val_a = Self::calculate_continuous(self.sigma_a, x as f32, y as f32);
                gaussian_a[ix + (3 * iy)] = val_a;
                sum_a += val_a;

                let val_b = Self::calculate_continuous(self.sigma_b, x as f32, y as f32);
                gaussian_b[ix + (3 * iy)] = val_b;
                sum_b += val_b;
            }
        }

        (gaussian_a, sum_a, gaussian_b, sum_b)
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
}
