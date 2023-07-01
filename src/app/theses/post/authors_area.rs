use yew::prelude::*;
use yew_router::prelude::*;
use yew::{use_state, UseStateHandle};
use yew::virtual_dom::VNode;
use bson::oid::ObjectId;
use web_sys::InputEvent;
use crate::app::common::{AppConfig, FetchRes};
use crate::app::profiles::{PublicProfile, TinyProfile};

pub struct AuthorsProps {
    cfg: AppConfig,
}

#[function_component]
pub fn Authors(cfg: UseStateHandle<AppConfig>, authors: UseStateHandle<Vec<(PublicProfile, TinyProfile)>>, msg_box: UseStateHandle<VNode>) -> Html {
    let author = use_state(|| ObjectId::from_bytes([0_u8; 12]));
    let is_good_author = use_state(bool::default);
    let oninput = {
        let authors = authors.clone();
        let author = author.clone();
        let is_good_author = is_good_author.clone();
        move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                match ObjectId::parse_str(input.value()) {
                    Ok(id) => {
                        is_good_author.set(authors.iter().find(|(p, _)| p._id == id).is_none());
                        author.set(id);
                    }
                    _ => {
                        is_good_author.set(false);
                    }
                }
            }
        }
    };
    let onclick = {
        let cfg = cfg.clone();
        let author = author.clone();
        let authors = authors.clone();
        let msg_box = msg_box.clone();
        move |_| {
            let cfg = cfg.clone();
            let author = author.clone();
            let authors = authors.clone();
            let msg_box = msg_box.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match PublicProfile::get((*cfg).clone(), *author).await {
                    Ok(res) => match res {
                        FetchRes::OK(body) => {
                            match ((*cfg).clone(), body.clone()).try_into() {
                                Ok(tiny_profile) => {
                                    let mut vals = (*authors).clone();
                                    vals.push((body, tiny_profile));
                                    authors.set(vals);
                                }
                                Err(e) => { msg_box.set(e.view()); }
                            }
                        }
                        FetchRes::Other(o) => {
                            msg_box.set(o.view());
                        }
                    }
                    Err(e) => { msg_box.set(e.view()); }
                }
            });
        }
    };
    html! {
            <div>
                <p>
                    <label>
                        { "Authors: " }
                        <ol>
                            { for authors.iter().map(|author| html!(<li>{ author.1.view() }</li>)) }
                        </ol>
                    </label>
                    <input type="text" {oninput} value={(*author).clone().to_hex()} />
                    <button {onclick} disabled={!(*is_good_author)}>{"Add keyword"}</button>
                </p>
            </div>
        }
}
