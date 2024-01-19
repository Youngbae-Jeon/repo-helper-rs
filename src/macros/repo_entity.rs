#[macro_export]
macro_rules! repo_entity {
	(
		$(#[doc = $doc:expr])*
		$(#[derive($($derive:ident),+)])*
		$(#[repo_filter = $filter:ident])?
		$(#[repo_partial = $partial:ident])?
		struct $name:ident {
			keys {
				$( $(#[doc = $doc_key:expr])* $key:ident : $ty_key:ty ),+ $(,)?
			},
			data $(: $data:ident)? {
				$( $(#[doc = $doc_prop:expr])* $prop:ident : $ty_prop:ty ),+ $(,)?
			} $(,)?
		}
	) => {
		$crate::repo_entity!(@define_struct
			$(#[doc = $doc])*
			$(#[derive($($derive),+)])* 
			struct $name {
				$( $(#[doc = $doc_key])* $key : $ty_key ),+ ,
				$( $(#[doc = $doc_prop])* $prop : $ty_prop ),+
			}
		);

		$crate::repo_entity!(@impl_entity $name
			keys { $( $(#[doc = $doc_key])* $key : $ty_key ),+ }
		);

		$crate::repo_entity!(@define_data_struct
			#[entity = $name]
			keys {
				$( $(#[doc = $doc_key])* $key : $ty_key ),+
			},
			data $(: $data )? {
				$( $(#[doc = $doc_prop])* $prop : $ty_prop ),+
			}
		);

		$crate::repo_entity!(@repo_filter
			$( #[repo_filter = $filter] )?
			#[entity = $name]
			{
				$( $(#[doc = $doc_key])* $key : $ty_key ),+ ,
				$( $(#[doc = $doc_prop])* $prop : $ty_prop ),+
			}
		);

		$crate::repo_entity!(@repo_partial
			$( #[repo_partial = $partial] )?
			#[entity = $name]
			{
				$( $(#[doc = $doc_prop])* $prop : $ty_prop ),+
			}
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
				pub $prop : $ty_prop,
			)+
		}
	};
	(@define_struct struct { $( $(#[doc = $doc_prop:expr])* $prop:ident : $ty_prop:ty ),+ }) => {};

	(@impl_entity $name:ident
		keys { $(#[doc = $doc_key:expr])* $key:ident : $ty_key:ty $(,)? }
	) => {
		impl $crate::Entity for $name {
			type Key = $ty_key;

			fn get_key(&self) -> Self::Key {
				self.$key.clone()
			}

			fn not_found(key: Self::Key) -> $crate::EntityNotFoundError<Self::Key> {
				$crate::EntityNotFoundError::new(stringify!($name), key)
			}
		}
	};
	(@impl_entity $name:ident
		keys { $( $(#[doc = $doc_key:expr])* $key:ident : $ty_key:ty ),+ $(,)? }
	) => {
		impl $crate::Entity for $name {
			type Key = ($($ty_key),+);

			fn get_key(&self) -> Self::Key {
				( $(self.$key.clone()),+ )
			}

			fn not_found(key: Self::Key) -> $crate::EntityNotFoundError<Self::Key> {
				$crate::EntityNotFoundError::new(stringify!($name), key)
			}
		}
	};

	(@define_data_struct
		#[entity = $entity:ident]
		keys {
			$( $(#[doc = $doc_key:expr])* $key:ident : $ty_key:ty ),+
		},
		data: $data:ident {
			$( $(#[doc = $doc_prop:expr])* $prop:ident : $ty_prop:ty ),+
		}
	) => {
		#[derive(Default, Clone, Debug)]
		pub struct $data {
			$(
				$(#[doc = $doc_prop])* 
				pub $prop : $ty_prop,
			)+
		}

		impl $entity {
			pub fn build($($key: $ty_key,)+ data: $data) -> $entity {
				$entity {
					$( $key, )+
					$( $prop : data.$prop, )+
				}
			}
		}

		impl From<$entity> for $data {
			fn from(entity: $entity) -> $data {
				$data {
					$( $prop : entity.$prop, )+
				}
			}
		}

		impl<'a> From<&'a $data> for $crate::SqlValues<'a> {
			fn from(data: &'a $data) -> $crate::SqlValues<'a> {
				$crate::SqlValues::default()
					$(
						.with(stringify!($prop), data.$prop.clone())
					)+
			}
		}
	};
	(@define_data_struct
		#[entity = $entity:ident]
		keys {
			$( $(#[doc = $doc_key:expr])* $key:ident : $ty_key:ty ),+
		},
		data {
			$( $(#[doc = $doc_prop:expr])* $prop:ident : $ty_prop:ty ),+
		}
	) => {
		impl<'a> From<&'a $entity> for $crate::SqlValues<'a> {
			fn from(entity: &'a $entity) -> $crate::SqlValues<'a> {
				$crate::SqlValues::default()
					$(
						.with(stringify!($key), entity.$key.clone())
					)+
					$(
						.with(stringify!($prop), entity.$prop.clone())
					)+
			}
		}
	};

	(@repo_filter
		#[repo_filter = $filter:ident]
		#[entity = $name:ident]
		{
			$( $(#[doc = $doc_prop:expr])* $prop:ident : $ty_prop:ty ),+ $(,)?
		}
	) => {
		$crate::repo_filter!(
			#[derive(Clone)]
			#[entity = $name]
			struct $filter<'a> {
				$( $(#[doc = $doc_prop])* $prop : $ty_prop ),+
			}
		);
	};
	(@repo_filter #[entity = $name:ident] $body:tt) => {};

	(@repo_partial
		#[repo_partial = $partial:ident]
		#[entity = $name:ident]
		{
			$( $(#[doc = $doc_prop:expr])* $prop:ident : $ty_prop:ty ),+ $(,)?
		}
	) => {
		$crate::repo_data_partial!(
			#[derive(Debug)]
			#[entity = $name]
			struct $partial {
				$( $(#[doc = $doc_prop])* $prop : $ty_prop ),+
			}
		);
	};
	(@repo_partial #[entity = $name:ident] $body:tt) => {};
}
