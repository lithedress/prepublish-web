use serde::{Deserialize, Serialize, de::DeserializeOwned};
use yew::{AttrValue, Properties};
use std::rc::Rc;
use yew_router::Routable;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Clone, Routable, PartialEq)]
pub(super) enum Route {
    #[at("/")]
    Home,
    #[at("/signup")]
    Signup,
    #[at("/login")]
    Login,
    #[at("/theses")]
    ThesesRoot,
    #[at("/theses/*")]
    Theses,
}

#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Properties)]
#[derive(Clone)]
pub(super) struct AppConfig {
    pub(super) api_addr: Rc<url::Url>,
}

impl AppConfig {
    pub(super) async fn get_token(&self) -> Result<String, yew::Html> {
        gloo::net::http::Request::get(self.api_addr.as_str())
            .send()
            .await
            .map(|res| res.headers().get("x-csrf-token").unwrap_or_default())
            .map_err(|e| yew::html! {
                <div>
                    { e.to_string() }
                    { "Please report this error to our administrator!" }
                    { self.api_addr.clone() }
                </div>
            })
    }
}

#[serde_as]
#[derive(Serialize, PartialEq, Clone)]
pub(super) struct AuthBody {
    #[serde_as(as = "DisplayFromStr")]
    pub(super)email: AttrValue,
    #[serde_as(as = "DisplayFromStr")]
    pub(super)password: AttrValue,
}

#[derive(thiserror::Error, Debug)]
pub(super) enum FetchError {
    #[error(transparent)]
    Parse(#[from] url::ParseError),
    #[error(transparent)]
    Response(#[from] gloo::net::Error)
}

impl FetchError {
    pub(super) fn view(&self) -> yew::Html {
        yew::html! {
            <div>
                <p>{ self.to_string() }</p>
                <p>{ "Please report this error to our administrator!" }</p>
            </div>
        }
    }
}

pub(super) enum FetchRes<T: DeserializeOwned> {
    OK(T),
    Other {
        status: u16, msg: String
    },
}