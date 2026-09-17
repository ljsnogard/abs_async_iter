use abs_cancel::TrMayCancel;

/// Iterate items asynchronously.
pub trait TrAsyncIterator {
    type Item: Sized;
    type Err: core::error::Error;

    type NextAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Option<Self::Item>, Self::Err>>
    where
        Self: 'f;

    fn next_async(&mut self) -> Self::NextAsync<'_>;
}

impl<I> TrAsyncIterator for I
where
    I: core::iter::Iterator,
{
    type Item = <Self as Iterator>::Item;
    type Err = !;

    type NextAsync<'f> = core::future::
        Ready<Result<Option<Self::Item>, Self::Err>>
    where
        Self: 'f;

    fn next_async(&mut self) -> Self::NextAsync<'_> {
        core::future::ready(Result::Ok(self.next()))
    }
}
