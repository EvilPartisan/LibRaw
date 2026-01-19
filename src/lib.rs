#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
pub mod libraw_sys {
    include!("libraw-sys.rs");
}
mod tests;

use std::ffi::{CStr, CString};
use std::slice;
use std::str::FromStr;

use crate::libraw_sys::*;

#[derive(Debug)]
pub struct Libraw<'a> {
    data: *mut libraw_data_t,
    rawdata: Rawdata<'a>,
    sizes: ImageSizes<'a>,
}
#[derive(Debug)]
struct Rawdata<'a> {
    raw_image: Option<&'a [u16]>,
}
#[derive(Debug)]
struct ImageSizes<'a> {
    pub raw_height: Option<&'a u16>,
    pub raw_width: Option<&'a u16>,
}

// pub struct libraw_image_sizes_t {
//     pub raw_height: ushort,
//     pub raw_width: ushort,
//     pub height: ushort,
//     pub width: ushort,
//     pub top_margin: ushort,
//     pub left_margin: ushort,
//     pub iheight: ushort,
//     pub iwidth: ushort,
//     pub raw_pitch: ::std::os::raw::c_uint,
//     pub pixel_aspect: f64,
//     pub flip: ::std::os::raw::c_int,
//     pub mask: [[::std::os::raw::c_int; 4usize]; 8usize],
//     pub raw_aspect: ushort,
//     pub raw_inset_crops: [libraw_raw_inset_crop_t; 2usize],
// }

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
                raw_height: None,
                raw_width: None,
            },
        })
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

        let raw_height = unsafe { &(*self.data).rawdata.sizes.raw_height };
        let raw_width = unsafe { &(*self.data).rawdata.sizes.raw_width };

        self.sizes.raw_width = Some(raw_width);
        self.sizes.raw_height = Some(raw_height);

        let raw_image_c = unsafe {
            slice::from_raw_parts(
                (*self.data).rawdata.raw_image,
                *raw_height as usize * *raw_width as usize,
            )
        };

        self.rawdata.raw_image = Some(raw_image_c);

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
}

impl Drop for Libraw<'_> {
    fn drop(&mut self) {
        self.recycle();
        self.close();
    }
}
