use crate::bindings;
use crate::c_types::*;
extern "C" {
    pub fn protect_fs() -> bindings::mm_segment_t;
    pub fn release_fs(oldfs : bindings::mm_segment_t);
}