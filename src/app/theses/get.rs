use bson::oid::ObjectId;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::common::AppConfig;

#[derive(PartialEq, Properties)]
pub(super) struct GetProps {
    pub(super) cfg: AppConfig,
    pub(super) id: ObjectId,
}

#[function_component]
pub(super) fn Get(props: &GetProps) -> Html {
    html! {
        <div></div>
    }
}