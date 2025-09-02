use std::fmt::Display;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use crate::types::AsStaticStr;

#[derive(Debug, Clone, PartialEq)]
pub enum RepoValue<'a> {
	Null,
	Int(i64),
	UInt(u64),
	Float(f32),
	Double(f64),
	Date(NaiveDate),
	Time(NaiveTime),
	DateTime(NaiveDateTime),
	Str(&'a str),
	String(String),
	Bytes(Vec<u8>),
}

impl Display for RepoValue<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			RepoValue::Null => write!(f, "NULL"),
			RepoValue::Int(v) => v.fmt(f),
			RepoValue::UInt(v) => v.fmt(f),
			RepoValue::Float(v) => v.fmt(f),
			RepoValue::Double(v) => v.fmt(f),
			RepoValue::Date(v) => write!(f, "'{}'", v),
			RepoValue::Time(v) => write!(f, "'{}'", v),
			RepoValue::DateTime(v) => write!(f, "'{}'", v),
			RepoValue::Str(v) => write!(f, "'{}'", v),
			RepoValue::String(v) => write!(f, "'{}'", v),
			RepoValue::Bytes(v) => {
				write!(f, "0x")?;
				for byte in v {
					write!(f, "{:02x}", byte)?;
				}
				Ok(())
			}
		}
	}
}

impl From<bool> for RepoValue<'_> {
	fn from(value: bool) -> Self {
		RepoValue::Int(if value { 1 } else { 0 })
	}
}
impl From<i32> for RepoValue<'_> {
	fn from(value: i32) -> Self {
		RepoValue::Int(i64::from(value))
	}
}
impl From<i64> for RepoValue<'_> {
	fn from(value: i64) -> Self {
		RepoValue::Int(value)
	}
}
impl From<u8> for RepoValue<'_> {
	fn from(value: u8) -> Self {
		RepoValue::UInt(u64::from(value))
	}
}
impl From<u16> for RepoValue<'_> {
	fn from(value: u16) -> Self {
		RepoValue::UInt(u64::from(value))
	}
}
impl From<u32> for RepoValue<'_> {
	fn from(value: u32) -> Self {
		RepoValue::UInt(u64::from(value))
	}
}
impl From<u64> for RepoValue<'_> {
	fn from(value: u64) -> Self {
		RepoValue::UInt(value)
	}
}

impl From<f32> for RepoValue<'_> {
	fn from(value: f32) -> Self {
		RepoValue::Float(value)
	}
}
impl From<f64> for RepoValue<'_> {
	fn from(value: f64) -> Self {
		RepoValue::Double(value)
	}
}
impl From<NaiveDate> for RepoValue<'_> {
	fn from(value: NaiveDate) -> Self {
		RepoValue::Date(value)
	}
}
impl From<NaiveTime> for RepoValue<'_> {
	fn from(value: NaiveTime) -> Self {
		RepoValue::Time(value)
	}
}
impl From<NaiveDateTime> for RepoValue<'_> {
	fn from(value: NaiveDateTime) -> Self {
		RepoValue::DateTime(value)
	}
}
impl<'a> From<&'a str> for RepoValue<'a> {
	fn from(value: &'a str) -> Self {
		RepoValue::Str(value)
	}
}
impl<'a> From<String> for RepoValue<'a> {
	fn from(value: String) -> Self {
		RepoValue::String(value)
	}
}
impl<'a> From<&'a String> for RepoValue<'a> {
	fn from(value: &'a String) -> Self {
		RepoValue::Str(value)
	}
}

impl<'a> From<Vec<u8>> for RepoValue<'a> {
	fn from(value: Vec<u8>) -> Self {
		RepoValue::Bytes(value)
	}
}

impl<'a> From<&'a [u8]> for RepoValue<'a> {
	fn from(value: &'a [u8]) -> Self {
		RepoValue::Bytes(value.to_vec())
	}
}

impl<'a, T> From<T> for RepoValue<'a>
where T: AsStaticStr {
	fn from(value: T) -> Self {
		RepoValue::Str(value.as_str())
	}
}

impl<'a, T> From<Option<T>> for RepoValue<'a>
where T: Into<RepoValue<'a>> {
	fn from(value_opt: Option<T>) -> Self {
		match value_opt {
			Some(value) => value.into(),
			None => RepoValue::Null
		}
	}
}
