use bson::oid::ObjectId;
use yew::prelude::*;

use super::common::AppConfig;

#[derive(PartialEq, Properties)]
pub struct GetProps {
    pub cfg: AppConfig,
    pub id: ObjectId,
}

#[function_component]
pub fn Get(props: &GetProps) -> HtmlResult {
    let GetProps { cfg, id } = props;
    Ok(html!{<div> </div>})
}