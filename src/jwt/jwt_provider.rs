/// 用于用户授权的工具模块
/// 封装了token的生成，加密，存储与校验流程
/// 用户可以自定义加密与解码方式，见[super::jwt_auth_provider::JwtAuthProvider]
/// 当前提供了一些默认的加密器，见[super::jwt_auth_provider::HmacAuthProvider]
/// 用户也可以自定义生成的token的存储方式，见[super::jwt_storage_provider::JwtStorageProvider]
///
#[cfg(feature = "jwt")]
use crate::jwt::jwt_auth_provider::JwtAuthProvider;
#[cfg(feature = "jwt")]
use crate::jwt::jwt_payload::JwtPayload;
#[cfg(feature = "jwt")]
use crate::jwt::jwt_storage_provider::JwtStorageProvider;
#[cfg(feature = "jwt")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "jwt")]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthBody {
    pub token_id: String,
    pub token: String,
    // pub token_type: String,
    pub expire_ms: i64,
    pub expire_in_ms: i64,
}

#[cfg(feature = "jwt")]
#[derive(Debug)]
pub enum AuthError<StorageError, DecodeError> {
    StorageError(StorageError),
    DecodeError(DecodeError),
    OutOfDate,
    NoAuthDataFound,
    AuthDataNotMatch,
}

#[cfg(feature = "jwt")]
pub struct JwtProvider<JwtPayloadType, JwtAuthProviderType, JwtStorageProviderType> {
    payload: JwtPayload<JwtPayloadType>,
    expire_in_ms: i64,
    // config: JwtAuthConfig,
    coder: JwtAuthProviderType,
    saver: JwtStorageProviderType,
}

#[cfg(feature = "jwt")]
impl<PayloadType, JwtAuthProviderType, JwtStorageProviderType>
    JwtProvider<PayloadType, JwtAuthProviderType, JwtStorageProviderType>
where
    PayloadType: serde::Serialize + std::cmp::PartialEq,
    JwtStorageProviderType: super::jwt_storage_provider::JwtStorageProvider,
    JwtAuthProviderType: super::jwt_auth_provider::JwtAuthProvider<JwtPayload<PayloadType>>,
{
    pub fn new(
        expire_in_ms: i64,
        payload: PayloadType,
        coder: JwtAuthProviderType,
        saver: JwtStorageProviderType,
    ) -> JwtProvider<PayloadType, JwtAuthProviderType, JwtStorageProviderType> {
        let token_id = scru128::new_string();
        let payload = JwtPayload::new(token_id, payload, expire_in_ms);
        JwtProvider {
            payload,
            expire_in_ms,
            coder,
            saver,
        }
    }

    /// 授权并返回token
    pub async fn authorize(
        &self,
    ) -> Result<
        String,
        AuthError<
            <JwtStorageProviderType as JwtStorageProvider>::Error,
            <JwtAuthProviderType as JwtAuthProvider<JwtPayload<PayloadType>>>::Error,
        >,
    > {
        let body = self.gen_auth_body();
        match body {
            Ok(body) => {
                let token = body.token.clone();
                match self.save(body).await {
                    Ok(_) => {
                        Ok(token)
                    }
                    Err(e) => {
                        Err(AuthError::StorageError(e))
                    }
                }
            }
            Err(e) => Err(AuthError::DecodeError(e)),
        }
    }

    /// 检查指定token是否有效
    /// 从客户端收到token后，可以将token传入此函数检查其是否是一个有效的token（未过时，数据是否正确）
    pub async fn verify(
        &self,
        token: &str,
    ) -> Result<
        PayloadType,
        AuthError<
            <JwtStorageProviderType as JwtStorageProvider>::Error,
            <JwtAuthProviderType as JwtAuthProvider<JwtPayload<PayloadType>>>::Error,
        >,
    > {
        // let auth = self.saver.load(token_id).await?;
        match self.coder.decode(token) {
            Ok(payload) => match self.saver.load(&payload.token_id).await {
                Ok(saved) => {
                    if let Some(saved) = saved {
                        let now = crate::time::now_millis() as i64;
                        println!("check timem, now: {now}, expire ms: {}", saved.expire_ms);
                        if now < saved.expire_ms {
                            match self.coder.decode(&saved.token) {
                                Ok(saved_payload) => {
                                    if saved_payload.pay_load == payload.pay_load {
                                        Ok(saved_payload.pay_load)
                                    } else {
                                        Err(AuthError::AuthDataNotMatch)
                                    }
                                }
                                Err(e) => Err(AuthError::DecodeError(e)),
                            }
                        } else {
                            Err(AuthError::OutOfDate)
                        }
                    } else {
                        Err(AuthError::NoAuthDataFound)
                    }
                }
                Err(e) => Err(AuthError::StorageError(e)),
            },
            Err(e) => Err(AuthError::DecodeError(e)),
        }
    }

    fn gen_auth_body(
        &self,
    ) -> Result<AuthBody, <JwtAuthProviderType as JwtAuthProvider<JwtPayload<PayloadType>>>::Error>
    {
        let token = self.coder.encode(&self.payload)?;
        let ab = AuthBody {
            token_id: self.payload.token_id.clone(),
            token,
            expire_ms: self.payload.expire_ms,
            expire_in_ms: self.expire_in_ms,
        };
        Ok(ab)
    }

    async fn save(
        &self,
        auth_body: AuthBody,
    ) -> Result<(), <JwtStorageProviderType as JwtStorageProvider>::Error> {
        self.saver.save(auth_body).await
    }
}
