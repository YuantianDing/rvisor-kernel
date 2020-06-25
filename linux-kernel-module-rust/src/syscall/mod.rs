
use super::bindings;

struct ProtFs {
    
}

impl ProtFs {
    pub fn prot() -> Self {
        Self{}
    }
}

pub fn protect_fs_run<T, F: Fn()->T> (func : F) -> T{
    let oldfs = unsafe{bindings::protect_fs()};
    let ret = func();
    unsafe{ bindings::release_fs(oldfs);}
    ret
}