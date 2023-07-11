use yew::{prelude::*, suspense::use_future};
use yew_router::prelude::*;

use crate::{models::common::AppConfig, views::Route};

#[function_component]
fn AppWithConfig(props: &AppConfig) -> Html {
    let cfg = use_state(|| props.clone());
    html! {
        <BrowserRouter>
            <Switch<Route> render={Route::switch(std::rc::Rc::from((*cfg).to_owned()))} />
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
