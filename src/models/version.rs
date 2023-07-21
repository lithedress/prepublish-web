use std::{rc::Rc, collections::BTreeSet};

use bson::oid::ObjectId;
use chrono::{DateTime, Utc};

use super::common::{FetchRes, AppResult, AppConfig};

#[derive(serde::Deserialize)]
#[derive(PartialEq)]
#[derive(Default)]
pub enum ReviewPattern {
    Editor(ObjectId),
    #[default]
    Reviewer,
}

#[derive(serde::Deserialize)]
#[serde(default)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct ReviewState {
    pub remainder_reviewer_ids: Rc<BTreeSet<Rc<ObjectId>>>,
    pub pattern: ReviewPattern,
}

#[derive(serde::Deserialize)]
#[derive(PartialEq)]
#[derive(Default)]
pub enum VersionState {
    #[default]
    Uploaded,
    Reviewing,
    Passed(bool),
    History,
}

#[derive(serde::Deserialize)]
#[serde(default)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct Version {
    pub _id: ObjectId,
    pub thesis_id: ObjectId,
    pub uploaded_at: Rc<DateTime<Utc>>,
    pub uploader_id: Option<ObjectId>,
    pub major_num: i32,
    pub minor_num: i32,
    pub file_id: ObjectId,
    pub source_id: Option<ObjectId>,
    pub state: VersionState,
    pub review_state: ReviewState,
    pub downloads: i32,
}

impl Version {
    pub async fn try_get(
        cfg: &AppConfig,
        id: ObjectId,
    ) -> AppResult<FetchRes<(Rc<Self>, bool, bool, bool)>> {
        <FetchRes<(Rc<Self>, bool, bool, bool)>>::try_from_gloo_res(
            gloo::net::http::Request::get(
                cfg.api.join(&format!("versions/{}", id.to_hex()))?.as_str(),
            )
            .credentials(web_sys::RequestCredentials::Include)
            .send()
            .await,
        )
        .await
    }
}