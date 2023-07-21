mod adjudge;
mod edit;
mod review;
use std::rc::Rc;

use yew::{
    Component, Context, html, Html, Properties, html_nested,
};
use yew_router::scope_ext::RouterScopeExt;

use crate::{models::{
    common::{AppConfig, AppError, FetchOther},
    version::Version,
}, views::{Route, alerts::AlertBox}};

mod pdf;

#[derive(Default)]
pub struct Page {
    val: Rc<Version>,
    err: Option<AppError>,
    alert: Option<Rc<FetchOther>>,
}

pub enum PageMsg {
    Err(AppError),
    Alert(FetchOther),
    Refresh(Version),
    Home,
}

#[derive(PartialEq, Properties)]
pub struct PageProps {
    pub val: Rc<Version>,
    pub edit: bool,
    pub review: bool,
    pub adjudge: bool,
    pub cfg: Rc<AppConfig>,
}

impl Component for Page {
    type Message = PageMsg;
    type Properties = PageProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            val: ctx.props().val.to_owned(),
            ..Default::default()
        }
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
            PageMsg::Refresh(val) => {
                self.val = Rc::new(val);
                true
            },
            PageMsg::Home => {
                if let Some(navigator) = ctx.link().navigator() {
                    navigator.push(&Route::Home);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref e) = self.err {
            return e.view();
        }
        let cfg = ctx.props().cfg.to_owned();
        let val = ctx.props().val.to_owned();
        let review = ctx.props().review;
        let adjudge = ctx.props().adjudge;
        let edit = ctx.props().review;
        let file = Rc::new(
            match cfg.api.join(&format!("files/{}", val.file_id.to_hex())) {
                Ok(file) => file,
                Err(e) => return AppError::from(e).into(),
            },
        );
        let source = match val.source_id {
            Some(source) => match cfg.api.join(&format!("files/{}", source.to_hex())) {
                Ok(source) => Some(source),
                Err(e) => return AppError::from(e).into(),
            },
            None => None,
        };
        let err = ctx.link().callback(PageMsg::Err);
        let alert = ctx.link().callback(PageMsg::Alert);
        let home = ctx.link().callback(|_| PageMsg::Home);
        let refresh = ctx.link().callback(PageMsg::Refresh);
        let alert_box = html_nested!(<AlertBox refresh={ self.alert.clone() } />);
        html! {
            <div>
                <h1>
                    { "Version " }{ val.major_num }{ "." }{ val.minor_num }
                </h1>

                <pdf::PDF cfg={ cfg.clone() } {file} />

                if let Some(source) = source {
                    <p>
                        <a href={ source.to_string() }>{ "Source Code" }</a>
                    </p>
                }

                if review {
                    <review::Review cfg={ cfg.clone() } err={ err.clone() } id={ val._id } refresh={ home } />
                }

                if adjudge {
                    <adjudge::Adjudge cfg={ cfg.clone() } err={ err.clone() } id={ val._id } refresh={ refresh.clone() } />
                }

                if edit {
                    <edit::Edit cfg={ cfg.clone() } err={ err.clone() } {alert} id={ val._id } refresh={ refresh.clone() } />
                }

                { alert_box }
            </div>
        }
    }
}
