//! Implements `Syscalls` for all types that implement `RawSyscalls`.

use crate::{RawSyscalls, Syscalls, YieldNoWaitReturn};

mod yield_op {
    pub const NO_WAIT: u32 = 0;
    pub const WAIT: u32 = 1;
}

impl<S: RawSyscalls> Syscalls for S {
    // -------------------------------------------------------------------------
    // Yield
    // -------------------------------------------------------------------------

    fn yield_no_wait() -> YieldNoWaitReturn {
        let mut flag = core::mem::MaybeUninit::<YieldNoWaitReturn>::uninit();

        unsafe {
            // Flag can be uninitialized here because the kernel promises to
            // only write to it, not read from it. MaybeUninit guarantees that
            // it is safe to write a YieldNoWaitReturn into it.
            Self::yield2([yield_op::NO_WAIT as *mut (), flag.as_mut_ptr() as *mut ()]);

            // yield-no-wait guarantees it sets (initializes) flag before
            // returning.
            flag.assume_init()
        }
    }

    fn yield_wait() {
        // Safety: yield-wait does not return a value, which satisfies yield1's
        // requirement. The yield-wait system call cannot trigger undefined
        // behavior on its own in any other way.
        unsafe {
            Self::yield1([yield_op::WAIT as *mut ()]);
        }
    }
}