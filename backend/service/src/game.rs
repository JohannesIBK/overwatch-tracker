use sea_orm::prelude::Uuid;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, DeleteResult, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect,
};

use crate::models::GameWithImages;
use entity::game::{self, Entity as Game};
use entity::image;

pub struct GameService;

impl GameService {
    pub async fn get_by_id(
        db: &impl ConnectionTrait,
        id: Uuid,
    ) -> Result<Option<game::Model>, DbErr> {
        Game::find().filter(game::Column::Id.eq(id)).one(db).await
    }

    pub async fn get_by_user_id(
        db: &impl ConnectionTrait,
        user_id: Uuid,
        page: usize,
    ) -> Result<Vec<GameWithImages>, DbErr> {
        Game::find()
            .filter(game::Column::UserId.eq(user_id))
            .limit(50)
            .offset((page * 50) as u64)
            .order_by_desc(game::Column::PlayedAt)
            .find_with_related(image::Entity)
            .all(db)
            .await
            .map(|v| {
                v.into_iter()
                    .map(|(game, images)| GameWithImages {
                        id: game.id,
                        user_id: game.user_id,
                        note: game.note,
                        rank_adjustment: game.rank_adjustment,
                        replay_id: game.replay_id,
                        result: game.result,
                        role: game.role,
                        played_at: game.played_at,
                        images,
                    })
                    .collect()
            })
    }

    pub async fn create_game(
        db: &impl ConnectionTrait,
        model: game::ActiveModel,
    ) -> Result<game::Model, DbErr> {
        model.insert(db).await
    }

    pub async fn delete_game(db: &impl ConnectionTrait, id: Uuid) -> Result<DeleteResult, DbErr> {
        Game::delete_by_id(id).exec(db).await
    }
}
