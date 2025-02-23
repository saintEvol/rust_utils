/// 专门处理http里的bear类型的jwt授权管理
/// 基本用法参考[crate::jwt::jwt_provider::JwtProvider]
///
#[cfg(feature = "jwt_bear")]
use crate::http_utils::utils::get_bear_token;
#[cfg(feature = "jwt_bear")]
use crate::jwt::jwt_auth_provider::JwtAuthProvider;
#[cfg(feature = "jwt_bear")]
use crate::jwt::jwt_payload::JwtPayload;
#[cfg(feature = "jwt_bear")]
use crate::jwt::jwt_provider::{AuthBody, AuthError, JwtProvider};
#[cfg(feature = "jwt_bear")]
use crate::jwt::jwt_storage_provider::JwtStorageProvider;
#[cfg(feature = "jwt_bear")]
use http::request::Parts;

#[cfg(feature = "jwt_bear")]
pub enum BearAuthError<StorageError, DecodeError> {
    BearError(String),
    AuthError(AuthError<StorageError, DecodeError>),
}

#[cfg(feature = "jwt_bear")]
pub struct JwtBearerProvider<JwtAuthProviderType, JwtStorageProviderType> {
    jwt_provider: JwtProvider<JwtAuthProviderType, JwtStorageProviderType>,
}

#[cfg(feature = "jwt_bear")]
impl<JwtAuthProviderType, JwtStorageProviderType>
    JwtBearerProvider<JwtAuthProviderType, JwtStorageProviderType>
where
    JwtStorageProviderType: super::jwt_storage_provider::JwtStorageProvider,
{
    pub fn new(
        expire_in_ms: i64,
        auth_provider: JwtAuthProviderType,
        storage_provider: JwtStorageProviderType,
    ) -> Self {
        let jwt_provider = JwtProvider::new(expire_in_ms, auth_provider, storage_provider);
        JwtBearerProvider { jwt_provider }
    }

    pub async fn authorize<JwtPayloadType>(
        &self,
        payload: JwtPayloadType,
    ) -> Result<
        AuthBody,
        AuthError<
            <JwtStorageProviderType as JwtStorageProvider>::Error,
            <JwtAuthProviderType as JwtAuthProvider<JwtPayload<JwtPayloadType>>>::Error,
        >,
    >
    where
        JwtPayloadType: serde::Serialize + std::cmp::PartialEq,
        JwtAuthProviderType: super::jwt_auth_provider::JwtAuthProvider<JwtPayload<JwtPayloadType>>,
    {
        self.jwt_provider.authorize(payload).await
    }

    /// 从 [http::request::Parts]中提取bear数据，并进行检验
    /// 如果检验成功，则返回token对应的payload数据
    pub async fn verify<JwtPayloadType>(
        &self,
        parts: &mut Parts,
    ) -> Result<
        JwtPayloadType,
        BearAuthError<
            <JwtStorageProviderType as JwtStorageProvider>::Error,
            <JwtAuthProviderType as JwtAuthProvider<JwtPayload<JwtPayloadType>>>::Error,
        >,
    >
    where
        JwtPayloadType: serde::Serialize + std::cmp::PartialEq,
        JwtAuthProviderType: super::jwt_auth_provider::JwtAuthProvider<JwtPayload<JwtPayloadType>>,
    {
        let token = get_bear_token(parts).await;
        match token {
            Ok(token) => {
                let ret = self.jwt_provider.verify::<JwtPayloadType>(&token).await;
                match ret {
                    Ok(ret) => Ok(ret),
                    Err(e) => Err(BearAuthError::AuthError(e)),
                }
            }
            Err(e) => Err(BearAuthError::BearError(format!("{:?}", e))),
        }
    }
}
