use sea_orm_migration::prelude::*;

use crate::{m20240520_104423_create_user::User, m20240520_104447_create_group::Group};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserGroup::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserGroup::GroupId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_UserGroup_group_groupId")
                            .from(UserGroup::Table, UserGroup::GroupId)
                            .to(Group::Table, Group::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(UserGroup::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_UserGroup_user_userId")
                            .from(UserGroup::Table, UserGroup::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .name("PK_UserGroup")
                            .col(UserGroup::GroupId)
                            .col(UserGroup::UserId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserGroup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserGroup {
    Table,
    GroupId,
    UserId,
}
