use std::fmt;
use std::pin::Pin;

use futures::io::{AsyncBufRead, AsyncRead, Initializer};

use crate::io;
use crate::task::{Context, Poll};

/// Creates a reader that contains no data.
///
/// # Examples
///
/// ```rust
/// # #![feature(async_await)]
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::io;
/// use async_std::prelude::*;
///
/// let mut buf = Vec::new();
/// let mut reader = io::empty();
/// reader.read_to_end(&mut buf).await?;
///
/// assert!(buf.is_empty());
/// #
/// # Ok(()) }) }
/// ```
pub fn empty() -> Empty {
    Empty { _priv: () }
}

/// A reader that contains no data.
///
/// This reader is constructed by the [`sink`] function.
///
/// [`sink`]: fn.sink.html
pub struct Empty {
    _priv: (),
}

impl fmt::Debug for Empty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Empty { .. }")
    }
}

impl AsyncRead for Empty {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        _: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(0))
    }

    #[inline]
    unsafe fn initializer(&self) -> Initializer {
        Initializer::nop()
    }
}

impl AsyncBufRead for Empty {
    #[inline]
    fn poll_fill_buf<'a>(
        self: Pin<&'a mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<io::Result<&'a [u8]>> {
        Poll::Ready(Ok(&[]))
    }

    #[inline]
    fn consume(self: Pin<&mut Self>, _: usize) {}
}
