use std::rc::Rc;

use yew::{AttrValue, Callback, Component, Context, Html, html, Properties};

mod input;
mod entity;

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
        let delete = ctx.link().callback(KeywordsMsg::Remove);
        let validate = Callback::from({
            let vals = self.vals.clone();
            move |val: AttrValue| {
                !(val.is_empty() || vals.contains(&val))
            }
        });
        let submit = ctx.link().callback(KeywordsMsg::Push);
        let reset = ctx.link().callback(|_| KeywordsMsg::New);
        html! {
            <div>
                <fieldset>
                    <legend>{ "Key Words" }</legend>
                    <menu>
                        { for self.vals.clone().iter().enumerate().map(|(index, val)| entity::view_entity(val, index, &delete)) }
                    </menu>
                    <input::Input {validate}{submit} />
                    <button onclick={ reset }>{ "Reset"}</button>
                </fieldset>
            </div>
        }
    }
}
