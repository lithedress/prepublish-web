use yew::prelude::*;
use yew_router::prelude::*;
use yew::{AttrValue, function_component, Html, Properties, use_state};
use web_sys::InputEvent;
use bson::oid::ObjectId;
use yew::virtual_dom::VNode;
use crate::app::common::{AppConfig, FetchRes};
use crate::app::profiles::{PublicProfile, TinyProfile};

#[derive(PartialEq, Properties)]
pub(super) struct PostProps {
    pub(super) cfg: AppConfig,
}

#[function_component]
pub(super) fn Post(props: &PostProps) -> Html {
    let cfg = use_state(|| props.cfg.clone());
    let title = use_state(AttrValue::default);
    let authors = use_state(<Vec<(PublicProfile, TinyProfile)>>::new);
    let abstraction = use_state(AttrValue::default);
    let keywords = use_state(<Vec<AttrValue>>::new);
    let msg_box = use_state(Html::default);
    let is_good_title = use_state(bool::default);
    let check_title = get_check_title(title.clone(), is_good_title.clone());

    let is_good_abstraction = use_state(bool::default);
    let check_abstraction = get_check_abstraction(abstraction.clone(), is_good_abstraction.clone());

    let authors_area = get_authors_area(cfg.clone(), authors.clone(), msg_box.clone());

    let keywords_area = {
        let keyword = use_state(AttrValue::default);

        let is_good_keyword = use_state(bool::default);
        let oninput = {
            let keywords = keywords.clone();
            let keyword = keyword.clone();
            let is_good_keyword = is_good_keyword.clone();
            move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let val = input.value().into();
                    is_good_keyword.set(!(input.value().is_empty() || keywords.contains(&val)));
                    keyword.set(val);
                }
            }
        };
        let onclick = {
            let keywords = keywords.clone();
            let keyword = keyword.clone();
            move |_| {
                let mut vals = (*keywords).clone();
                vals.push((*keyword).clone());
                keywords.set(vals);
                keyword.set(AttrValue::default());
            }
        };

        html! {
            <div>
                <p>
                    <label>
                        { "Key Words: " }
                        <ol>
                            { for keywords.iter().map(|keyword| html!(<li>{ keyword }</li>)) }
                        </ol>
                    </label>
                    <input type="text" {oninput} value={(*keyword).clone()} />
                    <button {onclick} disabled={!(*is_good_keyword)}>{"Add keyword"}</button>
                </p>
            </div>
        }
    };

    html! {
        <div>
            <p>
                <label>
                    { "Title: " }
                    <input type="text" oninput={check_title} value={(*title).clone()} />
                    { if *is_good_title { " ✅" } else { " ❎" }}
                </label>
            </p>

            <p>
                <label>
                    { "Abstraction: " }<br />
                    <textarea type="text" oninput={check_abstraction} value={(*abstraction).clone()} />
                    { if *is_good_abstraction { " ✅" } else { " ❎" }}
                </label>
            </p>

            <p>
                { authors_area }
            </p>

            <p>
                { keywords_area }
            </p>

            { (*msg_box).clone() }
        </div>
    }
}

fn get_authors_area(cfg: UseStateHandle<AppConfig>, authors: UseStateHandle<Vec<(PublicProfile, TinyProfile)>>, msg_box: UseStateHandle<VNode>) -> VNode {
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

fn get_check_abstraction(abstraction: UseStateHandle<IString>, is_good_abstraction: UseStateHandle<bool>) -> fn(_) {
    move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
            is_good_abstraction.set(input.value().len() > 140);
            abstraction.set(input.value().into());
        }
    }
}

fn get_check_title(title: UseStateHandle<IString>, is_good_title: UseStateHandle<bool>) -> fn(_) {
    move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
            is_good_title.set(!input.value().is_empty());
            title.set(input.value().into());
        }
    }
}
