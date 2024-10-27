use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("创建委托失败")]
    NewOrderError,
    #[error("{0}")]
    OtherError(String),
    #[error("{0}")]
    ChannelError(String),
}

impl<T> From<crossbeam::channel::SendError<T>> for ExecutionError {
    fn from(error: crossbeam::channel::SendError<T>) -> Self {
        ExecutionError::ChannelError(error.to_string())
    }
}
