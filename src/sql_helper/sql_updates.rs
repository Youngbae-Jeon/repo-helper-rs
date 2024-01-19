use std::fmt::Display;

use crate::RepoValue;
use super::SqlValues;


type BoxFnApply<'a, E> = Box<dyn FnOnce(&mut E) + Send + Sync + 'a>;

#[derive(Default)]
pub struct SqlUpdates<'a, E> {
	data: SqlValues<'a>,
	appliables: Vec<BoxFnApply<'a, E>>
}

impl<'a, E> SqlUpdates<'a, E> {
	pub fn push<T: Into<RepoValue<'a>> + Clone + 'a, F>(&mut self, field: &'a str, data: T, f: F)
	where F: FnOnce(&mut E) + Send + Sync + 'a {
		self.data.push(field, data);
		self.appliables.push(Box::new(f));
	}

	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}

	pub fn expressions(&self) -> String {
		self.data.expressions()
	}

	pub fn apply(self, target: &'a mut E) {
		for f in self.appliables.into_iter() {
			f(target)
		}
	}

	pub fn dataset(&self) -> &SqlValues<'a> {
		&self.data
	}
}

impl<E> Display for SqlUpdates<'_, E> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.expressions())
	}
}

impl<'a, E> AsRef<SqlValues<'a>> for SqlUpdates<'a, E> {
	fn as_ref(&self) -> &SqlValues<'a> {
		&self.data
	}
}
