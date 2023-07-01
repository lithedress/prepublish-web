use bson::oid::ObjectId;
use yew::prelude::*;

use super::common::AppConfig;

#[derive(PartialEq, Properties)]
pub struct GetProps {
    cfg: AppConfig,
    id: ObjectId,
}

#[function_component]
pub fn Get(props: &GetProps) -> HtmlResult {
    let GetProps { cfg, id } = props;
    html! {
        <div>
            
        </div>
    }
}