use mysql_async::Value;

use crate::{Filter, SqlFilter, SqlValues, SqlUpdates, RepoValue};

trait FromRepoValue {
	fn from_repo_value(value: &'_ RepoValue<'_>) -> Self;
}
impl FromRepoValue for Value {
	fn from_repo_value(value: &'_ RepoValue<'_>) -> Self {
		match value {
			RepoValue::Null => Value::NULL,
			RepoValue::Int(v) => Value::from(v),
			RepoValue::UInt(v) => Value::from(v),
			RepoValue::Float(v) => Value::from(v),
			RepoValue::Double(v) => Value::from(v),
			RepoValue::Date(v) => Value::from(v),
			RepoValue::Time(v) => Value::from(v),
			RepoValue::DateTime(v) => Value::from(v),
			RepoValue::Str(v) => Value::from(v),
			RepoValue::String(v) => Value::from(v),
			RepoValue::Bytes(v) => Value::from(v),
		}
	}
}

pub trait MySqlHelper {
	fn params(&self) -> Vec<(Vec<u8>, Value)>;
	fn with_named_binding_holder(&self) -> String;
}

impl MySqlHelper for SqlFilter<'_> {
	fn params(&self) -> Vec<(Vec<u8>, Value)> {
		let mut params = Vec::<(Vec<u8>, Value)>::new();
		for f in self.iter() {
			let field = f.name();
			let filter = f.filter();
			match filter {
				Filter::Equal(value) => {
					params.push((Vec::<u8>::from(field), Value::from_repo_value(value)));
				},
				Filter::Not(value) => {
					params.push((Vec::<u8>::from(field), Value::from_repo_value(value)));
				},
				Filter::LessorThan(value) => {
					params.push((Vec::<u8>::from(field), Value::from_repo_value(value)));
				},
				Filter::EqualOrLessorThan(value) => {
					params.push((Vec::<u8>::from(field), Value::from_repo_value(value)));
				},
				Filter::GreaterThan(value) => {
					params.push((Vec::<u8>::from(field), Value::from_repo_value(value)));
				},
				Filter::EqualOrGreaterThan(value) => {
					params.push((Vec::<u8>::from(field), Value::from_repo_value(value)));
				},
				Filter::In(v) => {
					for (i, value) in v.iter().enumerate() {
						params.push((Vec::<u8>::from(format!("{}_in_{}", field, i)), Value::from_repo_value(value)));
					}
				},
				Filter::NotIn(v) => {
					for (i, value) in v.iter().enumerate() {
						params.push((Vec::<u8>::from(format!("{}_not_in_{}", field, i)), Value::from_repo_value(value)));
					}
				},
				Filter::Between(v1, v2) => {
					params.push((Vec::<u8>::from(format!("{}_between_0", field)), Value::from_repo_value(v1)));
					params.push((Vec::<u8>::from(format!("{}_between_1", field)), Value::from_repo_value(v2)));
				},
			}
		}
		params
	}

	fn with_named_binding_holder(&self) -> String {
		self.iter()
			.map(|f| {
				let field = f.name();
				match f.filter() {
					Filter::Equal(_) => format!("{field}=:{field}"),
					Filter::Not(_) => format!("{field}<>:{field}"),
					Filter::LessorThan(_) => format!("{field}<:{field}"),
					Filter::EqualOrLessorThan(_) => format!("{field}<=:{field}"),
					Filter::GreaterThan(_) => format!("{field}>:{field}"),
					Filter::EqualOrGreaterThan(_) => format!("{field}>=:{field}"),
					Filter::In(v) => {
						let prefix = format!("{}_in", field);
						let holders = comma_seperated_named_binding_holders(v.len(), &prefix);
						format!("{field} IN ({holders})")
					},
					Filter::NotIn(v) => {
						let prefix = format!("{}_not_in", field);
						let holders = comma_seperated_named_binding_holders(v.len(), &prefix);
						format!("{field} NOT IN ({holders})")
					},
					Filter::Between(_, _) => {
						format!("{field} BETWEEN :{field}_between_0 AND :{field}_between_1")
					},
				}
			})
			.collect::<Vec<String>>().join(" AND ")
	}
}

/// returns ":{prefix}_0, :{prefix}_1, :{prefix}_2, ..."
fn comma_seperated_named_binding_holders(len: usize, prefix: &str) -> String {
	(0..len)
		.map(|i| format!(":{prefix}_{}", i))
		.collect::<Vec<String>>()
		.join(", ")
}

impl MySqlHelper for SqlValues<'_> {
	fn params(&self) -> Vec<(Vec<u8>, Value)> {
		self.iter()
			.map(|(field, data)| {
				(Vec::<u8>::from(*field), Value::from_repo_value(data))
			})
			.collect::<Vec<(Vec<u8>, Value)>>()
	}

	fn with_named_binding_holder(&self) -> String {
		self.iter()
			.map(|(field, _)| {
				format!("{field}=:{field}")
			})
			.collect::<Vec<String>>().join(", ")
	}
}

impl<E> MySqlHelper for SqlUpdates<'_, E> {
	fn params(&self) -> Vec<(Vec<u8>, Value)> {
		self.dataset().params()
	}

	fn with_named_binding_holder(&self) -> String {
		self.dataset().with_named_binding_holder()
	}
}
