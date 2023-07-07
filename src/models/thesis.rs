use std::{rc::Rc, collections::BTreeSet};

use bson::oid::ObjectId;

use super::common::{AppConfig, FetchResult, FetchRes};

#[derive(PartialEq)]
#[derive(serde::Deserialize)]
pub struct Thesis {
    #[serde(default)]
    pub _id: ObjectId,
    #[serde(default)]
    pub owner_id: bson::oid::ObjectId,
    #[serde(default)]
    pub is_passed: bool,
    #[serde(default)]
    pub created_at: Rc<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub author_ids: Rc<Vec<bson::oid::ObjectId>>,
    #[serde(default)]
    pub magazine_id: ObjectId,
    #[serde(default)]
    pub doi: Option<Rc<str>>,
    pub title: Rc<str>,
    pub abstraction: Rc<str>,
    #[serde(default)]
    pub keywords: Rc<Vec<Rc<str>>>,
    #[serde(default)]
    pub languages: Rc<BTreeSet<Rc<str>>>,
}

impl Thesis {
    pub async fn try_get(
        cfg: &AppConfig,
        id: ObjectId,
    ) -> FetchResult<FetchRes<Self>> {
        <FetchRes<Self>>::try_from_gloo_res(
            gloo::net::http::Request::get(
                cfg.api_addr.join("theses")?.join(&id.to_hex())?.as_str(),
            )
            .send()
            .await,
        )
        .await
    }
}
