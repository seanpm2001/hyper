#[cfg(feature = "http1")]
use std::convert::Infallible;
pub(crate) use std::task::{Context, Poll};

/// A function to help "yield" a future, such that it is re-scheduled immediately.
///
/// Useful for spin counts, so a future doesn't hog too much time.
#[cfg(feature = "http1")]
pub(crate) fn yield_now(cx: &mut Context<'_>) -> Poll<Infallible> {
    cx.waker().wake_by_ref();
    Poll::Pending
}
