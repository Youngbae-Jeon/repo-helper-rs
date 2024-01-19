use sqlx::mysql::MySqlQueryResult;
use tokio::sync::MutexGuard;
use futures_core::future::BoxFuture;

use crate::{InsertResult, UpdateResult};

type SqlxConn = sqlx::pool::PoolConnection<sqlx::MySql>;
type SqlxTransaction<'t> = sqlx::Transaction<'t, sqlx::MySql>;

#[derive(Debug)]
pub enum ExecutorObject<'a> {
	Conn(SqlxConn),
	MutexGuardTransaction(MutexGuard<'a, SqlxTransaction<'static>>)
}

impl<'c> sqlx::Executor<'c> for &'c mut ExecutorObject<'_> {
	type Database = sqlx::MySql;

	fn fetch_many<'e, 'q: 'e, E: 'q>(
		self,
		query: E,
	) -> futures_core::stream::BoxStream<
		'e,
		Result<
			sqlx::Either<<Self::Database as sqlx::Database>::QueryResult, <Self::Database as sqlx::Database>::Row>,
			sqlx::Error,
		>,
	>
	where
		'c: 'e,
		E: sqlx::Execute<'q, Self::Database> {
		match self {
			ExecutorObject::Conn(conn) => conn.fetch_many(query),
			ExecutorObject::MutexGuardTransaction(tx) => tx.fetch_many(query)
		}
	}

	fn fetch_optional<'e, 'q: 'e, E: 'q>(
		self,
		query: E,
	) -> BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
	where
		'c: 'e,
		E: sqlx::Execute<'q, Self::Database> {
		match self {
			ExecutorObject::Conn(conn) => conn.fetch_optional(query),
			ExecutorObject::MutexGuardTransaction(tx) => tx.fetch_optional(query)
		}
	}

	fn prepare_with<'e, 'q: 'e>(
		self,
		sql: &'q str,
		parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
	) -> BoxFuture<'e, Result<<Self::Database as sqlx::database::HasStatement<'q>>::Statement, sqlx::Error>>
	where
		'c: 'e {
		match self {
			ExecutorObject::Conn(conn) => conn.prepare_with(sql, parameters),
			ExecutorObject::MutexGuardTransaction(tx) => tx.prepare_with(sql, parameters)
		}
	}

	fn describe<'e, 'q: 'e>(
		self,
		sql: &'q str,
	) -> BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
	where
		'c: 'e {
		match self {
			ExecutorObject::Conn(conn) => conn.describe(sql),
			ExecutorObject::MutexGuardTransaction(tx) => tx.describe(sql)
		}
	}
}

impl From<MySqlQueryResult> for InsertResult {
	fn from(result: MySqlQueryResult) -> Self {
		InsertResult(result.last_insert_id())
	}
}

impl From<MySqlQueryResult> for UpdateResult {
	fn from(result: MySqlQueryResult) -> Self {
		UpdateResult(result.rows_affected())
	}
}
