use serde::{Deserialize, Serialize};
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
