use std::rc::Rc;

use web_sys::{FormData, HtmlFormElement, RequestCredentials, SubmitEvent};
use yew::{html, html_nested, Component, Context, Html, Properties, TargetCast};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    models::{
        common::{AppConfig, AppError, FetchOther, FetchRes},
        profile::PublicProfile,
        thesis::Thesis,
    },
    views::{alerts::AlertBox, Route},
};

#[derive(Default)]
pub struct Page {
    err: Option<crate::models::common::AppError>,
    alert: Option<Rc<FetchOther>>,
}

pub enum PageMsg {
    Err(AppError),
    Alert(FetchOther),
    File(FormData),
    Version(bson::oid::ObjectId),
}

#[derive(PartialEq, Properties)]
pub struct PageProps {
    pub val: Rc<Thesis>,
    pub cfg: Rc<AppConfig>,
}

impl Component for Page {
    type Message = PageMsg;
    type Properties = PageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PageMsg::Err(e) => {
                self.err = Some(e);
                true
            }
            PageMsg::Alert(other) => {
                self.alert = Some(Rc::new(other));
                true
            }
            PageMsg::File(data) => {
                ctx.link().send_future({
                    let cfg = ctx.props().cfg.clone();
                    let id = ctx.props().val.id._id;
                    async move {
                        let url = match cfg.api.join(&format!("theses/{}/commit", id.to_hex())) {
                            Ok(url) => url,
                            Err(e) => {
                                return PageMsg::Err(e.into());
                            }
                        };
                        let req = match gloo::net::http::Request::post(url.as_str())
                            .credentials(RequestCredentials::Include)
                            .body(&data)
                        {
                            Ok(req) => req,
                            Err(e) => {
                                return PageMsg::Err(e.into());
                            }
                        };
                        let id = match FetchRes::try_from_gloo_res(req.send().await).await {
                            Ok(res) => match res {
                                FetchRes::Body(body) => body,
                                FetchRes::Other(other) => return PageMsg::Alert(other),
                            },
                            Err(e) => return PageMsg::Err(e.into()),
                        };
                        PageMsg::Version(id)
                    }
                });
                true
            }
            PageMsg::Version(id) => {
                if let Some(navigator) = ctx.link().navigator() {
                    navigator.push(&Route::Versions { id });
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref err) = self.err {
            return err.view();
        }
        let cfg = ctx.props().cfg.to_owned();
        let val = ctx.props().val.clone();
        let file_upload = ctx.link().batch_callback(move |e: SubmitEvent| {
            e.prevent_default();
            e.target_dyn_into::<HtmlFormElement>().map(|form| {
                match FormData::new_with_form(&form) {
                    Ok(data) => PageMsg::File(data),
                    Err(e) => PageMsg::Err(e.into()),
                }
            })
        });

        let authors = val.author_ids.iter().map(|id| {
            html_nested! {
                <li>
                    { PublicProfile::tiny_from_id(id.to_owned(), cfg.clone()) }
                </li>
            }
        });
        let keywords = val
            .keywords
            .iter()
            .map(|k| html_nested!(<b>{ k }{ "&nbsp;" }</b>));
        let alert_box = html_nested!(<AlertBox refresh={ self.alert.clone() } />);
        html! {
            <div>
                <h1>
                    { val.title.clone() }
                </h1>

                <p><ul>
                    { for authors }
                </ul></p>

                <p>
                    { val.abstraction.clone() }
                </p>

                <p>
                    { for keywords }
                </p>

                <form enctype="multipart/form-data" onsubmit={ file_upload }><fieldset>
                    <p><label>
                        { "Commit Message: "}
                        <textarea type="text" name="message" required={ true } />
                    </label></p>

                    <p><label>
                        { "Release file: " }
                        <input type="file" name="release" accept={ mime::APPLICATION_PDF.to_string() } required={ true } />
                    </label></p>

                    <p><label>
                        { "Source file: " }
                        <input type="file" name="source"
                        accept=".tar.gz,.tgz,.tar.Z,.tar.bz2,.tbz2,.tar.lz,.tlz,.tar.xz,.txz,.tar.zst"
                        required={ false } />
                    </label></p>

                    <p>
                        <input type="submit" value="Commit" />
                    </p>
                </fieldset></form>

                { alert_box }
            </div>
        }
    }
}
