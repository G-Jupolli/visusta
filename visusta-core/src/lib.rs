use async_trait::async_trait;
use image::{ImageBuffer, LumaA, RgbaImage};

use crate::gaussians::{GaussianBuilder, GaussianColorData};
use crate::pipeline::LayerOutput;

pub mod gaussians;
pub mod pipeline;

#[derive(Debug, Clone, Copy)]
pub struct LuminanceFilter {
    pub multiplier: f32,
    pub min: u8,
}

impl LuminanceFilter {
    pub fn create() -> Self {
        LuminanceFilter {
            multiplier: 1.0,
            min: 0,
        }
    }

    pub fn multiplier(mut self, multiplier: f32) -> Self {
        self.multiplier = multiplier;
        self
    }

    pub fn min(mut self, min: u8) -> Self {
        self.min = min;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SobelColorData {
    pub magnitude_min: u8,
    pub r: SobelColorItem,
    pub g: SobelColorItem,
    pub b: SobelColorItem,
    pub a: SobelColorItem,
}

#[derive(Debug, Clone, Copy)]
pub enum SobelColorItem {
    NormalScale(f32),
    GxScale(f32),
    GyScale(f32),
    Absolute(u8),
    None,
}

#[derive(Debug, Clone)]
pub struct LuminanceAsciiFilter {
    pub font_size: usize,
    pub chars: [char; 10],
    pub space_type: AsciiSpaceType,
}

#[derive(Debug, Clone, Copy)]
pub enum AsciiSpaceType {
    Duplicate,
    Space,
    Raw(char),
}

#[derive(Debug, Clone)]
pub struct SobelAscii {
    pub font_size: usize,
    pub magnitude_min: u8,
    pub ascii_max: f32,
    pub chars: [char; 4],
    pub space_type: AsciiSpaceType,
}

impl LuminanceAsciiFilter {
    pub fn create() -> LuminanceAsciiFilter {
        LuminanceAsciiFilter {
            font_size: 10,
            chars: [' ', '.', ';', 'c', 'o', 'P', '0', '?', '@', '#'],
            space_type: AsciiSpaceType::Space,
        }
    }

    pub fn font_size(mut self, font_size: usize) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn chars(mut self, chars: [char; 10]) -> Self {
        self.chars = chars;
        self
    }

    pub fn space_type(mut self, space_type: AsciiSpaceType) -> Self {
        self.space_type = space_type;
        self
    }
}

impl SobelAscii {
    pub fn create() -> Self {
        SobelAscii {
            font_size: 10,
            magnitude_min: 10,
            ascii_max: 0.65,
            chars: ['|', '/', '-', '\\'],
            space_type: AsciiSpaceType::Space,
        }
    }

    pub fn font_size(mut self, font_size: usize) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn magnitude_min(mut self, magnitude_min: u8) -> Self {
        self.magnitude_min = magnitude_min;
        self
    }

    pub fn ascii_max(mut self, ascii_max: f32) -> Self {
        self.ascii_max = ascii_max;
        self
    }

    pub fn chars(mut self, chars: [char; 4]) -> Self {
        self.chars = chars;
        self
    }

    pub fn space_type(mut self, space_type: AsciiSpaceType) -> Self {
        self.space_type = space_type;
        self
    }
}

pub type LumaAImage = ImageBuffer<LumaA<u8>, Vec<u8>>;

pub struct CharImage {
    pub width: usize,
    pub height: usize,
    pub data: Vec<char>,
}

#[async_trait]
pub trait VisustaProcessor {
    async fn rgba_to_luma_a(&self, img: &RgbaImage, filter: LuminanceFilter) -> LumaAImage;

    async fn luma_to_rgba(&self, img: &LumaAImage) -> RgbaImage;

    async fn sobel_to_colour(&self, img: &LumaAImage, filter: SobelColorData) -> RgbaImage;

    async fn sobel_ascii_directional(&self, img: &LumaAImage, filter: SobelAscii) -> CharImage;

    async fn gaussian_on_luma(&self, img: &LumaAImage, builder: GaussianBuilder) -> LumaAImage;

    async fn gaussian_to_coloured(
        &self,
        img: &LumaAImage,
        builder: GaussianBuilder,
        filter: GaussianColorData,
    ) -> RgbaImage;

    async fn luminance_to_ascii(&self, img: &LumaAImage, filter: LuminanceAsciiFilter)
    -> CharImage;

    async fn overlay_layers(&self, layers: &[LayerOutput]) -> Option<LayerOutput>;
}
