use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, Set};
use sea_orm::prelude::Uuid;

use entity::image;

pub struct ImageService;

impl ImageService {
    pub async fn add_image(db: &impl ConnectionTrait, url: String, game_id: Uuid) -> Result<image::Model, DbErr> {
        let image = image::ActiveModel {
            url: Set(url),
            game_id: Set(game_id),
            ..Default::default()
        };

        image.insert(db).await
    }
}
