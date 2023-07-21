use std::rc::Rc;

use bson::oid::ObjectId;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, InputEvent, RequestCredentials};
use yew::{html, AttrValue, Callback, Component, Context, Html, Properties, TargetCast};

use crate::models::common::{AppConfig, AppError};

#[derive(Default)]
pub(super) struct Review {
    judgement: bool,
    criticism: AttrValue,
}

pub(super) enum ReviewMsg {
    Err(AppError),
    Judge(bool),
    Criticise(String),
    Submit(ObjectId),
}

#[derive(PartialEq, Properties)]
pub(super) struct ReviewProps {
    pub(super) cfg: Rc<AppConfig>,
    pub(super) err: Callback<AppError>,
    pub(super) id: ObjectId,
    pub(super) refresh: Callback<ObjectId>,
}

impl Component for Review {
    type Message = ReviewMsg;
    type Properties = ReviewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ReviewMsg::Err(e) => {
                ctx.props().err.emit(e);
                false
            }
            ReviewMsg::Judge(j) => {
                self.judgement = j;
                true
            }
            ReviewMsg::Criticise(c) => {
                self.criticism = c.into();
                true
            }
            ReviewMsg::Submit(id) => {
                ctx.props().refresh.emit(id);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let judge = ctx.link().batch_callback(|e: InputEvent| {
            e.target_dyn_into::<HtmlInputElement>()
                .map(|i| ReviewMsg::Judge(i.checked()))
        });
        let criticise = ctx.link().batch_callback(|e: InputEvent| {
            e.target_dyn_into::<HtmlTextAreaElement>()
                .map(|a| ReviewMsg::Criticise(a.value()))
        });
        let onclick = ctx.link().callback_future({
            let cfg = ctx.props().cfg.to_owned();
            let version_id = ctx.props().id;
            let judgement = self.judgement;
            let criticism = self.criticism.to_owned();
            move |_| {
                let judgement = judgement;
                let version_id = version_id;
                let criticism = criticism.clone();
                let cfg = cfg.clone();
                async move {
                    let url = match cfg.api.join(&format!("versions/{}/review", version_id.to_hex())) {
                        Ok(url) => url,
                        Err(e) => {
                            return ReviewMsg::Err(e.into());
                        }
                    };
                    let body = serde_json::json!({
                        "judgement": judgement,
                        "criticism": *criticism
                    });
                    match async move {
                        Ok(
                            gloo::net::http::Request::post(url.as_str())
                            .credentials(RequestCredentials::Include)
                                .json(&body)?
                                .send()
                                .await?
                                .json::<ObjectId>()
                                .await?
                        )
                    }
                    .await
                    {
                        Ok(id) => ReviewMsg::Submit(id),
                        Err(e) => ReviewMsg::Err(e),
                    }
                }
            }
        });
        html! {
            <fieldset>
                <legend>{ "Review" }</legend>
                <label>
                    { "Judgement: " }
                    <input type="checkbox" oninput={ judge } checked={ self.judgement } />
                </label>
                <label>
                    { "Criticism: " }
                    <textarea oninput={ criticise } value={ self.criticism.to_owned() } />
                </label>
                <button disabled={ self.criticism.len() > 0 } {onclick} >
                    { "Submit" }
                </button>
            </fieldset>
        }
    }
}
