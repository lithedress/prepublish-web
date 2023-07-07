use std::{ops::Not, rc::Rc};

use web_sys::{HtmlInputElement, InputEvent};
use yew::{html, html_nested, AttrValue, Component, Context, Html, Properties, TargetCast};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    models::{
        common::{AppConfig, FetchError, FetchOther},
        profile::PublicProfile,
    },
    views::alerts::AlertBox,
};

mod authors;
mod keywords;

#[derive(Default)]
pub(super) struct Post {
    err: Option<FetchError>,
    alert: Option<Rc<FetchOther>>,
    title: AttrValue,
    title_ref: yew::NodeRef,
    abstraction: AttrValue,
    keywords: Rc<Vec<AttrValue>>,
    authors: Rc<Vec<Rc<PublicProfile>>>,
}

pub(super) enum PostMsg {
    Err(FetchError),
    Alert(FetchOther),
    InputTitle(String),
    InputAbstraction(String),
    UpdateKeywords(Rc<Vec<AttrValue>>),
    UpdateAuthors(Rc<Vec<Rc<PublicProfile>>>),
    Post(bson::oid::ObjectId),
}

#[derive(PartialEq, Properties)]
pub(super) struct PostProps {
    pub(super) cfg: Rc<AppConfig>,
}

impl Component for Post {
    type Message = PostMsg;
    type Properties = PostProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PostMsg::Err(err) => {
                self.err = Some(err);
                true
            }
            PostMsg::Alert(other) => {
                self.alert = Some(Rc::new(other));
                true
            }
            PostMsg::InputTitle(title) => {
                self.title = title.into();
                true
            }
            PostMsg::InputAbstraction(abstraction) => {
                self.abstraction = abstraction.into();
                true
            }
            PostMsg::UpdateKeywords(keywords) => {
                self.keywords = keywords;
                false
            }
            PostMsg::UpdateAuthors(authors) => {
                self.authors = authors;
                false
            }
            PostMsg::Post(id) => {
                if let Some(navigator) = ctx.link().navigator() {
                    navigator.push(&super::route::Route::View { id })
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref err) = self.err {
            return err.view();
        }

        let err = ctx.link().callback(PostMsg::Err);
        let alert = ctx.link().callback(PostMsg::Alert);
        let check_title = ctx.link().batch_callback(|e: InputEvent| {
            e.target_dyn_into::<HtmlInputElement>()
                .map(|i| PostMsg::InputTitle(i.value()))
        });
        let check_abstraction = ctx.link().batch_callback(|e: InputEvent| {
            e.target_dyn_into::<HtmlInputElement>()
                .map(|i| PostMsg::InputAbstraction(i.value()))
        });
        let keywords = ctx.link().callback(PostMsg::UpdateKeywords);
        let authors = ctx.link().callback(PostMsg::UpdateAuthors);

        let onclick = ctx.link().callback_future({
            let cfg = ctx.props().cfg.clone();
            let title = self.title.clone();
            let abstraction = self.abstraction.clone();
            let keywords = self.keywords.clone();
            let authors = self.authors.clone();
            move |_| {
                let cfg = cfg.clone();
                let title = title.clone();
                let abstraction = abstraction.clone();
                let keywords = keywords.clone();
                let authors = authors.clone();
                async move {
                    let body = serde_json::json!({
                        "title": title.as_str(),
                        "abstraction": abstraction.as_str(),
                        "keywords": keywords.iter().map(|k| k.as_str()).collect::<Vec<_>>(),
                        "authors": authors.iter().map(|a| a._id).collect::<Vec<_>>()
                    });
                    match async move {
                        Ok(
                            gloo::net::http::Request::post(cfg.api_addr.join("theses")?.as_str())
                                .json(&body)?
                                .send()
                                .await?
                                .json()
                                .await?,
                        )
                    }
                    .await
                    {
                        Ok(id) => PostMsg::Post(id),
                        Err(e) => PostMsg::Err(e),
                    }
                }
            }
        });

        let alert_box = html_nested!(<AlertBox refresh={ self.alert.clone() }/>);
        html! {
            <div>
                <p>
                    <label>
                        { "Title: " }
                        <input type="text" ref={ self.title_ref.clone() } oninput={ check_title } value={ self.title.clone() } />
                        { if self.title.is_empty().not() { " ✅" } else { " ❎" }}
                    </label>
                </p>

                <p>
                    <label>
                        { "Abstraction: " }<br />
                        <textarea type="text" oninput={ check_abstraction } value={ self.abstraction.clone() } />
                        { if self.abstraction.len() >= 140 { " ✅" } else { " ❎" }}
                    </label>
                </p>

                <p>
                    <authors::Authors cfg={ctx.props().cfg.clone()} {err} {alert} vals={authors} />
                </p>

                <p>
                    <keywords::Keywords vals={ keywords }/>
                </p>

                <p>
                    <button {onclick} />
                </p>

                { alert_box }
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(input) = self.title_ref.cast::<HtmlInputElement>() {
                if let Err(jv) = input.focus() {
                    gloo::console::error!(jv);
                }
            }
        }
    }
}
