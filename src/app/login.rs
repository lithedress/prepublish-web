use yew::prelude::*;
use yew::{function_component, html, Properties};
use yew_router::prelude::*;
use crate::app::common::{AppConfig, AuthBody, Route};

async fn login_post(cfg: AppConfig, body: AuthBody) -> Html {
    let token = match cfg.get_token().await {
        Ok(token) => token,
        Err(e) => return e,
    };
    let url = match cfg.api_addr.join("login") {
        Ok(url) => url,
        Err(e) => {
            return html! {
                <div>
                    { e.to_string() }
                    { "Please report this error to our administrator!" }
                    { cfg.api_addr.clone() }
                </div>
            }
        }
    };
    enum Res {
        OK,
        Other { status: u16, msg: String },
    }
    let res = (async move {
        let res = gloo::net::http::Request::post(url.as_str())
            .header("x-csrf-token", &token)
            .json(&body)?
            .send()
            .await?;
        if res.ok() {
            Ok(Res::OK)
        } else {
            let status = res.status();
            res.text().await.map(|msg| Res::Other { status, msg })
        }
    })
    .await;
    match res {
        Ok(ref res) => match res {
            Res::OK => html! {
                <Redirect<Route> to={Route::Home}/>
            },
            Res::Other { status, msg } => match status {
                500 => html! {
                    <div>
                        <p>{ status }</p>
                        <p>{ msg }</p>
                        <p>{ "Please report this error to our administrator!" }</p>
                    </div>
                },
                _ => html!(msg),
            },
        },
        Err(ref failure) => html! {
            <div>
                <p>{ failure.to_string() }</p>
                <p>{ "Please report this error to our administrator!" }</p>
            </div>
        },
    }
}

#[derive(PartialEq, Properties)]
pub(super) struct LoginProps {
    pub(super) cfg: AppConfig,
}

#[function_component]
pub(super) fn Login(props: &LoginProps) -> Html {
    let cfg = use_state(|| props.cfg.clone());
    let email = use_state(AttrValue::default);
    let password = use_state(AttrValue::default);
    let msg_box = use_state(|| html!());
    let onclick = {
        let email = email.clone();
        let password = password.clone();
        let msg_box = msg_box.clone();
        move |_| {
            let msg_box = msg_box.clone();
            let cfg = (*cfg).clone();
            let body = AuthBody {
                password: (*password).clone(),
                email: (*email).clone(),
            };
            wasm_bindgen_futures::spawn_local(async move { msg_box.set(login_post(cfg, body).await) })
        }
    };
    html! {
        <div>
            <p>
                <label>
                    { "Email Address: " }
                    <input type="email" value={(*email).clone()} />
                </label>
            </p>

            <p>
                <label>
                    { "Password: " }
                    <input type="password" value={(*password).clone()} />
                </label>
            </p>

            <p>
                <button {onclick}>{"Log in"}</button><br />
            </p>

            { (*msg_box).clone() }<br />
        </div>
    }
}