//! For internal use. These APIs will never be stable and
//! are meant to be used internally by the tide repo.

use core::pin::Pin;
use futures::future::Future;

/// Convenience alias for pinned box of Future<EndpointResult<T>> + Send + 'static
pub type BoxTryFuture<T> =
    Pin<Box<dyn Future<Output = crate::error::EndpointResult<T>> + Send + 'static>>;
