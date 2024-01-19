#[macro_export]
macro_rules! repo_enum {
	($(#[doc = $enum_doc:expr])* $(#[default = $default:ident])? $name:ident { $( $(#[doc = $value_doc:expr])* $value:ident = $string:literal ),+ $(,)? }) => {
		repo_enum!(@define_enum $(#[doc = $enum_doc])* $name { $( $(#[doc = $value_doc])* $value = $string ),+ });
		repo_enum!(@impl_as_str $name { $( $value = $string ),+ });
		repo_enum!(@impl_from_str $name { $( $value = $string ),+ });
		repo_enum!(@impl_display $name);
		$( repo_enum!(@impl_default #[default = $default] $name); )?
	};

	(@define_enum $(#[doc = $enum_doc:expr])* $name:ident { $( $(#[doc = $value_doc:expr])* $value:ident = $string:literal ),+ $(,)? }) => {
		#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
		$( #[doc = $enum_doc] )*
		pub enum $name {
			$(
				$( #[doc = $value_doc] )*
				$value,
			)+
		}
	};
	(@impl_as_str $name:ident { $( $value:ident = $string:literal ),+ }) => {
		impl $crate::AsStaticStr for $name {
			fn as_str(&self) -> &'static str {
				match *self {
					$( Self::$value => $string, )+
				}
			}
		}
	};
	(@impl_from_str $name:ident { $( $value:ident = $string:literal ),+ }) => {
		impl std::str::FromStr for $name {
			type Err = $crate::FromStrError;

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				match s {
					$( $string => Ok(Self::$value), )+
					_ => Err(format!("Cannot convert {:?} to {}", s, stringify!($name)).into())
				}
			}
		}
	};
	(@impl_display $name:ident) => {
		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", $crate::AsStaticStr::as_str(self))
			}
		}
	};
	(@impl_default #[default = $default:ident] $name:ident) => {
		impl Default for $name {
			fn default() -> Self {
				Self::$default
			}
		}
	}
}
