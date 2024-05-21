use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Group::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(Group::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Group {
    Table,
    Id,
    Name,
}
