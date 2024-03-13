use axum::http::StatusCode;
use axum::{
    extract::{
        rejection::{JsonRejection, QueryRejection},
        FromRequest, FromRequestParts, Json, Query,
    },
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::configuration::Error;

#[derive(Debug, Clone)]
pub struct ValidatedJson<T>(pub T);

#[async_trait::async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + garde::validate::Validate<Context = ()>,
    T::Context:,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = Error;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| Error::Full {
                status: StatusCode::UNPROCESSABLE_ENTITY,
                error: "Unprocessable Entity",
                data: serde_json::json!({ "error": e.to_string(), "kind": "serialization" }),
            })?;

        value.validate(&()).map_err(|e| Error::Full {
            status: StatusCode::BAD_REQUEST,
            error: "Bad Request",
            data: serde_json::json!({ "error": e.to_string(), "kind": "validation" }),
        })?;

        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Clone)]
pub struct ValidatedQuery<T>(pub T);

#[async_trait::async_trait]
impl<T, S> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + garde::Validate<Context = ()>,
    S: Send + Sync,
    Query<T>: FromRequestParts<S, Rejection = QueryRejection>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| Error::Full {
                status: StatusCode::UNPROCESSABLE_ENTITY,
                error: "Unprocessable Entity",
                data: serde_json::json!({ "error": e.to_string(), "kind": "serialization" }),
            })?;

        value.validate(&()).map_err(|e| Error::Full {
            status: StatusCode::BAD_REQUEST,
            error: "Bad Request",
            data: serde_json::json!({ "error": e.to_string(), "kind": "validation" }),
        })?;

        Ok(ValidatedQuery(value))
    }
}
