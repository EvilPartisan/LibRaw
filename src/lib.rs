pub fn testlibraw(left: u64, right: u64) -> u64 {
    let res = left + right;
    println!("{}", res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = testlibraw(2, 2);
        assert_eq!(result, 4);
    }
}
