use std::path::Path;

use image::{DynamicImage, RgbaImage, imageops::FilterType};
use visusta_core::{
    get_processor, Processor,
    GaussianBuilder, GaussianColorData, GaussianColorItem,
    Layer, LayerOutput, Pipeline, ProcessingStep,
    LuminanceAsciiFilter, LuminanceFilter,
    SobelAscii, SobelColorData, SobelColorItem,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        anyhow::bail!("Usage: {} <image_path>", args[0]);
    }

    let img = image::open(Path::new(&args[1]))?
        .resize(1280, 720, FilterType::Lanczos3)
        .to_rgba8();

    let processor = get_processor().await;

    run_pipeline_process(&img, &processor)?;

    Ok(())
}

fn create_ascii_pipeline() -> Pipeline {
    let ascii_filter = LuminanceAsciiFilter::create();
    let font_size = 10;

    let background = Layer::new()
        .add_step(ProcessingStep::ToLuminance(
            LuminanceFilter::create().multiplier(0.45).min(52),
        ))
        .add_step(ProcessingStep::GaussianOnLuma(
            GaussianBuilder::create(1.8, 2.25).scalar(0.5).cutoff(20.0),
        ))
        .add_step(ProcessingStep::LuminanceToAscii(
            ascii_filter.font_size(font_size),
        ));

    let foreground = Layer::new()
        .add_step(ProcessingStep::ToLuminance(
            LuminanceFilter::create().multiplier(1.0),
        ))
        .add_step(ProcessingStep::GaussianOnLuma(
            GaussianBuilder::create(1.0, 1.20).scalar(0.7).cutoff(16.0),
        ))
        .add_step(ProcessingStep::SobelAsciiDirectional(
            SobelAscii::create()
                .magnitude_min(64)
                .ascii_max(0.75)
                .font_size(font_size),
        ));

    Pipeline::new()
        .add_layer(background)
        .add_layer(foreground)
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

fn run_pipeline_process(
    img: &RgbaImage,
    processor: &Processor,
) -> anyhow::Result<()> {
    let pipeline = create_ascii_pipeline();

    let outputs = pipeline.execute(img, processor).map_err(|err| {
        println!("Pipeline Failure {err:?}");
        anyhow::anyhow!("Failed to run pipeline")
    })?;

    let result = processor
        .overlay(&outputs)
        .ok_or_else(|| anyhow::anyhow!("No layers to composite or type mismatch"))?;

    match result {
        LayerOutput::Rgba(rgba) => {
            rgba.save("./pipeline_output.png")?;
            println!("Saved to ./pipeline_output.png");
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
            println!("Saved to ./pipeline_output.png");
        }
    }

    Ok(())
}
