pub mod ractor;
pub mod time;
pub mod math;
pub mod types;
pub mod rand;
pub mod jwt;
pub mod http_utils;
pub mod encrypt;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
