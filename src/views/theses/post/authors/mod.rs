use std::{rc::Rc, str::FromStr};

use bson::oid::ObjectId;
use yew::{AttrValue, Callback, Component, Context, Html, html, Properties, TargetCast, html_nested};
use web_sys::{HtmlInputElement, InputEvent};

use crate::models::{profile::PublicProfile, common::{AppConfig, FetchError, FetchRes, FetchOther}};  

#[derive(Default)]
pub struct Authors {
    vals: Rc<Vec<PublicProfile>>
}

pub enum AuthorsMsg {
    Err(FetchError),
    Alert(FetchOther),
    Push(PublicProfile),
    Remove(usize),
    New,
}

#[derive(PartialEq, Properties)]
pub struct AuthorsProps {
    cfg: AppConfig,
    err: Callback<FetchError>,
    alert: Callback<FetchOther>,
    vals: Callback<Rc<Vec<PublicProfile>>>,
}

impl Component for Authors {
    type Message = AuthorsMsg;
    type Properties = AuthorsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AuthorsMsg::Alert(h) => todo!(),
            _ => todo!(),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let delete = ctx.link().callback(|index| AuthorsMsg::Remove(index));
        let is_valid = Callback::from({
            let vals = self.vals.clone();
            move |id| {
                !(vals.iter().any(|v| v._id == id))
            }
        });
        let submit = ctx.link().callback_future({
            let cfg = ctx.props().cfg.clone();
            move |id| {
                let cfg = cfg.clone();
                async move {
                    let profile = PublicProfile::try_get(&cfg, id).await;
                    match profile {
                        Ok(res) => match res {
                            FetchRes::Body(profile) => AuthorsMsg::Push(profile),
                            FetchRes::Other(_) => AuthorsMsg::New,
                        },
                        Err(e) => AuthorsMsg::Err(e),
                    }
                }
            }
        });
        let reset = ctx.link().callback(|_| AuthorsMsg::New);
        html! {
            <div>
                <p>
                    <label>
                        { "Key Words: " }
                        <ol>
                            { for self.vals.clone().iter().enumerate().map(|(index, val)| view_entity(&val, index, &delete, &ctx.props().cfg)) }
                        </ol>
                    </label>
                    <Input {is_valid}{submit} />
                    <button onclick={reset}>{ "Reset Key Words "}</button>
                </p>
            </div>
        }
    }
}

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

fn view_entity(val: &PublicProfile, index: usize, delete: &Callback<usize>, cfg: &AppConfig) -> Html {
    let onclick = {
        let delete = delete.clone();
        move |_| delete.emit(index)
    };
    html! {
        <div>
            { val.clone().view_tiny(cfg) }
            <button {onclick} >{ " ❎" }</button>
        </div>
    }
}