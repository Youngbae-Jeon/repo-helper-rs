#[macro_export]
macro_rules! database_table {
	(#[table_name = $table:literal, derive($derive_ty:tt, TryInto<$into_ty:tt>)] $ent:ident { $($name:ident : $fty:ty),+ $(,)? }) => {
		database_table!(#[table_name = $table, derive($derive_ty)] $ent { $($name : $fty),+ });
		database_table!(@impl_try_from $into_ty, $ent { $($name),+ });
	};
	(#[table_name = $table:literal, derive($derive_ty:tt)] $ent:ident { $($name:ident : $fty:ty),+ $(,)? }) => {
		database_table!(@define_entity_struct derive($derive_ty) $ent { $($name : $fty),+ });
		database_table!(@impl_database_table $ent, $table { $($name),+ });
	};

	(@define_entity_struct derive($derive_ty:tt) $ent:ident { $($name:ident : $fty:ty),+ $(,)? }) => {
		#[derive($derive_ty)]
		pub struct $ent {
			$(pub $name : $fty,)+
		}
	};


	(@impl_database_table $ent:ident, $table:literal { $($name:ident),+ }) => {
		impl $ent {
			pub const TABLE_NAME: &'static str = $table;
			pub const TABLE_FIELDS: &'static str = stringify!($($name),+);
		}
	};

	(@impl_try_from $ty:tt, $ent:ident { $($name:ident),+ $(,)? }) => {
		impl TryFrom<$ent> for $ty {
			type Error = $crate::FromStrError;

			fn try_from(value: $ent) -> Result<Self, Self::Error> {
				Ok($ty {
					$( $name: value.$name, )+
				})
			}
		}
	};
}
