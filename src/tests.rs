#[cfg(test)]
mod tests {

    use std::{
        fs::File,
        io::{BufReader, BufWriter, Write},
        path::Path,
        process::exit,
    };

    use crate::{
        Libraw,
        libraw_sys::{
            LibRaw_KodakSensors_LIBRAW_Kodak_C14, libraw_COLOR, libraw_raw2image,
            libraw_subtract_black, strerror,
        },
    };
    use image::{
        self as LImage, DynamicImage, GenericImageView, ImageBuffer, ImageFormat, ImageReader,
        Luma, LumaA, Rgb, Rgb32FImage, RgbImage, buffer::ConvertBuffer, imageops,
    };

    #[test]
    fn init_and_close() {
        let mut lr_data = Libraw::new(0).unwrap();

        lr_data.open_file("./test_files/DSCF0730.RAF").unwrap();

        let unpkg_res = lr_data.unpack();

        match unpkg_res {
            Err(e) => assert!(
                false,
                "{}",
                unsafe { std::ffi::CStr::from_ptr(strerror(e)) }
                    .to_str()
                    .unwrap()
            ),
            Ok(_) => (),
        }

        let mut img_raw: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(
            *lr_data.sizes.raw_width as u32,
            *lr_data.sizes.raw_height as u32,
            lr_data.rawdata.raw_image.unwrap().to_vec(),
        )
        .unwrap();

        let img_raw = imageops::crop(
            &mut img_raw,
            0,
            0,
            (*lr_data.sizes.width).into(),
            (*lr_data.sizes.height).into(),
        )
        .to_image();

        let mut img_f32: ImageBuffer<Luma<f32>, Vec<f32>> = img_raw.convert();

        //выравнивание по максимальному значению
        for pixel in img_f32.pixels_mut() {
            pixel.0[0] = pixel.0[0] * (u16::MAX as f32 / *lr_data.color.maximum as f32);
        }

        //баланс белого
        let wb = unsafe { (*lr_data.data).color.cam_mul };
        let mut max = 0.;
        for el in wb {
            if el > max {
                max = el;
            };
        }
        for (x, y, px) in img_f32.enumerate_pixels_mut() {
            let c = unsafe { libraw_COLOR(lr_data.data, y as i32, x as i32) } as usize;
            px.0[0] *= wb[c] / max;
        }

        //Конвертация плоского Raw в RGB
        let mut img_rgb = RgbImage::new(
            (*lr_data.sizes.width).into(),
            (*lr_data.sizes.height).into(),
        );

        let mut img_f32_iter = img_f32.iter();
        for (x, y, pixel) in img_rgb.enumerate_pixels_mut() {
            let c = unsafe { libraw_COLOR(lr_data.data, y as i32, x as i32) } as usize;
            pixel.0[c] = (img_f32_iter.next().unwrap() * 255.0).round() as u8
        }

        let _ = img_rgb.save_with_format(Path::new("./my.png"), ImageFormat::Png);

        assert!(true);
    }
}
