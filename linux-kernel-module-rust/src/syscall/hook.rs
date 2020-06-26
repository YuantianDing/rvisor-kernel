//! 系统调用挂钩的模块
//! 

use crate::bindings;
use crate::c_types::*;
use crate::println;

pub mod syscall;

extern "C" { 
    /// init syscall replacer (find where the syscall is)
    fn replace_init() -> c_int; 
    /// replace the syscall (here we replace open for test)
    fn replace_syscall(sysnum : c_uint, f_ptr : *const()) -> c_int;
    /// recover the replace
    fn replace_clear() -> c_int;

    pub fn user_max() -> u64;
}
/// replace_syscall 的安全包装
pub fn safe_replace_syscall(sysnum : c_uint, f_ptr : *const()) {
    unsafe{
        let i = replace_syscall(sysnum, f_ptr);
        if i == -1 {
            panic!("replace_syscall failed!");
        }
    }
}

/// init的时候调用
pub fn init() {
    unsafe{
        if replace_init() == -1 {
            panic!("replace_init failed!");
        }
    }
}

/// 退出的时候调用
pub fn cleanup() {
    unsafe{
        if replace_clear() == -1 {
            panic!("replace_clear failed!");
        }
    }
}

