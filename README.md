## Description

[Русская версия](README.ru.md)

This crate is the second step (after creating [tokenizer](https://github.com/Ave-Sergeev/Tokenomicon)) towards a full
`llm` implementation on the `transformer` architecture.
It is designed to convert tokens into embeddings, as well as encode their positions.

Glossary:

- Tokenization (segmentation) is the process of breaking text into individual parts (words, characters, etc.)
- LLM (large language models) is a general-purpose mathematical model designed for a wide range of tasks related to
  natural language processing.
- Transformer is a deep neural network architecture introduced in 2017 by researchers from Google. It is designed to
  process sequences such as natural language text.
- Embedding is numerical representations of text (token).
- Positional Encoding is a technique used (e.g. in Transformers) to provide positional information to a model by adding
  position-dependent signals to word embeddings,

### Implementation details

- The Embeddings structure creates and manages a matrix of token embeddings, where each column represents a token in the
  dictionary.
- The PositionalEncoding structure implements the creation and management of position encoding.

The crate has the following dependencies:

1) [rand](https://github.com/rust-random/rand) crate to generate pseudo-random values.
2) [ndarray](https://github.com/rust-ndarray/ndarray) crate (math) for efficient matrix handling.
3) [approx](https://github.com/brendanzab/approx) crate to handle approximate comparisons of floating point numbers. Using for tests.
4) ...

## Usage
See [example](/example/src/main.rs) for usage.

### P.S.

Don't forget to leave a ⭐ if you found this project useful.
