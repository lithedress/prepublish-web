use yew::{AttrValue, Component, Context, Html};
use web_sys::{HtmlInputElement, InputEvent};
use crate::views::theses::post::keywords::{InputMsg, InputProps};

#[derive(Default)]
struct Input {
    val: AttrValue,
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
