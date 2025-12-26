use std::path::Path;

use image::imageops::FilterType;
use visusta_core::{SobelColorData, SobelColorItem, VisustaProcessor};
use visusta_cpu::VisustaCPU;

/// If this doesn't work I'm going to be so mad

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = Path::new("./test_images/owl_2.jpg");

    let img = image::open(path)?
        // Sizings
        // .resize(944, 531, FilterType::Lanczos3)
        // .resize(1280, 720, FilterType::Lanczos3)
        // .resize(2560, 1440, FilterType::Lanczos3)
        //buff
        .to_rgba8();

    let cpu = VisustaCPU;

    let filter = SobelColorData {
        magnitude_min: 10,
        r: SobelColorItem::NormalScale(1.0),
        g: SobelColorItem::GyScale(1.0),
        b: SobelColorItem::GxScale(1.0),
        a: SobelColorItem::Absolute(255),
    };

    let img_b = cpu.sobel_to_colour(&img, filter).await;

    cpu.overlay_image(&img, &img_b)
        .await
        .save("./processed.png")?;

    Ok(())
}
