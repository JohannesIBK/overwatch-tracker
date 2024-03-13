use sea_orm::prelude::Uuid;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, DeleteResult, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};

use entity::game::{self, Entity as Game};
use entity::sea_orm_active_enums::Role;

pub struct GameService;

impl GameService {
    pub async fn get_by_id(db: &impl ConnectionTrait, id: Uuid) -> Result<Option<game::Model>, DbErr> {
        Game::find()
            .filter(game::Column::Id.eq(id))
            .one(db)
            .await
    }

    pub async fn get_by_user_id(db: &impl ConnectionTrait, user_id: Uuid, page: usize) -> Result<Vec<game::Model>, DbErr> {
        Game::find()
            .filter(game::Column::UserId.eq(user_id))
            .order_by_asc(game::Column::PlayedAt)
            .limit(50)
            .offset((page * 50) as u64)
            .all(db)
            .await
    }

    pub async fn create_game(
        db: &impl ConnectionTrait,
        user_id: Uuid,
        note: Option<String>,
        rank_adjustment: i16,
        replay_id: Option<String>,
        result: entity::sea_orm_active_enums::GameResult,
        role: Role,
    ) -> Result<game::Model, DbErr> {
        let game = game::ActiveModel {
            user_id: Set(user_id),
            note: Set(note),
            rank_adjustment: Set(rank_adjustment),
            replay_id: Set(replay_id),
            result: Set(result),
            role: Set(role),
            ..Default::default()
        };


        game.insert(db).await
    }

    pub async fn delete_game(db: &impl ConnectionTrait, id: Uuid) -> Result<DeleteResult, DbErr> {
        Game::delete_by_id(id)
            .exec(db)
            .await
    }
}
