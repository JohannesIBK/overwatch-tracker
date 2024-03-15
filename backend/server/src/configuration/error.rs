use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;

#[derive(Debug)]
pub enum Error {
    Internal,
    Small {
        status: StatusCode,
        error: &'static str,
    },
    Full {
        status: StatusCode,
        error: &'static str,
        data: serde_json::Value,
    },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        let response: ErrorResponse = match self {
            Error::Internal => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error: "Internal Server Error",
                timestamp: chrono::Utc::now(),
                data: None,
            },
            Error::Small { status, error } => ErrorResponse {
                status,
                error,
                timestamp: chrono::Utc::now(),
                data: None,
            },
            Error::Full { status, error, data } => ErrorResponse {
                status,
                error,
                timestamp: chrono::Utc::now(),
                data: Some(data),
            },
        };

        (response.status, Json(response)).into_response()
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(err: sea_orm::DbErr) -> Self {
        tracing::error!(target: "database", "Database error: {:?}", err);

        Self::Full {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: "Database error",
            data: serde_json::json!({ "error": err.to_string(), "kind": "database" }),
        }
    }
}

impl From<sea_orm::TransactionError<sea_orm::DbErr>> for Error {
    fn from(err: sea_orm::TransactionError<sea_orm::DbErr>) -> Self {
        tracing::error!(target: "database", "Transaction error: {:?}", err);

        Self::Full {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: "Database error",
            data: serde_json::json!({ "error": err.to_string(), "kind": "database", "source": "transaction" }),
        }
    }
}


impl From<tower_sessions::session::Error> for Error {
    fn from(err: tower_sessions::session::Error) -> Self {
        Error::Full {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: "Session error",
            data: serde_json::json!({ "error": err.to_string(), "kind": "session" }),
        }
    }
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    #[serde(serialize_with = "crate::ext::serde_ext::status_code")]
    status: StatusCode,
    error: &'static str,
    timestamp: chrono::DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}
