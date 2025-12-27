use async_trait::async_trait;
use image::RgbaImage;
use visusta_core::{
    CharImage, LumaAImage, LuminanceAsciiFilter, LuminanceFilter, SobelAscii, SobelColorData,
    VisustaProcessor,
    gaussians::{GaussianBuilder, GaussianColorData},
};
use visusta_cpu::VisustaCPU;

pub struct VisustaGPU {
    pub cpu: VisustaCPU,
}

#[async_trait]
impl VisustaProcessor for VisustaGPU {
    async fn rgba_to_luma_a(&self, img: &RgbaImage, filter: LuminanceFilter) -> LumaAImage {
        self.cpu.rgba_to_luma_a(img, filter).await
    }

    async fn sobel_to_colour(&self, img: &LumaAImage, filter: SobelColorData) -> RgbaImage {
        self.cpu.sobel_to_colour(img, filter).await
    }

    async fn gaussian_on_luma(&self, img: &LumaAImage, builder: GaussianBuilder) -> LumaAImage {
        self.cpu.gaussian_on_luma(img, builder).await
    }

    async fn gaussian_to_coloured(
        &self,
        img: &LumaAImage,
        builder: GaussianBuilder,
        filter: GaussianColorData,
    ) -> RgbaImage {
        self.cpu.gaussian_to_coloured(img, builder, filter).await
    }

    async fn luminance_to_ascii(
        &self,
        img: &LumaAImage,
        filter: LuminanceAsciiFilter,
    ) -> CharImage {
        self.cpu.luminance_to_ascii(img, filter).await
    }

    async fn sobel_ascii_directional(&self, img: &LumaAImage, filter: SobelAscii) -> CharImage {
        self.cpu.sobel_ascii_directional(img, filter).await
    }

    async fn overlay_image(&self, img_bg: &RgbaImage, img_fg: &RgbaImage) -> RgbaImage {
        self.cpu.overlay_image(img_bg, img_fg).await
    }
}
