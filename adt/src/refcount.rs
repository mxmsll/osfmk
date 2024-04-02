pub trait RefCountable {
    fn add_ref_count(&self) -> usize;
    fn release(&self) -> usize;
}
