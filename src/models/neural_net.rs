use crate::Result;

use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb};
use std::io::Read;
use tract_onnx::{prelude::*, tract_core::ndarray};

type TractModel = RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

pub struct NeuralNet {
    model: TractModel,
}

const NUM_CHANNELS: usize = Rgb::<u8>::CHANNEL_COUNT as usize;
const WIDTH: usize = 512;
const HEIGHT: usize = 512;

impl NeuralNet {
    pub fn new<R: Read>(mut reader: R) -> Result<Self> {
        let model = tract_onnx::onnx()
            .model_for_read(&mut reader)?
            .with_input_fact(
                0,
                InferenceFact::dt_shape(f32::datum_type(), tvec!(1, NUM_CHANNELS, HEIGHT, WIDTH)),
            )?
            .into_optimized()?
            .into_runnable()?;

        Ok(Self { model })
    }

    pub fn predict(&self, img: DynamicImage) -> Result<Arc<Tensor>> {
        let resized = resize_and_pad_img(img, WIDTH as u32, HEIGHT as u32);

        const NORM_SCALE: f32 = 1. / 255.;
        let tensor =
            ndarray::Array4::from_shape_fn((1, NUM_CHANNELS, HEIGHT, WIDTH), |(_, c, y, x)| {
                resized[(x as _, y as _)][c] as f32 * NORM_SCALE
            })
            .into();

        let output = self.model.run(tvec!(tensor))?;
        Ok(output[0].clone())
    }
}

// based on https://github.com/image-rs/imageproc/blob/5a7a68bfe54d27d531edcadf16b032930fe1a54c/src/geometric_transformations.rs#L335-L376
fn resize_and_pad_img(
    img: DynamicImage,
    target_width: u32,
    target_height: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (tw, th) = (target_width, target_height);
    if img.dimensions() == (tw, th) {
        return img.into_rgb8();
    }

    let img = img.resize(tw, th, FilterType::CatmullRom).into_rgb8();
    let (w, h) = img.dimensions();
    if w == tw && h == th {
        return img;
    }

    let mut out = ImageBuffer::<Rgb<u8>, _>::new(tw, th);
    if w < tw {
        let margin = (tw as usize - w as usize) / 2;

        for y in 0..th {
            let p_min = *img.get_pixel(0, y);
            for x in 0..margin as u32 {
                out.put_pixel(x, y, p_min);
            }

            let in_base = y as usize * w as usize * NUM_CHANNELS;
            let out_base = (margin + y as usize * tw as usize) * NUM_CHANNELS;
            let len = w as usize * NUM_CHANNELS;
            (*out)[out_base..][..len].copy_from_slice(&(*img)[in_base..][..len]);

            let p_max = *img.get_pixel(w - 1, y);
            for x in tw - margin as u32 - 1..tw {
                out.put_pixel(x, y, p_max);
            }
        }

        return out;
    } else if h < th {
        let margin = (th as usize - h as usize) / 2;
        let line_len = tw as usize * NUM_CHANNELS;
        let whole_len = w as usize * h as usize * NUM_CHANNELS;

        for y in 0..margin {
            let out_base = y * tw as usize * NUM_CHANNELS;
            (*out)[out_base..][..line_len].copy_from_slice(&(*img)[..line_len]);
        }

        let out_base = margin * tw as usize * NUM_CHANNELS;
        (*out)[out_base..][..whole_len].copy_from_slice(&(*img)[..whole_len]);

        let in_base = (h as usize - 1) * tw as usize * NUM_CHANNELS;
        for y in th as usize - margin - 1..th as usize {
            let out_base = y * tw as usize * NUM_CHANNELS;
            (*out)[out_base..][..line_len].copy_from_slice(&(*img)[in_base..][..line_len]);
        }

        return out;
    }

    unreachable!()
}
