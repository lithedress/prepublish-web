use bson::oid::ObjectId;
use yew::prelude::*;
use yew_router::prelude::*;


use crate::app::profiles::TinyProfile;

use super::{common::{AppConfig, FetchRes}, profiles::PublicProfile};

mod get;
mod post;

#[derive(Clone, Routable, PartialEq)]
pub(super) enum Route {
    #[at("/theses")]
    List,
    #[at("/theses/new")]
    New,
    #[at("/theses/:id")]
    View { id: ObjectId },
}

impl Route {
    pub(super) fn switch(cfg: AppConfig) -> impl Fn(Self) -> Html {
        move |routes| match routes {
            Self::New => html! {
                <post::Post cfg={cfg.clone()} />
            },
            _ => html!()
        }
    }
}
