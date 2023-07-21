use std::rc::Rc;

use bson::oid::ObjectId;
use web_sys::{InputEvent, HtmlInputElement, RequestCredentials};
use yew::{Properties, Component, Context, html, Html, TargetCast, Callback};

use crate::models::{common::{AppError, AppConfig}, version::Version};

#[derive(Default)]
pub struct Adjudge {
    val: bool,
}

pub enum AdjudgeMsg {
    Err(AppError),
    Adjudge(bool),
    Submit(Version),
}

#[derive(PartialEq, Properties)]
pub struct AdjudgeProps {
    pub(super) cfg: Rc<AppConfig>,
    pub(super) err: Callback<AppError>,
    pub(super) id: ObjectId,
    pub(super) refresh: Callback<Version>,
}

impl Component for Adjudge {
    type Message = AdjudgeMsg;
    type Properties = AdjudgeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AdjudgeMsg::Err(e) => {
                ctx.props().err.emit(e);
                false
            }
            AdjudgeMsg::Adjudge(j) => {
                self.val = j;
                true
            }
            AdjudgeMsg::Submit(version) => {
                ctx.props().refresh.emit(version);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let checked = self.val;
        let oninput = ctx.link().batch_callback(|e: InputEvent| {
            e.target_dyn_into::<HtmlInputElement>()
                .map(|i| AdjudgeMsg::Adjudge(i.checked()))
        });
        let onclick = ctx.link().callback_future({
            let val = self.val;
            let cfg = ctx.props().cfg.to_owned();
            let version_id = ctx.props().id;
            move |_| {
                let val = val;
                let cfg = cfg.clone();
                let version_id = version_id;
                async move {
                    let url = match cfg.api.join(&format!("versions/{}/adjudge/{}", version_id.to_hex(), val)) {
                        Ok(url) => url,
                        Err(e) => return AdjudgeMsg::Err(e.into()),
                    };
                    match async move {
                        Ok(
                            gloo::net::http::Request::patch(url.as_str())
                            .credentials(RequestCredentials::Include)
                                .send()
                                .await?
                                .json::<Version>()
                                .await?
                        ) 
                    }
                    .await {
                        Ok(version) => AdjudgeMsg::Submit(version),
                        Err(e) => return AdjudgeMsg::Err(e),
                    }
                }
            }
        });
        html! {
            <fieldset>
                <legend>{ "Adjudge" }</legend>
                <label>
                    { "Pass: " }
                    <input type="checkbox" {oninput} {checked} />
                    <button {onclick}>{ "Submit" }</button>
                </label>
            </fieldset>
        }
    }
}