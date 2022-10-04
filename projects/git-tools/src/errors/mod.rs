use git2::{Error, ErrorClass, ErrorCode};

pub type GitResult<T = ()> = Result<T, Error>;

pub fn runtime_error<T, S>(msg: S) -> GitResult<T>
where
    S: Into<String>,
{
    Err(Error::new(ErrorCode::User, ErrorClass::Invalid, msg.into()))
}
