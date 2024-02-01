use async_trait::async_trait;
use salvo::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("MongoDB Error")]
    ErrorMongo(#[from] mongodb::error::Error),
}

pub type AppResult<T> = Result<T, Error>;

#[async_trait]
impl Writer for Error {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, _res: &mut Response) {}
}