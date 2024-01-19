use std::fmt::Display;

use crate::RepoValue;

type Pair<'a> = (&'a str, RepoValue<'a>);

#[derive(Default)]
pub struct SqlValues<'a>(Vec<Pair<'a>>);

impl<'a> SqlValues<'a> {
	pub fn with<T: Into<RepoValue<'a>> + Clone>(mut self, field: &'a str, data: T) -> Self {
		self.0.push((field, data.into()));
		self
	}

	pub fn push<T: Into<RepoValue<'a>> + Clone>(&mut self, field: &'a str, data: T) {
		self.0.push((field, data.into()));
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn expressions(&self) -> String {
		self.0.iter()
			.map(|(field, data)| {
				format!("{field}={data}")
			})
			.collect::<Vec<String>>()
			.join(", ")
	}

	pub fn iter(&self) -> impl Iterator<Item = &Pair<'a>> {
		self.0.iter()
	}
}

impl Display for SqlValues<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.expressions())
	}
}

impl<'a> AsRef<SqlValues<'a>> for SqlValues<'a> {
	fn as_ref(&self) -> &SqlValues<'a> {
		self
	}
}
