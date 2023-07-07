use std::ops::Not;

use web_sys::{InputEvent, HtmlInputElement};
use yew::{TargetCast, Context, Html, Component, Properties, html, Callback, AttrValue, html_nested};

use crate::models::common::AppConfig;

#[derive(Default)]
pub struct Post {
    title: String,
    title_ref: yew::NodeRef,

    abstraction: String,
}

pub enum PostMsg {
    InputTitle(String),
    InputAbstraction(String),
}

#[derive(PartialEq, Properties)]
pub struct PostProps {
    pub cfg: AppConfig,
}

impl Component for Post {
    type Message = PostMsg;
    type Properties = PostProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let check_title = ctx.link().batch_callback(|e: InputEvent| e.target_dyn_into::<HtmlInputElement>().map(|i| PostMsg::InputTitle(i.value())));
        let check_abstraction = ctx.link().batch_callback(|e: InputEvent| e.target_dyn_into::<HtmlInputElement>().map(|i| PostMsg::InputAbstraction(i.value())));
        html! {
            <div>
                <p>
                    <label>
                        { "Title: " }
                        <input type="text" ref={ self.title_ref } oninput={ check_title } value={ self.title } />
                        { if self.title.is_empty().not() { " ✅" } else { " ❎" }}
                    </label>
                </p>

                <p>
                    <label>
                        { "Abstraction: " }<br />
                        <textarea type="text" oninput={ check_abstraction } value={ self.abstraction} />
                        { if self.abstraction.len() >= 140 { " ✅" } else { " ❎" }}
                    </label>
                </p>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(input) = self.title_ref.cast::<HtmlInputElement>() {
                input.focus();
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PostMsg::InputTitle(title) => {
                self.title = title;
                true
            }
            PostMsg::InputAbstraction(abstraction) => {
                self.abstraction = abstraction;
                true
            },
        }
    }
}

#[derive(Default)]
pub struct Keywords {
    vals: std::rc::Rc<Vec<AttrValue>>,
}

pub enum KeywordsMsg {
    Push(AttrValue),
    Remove(usize),
    New,
}

#[derive(PartialEq, Properties)]
pub struct KeywordsProps {}

impl Component for Keywords {
    type Message = KeywordsMsg;
    type Properties = KeywordsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let delete = ctx.link().callback(|index| KeywordsMsg::Remove(index));
        let is_valid_keyword = Callback::from(|val: AttrValue| {
            !(val.is_empty() || self.vals.contains(&val))
        });
        let submit = ctx.link().callback(|val| KeywordsMsg::Push(val));
        let reset = ctx.link().callback(|_| KeywordsMsg::New);
        html! {
            <div>
                <p>
                    <label>
                        { "Key Words: " }
                        <ol>
                            { for self.vals.iter().enumerate().map(|(index, val)| html_nested!(<KeywordEntity {index} {delete} />)) }
                        </ol>
                    </label>
                    <KeywordInput {is_valid_keyword}{submit} />
                    <button onclick={reset}>{ "Reset Key Words "}</button>
                </p>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            KeywordsMsg::Push(val) => {
                self.vals.push(val);
            }
            KeywordsMsg::Remove(index) => {
                self.vals.remove(index);
            },
            KeywordsMsg::New => {
                self.vals = Default::default();
            }
        }
        true
    }
}

#[derive(Default)]
pub struct KeywordInput {
    val: AttrValue,
}

pub enum KeywordInputMsg {
    Input(String),
    Submit,
}

#[derive(PartialEq, Properties)]
pub struct KeywordInputProps {
    is_valid_keyword: Callback<AttrValue, bool>,
    submit: Callback<AttrValue>,
}

impl Component for KeywordInput {
    type Message = KeywordInputMsg;
    type Properties = KeywordInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().batch_callback(|e: InputEvent| e.target_dyn_into::<HtmlInputElement>().map(|i| KeywordInputMsg::Input(i.value())));
        let onclick = ctx.link().callback(|_| {
            KeywordInputMsg::Submit
        });
        html! {
            <div>
                <input type="text" {oninput} value={ self.val } />
                <button {onclick} disabled={!ctx.props().is_valid_keyword.emit(self.val)}>{"Add keyword"}</button>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            KeywordInputMsg::Input(val) => {
                self.val = val.into();
                true
            }
            KeywordInputMsg::Submit => {
                ctx.props().submit.emit(self.val);
                self.val = AttrValue::default();
                true
            }
        }
    }
}

#[derive(Default)]
pub struct KeywordEntity {
    val: AttrValue
}

#[derive(PartialEq, Properties)]
pub struct KeywordEntityProps {
    index: usize,
    delete: Callback<usize>,
}

impl Component for KeywordEntity {
    type Message = ();
    type Properties = KeywordEntityProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| ctx.props().delete.emit(ctx.props().index));
        html! {
            <div>
                { self.val }
                <button {onclick} >{ " ❎" }</button>
            </div>
        }
    }
}