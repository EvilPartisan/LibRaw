#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
pub mod libraw {
    include!("../bindings.rs");
}
mod tests;

use errno::{Errno, errno, set_errno};
use std::ffi::{CStr, CString};
use std::fmt::Error;
use std::ptr::{null, null_mut};
use std::str::FromStr;

use libraw::libraw_data_t as LibRaw;
use libraw::*;

impl LibRaw {
    pub fn new(flags: u32) -> Option<Self> {
        let lr_data = unsafe { libraw::libraw_init(flags) };
        if lr_data == null_mut() {
            return None;
        }
        return Some(unsafe { *lr_data });
    }
    pub fn close(&mut self) {
        unsafe { libraw_close(self) };
    }

    //int LibRaw::open_file(const char *filename[,INT64 bigfile_size])

    pub fn open_file(&mut self, filename: &str) -> Result<&Self, i32> {
        let a = unsafe { libraw_open_file(self, CString::from_str(filename).unwrap().as_ptr()) };
        //println!("{:#?}", errno);

        println!("============Error {:#?}", self.strerror(a));
        // if a != 0 {
        //     return Err(a);
        // };
        return Ok(self);
    }

    pub fn strerror(&self, e: ::std::os::raw::c_int) -> &'static str {
        unsafe { CStr::from_ptr(libraw_strerror(e as i32)).to_str().unwrap() }
    }
}
