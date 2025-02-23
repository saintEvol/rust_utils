#[cfg(feature = "jwt")]
pub mod hmac_auth_provider;
#[cfg(feature = "jwt")]
pub use hmac_auth_provider::HmacAuthProvider;

#[cfg(feature = "jwt")]
/// 提供
pub trait JwtAuthProvider<Data: serde::Serialize> {
    type Error;
    fn encode(&self, payload: &Data) -> Result<String, Self::Error>;
    fn decode(&self, jwt: &str) -> Result<Data, Self::Error>;
}