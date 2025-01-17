/// Error type of cf-r2-sdk.
#[derive(Debug)]
pub struct OperationError(pub anyhow::Error);

/// Type conversion to anyhow::error => OperationError
impl<E> From<E> for OperationError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
