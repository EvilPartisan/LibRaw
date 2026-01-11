#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn init_and_close() {
        let mut lr_data = LibRaw::new(0).unwrap();
        lr_data.open_file("./test_files/DSCF0730.RAF").unwrap();
        lr_data.close();
    }
}
