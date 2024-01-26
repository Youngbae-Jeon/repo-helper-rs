#[macro_export]
macro_rules! repo_filter {
	(
		$(#[doc = $doc:expr])*
		$(#[derive($($derive:ident),+)])* 
		$(#[entity = $entity:ty])?
		struct $name:ident<$life:lifetime> {
			$( $(#[doc = $doc_prop:expr])* $prop:ident : $($ty_life:lifetime)? $ty_prop:ty ),+ $(,)?
		}
	) => {
		$crate::repo_filter!(@define_struct
			$(#[doc = $doc])*
			$(#[derive($($derive),+)])* 
			struct $name<$life> {
				$(
					$(#[doc = $doc_prop])*
					$prop : $($ty_life)? $ty_prop
				),+
			}
		);
		$crate::repo_filter!(@impl_struct $name {
			$(
				$(#[doc = $doc_prop])*
				$prop : $($ty_life)? $ty_prop
			),+
		});
		$crate::repo_filter!(@impl_default $name { $( $prop ),+ });
		$crate::repo_filter!(@impl_into_sql_filter
			$(#[entity = $entity])?
			$name { $( $prop ),+ }
		);
	};

	(@define_struct
		$(#[doc = $doc:expr])*
		$(#[derive($($derive:ident),+)])* 
		struct $name:ident<$life:lifetime> {
			$( $(#[doc = $doc_prop:expr])* $prop:ident : $($ty_life:lifetime)? $ty_prop:ty ),+
		}
	) => {
		$(#[doc = $doc])* 
		$(#[derive($($derive),+)])* 
		pub struct $name<$life> {
			$(
				$(#[doc = $doc_prop])* 
				pub $prop : $crate::NamedFilterHolder<$life>,
			)+
			pub limit: Option<usize>,
			pub offset: Option<usize>,
		}
	};

	(@impl_struct
		$name:ident {
			$( $(#[doc = $doc_prop:expr])* $prop:ident : $($ty_life:lifetime)? $ty_prop:ty ),+
		}
	) => {
		impl<'a> $name<'a> {
			$(
				$(#[doc = $doc_prop])* 
				pub fn $prop(mut self, value: $ty_prop) -> Self {
					self.$prop.eq(value);
					self
				}

				paste::paste! {
					pub fn [< $prop _lt>](mut self, value: $ty_prop) -> Self {
						self.$prop.lt(value);
						self
					}
					pub fn [< $prop _elt>](mut self, value: $ty_prop) -> Self {
						self.$prop.elt(value);
						self
					}
					pub fn [< $prop _gt>](mut self, value: $ty_prop) -> Self {
						self.$prop.gt(value);
						self
					}
					pub fn [< $prop _egt>](mut self, value: $ty_prop) -> Self {
						self.$prop.egt(value);
						self
					}
					pub fn [< $prop _not>](mut self, value: $ty_prop) -> Self {
						self.$prop.not(value);
						self
					}
					pub fn [< $prop _in>](mut self, values: Vec::<$ty_prop>) -> Self {
						self.$prop.included_in(values);
						self
					}
					pub fn [< $prop _not_in>](mut self, values: Vec::<$ty_prop>) -> Self {
						self.$prop.excluded_from(values);
						self
					}
					pub fn [< $prop _between>](mut self, value1: $ty_prop, value2: $ty_prop) -> Self {
						self.$prop.between(value1, value2);
						self
					}
				}
			)+

			pub fn limit(mut self, limit: Option<usize>) -> Self {
				self.limit = limit;
				self
			}

			pub fn offset(mut self, offset: Option<usize>) -> Self {
				self.offset = offset;
				self
			}
		}
	};

	(@impl_default $name:ident { $( $prop:ident ),+ }
	) => {
		impl Default for $name<'_> {
			fn default() -> Self {
				Self {
					$(
						$prop : $crate::NamedFilterHolder::new(stringify!($prop)),
					)+
					limit: None,
					offset: None,
				}
			}
		}
	};

	(@impl_into_sql_filter $(#[entity = $entity:ty])?  $name:ident { $( $prop:ident ),+ }) => {
		impl<'a> From<&'a $name<'a>> for $crate::SqlFilter<'a> {
			fn from(filter: &'a $name<'a>) -> $crate::SqlFilter<'a> {
				$crate::SqlFilter::default()
					$(
						.with_named(&filter.$prop)
					)+
			}
		}

		$crate::repo_filter!(@impl_entity_shortcut $(#[entity = $entity])? $name);
	};

	(@impl_entity_shortcut #[entity = $entity:ty] $name:ident) => {
		impl $entity {
			pub fn filter() -> $name<'static> {
				$name::default()
			}
		}
	};
	(@impl_entity_shortcut $name:ident) => {};
}
