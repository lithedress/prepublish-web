use yew::prelude::*;
use yew_router::prelude::*;
use yew::{AttrValue, use_state, UseStateHandle};
use yew::virtual_dom::VNode;
use web_sys::InputEvent;

#[derive(PartialEq, Properties)]
pub struct KeywordsProps {
    keywords: Vec<AttrValue>,
}

#[function_component]
pub fn Keywords(props: &KeywordsProps) -> Html {
    let keyword = use_state(AttrValue::default);

    let is_good_keyword = use_state(bool::default);
    let oninput = {
        let keywords = props.keywords.clone();
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
        let keywords = props.keywords.clone();
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
}
