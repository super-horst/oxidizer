use tokio_postgres::Row;

use super::migration::Migration;
use super::async_trait;
use super::db::{DBResult, DB};
use super::db_types::ToSql;

/// Trait implemented by all derived Entitities
#[async_trait]
pub trait IEntity: Sized {
    async fn save(&mut self, db: &DB) -> DBResult<bool>;
    async fn delete(&mut self, db: &DB) -> DBResult<bool>;

    fn from_row(row: &Row) -> Self;
    fn create_migration() -> DBResult<Migration>;
    fn get_table_name() -> String;

    async fn find(db: &DB, query: &str, params: &'_ [&'_ (dyn ToSql + Sync)]) -> DBResult<Vec<Self>>;
    async fn first(db: &DB, query: &str, params: &'_ [&'_ (dyn ToSql + Sync)]) -> DBResult<Option<Self>>;
}