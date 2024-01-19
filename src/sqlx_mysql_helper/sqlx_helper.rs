use crate::{Filter, SqlFilter, SqlValues, SqlUpdates, RepoValue};

type SqlxQueryAs<'q, O> = sqlx::query::QueryAs<'q, sqlx::MySql, O, sqlx::mysql::MySqlArguments>;
type SqlxQuery<'q> = sqlx::query::Query<'q, sqlx::MySql, sqlx::mysql::MySqlArguments>;

const PARAM_SYMBOL: &str = "?";

pub trait SqlxHelper {
	fn with_binding_holder(&self) -> String;
}

impl SqlxHelper for SqlFilter<'_> {
	fn with_binding_holder(&self) -> String {
		self.iter()
			.map(|f| {
				let field = f.name();
				match f.filter() {
					Filter::Equal(_) => format!("{field}={PARAM_SYMBOL}"),
					Filter::Not(_) => format!("{field}<>{PARAM_SYMBOL}"),
					Filter::LessorThan(_) => format!("{field}<{PARAM_SYMBOL}"),
					Filter::EqualOrLessorThan(_) => format!("{field}<={PARAM_SYMBOL}"),
					Filter::GreaterThan(_) => format!("{field}>{PARAM_SYMBOL}"),
					Filter::EqualOrGreaterThan(_) => format!("{field}>={PARAM_SYMBOL}"),
					Filter::In(v) => {
						let holders = comma_seperated_binding_holders(v.len());
						format!("{field} IN ({holders})")
					},
					Filter::NotIn(v) => {
						let holders = comma_seperated_binding_holders(v.len());
						format!("{field} NOT IN ({holders})")
					},
					Filter::Between(_, _) => {
						format!("{field} BETWEEN {PARAM_SYMBOL} AND {PARAM_SYMBOL}")
					},
				}
			})
			.collect::<Vec<String>>().join(" AND ")
	}
}

/// returns "?, ?, ?, ..."
fn comma_seperated_binding_holders(len: usize) -> String {
	(0..len)
		.map(|_| PARAM_SYMBOL)
		.collect::<Vec<&str>>()
		.join(", ")
}

impl SqlxHelper for SqlValues<'_> {
	fn with_binding_holder(&self) -> String {
		self.iter()
			.map(|(field, _)| {
				format!("{field}={PARAM_SYMBOL}")
			})
			.collect::<Vec<String>>().join(", ")
	}
}

impl<E> SqlxHelper for SqlUpdates<'_, E> {
	fn with_binding_holder(&self) -> String {
		self.dataset().with_binding_holder()
	}
}


pub trait BindData<'q> {
	fn bind_data<'d: 'q>(self, data: &'d RepoValue<'_>) -> Self;
}
impl<'q, O> BindData<'q> for SqlxQueryAs<'q, O> {
	fn bind_data<'d: 'q>(self, data: &'d RepoValue<'_>) -> Self {
		match data {
			RepoValue::Null => self.bind(None::<&str>),
			RepoValue::Int(n) => self.bind(n),
			RepoValue::UInt(u) => self.bind(u),
			RepoValue::Double(f) => self.bind(f),
			RepoValue::Date(d) => self.bind(d),
			RepoValue::Time(t) => self.bind(t),
			RepoValue::DateTime(dt) => self.bind(dt),
			RepoValue::Str(s) => self.bind(s),
			RepoValue::String(s) => self.bind(s),
		}
	}
}
impl<'q> BindData<'q> for SqlxQuery<'q> {
	fn bind_data<'d: 'q>(self, data: &'d RepoValue<'_>) -> Self {
		match data {
			RepoValue::Null => self.bind(None::<&str>),
			RepoValue::Int(n) => self.bind(n),
			RepoValue::UInt(u) => self.bind(u),
			RepoValue::Double(f) => self.bind(f),
			RepoValue::Date(d) => self.bind(d),
			RepoValue::Time(t) => self.bind(t),
			RepoValue::DateTime(dt) => self.bind(dt),
			RepoValue::Str(s) => self.bind(s),
			RepoValue::String(s) => self.bind(s),
		}
	}
}

pub trait BindFilter<'q> {
	fn bind_filter<'d: 'q>(self, filters: &'d SqlFilter<'_>) -> Self;
}
impl<'q, O> BindFilter<'q> for SqlxQueryAs<'q, O> {
	fn bind_filter<'d: 'q>(self, filters: &'d SqlFilter<'_>) -> Self {
		let mut q = self;
		for f in filters.iter() {
			q = match f.filter() {
				Filter::Equal(data) => q.bind_data(data),
				Filter::Not(data) => q.bind_data(data),
				Filter::LessorThan(data) => q.bind_data(data),
				Filter::EqualOrLessorThan(data) => q.bind_data(data),
				Filter::GreaterThan(data) => q.bind_data(data),
				Filter::EqualOrGreaterThan(data) => q.bind_data(data),
				Filter::In(values) => {
					let mut q = q;
					for value in values.iter() {
						q = q.bind_data(value);
					}
					q
				},
				Filter::NotIn(values) => {
					let mut q = q;
					for value in values.iter() {
						q = q.bind_data(value);
					}
					q
				},
				Filter::Between(from, to) => {
					q = q.bind_data(from);
					q = q.bind_data(to);
					q
				},
			};
		}
		q
	}
}

pub trait BindValues<'q> {
	fn bind_values<'d: 'q, T>(self, values: &'d T) -> Self
	where T: AsRef<SqlValues<'d>>;
}
impl<'q> BindValues<'q> for SqlxQuery<'q> {
	fn bind_values<'d: 'q, T>(self, values: &'d T) -> Self
	where T: AsRef<SqlValues<'d>>
	{
		let mut q = self;
		for (_, data) in values.as_ref().iter() {
			q = q.bind_data(data);
		}
		q
	}
}
