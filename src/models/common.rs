#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Parse(#[from] url::ParseError),
    #[error(transparent)]
    Response(#[from] gloo::net::Error),
    #[error("{0}")]
    Js(serde_json::Value),
    #[error(transparent)]
    Json(serde_json::Error),
}

impl From<wasm_bindgen::JsValue> for AppError {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        match gloo::utils::format::JsValueSerdeExt::into_serde::<serde_json::Value>(&value) {
            Ok(json) => Self::Js(json),
            Err(e) => Self::Json(e),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(serde::Deserialize)]
#[derive(PartialEq, yew::Properties)]
#[derive(Clone)]
pub struct AppConfig {
    pub api: std::rc::Rc<url::Url>,
}

impl AppConfig {
    pub async fn get_token(&self) -> AppResult<String> {
        gloo::net::http::Request::get(self.api.as_str())
            .send()
            .await
            .map(|res| res.headers().get("x-csrf-token").unwrap_or_default()).map_err(Into::into)
    }
}

#[derive(PartialEq)]
#[derive(Clone)]
pub struct FetchOther {
    pub status: u16,
    pub msg: yew::AttrValue,
}

pub enum FetchRes<T: serde::de::DeserializeOwned> {
    Body(std::rc::Rc<T>),
    Other(FetchOther),
}

impl<T: serde::de::DeserializeOwned> FetchRes<T> {
    pub async fn try_from_gloo_res(res: Result<gloo::net::http::Response, gloo::net::Error>) -> AppResult<Self> {
        let res = res?;
        if res.ok() {
            let body = res.json().await;
            Ok(FetchRes::Body(body?))
        } else {
            let msg = res.text().await;
            Ok(FetchRes::Other(FetchOther {
                status: res.status(),
                msg: msg?.into(),
            }))
        }
    }
}