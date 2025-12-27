use std::path::Path;

use image::imageops::FilterType;
use visusta_core::{
    LuminanceAsciiFilter, LuminanceFilter, SobelAscii, VisustaProcessor,
    gaussians::{GaussianBuilder, GaussianColorData, GaussianColorItem},
};
use visusta_cpu::VisustaCPU;
use visusta_gpu::VisustaGPU;

/// If this doesn't work I'm going to be so mad

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = Path::new("./test_images/owl_2.jpg");

    let base_img = image::open(path)?
        // Sizings
        .resize(944, 531, FilterType::Lanczos3)
        // .resize(1280, 720, FilterType::Lanczos3)
    // .resize(2560, 1440, FilterType::Lanczos3)
    //buff
    ;
    let img = base_img.to_rgba8();

    let processor = get_image_processor().await;

    let luminance_img = processor
        .rgba_to_luma_a(&img, LuminanceFilter::create().multiplier(0.7))
        .await;
    let gaussian_builder = GaussianBuilder::create(0.5, 2.25).scalar(0.5).cutoff(40.0);
    let _gaussian_clr = GaussianColorData {
        r: GaussianColorItem::Absolute(7u8),
        g: GaussianColorItem::Absolute(98u8),
        b: GaussianColorItem::Absolute(180u8),
        a: GaussianColorItem::Absolute(255u8),
    };

    let img_b = processor
        .gaussian_on_luma(&luminance_img, gaussian_builder)
        // .gaussian_to_coloured(&luminance_img, gaussian_builder, gaussian_clr)
        .await;

    let ascii_filter = LuminanceAsciiFilter::create();

    let font_size = 12;

    let mut chars = ascii_filter.chars;
    // chars[1] = ' ';

    let a_chars = processor
        .luminance_to_ascii(
            &luminance_img,
            ascii_filter.chars(chars).font_size(font_size),
        )
        .await;

    let mut a_chunks = a_chars.data.chunks(a_chars.width);

    let b_chars = processor
        .sobel_ascii_directional(
            &img_b,
            SobelAscii::create()
                .magnitude_min(20)
                .ascii_max(0.65)
                .font_size(font_size),
        )
        .await;

    let mut b_chunks = b_chars.data.chunks(b_chars.width);

    assert!(
        a_chars.width == b_chars.width && a_chars.height == b_chars.height,
        "chars must be same size"
    );

    while let (Some(bg), Some(fg)) = (a_chunks.next(), b_chunks.next()) {
        let mut line_out = String::new();

        let mut fg = fg.into_iter();
        let mut bg = bg.into_iter();

        while let (Some(fg), Some(bg)) = (fg.next(), bg.next()) {
            line_out.push(if fg == &' ' { *bg } else { *fg });
        }

        println!("{line_out}");
    }

    // for c in a_chars.data.chunks(a_chars.width) {
    //     println!("{}", c.iter().collect::<String>());
    // }

    // .save("./j2_gaus_lum.png")
    // .save("./guas_clr.png")?;

    // let img_g = DynamicImage::from(img_g).to_rgba8();

    // let filter = SobelColorData {
    //     magnitude_min: 10,
    //     r: SobelColorItem::NormalScale(1.0),
    //     g: SobelColorItem::GyScale(1.0),
    //     b: SobelColorItem::GxScale(1.0),
    //     a: SobelColorItem::Absolute(255),
    // };

    // let filter = SobelColorData {
    // magnitude_min: 24,
    // r: SobelColorItem::Absolute(7),
    // g: SobelColorItem::Absolute(98),
    // b: SobelColorItem::Absolute(180),
    // a: SobelColorItem::Absolute(255),
    // };

    // let filter = SobelColorData {
    // magnitude_min: 10,
    // r: SobelColorItem::Absolute(255),
    // g: SobelColorItem::Absolute(255),
    // b: SobelColorItem::Absolute(255),
    // a: SobelColorItem::Absolute(255),
    // };

    // let img_a = base_img.grayscale().to_rgba8();
    // let img_b = processor.sobel_to_colour(&luminance_img, filter).await;

    // let img_b = processor.fill_sobel_gaps(&img_b).await;
    // let luminance_img = processor.rgba_to_luma_a(&img_b).await;

    // let gaussian_builder = GaussianBuilder::create(0.5, 2.25).scalar(0.5).cutoff(40.0);
    // let gaussian_clr = GaussianColorData {
    //     r: GaussianColorItem::Absolute(7u8),
    //     g: GaussianColorItem::Absolute(98u8),
    //     b: GaussianColorItem::Absolute(180u8),
    //     a: GaussianColorItem::Absolute(255u8),
    // };

    // let img_b = processor
    //     // .gaussian_on_luma(&luminance_img, gaussian_builder)
    //     .gaussian_to_coloured(&luminance_img, gaussian_builder, gaussian_clr)
    //     .await;

    // let img_b = image::imageops::resize(&img_b, 2560, 1440, FilterType::Lanczos3);

    // processor
    // .overlay_image(&img_g, &img_b)
    // .await
    // img_b.save("./b.png")?;

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
