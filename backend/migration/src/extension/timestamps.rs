use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;
use sea_orm_migration::{async_trait::async_trait, DbErr, SchemaManager};

#[derive(Iden)]
pub enum Timestamp {
    CreatedAt,
    UpdatedAt,
}

#[async_trait]
pub trait TimestampTrait {
    async fn timestamps(&self, table_name: String) -> Result<(), DbErr>;
}

#[async_trait]
impl<'a> TimestampTrait for SchemaManager<'a> {
    async fn timestamps(&self, table_name: String) -> Result<(), DbErr> {
        self.alter_table(
            Table::alter()
                .table(Alias::new(&table_name))
                .add_column_if_not_exists(
                    ColumnDef::new(Timestamp::CreatedAt)
                        .timestamp()
                        .extra("DEFAULT NOW()".to_string())
                        .not_null(),
                )
                .add_column_if_not_exists(
                    ColumnDef::new(Timestamp::UpdatedAt)
                        .timestamp()
                        .extra("DEFAULT NOW()".to_string())
                        .not_null(),
                )
                .take(),
        )
        .await?;

        self.get_connection()
            .execute(Statement::from_string(
                self.get_database_backend(),
                format!(
                    r#"CREATE TRIGGER set_{table_name}_timestamp 
                    BEFORE UPDATE ON "{table_name}" 
                    FOR EACH ROW 
                    EXECUTE PROCEDURE trigger_update_timestamp();"#
                ),
            ))
            .await
            .map(|_| ())
    }
}
