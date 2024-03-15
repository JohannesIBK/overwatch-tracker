use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse};
use sea_orm::DatabaseConnection;
use tower_sessions::Session;

use service::UserService;

use crate::configuration::session::SessionUser;
use crate::configuration::{utils, Error};
use crate::ext::axum_ext::ValidatedJson;

pub fn init_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", axum::routing::get(get_data))
        .route("/login", axum::routing::post(login))
        .route("/register", axum::routing::post(register))
}

async fn register(
    State(db): State<DatabaseConnection>,
    ValidatedJson(payload): ValidatedJson<RegisterPayload>,
) -> Result<impl IntoResponse, Error> {
    let password = utils::hash_password(payload.password).await?;
    let id = uuid::Uuid::new_v4();

    let result = UserService::create_user(&db, id, payload.username.to_lowercase(), password).await?;

    if result == 0 {
        Err(Error::Small {
            status: StatusCode::CONFLICT,
            error: "Username already in use",
        })
    } else {
        Ok(StatusCode::CREATED)
    }
}

async fn login(
    State(db): State<DatabaseConnection>,
    session: Session,
    ValidatedJson(payload): ValidatedJson<RegisterPayload>,
) -> Result<impl IntoResponse, Error> {
    let Some(user) = UserService::find_by_username(&db, payload.username.to_lowercase()).await?
    else {
        return Err(Error::Small {
            status: StatusCode::BAD_REQUEST,
            error: "User not found",
        });
    };

    if utils::validate_password(payload.password, user.password).await? {
        let session_user = SessionUser {
            id: user.id,
            username: user.username,
        };

        session
            .insert("user", session_user.clone())
            .await
            .expect("Failed to serialize user");

        Ok(axum::Json(session_user))
    } else {
        Err(Error::Small {
            status: StatusCode::UNAUTHORIZED,
            error: "Invalid password",
        })
    }
}

async fn get_data(session_user: SessionUser) -> Result<impl IntoResponse, Error> {
    Ok(axum::Json(session_user))
}

#[derive(serde::Deserialize, garde::Validate)]
struct RegisterPayload {
    #[garde(length(min = 3, max = 32))]
    username: String,
    #[garde(length(min = 8, max = 64))]
    password: String,
}
