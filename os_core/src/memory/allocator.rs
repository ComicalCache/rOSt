use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

use linked_list_allocator::LockedHeap;

/// The global memory allocator
#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
