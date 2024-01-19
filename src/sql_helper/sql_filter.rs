use std::fmt::Display;

use crate::{Filter, RepoValue, NamedFilterHolder, NamedFilter};

pub struct SqlFilter<'a>(Vec<NamedFilter<'a>>);

impl<'a> SqlFilter<'a> {
	pub fn new() -> SqlFilter<'a> {
		SqlFilter(Vec::new())
	}

	pub fn with<T: Into<RepoValue<'a>> + Clone>(mut self, field: &'a str, filter: &Filter<T>) -> Self {
		self.0.push(NamedFilter::new(field, filter.map(|v| v.into())));
		self
	}

	pub fn with_named(mut self, filter: &'a NamedFilterHolder<'a>) -> Self {
		if let Some(filter) = filter.to_named_filter() {
			self.0.push(filter);
		}
		self
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn expressions(&self) -> String {
		self.0.iter()
			.map(|f| f.sql_expression())
			.collect::<Vec<String>>()
			.join(", ")
	}

	pub fn iter(&self) -> impl Iterator<Item = &NamedFilter<'a>> {
		self.0.iter()
	}
}

impl Display for SqlFilter<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.expressions())
	}
}
