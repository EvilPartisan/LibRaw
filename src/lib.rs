#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
pub mod libraw_sys {
    include!("libraw-sys.rs");
}
mod tests;

use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr::slice_from_raw_parts_mut;
use std::slice;
use std::str::FromStr;

use image::{ImageBuffer, ImageFormat, Luma};

use crate::libraw_sys::*;

#[derive(Debug)]
pub struct Libraw<'a> {
    data: *mut libraw_data_t,
    rawdata: Rawdata<'a>,
    sizes: ImageSizes<'a>,
    color: ColorData<'a>,
    image: Option<&'a [[u16; 4]]>,
    image_rs: Option<ImageBuffer<Luma<f32>, Vec<f32>>>,
}
#[derive(Debug)]
struct Rawdata<'a> {
    raw_image: Option<&'a [u16]>,
}
#[derive(Debug)]
struct ImageSizes<'a> {
    pub raw_height: &'a u16,
    pub raw_width: &'a u16,
    pub height: &'a u16,
    pub width: &'a u16,
    pub iheight: &'a u16,
    pub iwidth: &'a u16,
}

#[derive(Debug)]
struct ColorData<'a> {
    pub maximum: &'a u32,
}

impl Libraw<'_> {
    pub fn new(flags: u32) -> Option<Self> {
        let lr_data = unsafe { libraw_init(flags) };

        if lr_data.is_null() {
            return None;
        }

        Some(Libraw {
            data: lr_data,
            rawdata: Rawdata { raw_image: None },
            sizes: ImageSizes {
                raw_height: &0,
                raw_width: &0,
                height: &0,
                width: &0,
                iheight: &0,
                iwidth: &0,
            },
            color: ColorData {
                maximum: unsafe { &(*lr_data).color.maximum },
            },
            image: None,
            image_rs: None,
            //image: unsafe { &(*lr_data).image},,
        })
    }

    pub fn luma_to_rgb() {
        todo!();
    }

    pub fn close(&mut self) {
        unsafe { libraw_close(self.data) };
    }

    pub fn recycle(&self) {
        unsafe { libraw_recycle(self.data) };
    }

    //int LibRaw::open_file(const char *filename[,INT64 bigfile_size])

    pub fn unpack(&mut self) -> Result<&Self, LibRaw_errors> {
        let unpkg_res = unsafe { libraw_unpack(self.data) };

        if unpkg_res != LibRaw_errors_LIBRAW_SUCCESS {
            return Err(unpkg_res);
        }

        self.sizes.raw_width = unsafe { &(*self.data).rawdata.sizes.raw_width };
        self.sizes.raw_height = unsafe { &(*self.data).rawdata.sizes.raw_height };
        self.sizes.width = unsafe { &(*self.data).rawdata.sizes.width };
        self.sizes.height = unsafe { &(*self.data).rawdata.sizes.height };

        let raw_image_c = unsafe {
            slice::from_raw_parts(
                (*self.data).rawdata.raw_image,
                *self.sizes.raw_height as usize * *self.sizes.raw_width as usize,
            )
        };

        self.rawdata.raw_image = Some(raw_image_c);

        let mut raw_float = Vec::new();
        for el in self.rawdata.raw_image.unwrap().to_vec() {
            raw_float.push((el as f32) / *self.color.maximum as f32);
        }

        self.image_rs = ImageBuffer::from_raw(
            *self.sizes.raw_width as u32,
            *self.sizes.raw_height as u32,
            raw_float,
        );

        Ok(self)
    }

    pub fn open_file(&mut self, filename: &str) -> Result<&Self, i32> {
        let a =
            unsafe { libraw_open_file(self.data, CString::from_str(filename).unwrap().as_ptr()) };

        if a != LibRaw_errors_LIBRAW_SUCCESS {
            return Err(a);
        }

        Ok(self)
    }

    pub fn ppm_tiff_writer(&mut self, filename: String) -> Result<&Self, LibRaw_errors> {
        unsafe {
            let res =
                libraw_dcraw_ppm_tiff_writer(self.data, CString::new(filename).unwrap().as_ptr());
            if res != LibRaw_errors_LIBRAW_SUCCESS {
                return Err(res);
            }
        }
        Ok(self)
    }

    pub fn dcraw_process(&mut self) -> Result<&Self, LibRaw_errors> {
        let e = unsafe { libraw_dcraw_process(self.data) };
        if e == LibRaw_errors_LIBRAW_SUCCESS {
            return Ok(self);
        }
        Err(e)
    }

    pub fn strerror(e: ::std::os::raw::c_int) -> &'static str {
        unsafe { CStr::from_ptr(libraw_strerror(e)).to_str().unwrap() }
    }
    pub fn raw2image(&mut self) {
        unsafe { libraw_raw2image(self.data) };

        self.sizes.iwidth = unsafe { &(*self.data).sizes.iwidth };
        self.sizes.iheight = unsafe { &(*self.data).sizes.iheight };
        let tmp = unsafe {
            &*slice_from_raw_parts_mut(
                (*self.data).image,
                (*self.sizes.iwidth as usize) * (*self.sizes.iheight as usize),
            )
        };
        self.image = Some(tmp);
    }
    pub fn demosaic(&self) -> &Self {
        self
    }
    pub fn gamma(&self) -> &Self {
        self
    }
    pub fn save_to_file_as(&self, format: ImageFormat, _file: &Path) -> Option<()> {
        let _ = format;
        Some(())
    }
}

impl Drop for Libraw<'_> {
    fn drop(&mut self) {
        self.recycle();
        self.close();
    }
}
