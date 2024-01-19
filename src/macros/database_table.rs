#[macro_export]
macro_rules! database_table {
	(#[table_name = $table:literal] $ent:ident { $($name:ident : $fty:ty),+ $(,)? }) => {
		database_table!(@define_entity_struct $ent { $($name : $fty),+ });
		database_table!(@impl_database_table $ent, $table { $($name),+ });
	};
	(#[table_name = $table:literal, derive(TryInto<$ty:tt>)] $ent:ident { $($name:ident : $fty:ty),+ $(,)? }) => {
		database_table!(#[table_name = $table] $ent { $($name : $fty),+ });
		database_table!(@impl_try_from $ty, $ent { $($name),+ });
	};

	(@define_entity_struct $ent:ident { $($name:ident : $fty:ty),+ $(,)? }) => {
		#[derive(mysql_async::prelude::FromRow)]
		pub struct $ent {
			$(pub $name : $fty,)+
		}
	};

	(@impl_database_table $ent:ident, $table:literal { $($name:ident),+ }) => {
		impl $ent {
			const TABLE_NAME: &'static str = $table;
			const TABLE_FIELDS: &'static str = stringify!($($name),+);
		}
	};

	(@impl_try_from $ty:tt, $ent:ident { $($name:ident),+ $(,)? }) => {
		impl TryFrom<$ent> for $ty {
			type Error = common::errors::FromStrError;

			fn try_from(value: $ent) -> Result<Self, Self::Error> {
				Ok($ty {
					$( $name: value.$name, )+
				})
			}
		}
	};
}
