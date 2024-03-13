pub use sea_orm_migration::prelude::*;

mod m20240311_200231_game;
mod m20240311_194457_user;
mod m20240312_103712_image;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240311_194457_user::Migration),
            Box::new(m20240311_200231_game::Migration),
            Box::new(m20240312_103712_image::Migration),
        ]
    }
}
