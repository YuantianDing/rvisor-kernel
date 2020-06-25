
use super::bindings;

mod chim;
use cshim::*;

struct ProtFs {
    oldfs: 
}

impl ProtFs {
    pub fn prot() -> Self {
        Self{}
    }
}

pub fn protect_fs_run<T, F: Fn()->T> (func : F) -> T{
    let  = unsafe{ protect_fs() };
    let ret = func();
    unsafe{ release_fs(oldfs);}
    ret
}