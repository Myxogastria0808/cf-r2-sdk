#[derive(Debug)]
pub struct OperationError(pub anyhow::Error);

//anyhow::error => OperationError への型変換
impl<E> From<E> for OperationError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
