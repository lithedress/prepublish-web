use std::rc::Rc;

use web_sys::{FormData, HtmlFormElement, SubmitEvent};
use yew::{html, html_nested, Component, Context, Html, Properties, TargetCast};

use crate::models::{
    common::{AppConfig, AppError},
    profile::PublicProfile,
    thesis::Thesis,
};

#[derive(Default)]
pub struct Page {
    err: Option<crate::models::common::AppError>,
}

pub enum PageMsg {
    Err(AppError),
    FileSubmit(FormData),
    FileUploaded,
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
            PageMsg::FileSubmit(data) => {
                ctx.link().send_future_batch({
                    let cfg = ctx.props().cfg.clone();
                    let id = ctx.props().val.id._id;
                    async move {
                        let url = match cfg.api.join(&format!("theses/{}/commit", id.to_hex())) {
                            Ok(url) => url,
                            Err(_) => {
                                return None;
                            }
                        };
                        let req = match gloo::net::http::Request::post(url.as_str()).body(&data) {
                            Ok(req) => req,
                            Err(_) => {
                                return None;
                            }
                        };
                        req.send().await.unwrap();
                        Some(PageMsg::FileUploaded)
                    }
                });
                false
            }
            _ => true,
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
                    Ok(data) => PageMsg::FileSubmit(data),
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
                        <textarea type="text" name="message" required={true} />
                    </label></p>

                    <p><label>
                        { "Release file: " }
                        <input type="file" name="release" accept={ mime::APPLICATION_PDF.to_string() } required={true} />
                    </label></p>

                    <p><label>
                        { "Source file: " }
                        <input type="file" name="source" accept="application/x-gtar" required={false} />
                    </label></p>

                    <p>
                        <input type="submit" value="Commit" />
                    </p>
                </fieldset></form>
            </div>
        }
    }
}
