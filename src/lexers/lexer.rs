
/// The `LexerConfig` struct holds configuration options for the `Lexer`.
/// We want parameters like stopwords, and punctuation to be configurable but also known at compile time.
pub struct LexerConfig {
  stopwords: Vec<String>,
  punctuation: Vec<String>,
}

/// The `Lexer` trait defines a set of methods for processing text data.
/// Between crawling and searching, the text data must be processed the same
/// way. This trait represents a builder pattern for text processing.
pub trait Lexer
{
  fn new(
    config: LexerConfig,
  ) -> Self;
  // Setters
  /// Sets the text to be processed.
  /// One lexer can process multiple texts.
  fn set_text(
    &mut self,
    text: &str,
  ) -> &mut Self;

  // Processors
  /// Break the words in the text into tokens.
  async fn tokenize_text(&mut self) -> &mut Self;

  /// Break the text into sentences tokens.
  async fn tokenize_sentences(&mut self) -> &mut Self;

  /// Stop words are common words that are usually filtered out in text.
  async fn remove_stopwords(&mut self) -> &mut Self;

  /// Sometimes, punctuation is not needed in the text and only adds to noise
  /// during a search.
  async fn remove_punctuation(&mut self) -> &mut Self;

  async fn lament_tokens(&self) -> &mut Self;

  // Getters
  /// Returns the processed text as a vector of strings.
  fn get_tokens(&self) -> Vec<String>;

  /// Returns the processed text as a vector of sentences.
  fn get_sentences(&self) -> Vec<String>;
}
