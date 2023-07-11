use std::rc::Rc;

use bson::oid::ObjectId;
use yew::{AttrValue, Html, html, function_component, Properties, HtmlResult, suspense::use_future, Suspense};

use crate::{models::{profile::PublicProfile, common::{AppConfig, FetchRes}}, views::alerts::AlertBox};

impl PublicProfile {
    pub fn view_tiny(&self, cfg: &AppConfig) -> Html {
        let src: AttrValue = match self.avatar_id {
            Some(aid) => match cfg.api.join(&format!("profiles/{}", aid.to_hex())) {
                Ok(avatar) => avatar.to_string().into(),
                Err(_) => "/default.jpg".into(),
            },
            None => "/default.jpg".into(),
        };
        html! {
            <>
                <img {src} />
                { self.name.clone() }
            </>
        }
    }

    pub fn tiny_from_id(id: ObjectId, cfg: Rc<AppConfig>) -> Html {
        let fallback = html! {<div>{format!("Loading Tiny Profile {id}...")}</div>};

        html! {
            <Suspense {fallback}>
                <TinyProfile {cfg} {id} />
            </Suspense>
        }
    }
}

#[derive(PartialEq, Properties)]
struct TinyProfileProps {
    cfg: Rc<AppConfig>,
    id: ObjectId,
}

#[function_component]
fn TinyProfile(props: &TinyProfileProps) -> HtmlResult {
    let cfg = props.cfg.clone();
    let id = props.id;

    let res = use_future({
        let cfg = cfg.clone();
        move || async move {
            PublicProfile::try_get(&cfg, id).await
        }
    })?;
    let res = match *res {
        Ok(ref res) => match res {
            FetchRes::Body(val) => val.view_tiny(&cfg.clone()),
            FetchRes::Other(other) => html!(<AlertBox refresh={ Some(Rc::new(other.to_owned())) } />),
        },
        Err(ref e) => e.view(),
    };
    Ok(res)
}