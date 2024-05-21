use sea_orm_migration::prelude::*;

use crate::{m20240520_104447_create_group::Group, m20240520_104508_create_message::Message};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Conversation::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Conversation::GroupId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_conversation_group_groupId")
                            .from(Conversation::Table, Conversation::GroupId)
                            .to(Group::Table, Group::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Conversation::MsgId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_conversation_message_msgId")
                            .from(Conversation::Table, Conversation::MsgId)
                            .to(Message::Table, Message::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .name("PK_conversation")
                            .col(Conversation::GroupId)
                            .col(Conversation::MsgId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Conversation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Conversation {
    Table,
    GroupId,
    MsgId,
}
