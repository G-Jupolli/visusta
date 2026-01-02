use std::path::Path;

use image::{DynamicImage, RgbaImage, imageops::FilterType};
use visusta_core::{
    LuminanceAsciiFilter, LuminanceFilter, SobelAscii, SobelColorData, SobelColorItem,
    VisustaProcessor,
    gaussians::{GaussianBuilder, GaussianColorData, GaussianColorItem},
    pipeline::{Layer, LayerOutput, Pipeline, ProcessingStep},
};
use visusta_cpu::VisustaCPU;
use visusta_gpu::VisustaGPU;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        anyhow::bail!("Usage: {} <image_path>", args[0]);
    }

    let path = Path::new(&args[1]);

    let base_img = image::open(path)?
        // Sizings
        // .resize(944, 531, FilterType::Lanczos3)
        .resize(1280, 720, FilterType::Lanczos3)
    // .resize(2560, 1440, FilterType::Lanczos3)
    //buff
    ;
    let img = base_img.to_rgba8();

    let processor = get_image_processor().await;

    run_pipeline_process(&img, processor.as_ref()).await?;

    Ok(())
}

pub async fn get_image_processor() -> Box<dyn VisustaProcessor> {
    let gpu_available = detect_gpu().await;

    if gpu_available {
        println!("GPU detected");
        Box::new(VisustaGPU { cpu: VisustaCPU })
    } else {
        println!("No GPU detected, using CPU processor");
        Box::new(VisustaCPU)
    }
}

async fn detect_gpu() -> bool {
    let instance = wgpu::Instance::default();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await;

    println!("GPU adapter {adapter:?}");

    adapter.is_ok()
}

fn create_ascii_pipeline() -> Pipeline {
    let ascii_filter = LuminanceAsciiFilter::create();

    let font_size = 16;

    let mut chars = ascii_filter.chars;
    chars[1] = ' ';

    let background = Layer::new()
        .add_step(ProcessingStep::ToLuminance(
            LuminanceFilter::create().multiplier(1.0),
        ))
        .add_step(ProcessingStep::LuminanceToAscii(
            ascii_filter.chars(chars).font_size(font_size),
        ));

    let foreground = Layer::new()
        .add_step(ProcessingStep::ToLuminance(
            LuminanceFilter::create().multiplier(1.0),
        ))
        .add_step(ProcessingStep::GaussianOnLuma(
            GaussianBuilder::create(0.5, 2.25).scalar(0.5).cutoff(25.0),
        ))
        .add_step(ProcessingStep::SobelAsciiDirectional(
            SobelAscii::create()
                .magnitude_min(20)
                .ascii_max(0.675)
                .font_size(font_size),
        ));

    Pipeline::new().add_layer(background).add_layer(foreground)
}

fn _create_main_pipeline() -> Pipeline {
    let background = Layer::new()
        .add_step(ProcessingStep::ToLuminance(
            LuminanceFilter::create().multiplier(0.7),
        ))
        .add_step(ProcessingStep::LumaToRgba);

    let middleground = Layer::new()
        .add_step(ProcessingStep::ToLuminance(
            LuminanceFilter::create().multiplier(0.7),
        ))
        .add_step(ProcessingStep::GaussianToColoured(
            GaussianBuilder::create(0.5, 2.25).scalar(0.5).cutoff(40.0),
            GaussianColorData {
                r: GaussianColorItem::Absolute(255),
                g: GaussianColorItem::NormalScale(2.0),
                b: GaussianColorItem::NormalScale(2.0),
                a: GaussianColorItem::Absolute(255),
            },
        ));

    let foreground = Layer::new()
        .add_step(ProcessingStep::ToLuminance(
            LuminanceFilter::create().multiplier(0.7),
        ))
        .add_step(ProcessingStep::GaussianOnLuma(
            GaussianBuilder::create(0.5, 2.25).scalar(0.5).cutoff(40.0),
        ))
        .add_step(ProcessingStep::SobelToColour(SobelColorData {
            magnitude_min: 24,
            r: SobelColorItem::Absolute(7),
            g: SobelColorItem::Absolute(98),
            b: SobelColorItem::Absolute(180),
            a: SobelColorItem::Absolute(255),
        }));

    Pipeline::new()
        .add_layer(background)
        .add_layer(middleground)
        .add_layer(foreground)
}

async fn run_pipeline_process(
    img: &RgbaImage,
    processor: &dyn VisustaProcessor,
) -> anyhow::Result<()> {
    // let pipeline = create_main_pipeline();
    let pipeline = create_ascii_pipeline();

    let outputs = pipeline.execute(img, processor).await.map_err(|err| {
        println!("Pipeline Failure {err:?}");
        anyhow::anyhow!("Failed to do pipeline")
    })?;

    let result = processor
        .overlay_layers(&outputs)
        .await
        .ok_or_else(|| anyhow::anyhow!("No layers to composite or type mismatch"))?;

    match result {
        LayerOutput::Rgba(rgba) => {
            rgba.save("./pipeline_output.png")?;
            println!("Pipeline output saved to ./pipeline_output.png");
        }
        LayerOutput::Char(chars) => {
            for row in chars.data.chunks(chars.width) {
                println!("{}", row.iter().collect::<String>());
            }
        }
        LayerOutput::LumaA(luma) => {
            DynamicImage::from(luma)
                .to_rgba8()
                .save("./pipeline_output.png")?;
            println!("Pipeline output saved to ./pipeline_output.png");
        }
    }

    Ok(())
}
