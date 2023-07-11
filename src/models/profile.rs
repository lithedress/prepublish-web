use std::rc::Rc;

#[derive(PartialEq)]
#[derive(serde::Deserialize)]
#[derive(Clone)]
pub struct PublicProfile {
    pub _id: bson::oid::ObjectId,
    pub email: String,
    pub avatar_id: Option<bson::oid::ObjectId>,
    pub joining_at: chrono::DateTime<chrono::Utc>,
    pub name: Rc<str>,
}

impl PublicProfile {
    pub async fn try_get(
        cfg: &super::common::AppConfig,
        id: bson::oid::ObjectId,
    ) -> super::common::AppResult<super::common::FetchRes<Self>> {
        <super::common::FetchRes<Self>>::try_from_gloo_res(
            gloo::net::http::Request::get(
                cfg.api.join(&format!("profiles/{}", id.to_hex()))?.as_str(),
            )
            .send()
            .await,
        )
        .await
    }
}
