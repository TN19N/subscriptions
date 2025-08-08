use thiserror::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SurrealdDb(#[from] surrealdb::Error),
    #[error("{0:?}")]
    Migrations(String),
}
