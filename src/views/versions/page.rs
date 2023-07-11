use std::rc::Rc;

use gloo::net::http::Method;
use yew::{
    function_component, html, suspense::use_future, AttrValue, Component, Context, Html,
    HtmlResult, Properties,
};

use crate::models::{
    common::{AppConfig, AppError},
    version::Version,
};

#[derive(Default)]
pub struct Page {
    err: Option<AppError>,
}

pub enum PageMsg {
}

#[derive(PartialEq, Properties)]
pub struct PageProps {
    pub val: Rc<Version>,
    pub cfg: AppConfig,
}

impl Component for Page {
    type Message = PageMsg;
    type Properties = PageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref e) = self.err {
            return e.view();
        }
        let cfg = ctx.props().cfg.clone();
        let val = ctx.props().val.clone();
        let file = Rc::new(
            match cfg
                .api
                .join(&format!("file/{}", val.file_id.to_hex()))
            {
                Ok(file) => file,
                Err(e) => return AppError::from(e).into(),
            },
        );
        let source = match val.source_id {
            Some(source) => match cfg
                .api
                .join(&format!("file/{}", source.to_hex()))
            {
                Ok(source) => Some(source),
                Err(_) => None,
            },
            None => None,
        };

        html! {
            <div>
                <h1>
                    { "Version " }{ val.major_num }{ "." }{ val.minor_num }
                </h1>

                <p>
                    <a href={ file.to_string() }>{ "Release File" }</a>
                </p>

                if let Some(source) = source {
                    <p>
                        <a href={ source.to_string() }>{ "Source Code" }</a>
                    </p>
                }

                <PDF {cfg} {file} />
            </div>
        }
    }
}

#[derive(PartialEq, Properties)]
struct PDFProps {
    cfg: AppConfig,
    file: Rc<url::Url>,
}

#[function_component]
fn PDF(props: &PDFProps) -> HtmlResult {
    let cfg = props.cfg.clone();
    let file = props.file.to_owned();
    let res = use_future(move || async move {
        let viewer = match cfg.api.join("static/web/viewer.html") {
            Ok(viewer) => viewer,
            Err(_) => {
                return None;
            }
        };
        let file = file.clone();
        let content_type = match gloo::net::http::Request::get(file.as_str())
            .method(Method::HEAD)
            .send()
            .await
        {
            Ok(res) => res.headers().get("content-type"),
            Err(_) => {
                return None;
            }
        };
        if content_type != Some(mime::APPLICATION_PDF.to_string()) {
            return None;
        }
        match gloo::net::http::Request::get(viewer.as_str()).send().await {
            Ok(res) => match res.text().await {
                Ok(res) => Some(AttrValue::from(res)),
                Err(_) => {
                    return None;
                }
            },
            Err(_) => {
                return None;
            }
        }
    })?;
    let res = match *res {
        Some(ref res) => Html::from_html_unchecked(res.to_owned()),
        None => Html::default(),
    };
    Ok(res)
}
