#![allow(dead_code)]
// see more in https://www.reddit.com/r/rust/comments/1gtaai2/question_why_cant_two_static_strs_be_concatenated/
const A: &str = "AAA";
const B: &str = "BBB";

const TEMP: [u8; A.len() + B.len()] = {
    let mut result = [0u8; A.len() + B.len()];
    let mut i = 0_usize;
    while i < A.len() + B.len() {
        if i < A.len() {
            result[i] = A.as_bytes()[i];
        } else {
            result[i] = B.as_bytes()[i - A.len()];
        }
        i += 1;
    }
    result
};

pub const C: &str = match std::str::from_utf8(&TEMP) {
    Ok(s) => s,
    Err(_) => panic!(),
};

#[cfg(test)]
mod tests {
    use crate::concat_two_str_at_compile_time::C;
    #[test]
    fn test() {
        assert_eq!((const { C }), "AAABBB");
    }
}
