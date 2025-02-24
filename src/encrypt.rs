#[cfg(feature = "encrypt")]
/// 使用盐对字符串进行加密
pub fn encrypt_with_salt(password: &str, salt: &str) -> String {
    use std::fmt::Write;
    let s = password.to_owned() + salt;
    let digest = md5::compute(s).to_vec();

    let mut result = String::new();
    for a in digest.iter() {
        write!(result, "{:02x}", a).unwrap();
    }
    result
}
