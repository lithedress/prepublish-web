use yew::{prelude::*, suspense::use_future};
use yew_router::prelude::*;

use self::common::{AppConfig, Route};

mod common;
mod signup;
mod login;
mod profiles;
mod theses;

#[function_component]
fn AppWithConfig(props: &AppConfig) -> Html {
    let cfg = use_state(|| props.clone());
    let switch = move |routes| match routes {
        Route::Home => html! {
            <div>
                { "WIP" }
            </div>
        },
        Route::Signup => html! {
            <signup::Signup cfg={(*cfg).clone()} />
        },
        Route::Login => html! {
            <login::Login cfg={(*cfg).clone()} />
        },
        Route::ThesesRoot | Route::Theses => html! {
            <Switch<theses::Route> render={theses::Route::switch((*cfg).clone())} />
        }
    };
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component]
fn AppConfigGet() -> HtmlResult {
    let res = use_future(|| async {
        gloo::net::http::Request::get("/config.json")
            .send()
            .await?
            .text()
            .await
    })?;
    let res = match *res {
        Ok(ref res) => {
            let cfg = serde_json::from_str::<AppConfig>(res);
            match cfg {
                Ok(cfg) => html! {
                    <AppWithConfig ..cfg />
                },
                Err(e) => html! {
                    <div>
                        { e.to_string() }<br />
                        { "Please report this error to our administrator!" }<br />
                        { res }
                    </div>
                },
            }
        }
        Err(ref failure) => html! {
            <div>
                { failure.to_string() }<br />
                { "Please report this error to our administrator!" }<br />
            </div>
        },
    };
    Ok(res)
}

#[function_component(App)]
pub fn app() -> Html {
    let fallback = html! {<div>{"Loading website configuration..."}</div>};
    html! {
        <Suspense {fallback}>
            <AppConfigGet />
        </Suspense>
    }
}
