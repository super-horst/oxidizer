#[derive(Debug)]
pub enum Error {
    PostgresError(tokio_postgres::Error),
    #[cfg(target_feature = "tls-openssl")]
    OpensslError(openssl::error::ErrorStack),
    #[cfg(target_feature = "tls-rustls")]
    RustlsError(String),
    MobcError(mobc::Error<tokio_postgres::Error>),
    RefineryError(refinery::Error),
    DoesNotExist,
    ReferencedModelIsNotInDB,
    Other(String),
}

pub type DBResult<T> = std::result::Result<T, Error>;

impl<R> std::convert::From<R> for Error
where
    R: std::fmt::Display,
{
    fn from(v: R) -> Self {
        Error::Other(v.to_string())
    }
}
