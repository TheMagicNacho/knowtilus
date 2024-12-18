use crate::common::{PUNCTUATION_EN, STOPWORDS_EN};
use crate::lexer::Lexer;
use rust_bert::pipelines::sentence_embeddings::{
  Embedding, SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
use std::collections::HashSet;

pub(crate) struct LexerEnglish
{
  // Stop words are words that are filtered out of the input text because they do not provide
  // useful information about the content of the text.
  stopwords: HashSet<String>,

  // Punctuation is a set of characters that are removed from the input text.
  punctuation: HashSet<String>,
}

impl Lexer for LexerEnglish
{
  fn new() -> self::LexerEnglish
  {
    let stopwords_temp: HashSet<String> = STOPWORDS_EN.iter().map(|s| s.to_string()).collect();

    let punctuation_temp: HashSet<String> = PUNCTUATION_EN.iter().map(|s| s.to_string()).collect();

    LexerEnglish {
      stopwords: stopwords_temp,
      punctuation: punctuation_temp,
    }
  }

  fn normalize_input_text(
    &self,
    text: String,
  ) -> String
  {
    text.to_ascii_lowercase()
  }

  fn remove_punctuation(
    &self,
    text: String,
  ) -> String
  {
    let mut text_cleaned = text.clone();
    for p in self.punctuation.iter() {
      text_cleaned = text_cleaned.replace(p, "");
    }
    text_cleaned
  }

  fn generate_sentence_tokens(
    &self,
    text: String,
  ) -> Vec<String>
  {
    let text_lowered = self.normalize_input_text(text);

    let mut output: Vec<String> = text_lowered
      .split(|c: char| self.punctuation.contains(&c.to_string()))
      .map(|s| s.to_string())
      .filter(|s| !s.is_empty())
      .map(|s| s.trim().to_string())
      .collect();

    output = output
      .into_iter()
      .map(|s| s.replace("\n", ""))
      .map(|s| s.replace("\r", ""))
      .map(|s| s.replace("\t", ""))
      .map(|s| s.replace("\\u", ""))
      .map(|s| s.replace("*", ""))
      .map(|s| s.replace("_", ""))
      .collect();

    output
  }

  fn generate_word_tokens(
    &self,
    text: String,
  ) -> Vec<String>
  {
    let text_lowered = self.normalize_input_text(text);
    let text_cleaned = self.remove_punctuation(text_lowered);
    text_cleaned
      .split_whitespace()
      .map(|s| s.to_string())
      .collect()
  }

  fn remove_stopwords(
    &self,
    tokens: Vec<String>,
  ) -> Vec<String>
  {
    tokens
      .into_iter()
      .filter(|t| !self.stopwords.contains(t))
      .collect()
  }

  fn generate_embedding(
    &self,
    tokens: Vec<String>,
  ) -> Vec<Embedding>
  {
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
      .create_model()
      .unwrap();

    model.encode(&tokens).unwrap()
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_lexer_english()
  {
    let lexer = LexerEnglish::new();
    let text = "The quick brown fox jumps over the lazy dog.".to_string();
    let tokens = lexer.generate_word_tokens(text);
    assert_eq!(
      tokens,
      vec!["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
    );
  }

  #[test]
  fn test_lexer_english_remove_stopwords()
  {
    let lexer = LexerEnglish::new();
    let text = "The quick brown fox jumps over the lazy dog.".to_string();
    let tokens = lexer.generate_word_tokens(text);
    let tokens = lexer.remove_stopwords(tokens);
    assert_eq!(
      tokens,
      vec!["quick", "brown", "fox", "jumps", "over", "lazy", "dog"]
    );
  }

  #[test]
  fn test_lexer_english_generate_sentence_tokens()
  {
    let lexer = LexerEnglish::new();
    let text = "The quick brown fox. Jumps over, the lazy... DOG!".to_string();
    let tokens = lexer.generate_sentence_tokens(text);
    assert_eq!(
      tokens,
      vec!["the quick brown fox", "jumps over", "the lazy", "dog"]
    );
  }

  #[test]
  fn test_lexer_english_generate_word_tokens()
  {
    let lexer = LexerEnglish::new();
    let text = "The quick brown fox. Jumps over, the lazy... DOG!".to_string();
    let tokens = lexer.generate_word_tokens(text);
    assert_eq!(
      tokens,
      vec!["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
    );
  }
}
