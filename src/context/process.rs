use core::sync::atomic::AtomicUsize;

int_like!(ProcessId, AtomicProcessId, usize, AtomicUsize);

#[derive(Debug)]
pub struct Process {}

#[derive(Debug, Clone, Copy, Default)]
pub struct ProcessInfo {
    /// 进程ID
    pub pid: ProcessId,
    /// 进程组ID
    pub pgid: ProcessId,
    /// 父进程ID
    pub ppid: ProcessId,
}
