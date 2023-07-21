use std::rc::Rc;

use web_sys::RequestCredentials;
use yew::{Properties, Component, Context, Html, html, Callback};

use crate::{
    models::{common::{AppError, AppConfig, FetchOther}, version::Version, profile::PublicProfile},
    views::{profile_list::ProfileList, versions::page::adjudge}
};

#[derive(Default)]
pub(super) struct Edit {
    to_adjudge: bool,
    reviewers: Rc<Vec<Rc<PublicProfile>>>,
}

pub(super) enum EditMsg {
    Err(AppError),
    Alert(FetchOther),
    ToAdjudge,
    UpdateReviewers(Rc<Vec<Rc<PublicProfile>>>),
    Submit(Version),
}

#[derive(PartialEq, Properties)]
pub(super) struct EditProps {
    pub(super) cfg: Rc<AppConfig>,
    pub(super) err: Callback<AppError>,
    pub(super) alert: Callback<FetchOther>,
    pub(super) id: bson::oid::ObjectId,
    pub(super) refresh: Callback<Version>,
}

impl Component for Edit {
    type Message = EditMsg;
    type Properties = EditProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditMsg::Err(e) => {
                ctx.props().err.emit(e);
                false
            }
            EditMsg::Alert(other) => {
                ctx.props().alert.emit(other);
                false
            }
            EditMsg::ToAdjudge => {
                self.to_adjudge = true;
                true
            }
            EditMsg::UpdateReviewers(rs) => {
                self.reviewers = rs;
                false
            }
            EditMsg::Submit(version) => {
                ctx.props().refresh.emit(version);
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.to_adjudge {
            let cfg = ctx.props().cfg.to_owned();
            let err = ctx.props().err.to_owned();
            let id = ctx.props().id;
            let refresh = ctx.props().refresh.to_owned();
            return html!(<adjudge::Adjudge {cfg} {err} {id} {refresh} />);
        }
        let onclick = ctx.link().callback(|_| EditMsg::ToAdjudge);
        let err = ctx.link().callback(EditMsg::Err);
        let alert = ctx.link().callback(EditMsg::Alert);
        let reviewers = ctx.link().callback(EditMsg::UpdateReviewers);
        let submit = ctx.link().callback_future({
            let cfg = ctx.props().cfg.to_owned();
            let version_id = ctx.props().id;
            let reviewers = self.reviewers.to_owned();
            move |_| {
                let cfg = cfg.clone();
                let version_id = version_id;
                let reviewers = reviewers.clone();
                async move {
                    let url = match cfg.api.join(&format!("versions/{}/edit", version_id.to_hex())) {
                        Ok(url) => url,
                        Err(e) => {
                            return EditMsg::Err(e.into());
                        }
                    };
                    let body = serde_json::json!({
                        "remainder_reviewer_ids": reviewers,
                        "pattern": "Reviewer"
                    });
                    match async move {
                        Ok(
                            gloo::net::http::Request::patch(url.as_str())
                            .credentials(RequestCredentials::Include)
                                .json(&body)?
                                .send()
                                .await?
                                .json::<Version>()
                                .await?
                        )
                    }
                    .await
                    {
                        Ok(version) => EditMsg::Submit(version),
                        Err(e) => EditMsg::Err(e),
                    }
                }
            }
        });
        html! {
            <fieldset>
                <p>
                    <button {onclick}>
                        { "Pass directly" }
                    </button>
                </p>
                <p>
                    <ProfileList cfg={ctx.props().cfg.clone()} {err} {alert} vals={reviewers} />
                </p>
                <p>
                    <button onclick={ submit }>
                        { "Okay" }
                    </button>
                </p>
            </fieldset>
        }
    }
}