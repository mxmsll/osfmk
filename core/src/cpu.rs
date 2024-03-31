use core::ptr::NonNull;

pub trait Processor {}

pub struct PerCpuData<T> {
    ptr: NonNull<T>,
}
