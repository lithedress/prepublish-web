use std::rc::Rc;

use yew::{function_component, Properties, html, HtmlResult, suspense::use_future};

use crate::models::{common::{AppConfig, FetchRes}, version::Version};
use super::alerts::AlertBox;

mod page;

#[derive(PartialEq, Properties)]
pub struct GetProps {
    pub cfg: Rc<AppConfig>,
    pub id: bson::oid::ObjectId,
}

#[function_component]
pub fn Get(props: &GetProps) -> HtmlResult {
    let cfg = props.cfg.to_owned();
    let id = props.id;

    let res = use_future({
        let cfg = cfg.clone();
        move || async move {
            Version::try_get(&cfg, id).await
        }
    })?;
    let res = match *res {
        Ok(ref res) => match res {
            FetchRes::Body(body) => html! {
                <page::Page val={ body.0.clone() } edit={ body.1 } review={ body.2 } adjudge={ body.3 } {cfg} />
            },
            FetchRes::Other(other) => html!(<AlertBox refresh={ Some(Rc::new(other.to_owned())) } />),
        }
        Err(ref e) => e.view(),
    };
    Ok(res)
}