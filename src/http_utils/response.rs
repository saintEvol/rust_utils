#[cfg(feature = "http_types")]
use serde::{Deserialize, Serialize};

#[cfg(all(feature = "http_types", feature = "http_serde_camel_case"))]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T, CodeType = i32, >
where
    T: Serialize
{
    pub code: CodeType,
    pub msg: Option<String>,
    pub data: Option<T>
}

#[cfg(all(feature = "http_types", not(feature = "http_serde_camel_case")))]
#[derive(Serialize, Deserialize)]
pub struct Response<T, CodeType = i32, >
where
    T: Serialize
{
    pub code: CodeType,
    pub msg: Option<String>,
    pub data: Option<T>
}

#[cfg(feature = "http_types")]
impl<T> Response<T>
where
    T: Serialize
{
    pub fn success(data: Option<T>) -> Response<T> {
        Response {
            code: 0,
            msg: None,
            data,
        }
    }

    pub fn fail(code: i32, msg: Option<String>) -> Response<T> {
        Response {
            code,
            msg,
            data: None
        }
    }

    pub fn fail_with_data(code: i32, msg: Option<String>, data: T) -> Response<T> {
        Response {
            code,
            msg,
            data: Some(data)
        }
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // pub fn data(&self) -> Option<&T> {
    //     self.data.as_ref()
    // }
}