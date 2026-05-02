mod filters;
mod gaussians;
mod pipeline;
mod cpu;
mod gpu;
pub mod processors;

pub use filters::{
    AsciiSpaceType, CharImage, LumaAImage,
    LuminanceAsciiFilter, LuminanceFilter,
    SobelAscii, SobelColorData, SobelColorItem,
};
pub use gaussians::{GaussianBuilder, GaussianColorData, GaussianColorItem, GaussianKernelData};
pub use pipeline::{Layer, LayerOutput, PipelineErrorKind, Pipeline, ProcessingStep};

pub enum Processor {
    Cpu(cpu::VisustaCPU),
    Gpu(gpu::VisustaGPU),
}

impl Processor {
    pub fn process(&self, step: &ProcessingStep, input: LayerOutput) -> Result<LayerOutput, PipelineErrorKind> {
        match self {
            Processor::Cpu(p) => p.process(step, input),
            Processor::Gpu(p) => p.process(step, input),
        }
    }

    pub fn overlay(&self, layers: &[LayerOutput]) -> Option<LayerOutput> {
        match self {
            Processor::Cpu(p) => p.overlay(layers),
            Processor::Gpu(p) => p.overlay(layers),
        }
    }
}

pub async fn get_processor() -> Processor {
    if detect_gpu().await {
        Processor::Gpu(gpu::VisustaGPU { cpu: cpu::VisustaCPU })
    } else {
        Processor::Cpu(cpu::VisustaCPU)
    }
}

async fn detect_gpu() -> bool {
    let instance = wgpu::Instance::default();
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .is_ok()
}
