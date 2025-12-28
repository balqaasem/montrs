use rusqlite::{Connection, ToSql, Params};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database connection error: {0}")]
    Connection(String),
    #[error("Query execution error: {0}")]
    Query(String),
    #[error("Migration error: {0}")]
    Migration(String),
}

pub trait FromRow: Sized {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self>;
}

pub trait DbBackend: Send + Sync + 'static {
    fn execute(&self, sql: &str, params: &[&dyn ToSql]) -> Result<usize, DbError>;
    fn query<T: FromRow>(&self, sql: &str, params: &[&dyn ToSql]) -> Result<Vec<T>, DbError>;
}

pub struct SqliteBackend {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteBackend {
    pub fn new(path: &str) -> Result<Self, DbError> {
        let conn = if path == ":memory:" {
            Connection::open_in_memory()
        } else {
            Connection::open(path)
        }.map_err(|e| DbError::Connection(e.to_string()))?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

impl DbBackend for SqliteBackend {
    fn execute(&self, sql: &str, params: &[&dyn ToSql]) -> Result<usize, DbError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(sql, params)
            .map_err(|e| DbError::Query(e.to_string()))
    }

    fn query<T: FromRow>(&self, sql: &str, params: &[&dyn ToSql]) -> Result<Vec<T>, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(sql).map_err(|e| DbError::Query(e.to_string()))?;
        let rows = stmt.query_map(params, |row| T::from_row(row))
            .map_err(|e| DbError::Query(e.to_string()))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| DbError::Query(e.to_string()))?);
        }
        Ok(results)
    }
}
