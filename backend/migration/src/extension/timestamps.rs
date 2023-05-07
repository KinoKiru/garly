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
    async fn remove_timestamps(&self, table_name: String) -> Result<(), DbErr>;
}

#[async_trait]
impl<'a> TimestampTrait for SchemaManager<'a> {
    /// Created timestamp columns and adds trigger to table
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

    /// Deletes Timestamp columns and removes trigger from table
    async fn remove_timestamps(&self, table_name: String) -> Result<(), DbErr> {
        if self
            .has_column(&table_name, Timestamp::CreatedAt.to_string())
            .await?
            && self
                .has_column(&table_name, Timestamp::UpdatedAt.to_string())
                .await?
        {
            self.alter_table(
                Table::alter()
                    .drop_column(Timestamp::CreatedAt)
                    .drop_column(Timestamp::UpdatedAt)
                    .take(),
            )
            .await?;

            self.get_connection()
                .execute(Statement::from_string(
                    self.get_database_backend(),
                    format!(r#"DROP TRIGGER IF EXISTS set_{table_name}_timestamp"#),
                ))
                .await
                .map(|_| ())
        } else {
            return Err(DbErr::Custom(
                "Table does not contain the timestamp columns".to_string(),
            ));
        }
    }
}
