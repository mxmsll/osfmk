pub enum ThreadState {
    New,
    Ready,
    Running,
    Blocked,
    Dead,
}

pub trait Thread {
    fn current_state(&self) -> ThreadState;
}
