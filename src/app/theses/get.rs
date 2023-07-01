use bson::oid::ObjectId;
use yew::{prelude::*, suspense::use_future};

use crate::app::{common::{AppConfig, FetchRes}, profiles::PublicProfile};

#[derive(PartialEq, Properties)]
pub(super) struct GetProps {
    pub(super) cfg: AppConfig,
    pub(super) id: ObjectId,
}

#[function_component]
pub(super) fn Get(props: &GetProps) -> HtmlResult {
    let cfg = props.cfg.clone();
    let id = props.id.clone();
    let res = use_future(move || async move {
        PublicProfile::get(cfg.clone(), id.clone()).await
    })?;
    let res = match *res {
        Ok(ref res) => {
            match res {
                FetchRes::OK(body) => {
                    match body.try_view(props.cfg.clone()) {
                        Ok(page) => page,
                        Err(failure) => failure.view(),
                    }
                }
                FetchRes::Other(other) => other.view(),
            }
        }
        Err(ref failure) => failure.view(),
    };
    Ok(res)
}