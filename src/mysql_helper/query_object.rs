use async_trait::async_trait;
use mysql_async::{Conn, prelude::{Queryable, StatementLike, AsQuery}, Params, Result, Transaction, QueryResult, TextProtocol, Statement, BinaryProtocol};
use mysql_common::prelude::FromRow;
use tokio::sync::MutexGuard;

type BoxFuture<'a, T> = futures_core::future::BoxFuture<'a, Result<T>>;

pub enum QueryObject<'a> {
	Conn(Conn),
	Tx(MutexGuard<'a, Transaction<'static>>)
}

impl Queryable for QueryObject<'_> {
	fn ping(&mut self) -> BoxFuture<'_, ()> {
		match self {
			QueryObject::Conn(conn) => conn.ping(),
			QueryObject::Tx(tx) => tx.ping(),
		}
	}

	fn query_iter<'a, Q>(&'a mut self, query: Q) -> BoxFuture<'a, QueryResult<'a, 'static, TextProtocol>>
	where
		Q: AsQuery + 'a {
		match self {
			QueryObject::Conn(conn) => conn.query_iter(query),
			QueryObject::Tx(tx) => tx.query_iter(query),
		}
	}

	fn prep<'a, Q>(&'a mut self, query: Q) -> BoxFuture<'a, Statement>
	where
		Q: AsQuery + 'a {
		match self {
			QueryObject::Conn(conn) => conn.prep(query),
			QueryObject::Tx(tx) => tx.prep(query),
		}
	}

	fn close(&mut self, stmt: Statement) -> BoxFuture<'_, ()> {
		match self {
			QueryObject::Conn(conn) => conn.close(stmt),
			QueryObject::Tx(tx) => tx.close(stmt),
		}
	}

	fn exec_iter<'a: 's, 's, Q, P>(&'a mut self, stmt: Q, params: P) -> BoxFuture<'s, QueryResult<'a, 'static, BinaryProtocol>>
	where
		Q: StatementLike + 'a,
		P: Into<Params> {
		match self {
			QueryObject::Conn(conn) => conn.exec_iter(stmt, params),
			QueryObject::Tx(tx) => tx.exec_iter(stmt, params),
		}
	}

	fn exec_batch<'a: 'b, 'b, S, P, I>(&'a mut self, stmt: S, params_iter: I) -> BoxFuture<'b, ()>
	where
		S: StatementLike + 'b,
		I: IntoIterator<Item = P> + Send + 'b,
		I::IntoIter: Send,
		P: Into<Params> + Send {
		match self {
			QueryObject::Conn(conn) => conn.exec_batch(stmt, params_iter),
			QueryObject::Tx(tx) => tx.exec_batch(stmt, params_iter),
		}
	}
}

pub struct InsertResult(u64);
impl InsertResult {
	pub fn last_insert_id(&self) -> u64 {
		self.0
	}
}

pub struct UpdateResult(u64);
impl UpdateResult {
	pub fn affected_rows(&self) -> u64 {
		self.0
	}
}

impl QueryObject<'_> {
	pub async fn _query<'a, T, Q>(&'a mut self, query: Q) -> Result<Vec<T>>
	where
		Q: AsQuery + 'a,
		T: FromRow + Send + 'static,
	{
		Queryable::query(self, query).await
	}

	pub async fn query_update<'a, Q>(&'a mut self, query: Q) -> Result<UpdateResult>
	where
		Q: AsQuery + 'a,
	{
		let qr = Queryable::query_iter(self, query).await?;
		let affected_rows = qr.affected_rows();
		qr.drop_result().await?;
		Ok(UpdateResult(affected_rows))
	}

	pub async fn prep<'a, Q>(&'a mut self, query: Q) -> Result<Statement>
	where
		Q: AsQuery + 'a
	{
		Queryable::prep(self, query).await
	}

	pub async fn exec<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> Result<Vec<T>>
	where
		S: StatementLike + 'b,
		P: Into<Params> + Send + 'b,
		T: FromRow + Send + 'static,
	{
		Queryable::exec(self, stmt, params).await
	}

	pub async fn exec_first<'a: 'b, 'b, T, S, P>(&'a mut self, stmt: S, params: P) -> Result<Option<T>>
	where
		S: StatementLike + 'b,
		P: Into<Params> + Send + 'b,
		T: FromRow + Send + 'static,
	{
		Queryable::exec_first(self, stmt, params).await
	}

	pub async fn exec_drop<'a: 'b, 'b, S, P>(&'a mut self, stmt: S, params: P) -> Result<()>
	where
		S: StatementLike + 'b,
		P: Into<Params> + Send + 'b,
	{
		Queryable::exec_drop(self, stmt, params).await
	}

	pub async fn exec_insert<'a: 's, 's, Q, P>(&'a mut self, stmt: Q, params: P) -> Result<InsertResult>
	where
		Q: StatementLike + 'a,
		P: Into<Params>
	{
		let qr = Queryable::exec_iter(self, stmt, params).await?;
		let id = qr.last_insert_id().unwrap();
		qr.drop_result().await?;
		Ok(InsertResult(id))
	}

	pub async fn exec_update<'a: 's, 's, Q, P>(&'a mut self, stmt: Q, params: P) -> Result<UpdateResult>
	where
		Q: StatementLike + 'a,
		P: Into<Params>
	{
		let qr = Queryable::exec_iter(self, stmt, params).await?;
		let affected_rows = qr.affected_rows();
		qr.drop_result().await?;
		Ok(UpdateResult(affected_rows))
	}
}

#[async_trait]
pub trait GetQueryObject: Sync + Send {
	async fn get_query_object(&self) -> Result<QueryObject>;
}
