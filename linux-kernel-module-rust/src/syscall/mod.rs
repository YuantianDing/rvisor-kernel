

use {
    crate::{
        bindings,
        KernelResult,
        Error,
    }
};

mod cshim;
use cshim::*;

mod orig;
pub use orig::*;

pub struct ProtFs {
    oldfs: bindings::mm_segment_t,
}

impl ProtFs {
    pub fn prot() -> Self {
        Self{
            oldfs: unsafe{protect_fs()},
        }
    }
}

impl Drop for ProtFs {
    fn drop(&mut self) {
        unsafe{release_fs(self.oldfs)}
    }
}