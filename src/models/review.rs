use std::rc::Rc;

use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::common::{AppConfig, AppResult, FetchRes};

#[derive(Deserialize)]
#[derive(PartialEq)]
pub struct Review {
    #[serde(default)]
    pub _id: ObjectId,
    #[serde(default)]
    pub version_id: ObjectId,
    #[serde(default)]
    pub reviewed_at: Rc<DateTime<Utc>>,
    #[serde(default)]
    pub reviewer_id: Option<ObjectId>,
    #[serde(default)]
    pub judgement: bool,
    pub criticism: Rc<str>,
}

impl Review {
    pub async fn try_get(
        cfg: &AppConfig,
        id: ObjectId,
    ) -> AppResult<FetchRes<Rc<Self>>> {
        <FetchRes<Rc<Self>>>::try_from_gloo_res(
            gloo::net::http::Request::get(
                cfg.api.join(&format!("reviews/{}", id.to_hex()))?.as_str(),
            )
            .credentials(web_sys::RequestCredentials::Include)
            .send()
            .await,
        )
        .await
    }
}