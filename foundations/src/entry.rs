use crate::lexer::Lexer;
use crate::lexer_en::LexerEnglish;
use rust_bert::pipelines::sentence_embeddings::Embedding;
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use rust_bert::RustBertError;
use std::collections::HashMap;
use std::io::Error;

// Originally called the FileReport
#[derive(Debug)]
pub(crate) struct Entry
{
  /// The filename of the document crawled.
  title: String,
  /// The number index, in human terms, of the page crawled.
  location: u32,
  /// A summary of the document crawled.
  /// AI Generated.
  summary: String,
  /// The number (value) of times each word (key) appears on a page.
  frequency: HashMap<String, u32>,
  /// The vectorized array of words.
  /// NOTE: Vector does not mean a rust Vec.
  // TODO : Update this to match the vectorization library
  embedding_words: Vec<Embedding>,
  /// The vectorized array of the entire page.
  /// NOTE: Vector does not mean a rust Vec.
  embedding_page: Vec<Embedding>,
  /// The vectorized array of the sentences.
  /// NOTE: Vector does not mean a rust Vec.
  embedding_sentences: Vec<Embedding>,
  /// Keywords extracted from the document using n-gram analysis.
  keywords: Vec<String>,
}

impl Entry
{
  pub(crate) fn new(
    title: String,
    location: u32,
    content: String,
  ) -> Result<Entry, RustBertError>
  {
    let config = SummarizationConfig {
      min_length: 10,
      max_length: Some(255),
      repetition_penalty: 2.0,
      ..Default::default()
    };

    let model = SummarizationModel::new(config)?;

    let input_array = [content.clone()];
    let summary = model.summarize(&input_array)?[0].clone();

    // General Lexing
    let lexer = LexerEnglish::new();
    let text_lowered = lexer.normalize_input_text(content);
    let text_cleaned = lexer.remove_punctuation(text_lowered);
    // Tokenization
    // Words
    let token_words_raw = lexer.generate_word_tokens(text_cleaned.clone());
    let tokens_words = lexer.remove_stopwords(token_words_raw);
    // Sentences
    let tokens_sentences = lexer.generate_sentence_tokens(text_cleaned);

    // Embedding
    let embedding_sentences = lexer.generate_embedding(tokens_sentences.clone());
    let embedding_words = lexer.generate_embedding(tokens_words.clone());
    let embedding_page = lexer.generate_embedding(tokens_words.clone());

    // Heuristic Analysis
    let frequency = Entry::frequency_analysis(tokens_words.clone())?;
    let keywords = Entry::keyword_analysis(tokens_words)?;

    Ok(Entry {
      title,
      location,
      summary,
      frequency,
      embedding_words,
      embedding_page,
      embedding_sentences,
      keywords,
    })
  }

  pub fn frequency_analysis(tokens: Vec<String>) -> Result<HashMap<String, u32>, Error>
  {
    let mut frequency: HashMap<String, u32> = HashMap::new();

    for token in tokens {
      let count = frequency.entry(token).or_insert(0);
      *count += 1;
    }

    Ok(frequency)
  }

  pub fn keyword_analysis(tokens: Vec<String>) -> Result<Vec<String>, Error>
  {
    let n = 1;

    let mut ngrams: Vec<Vec<String>> = Vec::new();
    for i in 0..tokens.len() - n + 1 {
      let ngram = tokens[i..i + n].to_vec();
      ngrams.push(ngram);
    }

    let mut ngram_counts: HashMap<Vec<String>, u32> = HashMap::new();
    for ngram in ngrams {
      let count = ngram_counts.entry(ngram).or_insert(0);
      *count += 1;
    }

    let mut filtered_counts: HashMap<Vec<String>, u32> = HashMap::new();
    for (ngram, count) in ngram_counts.iter() {
      if *count >= 2 {
        filtered_counts.insert(ngram.clone(), *count);
      }
    }

    let mut top_ngrams: Vec<(&Vec<String>, &u32)> = filtered_counts.iter().collect();
    top_ngrams.sort_by(|a, b| b.1.cmp(a.1));

    let mut keywords: Vec<String> = Vec::new();
    for (ngram, _) in top_ngrams.iter().take(10) {
      let keyword = ngram.join(" ");
      if keyword.len() > 1 {
        keywords.push(keyword);
      }
    }

    Ok(keywords)
  }
}
