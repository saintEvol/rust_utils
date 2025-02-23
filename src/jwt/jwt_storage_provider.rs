#[cfg(feature = "jwt")]
pub trait JwtStorageProvider {
    type Error;

    fn save(
        &self,
        auth_body: super::jwt_provider::AuthBody,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    fn load(
        &self,
        token_id: &str,
    ) -> impl std::future::Future<Output = Result<Option<super::jwt_provider::AuthBody>, Self::Error>>;
}
