use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmbeddingError {
    #[error("Token {token} is out of vocabulary bounds (vocab size: {vocab_size})")]
    TokenOutOfBounds { token: usize, vocab_size: usize },
}
