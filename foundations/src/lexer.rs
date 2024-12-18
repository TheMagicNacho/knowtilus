use rust_bert::pipelines::sentence_embeddings::Embedding;

/// A lexer contains tools required to actively and in a common manner process
/// input texts.
pub trait Lexer
{
  fn new() -> Self;

  /// Normalize the input text to a common format for processing.
  /// Attempt to remove any special characters, and convert to lowercase.
  fn normalize_input_text(
    &self,
    text: String,
  ) -> String;

  /// Remove punctuation from the input text.
  /// In English, punctuation does not provide information about the content of
  /// the text.
  fn remove_punctuation(
    &self,
    text: String,
  ) -> String;

  /// A sentence token is a collection of words that form a sentence.
  /// A sentence token can be hashed and vectorized for further processing.
  /// An example of a sentence token is "the quick brown fox".
  /// Notice the token does not contain punctuation and is normalized.
  fn generate_sentence_tokens(
    &self,
    text: String,
  ) -> Vec<String>;

  /// A word token is a single word.
  /// For example, "the", "quick", "brown", "fox".
  fn generate_word_tokens(
    &self,
    text: String,
  ) -> Vec<String>;

  /// A stop word is a word that is filtered out of the input text because it
  /// does not provide useful information about the content of the text. This
  /// is a common practice in natural language processing.
  fn remove_stopwords(
    &self,
    tokens: Vec<String>,
  ) -> Vec<String>;

  /// An embedding is a vector representation of a word or sentence.
  /// The vector representation is used to compare words and sentences.
  fn generate_embedding(
    &self,
    tokens: Vec<String>,
  ) -> Vec<Embedding>;

  // TODO: Lament the tokens

  // TODO: Check if token is in language dictionary
}
