use std::rc::Rc;

use yew::{
    html, Callback, Component, Context, Html, Properties,
};

use crate::models::{
    common::{AppConfig, AppError, FetchOther, FetchRes},
    profile::PublicProfile,
};

mod entity;
mod input;

#[derive(Default)]
pub(super) struct Authors {
    vals: Rc<Vec<Rc<PublicProfile>>>,
}

pub(super) enum AuthorsMsg {
    Err(AppError),
    Alert(FetchOther),
    Push(Rc<PublicProfile>),
    Remove(usize),
    New,
}

#[derive(PartialEq, Properties)]
pub(super) struct AuthorsProps {
    pub(super) cfg: Rc<AppConfig>,
    pub(super) err: Callback<AppError>,
    pub(super) alert: Callback<FetchOther>,
    pub(super) vals: Callback<Rc<Vec<Rc<PublicProfile>>>>,
}

impl Component for Authors {
    type Message = AuthorsMsg;
    type Properties = AuthorsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AuthorsMsg::Err(e) => {
                ctx.props().err.emit(e);
                false
            }
            AuthorsMsg::Alert(res) => {
                ctx.props().alert.emit(res);
                false
            }
            AuthorsMsg::Push(profile) => {
                Rc::make_mut(&mut self.vals).push(profile);
                ctx.props().vals.emit(self.vals.clone());
                true
            }
            AuthorsMsg::Remove(index) => {
                Rc::make_mut(&mut self.vals).remove(index);
                ctx.props().vals.emit(self.vals.clone());
                true
            }
            AuthorsMsg::New => {
                self.vals = Default::default();
                ctx.props().vals.emit(self.vals.clone());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let delete = ctx.link().callback(AuthorsMsg::Remove);
        let validate = Callback::from({
            let vals = self.vals.clone();
            move |id| !(vals.iter().any(|v| v._id == id))
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
                            FetchRes::Other(other) => AuthorsMsg::Alert(other),
                        },
                        Err(e) => AuthorsMsg::Err(e),
                    }
                }
            }
        });
        let reset = ctx.link().callback(|_| AuthorsMsg::New);
        html! {
            <div>
                <fieldset>
                    <legend>{ "Authors" }</legend>
                    <ol>
                        { for self.vals.clone().iter().enumerate().map(|(index, val)| entity::view_entity(&val, index, &delete, &ctx.props().cfg)) }
                    </ol>
                    <input::Input {validate}{submit} />
                    <button onclick={reset}>{ "Reset" }</button>
                </fieldset>
            </div>
        }
    }
}