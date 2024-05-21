use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FileMsg::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FileMsg::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(FileMsg::MsgId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_fileMsg_msg_msgId")
                            .from(FileMsg::Table, FileMsg::MsgId)
                            .to(FileMsg::Table, FileMsg::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(FileMsg::FileName).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FileMsg::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FileMsg {
    Table,
    Id,
    MsgId,
    FileName,
}
