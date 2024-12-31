use crate::lexer::Lexer;
use crate::lexer_en::LexerEnglish;
use rust_bert::pipelines::sentence_embeddings::Embedding;
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use std::collections::HashMap;
use std::fmt::Error;

// Originally called the FileReport
#[derive(Debug)]
pub(crate) struct Entry
{
  /// The filename of the document crawled.
  title: String,
  /// The number index, in human terms, of the page crawled.
  location: u32,
  // TODO: Allow for path
  /// A summary of the document crawled.
  /// AI Generated.
  summary: String,
  /// The number (value) of times each word (key) appears on a page.
  frequency: HashMap<String, u32>,
  /// The vectorized array of words.
  /// NOTE: Vector does not mean a rust Vec.
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
    // ) -> Option<Entry>
  ) -> Result<Entry, Error>
  {
    let config = SummarizationConfig {
      min_length: 10,
      max_length: Some(255),
      repetition_penalty: 2.0,
      ..Default::default()
    };

    let model = match SummarizationModel::new(config) {
      Ok(model) => model,
      _ => return Err(std::fmt::Error),
    };

    let input_array = [content.clone()];
    let summary = match model.summarize(&input_array) {
      Ok(summary) => summary[0].clone(),
      _ => return Err(std::fmt::Error),
    };

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

  /// Generates a hash map of the tokens found within a string.
  /// For example, "Forest ran, and ran, and ran."
  /// forest: 1, ran: 3, and, 2
  pub fn frequency_analysis(tokens: Vec<String>) -> Result<HashMap<String, u32>, Error>
  {
    let mut frequency: HashMap<String, u32> = HashMap::new();

    for token in tokens {
      let count = frequency.entry(token).or_insert(0);
      *count += 1;
    }
    Ok(frequency)
  }

  /// Keywords are extracted from text using n-grams. If there is not a
  /// sufficient amount of information provided, the analysis will return an
  /// empty vector.
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

mod tests
{
  use super::*;

  #[test]
  fn can_generate_entity()
  {
    let title = "Test Title";
    let location = 42;

    let story = "Once upon, a time there was a beautiful princess who lived in a dead tree beside a lake, more than anything she desired indoor plumbing. But here mother was missguided and thought that indoor plumbing was invented by the devil. So the princess had do go without.";
    let content = String::from(story);

    let entry = Entry::new(title.into(), location, content).unwrap();

    println!("Entry: {:?}", entry);
    assert_eq!(1, 1);
  }

  #[test]
  fn frequency_test1()
  {
    let sentence = "The Itsy Bitsy Spider climbed up the waterspout. Down came the rain and washed the spider out. Out came the sun and dried up all the rain and the Itsy Bitsy spider climbed up the spout again.".to_string();
    let lex = LexerEnglish::new();
    let lower = lex.normalize_input_text(sentence);
    let cleaned = lex.remove_punctuation(lower);
    let tokens = lex.generate_word_tokens(cleaned);
    let stopless = lex.remove_stopwords(tokens);
    let frequency = Entry::frequency_analysis(stopless).unwrap();

    let mut expected = HashMap::new();
    expected.insert("rain".to_string(), 2);
    expected.insert("dried".to_string(), 1);
    expected.insert("bitsy".to_string(), 2);
    expected.insert("itsy".to_string(), 2);
    expected.insert("spider".to_string(), 3);
    expected.insert("out".to_string(), 2);
    expected.insert("waterspout".to_string(), 1);
    expected.insert("again".to_string(), 1);
    expected.insert("washed".to_string(), 1);
    expected.insert("sun".to_string(), 1);
    expected.insert("down".to_string(), 1);
    expected.insert("all".to_string(), 1);
    expected.insert("spout".to_string(), 1);
    expected.insert("up".to_string(), 3);
    expected.insert("came".to_string(), 2);
    expected.insert("climbed".to_string(), 2);

    assert_eq!(frequency, expected);
  }

  #[test]
  fn keyword_test_basic()
  {
    let sentence = "the itsy bitsy spider climbed up the waterspout down came the rain and washed the spider out out came the sun and dried up all the rain and the itsy bitsy spider climbed up the spout again".to_string();
    let lex = LexerEnglish::new();
    let lower = lex.normalize_input_text(sentence);
    let cleaned = lex.remove_punctuation(lower);
    let tokens = lex.generate_word_tokens(cleaned);
    let stopless = lex.remove_stopwords(tokens);
    let keywords = Entry::keyword_analysis(stopless).unwrap();

    let expected = vec![
      "spider".to_string(),
      "up".to_string(),
      "itsy".to_string(),
      "climbed".to_string(),
      "rain".to_string(),
      "came".to_string(),
      "bitsy".to_string(),
      "out".to_string(),
    ];

    for keyword in expected {
      assert!(keywords.contains(&keyword));
    }
  }

  #[test]
  fn keyword_test_small_sentence()
  {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    let lex = LexerEnglish::new();
    let lower = lex.normalize_input_text(sentence);
    let cleaned = lex.remove_punctuation(lower);
    let tokens = lex.generate_word_tokens(cleaned);
    let stopless = lex.remove_stopwords(tokens);
    let keywords = Entry::keyword_analysis(stopless).unwrap();

    let expected: Vec<String> = Vec::new();

    assert_eq!(keywords, expected);
  }

  #[test]
  fn keyword_test3()
  {
    let sentence = "Planetary exploration missions are conducted by some of the most sophisticated robots ever built. Through them we extend our senses to the farthest reaches of the solar system and into remote and hostile environments, where the secrets of our origins and destiny lie hidden. The coming years of solar system exploration promise to be the most exciting and productive yet, as we explore entirely new worlds and probe in even greater detail the fascinating environments we have discovered.".to_string();
    let lex = LexerEnglish::new();
    let lower = lex.normalize_input_text(sentence);
    let cleaned = lex.remove_punctuation(lower);
    let tokens = lex.generate_word_tokens(cleaned);
    let stopless = lex.remove_stopwords(tokens);
    let keywords = Entry::keyword_analysis(stopless).unwrap();

    let expected = vec![
      "exploration".to_string(),
      "environments".to_string(),
      "most".to_string(),
      "solar".to_string(),
      "our".to_string(),
      "system".to_string(),
    ];

    for keyword in expected {
      assert!(keywords.contains(&keyword));
    }
  }
}
