use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Rusqlite Error: {0}")]
    Sql(#[from] rusqlite::Error),
    #[error("Failed fetching an OS directory")]
    Dir,
}

pub type LaResult<T> = Result<T, Error>;
