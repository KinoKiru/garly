use sea_orm_migration::prelude::*;

use crate::extension::timestamps::TimestampTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Password).string_len(256).not_null())
                    .take(),
            )
            .await?;

        return manager.timestamps(User::Table.to_string()).await;
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).take())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    Email,
    Password,
}
