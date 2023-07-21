use std::rc::Rc;

use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use super::common::{AppConfig, AppError, FetchRes};

#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct PublicProfile {
    pub _id: ObjectId,
    pub email: Rc<str>,
    pub avatar_id: Option<ObjectId>,
    pub joining_at: Rc<DateTime<Utc>>,
    pub name: Rc<str>,
}

impl PublicProfile {
    pub async fn try_get(
        cfg: &AppConfig,
        id: ObjectId,
    ) -> Result<FetchRes<Rc<Self>>, AppError> {
        <FetchRes<Rc<Self>>>::try_from_gloo_res(
            gloo::net::http::Request::get(
                cfg.api.join(&format!("profiles/{}", id.to_hex()))?.as_str(),
            )
            .send()
            .await,
        )
        .await
    }
}

#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
#[derive(Clone, Copy)]
pub struct Setting {
    pub email_notice: bool,
    pub push: bool,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            email_notice: true,
            push: true,
        }
    }
}

#[derive(PartialEq)]
#[derive(Deserialize)]
pub struct Profile {
    pub public_profile: PublicProfile,
    pub setting: Setting,
}