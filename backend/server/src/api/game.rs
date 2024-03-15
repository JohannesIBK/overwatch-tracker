use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sea_orm::{DatabaseConnection, Set, TransactionTrait};
use uuid::Uuid;

use entity::game;
use entity::sea_orm_active_enums::{GameResult, Role};
use service::{GameService, ImageService};
use service::models::GameWithImages;

use crate::configuration::app::AppState;
use crate::configuration::session::SessionUser;
use crate::configuration::Error;
use crate::ext::axum_ext::ValidatedJson;

pub fn init_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", axum::routing::put(create_game))
        .route("/:id", axum::routing::delete(delete_game))
        .route("/user/:id", axum::routing::get(get_games))
}

async fn get_games(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Query(query): Query<GetGameQuery>,
) -> Result<impl IntoResponse, Error> {
    let games = GameService::get_by_user_id(&db, id, query.page.unwrap_or(0)).await?;

    Ok(axum::Json(games))
}

async fn create_game(
    State(db): State<DatabaseConnection>,
    user: SessionUser,
    ValidatedJson(payload): ValidatedJson<CreateGamePayload>,
) -> Result<impl IntoResponse, Error> {
    let result = db
        .transaction(|txn| {
            Box::pin(async move {
                let game_model = game::ActiveModel {
                    user_id: Set(user.id),
                    id: Set(Uuid::new_v4()),
                    note: Set(payload.note),
                    rank_adjustment: Set(payload.rank_adjustment),
                    replay_id: Set(payload.replay_id),
                    result: Set(payload.result),
                    role: Set(payload.role),
                    ..Default::default()
                };

                let game = GameService::create_game(txn, game_model).await?;
                let image = ImageService::add_image(txn, payload.stats_url, game.id).await?;

                Ok((game, image))
            })
        })
        .await?;

    let response = GameWithImages {
        id: result.0.id,
        user_id: result.0.user_id,
        note: result.0.note,
        rank_adjustment: result.0.rank_adjustment,
        replay_id: result.0.replay_id,
        result: result.0.result,
        role: result.0.role,
        played_at: result.0.played_at,
        images: vec![result.1],
    };

    Ok(axum::Json(response))
}

async fn delete_game(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    user: SessionUser,
) -> Result<impl IntoResponse, Error> {
    let Some(game) = GameService::get_by_id(&db, id).await? else {
        return Err(Error::Small {
            status: StatusCode::NOT_FOUND,
            error: "Game not found",
        });
    };

    if game.user_id != user.id {
        return Err(Error::Small {
            status: StatusCode::FORBIDDEN,
            error: "Forbidden",
        });
    }

    GameService::delete_game(&db, id).await?;

    Ok(StatusCode::NO_CONTENT.into_response())
}

#[derive(serde::Deserialize, garde::Validate)]
struct GetGameQuery {
    #[garde(range(min = 0))]
    page: Option<usize>,
}

#[derive(serde::Deserialize, garde::Validate)]
#[garde(allow_unvalidated)]
struct CreateGamePayload {
    #[garde(length(min = 1, max = 2000))]
    note: Option<String>,
    result: GameResult,
    role: Role,
    #[garde(range(min = -100, max = 100))]
    rank_adjustment: i16,
    #[garde(length(min = 6, max = 6))]
    replay_id: Option<String>,
    stats_url: String,
}

#[derive(serde::Serialize)]
struct GameResponse {
    game: entity::game::Model,
    images: Vec<entity::image::Model>,
}
