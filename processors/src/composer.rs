use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::{
    GaussianKernelData,
    sobel::{SobelAscii, SobelColorData},
    utils::LuminanceAsciiMeta,
};

/// I still need to figure out the correct way to do this.
///
/// The general idea is that every step is creating it's own data and
/// I need a way to collect it all to render
///
/// E.g.
/// Start (Get RGB Page) (render 1) - - - - - - - - - - - ┐
/// └ Get Luminance (Makes a new Luminance Page)          |
///     └ Get Sobel Colored (new RGB Page) (render 2) - - ┤
///                                                       v
///                                                   OUT as RGB
///
/// In this example, we've made a new RGB image.
/// The render priority means that `render 2` get applied on top of render 1.
/// This results in an image with border highlighted in the sobel colour.
///
/// Considerations:
///  - Opacity, RGB has no opacity so render 2 will have every pixel data of render 1
///     - Maybe include an inclusion array?
///     - Maybe use RGBA?
///
///
///
#[derive(Debug)]
pub struct ProcessorComposer {
    steps: Vec<ComposerStep>,
}

#[derive(Debug)]
pub struct ProcessorPage {
    pub(crate) signal: ProcessorPageSignal,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) data: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum ProcessorPageSignal {
    RGB,
    Luminance,
    Char,
}

#[derive(Debug)]
struct ComposerStep {
    // function: ComposerStepItem,
    parent: usize,
    children: Vec<usize>,
    render: usize,
    is_active: bool,
}

#[derive(Debug)]
pub enum ComposerStepItem {
    FlipY,
    Luminance(LuminanceSteps),
    RGB(RGBSteps),
}

#[derive(Debug)]
pub enum LuminanceSteps {
    Gaussian(GaussianKernelData),
    SobelColour(SobelColorData),
    SobelAscii(SobelAscii),
    LuminanceAscii(LuminanceAsciiMeta),
}

#[derive(Debug)]
pub enum RGBSteps {
    ToLuminance(f32),
}

impl ProcessorPage {
    pub fn flip_y(&self) -> Self {
        let mut out_buff = vec![0u8; self.data.len()];

        match self.signal {
            ProcessorPageSignal::Luminance | ProcessorPageSignal::Char => {
                out_buff
                    .par_chunks_mut(self.width)
                    .enumerate()
                    .for_each(|(y, row)| {
                        let mut left = y * self.width;
                        let mut right = left + self.width - 1;

                        let mut step = 0;

                        while right >= left {
                            row[step] = self.data[right];
                            row[(self.width - step) - 1] = self.data[left];

                            left += 1;
                            right -= 1;
                            step += 1;
                        }
                    });
            }
            // Need to account for this correctly
            // [r1, g1, b1, r2, g2, b2]
            // Need to swap these all right to result:
            // [r2, g2, b2, r1, g1, b1]
            ProcessorPageSignal::RGB => {
                todo!("Flip Y on RGB Page");
                // out_buff
                //     .par_chunks_mut(self.width)
                //     .enumerate()
                //     .for_each(|(y, row)| {
                //         let mut left = y * self.width;

                //         let mut step = 0;

                //         for x in (left..(left + self.width.div_ceil(3))).rev() {
                //             let out = step + 3;

                //             row[out] = self.data[left + x * 3];

                //             step += 1;
                //         }

                //         let mut right = left + self.width - 1;

                //         while right >= left {
                //             row[step] = self.data[right];
                //             row[(self.width - step) - 1] = self.data[left];

                //             left += 1;
                //             right -= 1;
                //             step += 1;
                //         }
                //     });
            }
        }

        ProcessorPage {
            width: self.width,
            height: self.height,
            signal: self.signal,
            data: out_buff,
        }
    }
}

impl ProcessorComposer {
    pub fn create() -> ProcessorComposer {
        ProcessorComposer {
            steps: vec![ComposerStep {
                parent: 0,
                children: vec![],
                render: 0,
                is_active: true,
            }],
        }
    }

    pub fn add_step(&mut self, parent: usize) {
        let new_idx = self.steps.len();

        let parent_item = self
            .steps
            .get_mut(parent)
            .expect("Should be able to get parent here");

        parent_item.children.push(new_idx);

        self.steps.push(ComposerStep {
            parent,
            children: vec![],
            render: 0,
            is_active: true,
        });
    }

    pub fn remove_step(&mut self, item_idx: usize) {
        assert_ne!(item_idx, 0, "Not allowed to remove first item");

        let item = self.steps.get_mut(item_idx).expect("Item should exist");
        item.is_active = false;

        let parent_idx = item.parent;

        if parent_idx != 0 {
            let parent = self
                .steps
                .get_mut(parent_idx)
                .expect("Should be able to get item parent here");

            parent.children.retain(|c| *c != item_idx);
        }
    }

    /// Todo:
    /// I want to make these like os file path lines:
    ///
    /// Start
    ///  ├ A
    ///  | └ Aa
    ///  ├ B
    ///  | ├ Ba
    ///  | | ├ Baa
    ///  | | └ Bab
    ///  | └ Bb
    ///  └ C
    ///
    /// Let's assume we have this Vec:
    /// [ Start, A, Aa, B, Ba, Baa, Bab, Bb, C]
    ///
    ///
    ///
    /// We first start by getting the amount of nested items each index has:
    ///
    /// [
    ///   (Start) -> 8, ( Vec len - 1 )
    ///   (A)     -> 1, ( Aa )
    ///   (Aa)    -> 0,
    ///   (B)     -> 4, (Ba, Baa, Bab, Bb)
    ///   (Ba)    -> 2, (Baa, Bab)
    ///   (Baa)   -> 0,
    ///   (Bab)   -> 0,
    ///   (Bb)    -> 0,
    ///   (C)     -> 0,
    /// ]
    ///
    ///
    pub fn get_lines(&self) -> Vec<String> {
        let mut res = vec![];

        let abs_children: Vec<usize> = self
            .steps
            .iter()
            .enumerate()
            .skip(1)
            .filter_map(|(idx, c)| if c.parent == 0 { Some(idx) } else { None })
            .collect();

        if abs_children.is_empty() {
            return res;
        }

        let _z_children_len = abs_children.len();

        // for (stack_idx, curr_idx) in abs_children.into_iter().enumerate() {
        //     let meta_char = if stack_idx == z_children_len - 1 {
        //         '├'
        //     } else {
        //         '└'
        //     };

        //     res.push(format!("{meta_char} {curr_idx}"));

        //     let curr = self.steps.get(curr_idx).expect("parent");

        //     let children = curr.children.iter().filter_map(|idx| {
        //         let child = self.steps.get(curr_idx).expect("chile");

        //         if child.is_active { Some(idx) } else { None }
        //     });

        //     let children: Vec<usize> = self
        //         .steps
        //         .iter()
        //         .enumerate()
        //         .skip(abs_parent + 1)
        //         .filter_map(|(idx, c)| {
        //             if c.parent == abs_parent {
        //                 Some(idx)
        //             } else {
        //                 None
        //             }
        //         })
        //         .collect();
        // }

        res
    }
}
