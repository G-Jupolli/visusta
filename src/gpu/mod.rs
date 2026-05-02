use crate::cpu::VisustaCPU;
use crate::pipeline::{LayerOutput, PipelineErrorKind, ProcessingStep};

pub struct VisustaGPU {
    pub cpu: VisustaCPU,
}

impl VisustaGPU {
    pub fn process(&self, step: &ProcessingStep, input: LayerOutput) -> Result<LayerOutput, PipelineErrorKind> {
        self.cpu.process(step, input)
    }

    pub fn overlay(&self, layers: &[LayerOutput]) -> Option<LayerOutput> {
        self.cpu.overlay(layers)
    }
}
