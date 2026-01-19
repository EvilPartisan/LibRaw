#[cfg(test)]
mod tests {

    use std::{
        fs::File,
        io::{BufWriter, Write},
    };

    use crate::{Libraw, libraw_sys::strerror};

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

        let mut file = File::create("foo.pgm").unwrap();
        let mut buf_file = BufWriter::new(file);

        let _ = buf_file.write_all(b"P5\n");

        let _ = buf_file.write_all(
            lr_data
                .sizes
                .raw_width
                .unwrap()
                .clone()
                .to_string()
                .as_bytes(),
        );
        let _ = buf_file.write_all(b"\n");
        let _ = buf_file.write_all(
            lr_data
                .sizes
                .raw_height
                .unwrap()
                .clone()
                .to_string()
                .as_bytes(),
        );

        let _ = buf_file.write_all(b"\n255\n");

        let img = lr_data.rawdata.raw_image.unwrap();

        for el in img {
            let _ = buf_file.write_all(&[el.to_le_bytes()[0]]);
        }

        let _ = buf_file.flush();

        assert!(true);
    }
}
