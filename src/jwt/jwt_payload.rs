#[cfg(feature = "jwt")]
use chrono::{Duration, Local};
#[cfg(feature = "jwt")]
use serde::Deserialize;
#[cfg(feature = "jwt")]
use serde::Serialize;

#[cfg(feature = "jwt")]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct JwtPayload<PayLoadType> {
    pub token_id: String,
    pub payload: PayLoadType,
    pub expire_ms: i64,
}

#[cfg(feature = "jwt")]
impl<PayLoadType> JwtPayload<PayLoadType> {
    pub fn new(token_id: String, payload: PayLoadType, expire_in_ms: i64) -> Self {
        let iat = Local::now();
        let expire_ms = iat + Duration::milliseconds(expire_in_ms as i64);
        let expire_ms = expire_ms.timestamp_millis() as i64;
        JwtPayload {
            token_id,
            payload,
            expire_ms,
        }
    }
}
