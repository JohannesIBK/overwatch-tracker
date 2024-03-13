use axum::{
    http::request::Parts,
    extract::FromRequestParts,
    http::StatusCode
};
use sea_orm::prelude::async_trait;
use tower_sessions::Session;
use uuid::Uuid;

use crate::configuration::Error;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionUser {
    pub id: Uuid,
    pub username: String,
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for SessionUser
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|(code, err)| Error::Small {
                status: code,
                error: err,
            })?;

        if let Some(user) = session.get::<SessionUser>("user").await? {
            Ok(user)
        } else {
            Err(Error::Small {
                error: "Unauthorized",
                status: StatusCode::UNAUTHORIZED,
            })
        }
    }
}
