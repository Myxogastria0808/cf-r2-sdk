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
/// Type conversion 
/// Implementing the Display trait for OperationError
// impl std::fmt::Display for OperationError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self.0)
//     }
// }
// impl std::error::Error for OperationError {}