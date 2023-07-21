use yew::{function_component, html, HtmlResult, Properties, suspense::use_future};
use std::rc::Rc;
use gloo::net::http::Method;
use crate::models::common::AppConfig;

#[derive(PartialEq, Properties)]
pub struct PDFProps {
    pub cfg: Rc<AppConfig>,
    pub file: Rc<url::Url>,
}

#[function_component]
pub fn PDF(props: &PDFProps) -> HtmlResult {
    let cfg = props.cfg.to_owned();
    let file = props.file.to_owned();
    let res = use_future({
        let file = file.clone();
        move || async move {
            let mut viewer = match cfg.api.join("static/web/viewer.html") {
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
            viewer.set_query(Some(&format!("file={}", file.as_str())));
            Some(viewer)
        }
    })?;
    let res = match *res {
        Some(ref res) => html!(<iframe scrolling="auto" frameborder="0" src={res.to_string()} />),
        None => html!(<p><a href={ file.to_string() }>{ "Release File" }</a></p>),
    };
    Ok(res)
}
