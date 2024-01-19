pub struct InsertResult(pub u64);
impl InsertResult {
	pub fn last_insert_id(&self) -> u64 {
		self.0
	}
}

pub struct UpdateResult(pub u64);
impl UpdateResult {
	pub fn affected_rows(&self) -> u64 {
		self.0
	}

}
