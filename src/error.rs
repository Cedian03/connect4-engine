#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Conversion error: {0}")]
    Convert(String),
    #[error("Error while loading opening book: {0}")]
    LoadBook(String),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
