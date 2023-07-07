use std::rc::Rc;

use yew::{AttrValue, Callback, Component, Context, Html, html, Properties, TargetCast};
use web_sys::{HtmlInputElement, InputEvent};

#[derive(Default)]
pub(super) struct Keywords {
    vals: Rc<Vec<AttrValue>>,
}

pub(super) enum KeywordsMsg {
    Push(AttrValue),
    Remove(usize),
    New,
}

#[derive(PartialEq, Properties)]
pub(super) struct KeywordsProps {
    pub(super) vals: Callback<Rc<Vec<AttrValue>>>
}

impl Component for Keywords {
    type Message = KeywordsMsg;
    type Properties = KeywordsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            KeywordsMsg::Push(val) => {
                Rc::make_mut(&mut self.vals).push(val);
            }
            KeywordsMsg::Remove(index) => {
                Rc::make_mut(&mut self.vals).remove(index);
            },
            KeywordsMsg::New => {
                self.vals = Default::default();
            }
        }
        ctx.props().vals.emit(self.vals.clone());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let delete = ctx.link().callback(|index| KeywordsMsg::Remove(index));
        let is_valid = Callback::from({
            let vals = self.vals.clone();
            move |val: AttrValue| {
                !(val.is_empty() || vals.contains(&val))
            }
        });
        let submit = ctx.link().callback(|val| KeywordsMsg::Push(val));
        let reset = ctx.link().callback(|_| KeywordsMsg::New);
        html! {
            <div>
                <p>
                    <label>
                        { "Key Words: " }
                        <ol>
                            { for self.vals.clone().iter().enumerate().map(|(index, val)| view_entity(&val, index, &delete)) }
                        </ol>
                    </label>
                    <Input {is_valid}{submit} />
                    <button onclick={ reset }>{ "Reset Key Words "}</button>
                </p>
            </div>
        }
    }
}

#[derive(Default)]
struct Input {
    val: AttrValue,
}

enum InputMsg {
    Input(String),
    Submit,
}

#[derive(PartialEq, Properties)]
struct InputProps {
    is_valid: Callback<AttrValue, bool>,
    submit: Callback<AttrValue>,
}

impl Component for Input {
    type Message = InputMsg;
    type Properties = InputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputMsg::Input(val) => {
                self.val = val.into();
                true
            }
            InputMsg::Submit => {
                ctx.props().submit.emit(self.val.clone());
                self.val = AttrValue::default();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().batch_callback(|e: InputEvent| e.target_dyn_into::<HtmlInputElement>().map(|i| InputMsg::Input(i.value())));
        let onclick = ctx.link().callback(|_| {
            InputMsg::Submit
        });
        html! {
            <div>
                <input type="text" {oninput} value={ self.val.clone() } />
                <button {onclick} disabled={!ctx.props().is_valid.emit(self.val.clone())}>{"Add keyword"}</button>
            </div>
        }
    }
}

fn view_entity(val: &AttrValue, index: usize, delete: &Callback<usize>) -> Html {
    let onclick = {
        let delete = delete.clone();
        move |_| delete.emit(index)
    };
    html! {
        <div>
            { val.clone() }
            <button {onclick} >{ " ❎" }</button>
        </div>
    }
}
