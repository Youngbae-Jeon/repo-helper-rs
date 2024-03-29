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
	pub fn is_defined_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
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
	pub fn is_some_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
		self.is_defined_and(f)
	}

	pub fn map<U, F>(self, f: F) -> Definable<U>
	where F: FnOnce(T) -> U {
		match self {
			Self::Defined(x) => Definable::Defined(f(x)),
			Self::Undefined => Definable::Undefined,
		}
	}

	pub const fn as_option(&self) -> Option<&T> {
		match *self {
			Self::Defined(ref x) => Some(x),
			Self::Undefined => None,
		}
	}

	pub const fn as_ref(&self) -> Definable<&T> {
		match *self {
			Self::Defined(ref x) => Definable::Defined(x),
			Self::Undefined => Definable::Undefined,
		}
	}
	pub fn as_mut(&mut self) -> Definable<&mut T> {
		match *self {
			Self::Defined(ref mut x) => Definable::Defined(x),
			Self::Undefined => Definable::Undefined,
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

impl<T: Copy> Copy for Definable<T> {}

impl<T: Copy> Definable<T> {
	pub fn get(&self) -> Option<T> {
		match *self {
			Definable::Defined(value) => Some(value),
			Definable::Undefined => None,
		}
	}
}
