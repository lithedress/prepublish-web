use std::rc::Rc;

use serde::Deserialize;
use yew::AttrValue;

use super::common::{AppConfig, self};

#[derive(Deserialize)]
#[derive(Clone)]
pub(super) struct PublicProfile {
    pub(super) _id: bson::oid::ObjectId,
    pub(super) email: Rc<str>,
    pub(super) avatar_id: Option<bson::oid::ObjectId>,
    pub(super) joining_at: chrono::DateTime<chrono::Utc>,
    pub(super) name: Rc<str>,
}

impl PublicProfile {
    pub(super) async fn get(cfg: AppConfig, id: bson::oid::ObjectId) -> Result<common::FetchRes<Self>, common::FetchError> {
        let url = cfg.api_addr.join("profiles")?.join(&id.to_hex())?;
        let res = gloo::net::http::Request::get(url.as_str())
                .send()
                .await?;
        if res.ok() {
            let data = res.json().await?;
            Ok(common::FetchRes::OK(data))
        } else {  
            let status = res.status();
            let msg = res.text().await?;
            Ok(common::FetchRes::Other { status, msg })
        }
    }
}

pub(super) struct TinyProfile {
    pub(super) avatar: Option<Rc<url::Url>>,
    pub(super) name: AttrValue,
}

impl TinyProfile {
    pub(super) fn view(&self) -> yew::Html {
        let src: AttrValue = match &self.avatar {
            Some(avatar) => avatar.to_string().into(),
            None => AttrValue::Static("/default.jpg"),
        };
        yew::html! {
            <div>
                <img {src} />
                { self.name.clone() }
            </div>
        }
    }
}

impl TryFrom<(AppConfig, PublicProfile)> for TinyProfile {
    type Error = common::FetchError;

    fn try_from((cfg, public_profile): (AppConfig, PublicProfile)) -> Result<Self, Self::Error> {
        let avatar = match public_profile.avatar_id {
            Some(avatar) => Some(Rc::new(cfg.api_addr.join("file")?.join(&avatar.to_hex())?)),
            None => None,
        };
        let name = AttrValue::Rc(public_profile.name);
        Ok(Self { avatar, name })
    }
}