#[derive(Debug, Clone)]
pub enum CommandExecError {
    INVALID_ARGUMENTS,
}

impl<T: fmt::Display> From<T> for CommandExecError {
    #[inline]
    fn from(d: T) -> Self {
        CommandExecError(d.to_string())
    }
}
