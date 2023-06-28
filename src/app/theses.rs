use bson::oid::ObjectId;
use yew::prelude::*;
use yew_router::prelude::*;


use super::{common::AppConfig, profiles::PublicProfile};

#[derive(Clone, Routable, PartialEq)]
pub(super) enum Route {
    #[at("/theses")]
    List,
    #[at("/theses/new")]
    New,
    #[at("/theses/:id")]
    View { id: ObjectId },
}

#[derive(PartialEq, Properties)]
struct PostProps {
    cfg: AppConfig,
}

#[function_component]
fn Post(props: &PostProps) -> Html {
    let cfg = use_state(|| props.cfg.clone());
    let title = use_state(AttrValue::default);
    let authors = use_state(<Vec<PublicProfile>>::new);
    let abstraction = use_state(AttrValue::default);
    let keywords = use_state(<Vec<AttrValue>>::new);
    let msg_box = use_state(Html::default);

    let is_good_title = use_state(bool::default);
    let check_title = {
        let title = title.clone();
        let is_good_title = is_good_title.clone();
        move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                is_good_title.set(!input.value().is_empty());
                title.set(input.value().into());
            }
        }
    };
    
    let is_good_abstraction = use_state(bool::default);
    let check_abstraction = {
        let abstraction = abstraction.clone();
        let is_good_abstraction = is_good_abstraction.clone();
        move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                is_good_abstraction.set(input.value().len() > 140);
                abstraction.set(input.value().into());
            }
        }
    };

    let authors_ares = {
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
                            is_good_author.set(authors.iter().find(|p| p._id == id).is_none());
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
                let mut vals = (*authors).clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match PublicProfile::get(*cfg, *author).await {
                        Ok(res) => match res {
                            Res::Other(o) => {
                                msg_box.set(o)
                            }
                        }
                        Err(e) => { msg_box.set(e.view()); }
                    }
                    
                });
                authors.set(vals);
            }
        };
        html! {
            <div>
                <p>
                    <label>
                        { "Authors: " }
                        <ol>
                            { for authors.iter().map(|author| html!(<li>{ author.view() }</li>)) }
                        </ol>
                    </label>
                    <input type="text" {oninput} value={(*author).clone().to_hex()} />
                    <button {onclick} disabled={!(*is_good_author)}>{"Add keyword"}</button>
                </p>
            </div>
        }
    };

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
                { keywords_area }
            </p>

            { (*msg_box).clone() }
        </div>
    }
}

impl Route {
    pub(super) fn switch(cfg: AppConfig) -> impl Fn(Self) -> Html {
        move |routes| match routes {
            Self::New => html! {
                //<Post cfg={cfg.clone()} />
            },
            _ => html!()
        }
    }
}