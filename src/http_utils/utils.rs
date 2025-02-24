#[cfg(feature = "http_utils")]
use axum_core::RequestPartsExt;
#[cfg(feature = "http_utils")]
use axum_core::extract::FromRequestParts;
#[cfg(feature = "http_utils")]
use axum_extra::TypedHeader;
#[cfg(feature = "http_utils")]
use axum_extra::headers::Authorization;
#[cfg(feature = "http_utils")]
use axum_extra::headers::authorization::Bearer;
#[cfg(feature = "http_utils")]
use http::request::Parts;

#[cfg(feature = "http_utils")]
pub async fn get_bear_token(
    parts: &mut Parts,
) -> Result<String, <TypedHeader<Authorization<Bearer>> as FromRequestParts<TypedHeader<Authorization<Bearer>>>>::Rejection>{
    // Extract the token from the authorization header
    match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
        Ok(TypedHeader(Authorization(bearer))) => {
            // Decode the user data
            let bearer_data = bearer.token();
            Ok(bearer_data.to_owned())
        }
        Err(e) => Err(e),
    }
    // let TypedHeader(Authorization(bearer)) =
    //     parts.extract::<TypedHeader<Authorization<Bearer>>>().await;
}
