pub use sea_orm_migration::prelude::*;

mod m20240520_104423_create_user;
mod m20240520_104447_create_group;
mod m20240520_104508_create_message;
mod m20240520_104520_create_file_msg;
mod m20240520_104527_create_conversation;
mod m20240520_111859_create_uuid_extension;
mod m20240521_123300_create_user_group;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240520_104423_create_user::Migration),
            Box::new(m20240520_104447_create_group::Migration),
            Box::new(m20240520_104508_create_message::Migration),
            Box::new(m20240520_104520_create_file_msg::Migration),
            Box::new(m20240520_104527_create_conversation::Migration),
            Box::new(m20240520_111859_create_uuid_extension::Migration),
            Box::new(m20240521_123300_create_user_group::Migration),
        ]
    }
}
