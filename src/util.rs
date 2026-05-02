pub trait OptionFutureTransposeExt {
    type Output;

    fn transpose(self) -> impl Future<Output = Option<Self::Output>>;
}

impl<F, T> OptionFutureTransposeExt for Option<F>
where
    F: Future<Output = T>,
{
    type Output = T;

    async fn transpose(self) -> Option<T> {
        match self {
            Some(fut) => Some(fut.await),
            None => None,
        }
    }
}
