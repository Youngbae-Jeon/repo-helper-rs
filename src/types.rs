use std::fmt::Debug;

use crate::errors::EntityNotFoundError;

pub trait Entity: Send {
	type Key: Debug + Send + Sync;

	fn get_key(&self) -> Self::Key;
	fn not_found(key: Self::Key) -> EntityNotFoundError<Self::Key>;
}

pub trait AsStaticStr {
	fn as_str(&self) -> &'static str;
}
