use std::{rc::Rc, collections::BTreeSet};

use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use super::common::{AppConfig, AppResult, FetchRes};

#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct ThesisId {
    pub _id: ObjectId,
    pub owner_id: ObjectId,
    pub is_passed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Thesis {
    #[serde(flatten)]
    pub id: ThesisId,
    #[serde(default)]
    pub author_ids: Rc<Vec<ObjectId>>,
    // #[serde(default)]
    // pub magazine_id: ObjectId,
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
    ) -> AppResult<FetchRes<Rc<Self>>> {
        <FetchRes<Rc<Self>>>::try_from_gloo_res(
            gloo::net::http::Request::get(
                cfg.api.join(&format!("theses/{}", id.to_hex()))?.as_str(),
            )
            .credentials(web_sys::RequestCredentials::Include)
            .send()
            .await,
        )
        .await
    }
}
