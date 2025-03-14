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
        let uniform = Uniform::new_inclusive(-1.0, 1.0).expect("Fail to create a new Uniform instance");

        let matrix = Array2::from_shape_fn((embedding_dim, vocab_size), |_| uniform.sample(&mut rng));

        Self {
            matrix,
            vocab_size,
            embedding_dim,
        }
    }

    /// Преобразование вектора токенов в матрицу эмбеддингов
    pub fn tokens_to_embeddings(&self, tokens: &[usize]) -> Result<Array2<f32>, String> {
        let mut embeddings = Array2::zeros((self.embedding_dim, tokens.len()));

        for (i, &token) in tokens.iter().enumerate() {
            if token >= self.vocab_size {
                return Err("Token is out of vocabulary bounds".to_string());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embeddings_new() {
        let vocab_size = 10;
        let embedding_dim = 5;

        let embeddings = Embeddings::new(vocab_size, embedding_dim);

        // Проверяем размеры матрицы
        assert_eq!(embeddings.matrix.shape(), &[embedding_dim, vocab_size]);
        assert_eq!(embeddings.vocab_size, vocab_size);
        assert_eq!(embeddings.embedding_dim, embedding_dim);

        // Проверяем, что все значения находятся в диапазоне [-1.0, 1.0]
        for &value in embeddings.matrix.iter() {
            assert!(value >= -1.0 && value <= 1.0);
        }
    }

    #[test]
    fn test_tokens_to_embeddings() {
        let vocab_size = 10;
        let embedding_dim = 5;
        let embeddings = Embeddings::new(vocab_size, embedding_dim);

        let tokens = vec![0, 3, 7];
        let result = embeddings.tokens_to_embeddings(&tokens).unwrap();

        // Проверяем размеры результирующей матрицы
        assert_eq!(result.shape(), &[embedding_dim, tokens.len()]);

        // Проверяем, что эмбеддинги соответствуют ожидаемым
        for (i, &token) in tokens.iter().enumerate() {
            let expected_embedding = embeddings.matrix.column(token);
            let actual_embedding = result.column(i);

            for (e, a) in expected_embedding.iter().zip(actual_embedding.iter()) {
                assert_eq!(*e, *a);
            }
        }
    }

    #[test]
    fn test_tokens_to_embeddings_out_of_bounds() {
        let vocab_size = 10;
        let embedding_dim = 5;
        let embeddings = Embeddings::new(vocab_size, embedding_dim);

        // Токен 15 превышает размер словаря (10)
        let tokens = vec![1, 5, 15];
        let result = embeddings.tokens_to_embeddings(&tokens);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Token is out of vocabulary bounds");
    }

    #[test]
    fn test_tokens_to_embeddings_empty() {
        let vocab_size = 10;
        let embedding_dim = 5;
        let embeddings = Embeddings::new(vocab_size, embedding_dim);

        let tokens = vec![];
        let result = embeddings.tokens_to_embeddings(&tokens).unwrap();

        // Проверяем, что получили матрицу правильной размерности (embedding_dim x 0)
        assert_eq!(result.shape(), &[embedding_dim, 0]);
    }

    #[test]
    fn test_get_matrix() {
        let vocab_size = 10;
        let embedding_dim = 5;
        let embeddings = Embeddings::new(vocab_size, embedding_dim);

        let matrix = embeddings.get_matrix();

        // Проверяем, что геттер возвращает матрицу с правильными размерами
        assert_eq!(matrix.shape(), &[embedding_dim, vocab_size]);
        // Проверяем, что матрица совпадает с оригинальной
        assert_eq!(*matrix, embeddings.matrix);
    }
}
