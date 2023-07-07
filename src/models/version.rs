#[derive(serde::Deserialize)]
#[derive(PartialEq)]
#[derive(Default)]
pub enum ReviewPattern {
    Editor(bson::oid::ObjectId),
    #[default]
    Reviewer,
}

#[derive(serde::Deserialize)]
#[serde(default)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct ReviewState {
    pub remainder_reviewer_ids: std::rc::Rc<std::collections::BTreeSet<std::rc::Rc<bson::oid::ObjectId>>>,
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
    pub _id: bson::oid::ObjectId,
    pub thesis_id: bson::oid::ObjectId,
    pub uploaded_at: std::rc::Rc<chrono::DateTime<chrono::Utc>>,
    pub uploader_id: Option<bson::oid::ObjectId>,
    pub major_num: i32,
    pub minor_num: i32,
    pub file_id: bson::oid::ObjectId,
    pub source_id: Option<bson::oid::ObjectId>,
    pub state: VersionState,
    pub review_state: ReviewState,
    pub downloads: i32,
}

impl Version {
    pub async fn try_get(
        cfg: &super::common::AppConfig,
        id: bson::oid::ObjectId,
    ) -> super::common::FetchResult<super::common::FetchRes<Self>> {
        <super::common::FetchRes<Self>>::try_from_gloo_res(
            gloo::net::http::Request::get(
                cfg.api_addr.join("profiles")?.join(&id.to_hex())?.as_str(),
            )
            .send()
            .await,
        )
        .await
    }
}