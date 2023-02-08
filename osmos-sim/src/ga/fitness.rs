impl crate::object::Object {
    pub fn fitness(&self) -> isize {
        self.cell.energy as isize
    }
}
