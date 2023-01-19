use crate::{
    log_syscall_entry,
    prelude::*,
    process::clone::{clone_child, CloneArgs},
};
use jinux_frame::cpu::CpuContext;

use crate::syscall::SYS_FORK;

use super::SyscallReturn;

pub fn sys_fork(parent_context: CpuContext) -> Result<SyscallReturn> {
    log_syscall_entry!(SYS_FORK);
    let current = current!();
    // FIXME: set correct args for fork
    let clone_args = CloneArgs::default();
    let child_pid = clone_child(parent_context, clone_args).unwrap();
    Ok(SyscallReturn::Return(child_pid as _))
}
