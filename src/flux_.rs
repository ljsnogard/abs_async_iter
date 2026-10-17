use abs_cancel::TrMayCancel;

use crate::TrAsyncIterator;

pub trait TrFlux<T> {
    type Iter<'f>: 'f + TrAsyncIterator<Item = T>
    where
        Self: 'f;

    type PullAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Self::Iter<'f>>
    where
        Self: 'f;

    fn pull_async(&mut self) -> Self::PullAsync<'_>;
}

pub struct IntoIterAsFlux<I, T>(<I as IntoIterator>::IntoIter)
where
    I: IntoIterator<Item = T>;

impl<I, T> IntoIterAsFlux<I, T>
where
    I: IntoIterator<Item = T>
{
    pub fn new(i: <I as IntoIterator>::IntoIter) -> Self {
        IntoIterAsFlux(i.into_iter())
    }

    pub fn pull_async(
        &mut self,
    ) -> core::future::Ready<&mut <I as IntoIterator>::IntoIter> {
        core::future::ready(&mut self.0)
    }
}

impl<I, T> TrFlux<T> for IntoIterAsFlux<I, T>
where
    I: IntoIterator<Item = T>,
{
    type Iter<'f> = &'f mut <I as IntoIterator>::IntoIter
    where
        Self: 'f;

    type PullAsync<'f> = core::future::Ready<Self::Iter<'f>>
    where
        Self: 'f;

    fn pull_async(&mut self) -> Self::PullAsync<'_> {
        core::future::ready(&mut self.0)
    }
}
