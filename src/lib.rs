#![no_std]

mod stream_;

pub use stream_::TrAsyncIterator;

pub mod x_deps {
    pub use abs_cancel;
}
