use super::RepoValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Filter<T> {
	Equal(T),
	Not(T),
	LessorThan(T),
	EqualOrLessorThan(T),
	GreaterThan(T),
	EqualOrGreaterThan(T),
	In(Vec<T>),
	NotIn(Vec<T>),
	Between(T, T),
}

impl<T: Clone> Filter<T> {
	pub fn map<F, R:Clone>(&self, f: F) -> Filter<R>
	where F: Fn(T) -> R {
		match self {
			Self::Equal(value) => Filter::Equal(f(value.clone())),
			Self::Not(value) => Filter::Not(f(value.clone())),
			Self::LessorThan(value) => Filter::LessorThan(f(value.clone())),
			Self::EqualOrLessorThan(value) => Filter::EqualOrLessorThan(f(value.clone())),
			Self::GreaterThan(value) => Filter::GreaterThan(f(value.clone())),
			Self::EqualOrGreaterThan(value) => Filter::EqualOrGreaterThan(f(value.clone())),
			Self::In(v) => Filter::In(v.iter().map(|v| f(v.clone())).collect()),
			Self::NotIn(v) => Filter::NotIn(v.iter().map(|v| f(v.clone())).collect()),
			Self::Between(v1, v2) => Filter::Between(f(v1.clone()), f(v2.clone())),
		}
	}
}

#[derive(Clone)]
pub struct NamedFilter<'a>(&'a str, Filter<RepoValue<'a>>);

impl<'a> NamedFilter<'a> {
	pub fn new(name: &'a str, filter: Filter<RepoValue<'a>>) -> NamedFilter<'a> {
		NamedFilter(name, filter)
	}

	pub fn name(&'a self) -> &'a str {
		self.0
	}

	pub fn filter(&'a self) -> &Filter<RepoValue<'a>> {
		&self.1
	}

	pub fn sql_expression(&self) -> String {
		let field = self.name();
		match self.filter() {
			Filter::Equal(value) => format!("{field}={value}"),
			Filter::Not(value) => format!("{field}<>{value}"),
			Filter::LessorThan(value) => format!("{field}<{value}"),
			Filter::EqualOrLessorThan(value) => format!("{field}<={value}"),
			Filter::GreaterThan(value) => format!("{field}>{value}"),
			Filter::EqualOrGreaterThan(value) => format!("{field}>={value}"),
			Filter::In(v) => {
				let expr = Self::expression_of_values(v);
				format!("{field} IN ({expr})")
			},
			Filter::NotIn(v) => {
				let expr = Self::expression_of_values(v);
				format!("{field} NOT IN ({expr})")
			},
			Filter::Between(v1, v2) => format!("{field} BETWEEN {v1} AND {v2}"),
		}
	}

	fn expression_of_values(v: &[RepoValue<'_>]) -> String {
		v.iter()
			.map(|v| format!("{}", v))
			.collect::<Vec<String>>()
			.join(",")
	}
}

#[derive(Clone)]
pub struct NamedFilterHolder<'a>(&'a str, Option<Filter<RepoValue<'a>>>);

impl<'a> NamedFilterHolder<'a> {
	pub fn new(name: &'a str) -> NamedFilterHolder<'a> {
		NamedFilterHolder(name, None)
	}

	pub fn name(&'a self) -> &'a str {
		self.0
	}

	pub fn filter(&'a self) -> Option<&'a Filter<RepoValue<'a>>> {
		self.1.as_ref()
	}

	pub fn is_some(&self) -> bool {
		self.1.is_some()
	}

	pub fn is_none(&self) -> bool {
		self.1.is_none()
	}

	pub fn eq<T: Into<RepoValue<'a>>>(&mut self, value: T) {
		self.1 = Some(Filter::Equal(value.into()));
	}

	pub fn lt<T: Into<RepoValue<'a>>>(&mut self, value: T) {
		self.1 = Some(Filter::LessorThan(value.into()));
	}

	pub fn elt<T: Into<RepoValue<'a>>>(&mut self, value: T) {
		self.1 = Some(Filter::EqualOrLessorThan(value.into()));
	}

	pub fn gt<T: Into<RepoValue<'a>>>(&mut self, value: T) {
		self.1 = Some(Filter::GreaterThan(value.into()));
	}

	pub fn egt<T: Into<RepoValue<'a>>>(&mut self, value: T) {
		self.1 = Some(Filter::EqualOrGreaterThan(value.into()));
	}

	pub fn included_in<T: Into<RepoValue<'a>> + Clone>(&mut self, values: Vec::<T>) {
		let repo_values = values.iter().map(|v| v.clone().into()).collect();
		self.1 = Some(Filter::In(repo_values));
	}

	pub fn excluded_from<T: Into<RepoValue<'a>> + Clone>(&mut self, values: Vec::<T>) {
		let repo_values = values.iter().map(|v| v.clone().into()).collect();
		self.1 = Some(Filter::NotIn(repo_values));
	}

	pub fn between<T: Into<RepoValue<'a>> + Clone>(&mut self, value1: T, value2: T) {
		self.1 = Some(Filter::Between(value1.into(), value2.into()));
	}

	pub fn to_named_filter(&'a self) -> Option<NamedFilter<'a>> {
		self.filter()
			.map(|f| NamedFilter::new(self.name(), f.clone()))
	}
}
