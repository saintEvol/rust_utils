use libm;

/// 对小数进行四舍五入
pub fn round(f: f64, n: f64) -> f64 {
    let t = libm::pow(10., n);
    (f * t).round() / t
}
