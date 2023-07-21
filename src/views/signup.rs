use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use yew::prelude::*;
use yew::{function_component, use_state, AttrValue, Html, Properties};
use yew_router::prelude::*;

use crate::models::common::AppConfig;

#[serde_as]
#[derive(Serialize)]
struct SignupBody {
    #[serde_as(as = "DisplayFromStr")]
    password: AttrValue,
    #[serde_as(as = "DisplayFromStr")]
    email: AttrValue,
    #[serde_as(as = "DisplayFromStr")]
    name: AttrValue,
}


async fn signup_post(cfg: &AppConfig, body: SignupBody) -> Html {
    let token = gloo::net::http::Request::get(cfg.api.as_str())
        .send()
        .await
        .map(|res| res.headers().get("x-csrf-token"));
    let token = match token {
        Ok(token) => token.unwrap_or_default(),
        Err(e) => {
            return html! {
                <div>
                    { e.to_string() }
                    { "Please report this error to our administrator!" }
                    { cfg.api.clone() }
                </div>
            }
        }
    };
    let url = match cfg.api.join("signup") {
        Ok(url) => url,
        Err(e) => {
            return html! {
                <div>
                    { e.to_string() }
                    { "Please report this error to our administrator!" }
                    { cfg.api.clone() }
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
                <Redirect<super::route::Route> to={super::route::Route::Login}/>
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
pub(crate) struct SignupProps {
    pub(crate) cfg: std::rc::Rc<AppConfig>,
}

#[function_component]
pub(crate) fn Signup(props: &SignupProps) -> Html {
    let cfg = use_state(|| props.cfg.clone());
    let email = use_state(AttrValue::default);
    let password = use_state(AttrValue::default);
    let password_again = use_state(AttrValue::default);
    let good_password = use_state(bool::default);
    let name = use_state(AttrValue::default);
    let msg_box = use_state(Html::default);
    let onclick = {
        let email = email.clone();
        let password = password.clone();
        let name = name.clone();
        let msg_box = msg_box.clone();
        move |_| {
            let msg_box = msg_box.clone();
            let cfg = (*cfg).clone();
            let body = SignupBody {
                password: (*password).clone(),
                email: (*email).clone(),
                name: (*name).clone(),
            };
            wasm_bindgen_futures::spawn_local(async move { msg_box.set(signup_post(&cfg, body).await) })
        }
    };
    let check_email = {
        let email = email.clone();
        move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                email.set(input.value().into());
            }
        }
    };
    let check_password = {
        let password = password.clone();
        let password_again = password_again.clone();
        let good_password = good_password.clone();
        move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                good_password.set(
                    !input.value().is_empty()
                        && !password_again.is_empty()
                        && &input.value() == password_again.as_str(),
                );
                password.set(input.value().into());
            }
        }
    };
    let check_password_again = {
        let password = password.clone();
        let password_again = password_again.clone();
        let good_password = good_password.clone();
        move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                good_password.set(
                    !password.is_empty()
                        && !input.value().is_empty()
                        && password.as_str() == &input.value(),
                );
                password_again.set(input.value().into());
            }
        }
    };
    let check_name = {
        let name = name.clone();
        move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                name.set(input.value().into());
            }
        }
    };
    html! {
        <div>
            <p>
                <label>
                    { "Email Address: " }
                    <input type="email" oninput={check_email} value={(*email).clone()} />
                </label>
            </p>

            <p>
                <label>
                    { "Password: " }
                    <input type="password" oninput={check_password} value={(*password).clone()} />
                    { if *good_password { " ✅" } else { " ❎" }}
                </label>
            </p>

            <p>
                <label>
                    { "Password again: " }
                    <input type="password" oninput={check_password_again} value={(*password_again).clone()} />
                </label>
            </p>

            <p>
                <label>
                    { "Name: " }
                    <input type="text" oninput={check_name} value={(*name).clone()} />
                </label>
            </p>

            <p>
                <button {onclick} disabled={!(*good_password)}>{"Sign up"}</button>
            </p>

            { (*msg_box).clone() }
        </div>
    }
}
