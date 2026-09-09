use rust_releases_io::{BoxFuture, Document};

pub trait DistIndexClient {
    type Error;

    fn download(&self) -> Result<Document, Self::Error>;
}

pub trait AsyncDistIndexClient: Send + Sync {
    type Error;

    fn download(&self) -> BoxFuture<'_, Result<Document, Self::Error>>;
}
