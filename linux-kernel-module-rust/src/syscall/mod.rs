
use super::bindings;

mod cshim;
use cshim::*;

struct ProtFs {
    oldfs: bindings::mm_segment_t,
}

impl ProtFs {
    pub fn prot() -> Self {
        Self{
            oldfs: protect_fs(),
        }
    }
}

pub fn protect_fs_run<T, F: Fn()->T> (func : F) -> T{
    let  = unsafe{ protect_fs() };
    let ret = func();
    unsafe{ release_fs(oldfs);}
    ret
}