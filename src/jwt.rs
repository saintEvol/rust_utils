///
/// jwt相关功能
/// 用户可以通过[jwt_provider::JwtProvider]进行授权token的管理与校验
/// 用户可以通过实现[jwt_auth_provider::JwtAuthProvider]，来自定义token的加密与解密方式
/// 当前提供了[jwt_auth_provider::HmacAuthProvider]以提供默认的Hmac加解密功能
/// 用户可以通过实现[jwt_storage_provider::JwtStorageProvider]来管理token的存储
///
///
pub mod jwt_auth_provider;
pub mod jwt_payload;
pub mod jwt_provider;
pub mod jwt_storage_provider;
pub mod jwt_bear_provider;

#[cfg(feature = "jwt_test")]
#[cfg(test)]
mod test {
    use crate::jwt::jwt_provider::AuthBody;
    use crate::jwt::{jwt_auth_provider, jwt_provider, jwt_storage_provider};
    use std::sync::RwLock;

    struct TestAutoStorageProvider {
        auth: RwLock<Option<AuthBody>>,
    }

    impl TestAutoStorageProvider {
        fn new() -> TestAutoStorageProvider {
            TestAutoStorageProvider {
                auth: RwLock::new(None),
            }
        }
    }

    impl jwt_storage_provider::JwtStorageProvider for TestAutoStorageProvider {
        type Error = ();

        async fn save(&self, auth_body: AuthBody) -> Result<(), Self::Error> {
            let mut write = self.auth.write().unwrap();
            *write = Some(auth_body);
            Ok(())
        }

        async fn load(&self, _token_id: &str) -> Result<Option<AuthBody>, Self::Error> {
            let read = self.auth.read().unwrap();
            Ok(read.clone())
        }

        async fn remove(&self, _token_id: &str) -> Result<Option<AuthBody>, Self::Error> {
            let mut write = self.auth.write().unwrap();
            let ret = write.take();
            Ok(ret)
        }
    }

    #[cfg(feature = "jwt_test")]
    #[tokio::test]
    async fn test_jwt_auth_provider() {
        let auth_provider = jwt_auth_provider::HmacAuthProvider::from_secret(
            "dsfwerwerw".as_bytes(),
        );
        let storage_provider = TestAutoStorageProvider::new();
        let jwt = jwt_provider::JwtProvider::new(
            1000,
            auth_provider,
            storage_provider,
        );
        // let payload = &(1, 100);
        let auth_ret = jwt.authorize((1, 100)).await;
        assert_eq!(true, auth_ret.is_ok());
        let auth = auth_ret.unwrap();
        println!("auth: {:?}", auth);
        let ret = jwt.verify::<(i32, i32)>(&auth.token).await;
        println!("verify ret: {:?}", ret);
        let ret = ret.unwrap();
        assert_eq!((1, 100), ret.payload);

        // 测试过期
        tokio::time::sleep(std::time::Duration::from_millis(990)).await;
        let ret = jwt.verify::<(i32, i32)>(&auth.token).await;
        assert!(ret.is_ok());
        tokio::time::sleep(std::time::Duration::from_millis(1001)).await;
        let ret = jwt.verify::<(i32, i32)>(&auth.token).await;
        println!("verify ret 1: {:?}", ret);
        assert!(ret.is_err());
    }
}
