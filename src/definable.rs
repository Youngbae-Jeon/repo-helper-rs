#[derive(Debug, PartialEq, Clone)]
pub enum Definable<T> {
	Defined(T),
	Undefined,
}

impl<T> Default for Definable<T> {
	fn default() -> Self {
		Self::Undefined
	}
}

impl<T> Definable<T> {
	pub const fn is_undefined(&self) -> bool {
		matches!(self, Definable::Undefined)
	}
	pub const fn is_defined(&self) -> bool {
		matches!(self, Definable::Defined(_))
	}
	pub fn is_defined_and(self, f: impl FnOnce(T) -> bool) -> bool {
		match self {
			Self::Undefined => false,
			Self::Defined(x) => f(x),
		}
	}

	pub const fn is_none(&self) -> bool {
		self.is_undefined()
	}
	pub const fn is_some(&self) -> bool {
		self.is_defined()
	}
	pub fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool {
		self.is_defined_and(f)
	}

	pub const fn as_ref(&self) -> Option<&T> {
		match *self {
			Self::Defined(ref x) => Some(x),
			Self::Undefined => None,
		}
	}

	pub fn set(&mut self, value: T) {
		*self = Definable::Defined(value);
	}

	pub fn take(&mut self) -> Option<T> {
		match std::mem::take(self) {
			Definable::Defined(value) => Some(value),
			Definable::Undefined => None,
		}
	}
}

impl<T: Copy> Definable<T> {
	pub fn get(&self) -> Option<T> {
		match *self {
			Definable::Defined(value) => Some(value),
			Definable::Undefined => None,
		}
	}
}
