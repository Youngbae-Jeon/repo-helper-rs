use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EntityNotFoundError<Key: Debug>(&'static str, Key);

impl<Key: Debug> EntityNotFoundError<Key> {
	pub fn new(entity_name: &'static str, key: Key) -> Self {
		Self(entity_name, key)
	}
}
impl<Key: Debug> Display for EntityNotFoundError<Key> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Not found {} {{id={:?}}})", self.0, self.1)
	}
}
impl<Key: Debug> std::error::Error for EntityNotFoundError<Key> {}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UnexpectedAffectedRowsError(u64, u64);
impl UnexpectedAffectedRowsError {
	pub fn new(expected: u64, actual: u64) -> Self {
		Self(expected, actual)
	}
}
impl Display for UnexpectedAffectedRowsError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "UnexpectedAffectedRowsError(expected:{}, actual:{})", self.0, self.1)
	}
}
impl std::error::Error for UnexpectedAffectedRowsError {}


#[derive(Debug)]
pub struct FromStrError {
	pub message: String,
}
impl From<&str> for FromStrError {
	fn from(message: &str) -> Self {
		Self { message: String::from(message) }
	}
}
impl From<String> for FromStrError {
	fn from(message: String) -> Self {
		Self { message }
	}
}
impl Display for FromStrError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}
impl std::error::Error for FromStrError {} 
