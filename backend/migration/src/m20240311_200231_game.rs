use sea_orm_migration::prelude::*;
use crate::extension::postgres::Type;

use crate::m20240311_194457_user::User;
use crate::sea_orm::{EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(GameResult::Table)
                    .values(GameResult::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Game::Id)
                            .uuid()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::UserId).uuid().not_null())
                    .col(ColumnDef::new(Game::Note).string_len(2048))
                    .col(ColumnDef::new(Game::RankAdjustment).small_integer().not_null())
                    .col(ColumnDef::new(Game::ReplayId).string_len(8))
                    .col(ColumnDef::new(Game::Result).custom(GameResult::Table))
                    .col(
                        ColumnDef::new(Game::PlayedAt)
                            .timestamp()
                            .not_null()
                            .default(Keyword::CurrentTimestamp),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Game::Table, Game::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(GameResult::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Game {
    Table,
    Id,
    UserId,
    RankAdjustment,
    Result,
    ReplayId,
    Note,
    PlayedAt,
}

#[derive(Iden, EnumIter)]
enum GameResult {
    Table,
    Won,
    Lost,
    Draw,
}
