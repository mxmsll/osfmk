use crate::task::Task;

pub enum ThreadState {
    New,
    Ready,
    Running,
    Blocked,
    Dead,
}

pub trait Thread: Task {
    fn current_state(&self) -> ThreadState;
}
