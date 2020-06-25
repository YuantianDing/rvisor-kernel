
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

impl Drop for ProtFs {
    pub fn drop(&mut self) {
        release_fs(self.oldfs)
    }
}