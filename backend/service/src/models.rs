use sea_orm::prelude::{DateTime, Uuid};

use entity::image;
use entity::sea_orm_active_enums::{GameResult, Role};

#[derive(serde::Serialize)]
pub struct GameWithImages {
    pub id: Uuid,
    pub user_id: Uuid,
    pub note: Option<String>,
    pub rank_adjustment: i16,
    pub replay_id: Option<String>,
    pub result: GameResult,
    pub role: Role,
    pub played_at: DateTime,
    pub images: Vec<image::Model>,
}
