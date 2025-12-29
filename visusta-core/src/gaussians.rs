use std::f32::consts::PI;

type GaussianKernel = [f32; 9];

#[derive(Debug, Clone)]
pub struct GaussianKernelData {
    pub kernel: GaussianKernel,
    pub cutoff: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct GaussianColorData {
    pub r: GaussianColorItem,
    pub g: GaussianColorItem,
    pub b: GaussianColorItem,
    pub a: GaussianColorItem,
}

#[derive(Debug, Clone, Copy)]
pub enum GaussianColorItem {
    NormalScale(f32),
    Absolute(u8),
    None,
}

#[derive(Debug, Clone)]
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

    fn calculate_continuous(sigma: f32, x: f32, y: f32) -> f32 {
        let base = 2.0 * PI * sigma * sigma;
        let exp_power = 0.0 - ((x * x + y * y) / (2.0 * sigma * sigma));

        exp_power.exp() / base
    }
}
