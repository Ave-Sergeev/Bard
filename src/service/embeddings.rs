use crate::error::error::EmbeddingError;
use ndarray::Array2;
use rand::distr::{Distribution, Uniform};
use rand::rng;

pub struct Embeddings {
    matrix: Array2<f32>,
    vocab_size: usize,
    embedding_dim: usize,
}

impl Embeddings {
    /// Создание новой матрицы эмбеддингов со рандомным наполнением
    pub fn new(vocab_size: usize, embedding_dim: usize) -> Self {
        let mut rng = rng();
        let uniform = Uniform::new_inclusive(-1.0, 1.0).unwrap();

        let matrix = Array2::from_shape_fn((embedding_dim, vocab_size), |_| uniform.sample(&mut rng));

        Self {
            matrix,
            vocab_size,
            embedding_dim,
        }
    }

    /// Преобразование вектора токенов в матрицу эмбеддингов
    pub fn tokens_to_embeddings(&self, tokens: &[usize]) -> Result<Array2<f32>, EmbeddingError> {
        let mut embeddings = Array2::zeros((self.embedding_dim, tokens.len()));

        for (i, &token) in tokens.iter().enumerate() {
            if token >= self.vocab_size {
                return Err(EmbeddingError::TokenOutOfBounds {
                    token,
                    vocab_size: self.vocab_size,
                });
            }

            embeddings.column_mut(i).assign(&self.matrix.column(token));
        }

        Ok(embeddings)
    }

    /// Геттер для матрицы эмбеддингов
    pub fn get_matrix(&self) -> &Array2<f32> {
        &self.matrix
    }
}
