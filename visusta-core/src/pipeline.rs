use image::RgbaImage;

use crate::{
    CharImage, LumaAImage, LuminanceAsciiFilter, LuminanceFilter, SobelAscii, SobelColorData,
    VisustaProcessor,
    gaussians::{GaussianBuilder, GaussianColorData},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Rgba,
    LumaA,
    Char,
}

#[derive(Debug)]
pub struct PipelineError {
    pub kind: PipelineErrorKind,
    pub location: PipelineLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineErrorKind {
    TypeMismatch { expected: DataType, got: DataType },
    LayerOutputMismatch { expected: DataType, got: DataType },
    ExecutionMismatch { expected: DataType, got: DataType },
    EmptyLayer,
    EmptyPipeline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineLocation {
    Pipeline,
    Layer { layer: usize },
    Step { layer: usize, step: usize },
}

#[derive(Debug, Clone)]
pub enum ProcessingStep {
    // RgbaImage -> LumaAImage
    ToLuminance(LuminanceFilter),

    // LumaAImage -> RgbaImage
    LumaToRgba,
    SobelToColour(SobelColorData),
    GaussianToColoured(GaussianBuilder, GaussianColorData),

    // LumaAImage -> LumaAImage
    GaussianOnLuma(GaussianBuilder),

    // LumaAImage -> CharImage
    LuminanceToAscii(LuminanceAsciiFilter),
    SobelAsciiDirectional(SobelAscii),
}

impl ProcessingStep {
    fn signature(&self) -> (DataType, DataType) {
        match self {
            // RgbaImage -> LumaAImage
            ProcessingStep::ToLuminance(_) => (DataType::Rgba, DataType::LumaA),

            // LumaAImage -> RgbaImage
            ProcessingStep::LumaToRgba => (DataType::LumaA, DataType::Rgba),
            ProcessingStep::SobelToColour(_) => (DataType::LumaA, DataType::Rgba),
            ProcessingStep::GaussianToColoured(_, _) => (DataType::LumaA, DataType::Rgba),

            // LumaAImage -> LumaAImage
            ProcessingStep::GaussianOnLuma(_) => (DataType::LumaA, DataType::LumaA),

            // LumaAImage -> CharImage
            ProcessingStep::LuminanceToAscii(_) => (DataType::LumaA, DataType::Char),
            ProcessingStep::SobelAsciiDirectional(_) => (DataType::LumaA, DataType::Char),
        }
    }

    async fn execute(
        &self,
        input: LayerOutput,
        processor: &dyn VisustaProcessor,
    ) -> Result<LayerOutput, PipelineErrorKind> {
        let output = match self {
            ProcessingStep::ToLuminance(filter) => {
                let img = input.into_rgba()?;
                LayerOutput::LumaA(processor.rgba_to_luma_a(&img, *filter).await)
            }
            ProcessingStep::LumaToRgba => {
                let img = input.into_luma()?;
                LayerOutput::Rgba(processor.luma_to_rgba(&img).await)
            }
            ProcessingStep::SobelToColour(filter) => {
                let img = input.into_luma()?;
                LayerOutput::Rgba(processor.sobel_to_colour(&img, filter.clone()).await)
            }
            ProcessingStep::GaussianToColoured(builder, filter) => {
                let img = input.into_luma()?;
                LayerOutput::Rgba(
                    processor
                        .gaussian_to_coloured(&img, builder.clone(), filter.clone())
                        .await,
                )
            }
            ProcessingStep::GaussianOnLuma(builder) => {
                let img = input.into_luma()?;
                LayerOutput::LumaA(processor.gaussian_on_luma(&img, builder.clone()).await)
            }
            ProcessingStep::LuminanceToAscii(filter) => {
                let img = input.into_luma()?;
                LayerOutput::Char(processor.luminance_to_ascii(&img, filter.clone()).await)
            }
            ProcessingStep::SobelAsciiDirectional(filter) => {
                let img = input.into_luma()?;
                LayerOutput::Char(
                    processor
                        .sobel_ascii_directional(&img, filter.clone())
                        .await,
                )
            }
        };

        Ok(output)
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    steps: Vec<ProcessingStep>,
    output_type: DataType,
}

impl Layer {
    pub fn new() -> Self {
        Layer {
            steps: Vec::new(),
            output_type: DataType::Rgba,
        }
    }

    pub fn add_step(mut self, step: ProcessingStep) -> Self {
        let (_, res_type) = step.signature();
        self.steps.push(step);
        if self.output_type != res_type {
            self.output_type = res_type;
        }
        self
    }

    fn validate(&self, self_idx: usize) -> Result<DataType, PipelineError> {
        let mut steps_iter = self.steps.iter();

        let Some(prev) = steps_iter.next() else {
            return Err(PipelineError {
                kind: PipelineErrorKind::EmptyLayer,
                location: PipelineLocation::Layer { layer: self_idx },
            });
        };

        let (prev_input, mut out) = prev.signature();

        if prev_input != DataType::Rgba {
            return Err(PipelineError {
                kind: PipelineErrorKind::TypeMismatch {
                    expected: DataType::Rgba,
                    got: prev_input,
                },
                location: PipelineLocation::Step {
                    layer: self_idx,
                    step: 0,
                },
            });
        }

        let mut step = 0;
        while let Some((next_input, next_output)) = steps_iter.next().map(|s| s.signature()) {
            step += 1;
            if out != next_input {
                return Err(PipelineError {
                    kind: PipelineErrorKind::TypeMismatch {
                        expected: out,
                        got: next_input,
                    },
                    location: PipelineLocation::Step {
                        layer: self_idx,
                        step,
                    },
                });
            }

            out = next_output;
        }

        Ok(out)
    }

    async fn execute(
        &self,
        processor: &dyn VisustaProcessor,
        img: &RgbaImage,
    ) -> Result<LayerOutput, (PipelineErrorKind, usize)> {
        let mut current = LayerOutput::Rgba(img.clone());

        for (step_index, step) in self.steps.iter().enumerate() {
            current = step
                .execute(current, processor)
                .await
                .map_err(|kind| (kind, step_index))?;
        }

        Ok(current)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Pipeline {
    layers: Vec<Layer>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline { layers: Vec::new() }
    }

    pub fn add_layer(mut self, layer: Layer) -> Self {
        self.layers.push(layer);
        self
    }

    pub fn validate(&self) -> Result<DataType, PipelineError> {
        let mut layers = self.layers.iter();

        let Some(prev) = layers.next() else {
            return Err(PipelineError {
                kind: PipelineErrorKind::EmptyPipeline,
                location: PipelineLocation::Pipeline,
            });
        };

        let expected_out = prev.validate(0)?;

        let mut layer_idx = 0;

        while let Some(layer) = layers.next() {
            layer_idx += 1;
            let out = layer.validate(layer_idx)?;

            if expected_out != out {
                return Err(PipelineError {
                    kind: PipelineErrorKind::LayerOutputMismatch {
                        expected: expected_out,
                        got: out,
                    },
                    location: PipelineLocation::Layer { layer: layer_idx },
                });
            }
        }

        Ok(expected_out)
    }

    pub async fn execute(
        &self,
        img: &RgbaImage,
        processor: &dyn VisustaProcessor,
    ) -> Result<Vec<LayerOutput>, PipelineError> {
        self.validate()?;

        let mut outputs = Vec::with_capacity(self.layers.len());

        for (layer_index, layer) in self.layers.iter().enumerate() {
            outputs.push(
                layer
                    .execute(processor, img)
                    .await
                    .map_err(|(kind, step_idx)| PipelineError {
                        kind,
                        location: PipelineLocation::Step {
                            layer: layer_index,
                            step: step_idx,
                        },
                    })?,
            );
        }

        Ok(outputs)
    }
}

pub enum LayerOutput {
    Rgba(RgbaImage),
    LumaA(LumaAImage),
    Char(CharImage),
}

impl LayerOutput {
    pub fn data_type(&self) -> DataType {
        match self {
            LayerOutput::Rgba(_) => DataType::Rgba,
            LayerOutput::LumaA(_) => DataType::LumaA,
            LayerOutput::Char(_) => DataType::Char,
        }
    }

    pub fn into_rgba(self) -> Result<RgbaImage, PipelineErrorKind> {
        match self {
            LayerOutput::Rgba(img) => Ok(img),
            _ => Err(PipelineErrorKind::ExecutionMismatch {
                expected: DataType::Rgba,
                got: self.data_type(),
            }),
        }
    }

    pub fn into_luma(self) -> Result<LumaAImage, PipelineErrorKind> {
        match self {
            LayerOutput::LumaA(img) => Ok(img),
            _ => Err(PipelineErrorKind::ExecutionMismatch {
                expected: DataType::LumaA,
                got: self.data_type(),
            }),
        }
    }

    pub fn into_char(self) -> Result<CharImage, PipelineErrorKind> {
        match self {
            LayerOutput::Char(img) => Ok(img),
            _ => Err(PipelineErrorKind::ExecutionMismatch {
                expected: DataType::Char,
                got: self.data_type(),
            }),
        }
    }
}
