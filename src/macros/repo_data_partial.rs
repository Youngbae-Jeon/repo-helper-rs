#[macro_export]
macro_rules! repo_data_partial {
	(
		$(#[doc = $doc:expr])*
		$(#[derive($($derive:ident),+)])* 
		$(#[entity = $entity:ty])?
		struct $name:ident {
			$( $(#[doc = $doc_prop:expr])* $prop:ident : $ty_prop:ty ),+ $(,)?
		}
	) => {
		$crate::repo_data_partial!(@define_struct
			$(#[doc = $doc])*
			$(#[derive($($derive),+)])* 
			struct $name {
				$(
					$(#[doc = $doc_prop])*
					$prop : $ty_prop
				),+
			}
		);
		$crate::repo_data_partial!(@impl_struct $name {
			$(
				$(#[doc = $doc_prop])*
				$prop : $ty_prop
			),+
		});
		$crate::repo_data_partial!(@impl_default $name { $( $prop ),+ });
		$crate::repo_data_partial!(@impl_into_sql_updates
			$(#[entity = $entity])?
			$name { $( $prop : $ty_prop ),+ }
		);
	};

	(@define_struct
		$(#[doc = $doc:expr])*
		$(#[derive($($derive:ident),+)])* 
		struct $name:ident {
			$( $(#[doc = $doc_prop:expr])* $prop:ident : $ty_prop:ty ),+
		}
	) => {
		$(#[doc = $doc])* 
		$(#[derive($($derive),+)])* 
		pub struct $name {
			$(
				$(#[doc = $doc_prop])* 
				pub $prop : $crate::Definable<$ty_prop>,
			)+
		}
	};

	(@impl_struct $name:ident { $( $(#[doc = $doc_prop:expr])* $prop:ident : $ty_prop:ty ),+ }) => {
		impl $name {
			$(
				$(#[doc = $doc_prop])* 
				pub fn $prop(mut self, value: $ty_prop) -> Self {
					self.$prop.set(value);
					self
				}
			)+

			pub fn is_empty(&self) -> bool {
				$(
					if self.$prop.is_defined() {
						return false;
					}
				)+
				true
			}
		}
	};

	(@impl_default $name:ident { $( $prop:ident ),+ }) => {
		impl Default for $name {
			fn default() -> Self {
				Self {
					$(
						$prop : $crate::Definable::default(),
					)+
				}
			}
		}
	};

	(@impl_into_sql_updates #[entity = $entity:ty] $name:ident { $( $prop:ident : $ty_prop:ty ),+ }) => {
		impl $entity {
			pub fn partial() -> $name {
				$name::default()
			}
		}

		impl From<$name> for $crate::SqlUpdates<'static, $entity> {
			fn from(mut d: $name) -> $crate::SqlUpdates<'static, $entity> {
				let mut updates = $crate::SqlUpdates::<$entity>::default();
				$(
					if let Some(value) = d.$prop.take() {
						updates.push(stringify!($prop), value.clone(), move |a| { a.$prop = value; });
					}
				)+
				updates
			}
		}
	};
	(@impl_into_sql_updates $name:ident $body:tt) => {}
}
