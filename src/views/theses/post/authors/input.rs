use yew::{AttrValue, Callback, Component, Context, Html, Properties};
use bson::oid::ObjectId;
use web_sys::{HtmlInputElement, InputEvent};
use std::str::FromStr;

#[derive(Default)]
struct Input {
    id: Option<ObjectId>,
    hex: AttrValue,
}

enum InputMsg {
    Input(String),
    Submit,
}

#[derive(PartialEq, Properties)]
struct InputProps {
    is_valid: Callback<ObjectId, bool>,
    submit: Callback<ObjectId>,
}

impl Component for Input {
    type Message = InputMsg;
    type Properties = InputProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputMsg::Input(val) => {
                self.id = match ObjectId::from_str(&val) {
                    Ok(id) => Some(id),
                    Err(_) => None,
                };
                self.hex = val.into();
                true
            }
            InputMsg::Submit => {
                self.hex = AttrValue::default();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().batch_callback(|e: InputEvent| e.target_dyn_into::<HtmlInputElement>().map(|i| InputMsg::Input(i.value())));

        let button = match self.id {
            Some(id) => {
                let onclick = ctx.link().callback({
                    let submit = ctx.props().submit.clone();
                    move |_| {
                        submit.emit(id);
                        InputMsg::Submit
                    }
                });
                html_nested!(<button {onclick} disabled={!ctx.props().is_valid.emit(id)}>{"Add keyword"}</button>)
            }
            None => html_nested!(<button disabled={ true }>{"Add keyword"}</button>)
        };
        html! {
            <div>
                <input type="text" {oninput} value={ self.hex.clone() } />
                { button }
            </div>
        }
    }
}
