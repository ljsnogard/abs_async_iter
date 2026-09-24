use abs_cancel::TrMayCancel;

/// Iterate items asynchronously.
pub trait TrAsyncIterator {
    type Item: Sized;
    type Err: core::error::Error;

    type NextAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::Item, Self::Err>>
    where
        Self: 'f;

    fn next_async(&mut self) -> Self::NextAsync<'_>;
}

impl<I> TrAsyncIterator for I
where
    I: core::iter::Iterator,
{
    type Item = <Self as Iterator>::Item;
    type Err = IterEndedError;

    type NextAsync<'f> = core::future::
        Ready<Result<Self::Item, Self::Err>>
    where
        Self: 'f;

    fn next_async(&mut self) -> Self::NextAsync<'_> {
        if let Option::Some(item) = self.next() {
            core::future::ready(Result::Ok(item))
        } else {
            core::future::ready(Result::Err(IterEndedError))
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct IterEndedError;

impl core::fmt::Display for IterEndedError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "IterEndedError")
    }
}

impl core::error::Error for IterEndedError
{}
