use axum::http::StatusCode;
use serde::Serialize;

impl<T: Serialize> axum::response::IntoResponse for Res<T> {
    fn into_response(self) -> axum::response::Response {
        let code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let mut t = axum::Json(self).into_response();
        *t.status_mut() = code;

        t
    }
}

#[derive(Serialize, Debug)]
pub struct Res<T>
where
    T: Serialize,
{
    pub data: Option<T>,
    pub code: u16,
    pub message: String,
}

impl<T: Serialize> Res<T> {
    const CODE_SUCCESS: u16 = 200;
    const CODE_FAIL: u16 = 500;
    const MESSAGE_SUCCESS: &'static str = "success";
    const MESSAGE_FAIL: &'static str = "fail";

    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            code: Self::CODE_SUCCESS,
            message: Self::MESSAGE_SUCCESS.to_string(),
        }
    }

    pub fn failed(data: T) -> Self {
        Self {
            data: Some(data),
            code: Self::CODE_FAIL,
            message: Self::MESSAGE_SUCCESS.to_string(),
        }
    }

    pub fn fail() -> Self {
        Self {
            data: None,
            code: Self::CODE_FAIL,
            message: Self::MESSAGE_FAIL.to_string(),
        }
    }

    pub fn custom_fail(code: StatusCode, message: String) -> Self {
        Self {
            data: None,
            code: code.as_u16(),
            message,
        }
    }
}
