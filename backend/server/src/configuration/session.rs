use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    http::StatusCode
};
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer, cookie::time::Duration};
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

pub fn load_session() -> SessionManagerLayer<MemoryStore> {
    let session_store = MemoryStore::default();

    SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)))
}
