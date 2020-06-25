
use {
    crate::{
        bindings,
        KernelResult,
        Error,
    }
}
mod cshim;

use cshim::*;

pub struct ProtFs {
    oldfs: bindings::mm_segment_t,
}

impl ProtFs {
    pub fn prot() -> Self {
        Self{
            oldfs: protect_fs(),
        }
    }
}

impl Drop for ProtFs {
    fn drop(&mut self) {
        release_fs(self.oldfs)
    }
}