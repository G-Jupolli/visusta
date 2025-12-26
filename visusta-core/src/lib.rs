use async_trait::async_trait;
use image::RgbaImage;

#[derive(Debug)]
pub struct SobelColorData {
    pub magnitude_min: u8,
    pub r: SobelColorItem,
    pub g: SobelColorItem,
    pub b: SobelColorItem,
    pub a: SobelColorItem,
}

#[derive(Debug)]
pub enum SobelColorItem {
    NormalScale(f32),
    GxScale(f32),
    GyScale(f32),
    Absolute(u8),
    None,
}

#[async_trait]
pub trait VisustaProcessor {
    async fn sobel_to_colour(&self, img: &RgbaImage, filter: SobelColorData) -> RgbaImage;

    async fn overlay_image(&self, img_bg: &RgbaImage, img_fg: &RgbaImage) -> RgbaImage;
}

// pub async fn get_image_processor() -> Box<dyn ImageProcessor> {
//     let gpu_available = detect_gpu().await;

//     if gpu_available {
//         log::info!("GPU detected, using GPU processor");
//         // TODO: Return GPU processor once implemented
//         // Box::new(GpuImageProcessor::new().await.unwrap())
//         unimplemented!("GPU processor not yet implemented")
//     } else {
//         log::info!("No GPU detected, using CPU processor");
//         // TODO: Return CPU processor once implemented
//         // Box::new(CpuImageProcessor::new())
//         unimplemented!("CPU processor not yet implemented")
//     }
// }

// async fn detect_gpu() -> bool {
//     let instance = wgpu::Instance::default();

//     let adapter = instance
//         .request_adapter(&wgpu::RequestAdapterOptions {
//             power_preference: wgpu::PowerPreference::HighPerformance,
//             compatible_surface: None,
//             force_fallback_adapter: false,
//         })
//         .await;

//     adapter.is_some()
// }
