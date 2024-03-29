use sea_orm::prelude::Uuid;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, Set};

use entity::user;

pub struct UserService;

impl UserService {
    pub async fn create_user(
        db: &impl ConnectionTrait,
        id: Uuid,
        username: String,
        password: String,
    ) -> Result<u64, DbErr> {
        let user = user::ActiveModel {
            id: Set(id),
            username: Set(username),
            password: Set(password),
            ..Default::default()
        };

        user::Entity::insert(user)
            .on_conflict(
                OnConflict::column(user::Column::Username)
                    .do_nothing()
                    .to_owned(),
            )
            .exec_without_returning(db)
            .await
    }

    pub async fn find_by_username(
        db: &impl ConnectionTrait,
        username: String,
    ) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await
    }
}
