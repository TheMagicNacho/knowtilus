use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::lexer_en::LexerEnglish;
use lopdf;
use std::fs::DirEntry;
use std::io::Error;

pub fn add(
  left: u64,
  right: u64,
) -> u64
{
  left + right
}

// Originally called the FileReport
struct Entry
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
  vector_words: Vec<u32>,
  /// The vectorized array of the entire page.
  /// NOTE: Vector does not mean a rust Vec.
  vector_page: Vec<u32>,
  /// The vectorized array of the sentences.
  /// NOTE: Vector does not mean a rust Vec.
  vector_sentences: Vec<u32>,
  /// Keywords extracted from the document using n-gram analysis.
  keywords: Vec<String>,
}

impl Entry
{
  fn new(
    &self,
    title: String,
    location: u32,
    content: String,
  ) -> Result<Entry, Error>
  {
    let lexer = LexerEnglish::new();

    let text_lowered = lexer.normalize_input_text(content);
    let text_cleaned = lexer.remove_punctuation(text_lowered);
    let token_words_raw = lexer.generate_word_tokens(text_cleaned.clone());
    let tokens_words = lexer.remove_stopwords(token_words_raw);

    let tokens_sentences = lexer.generate_sentence_tokens(text_cleaned);

    let frequency = Entry::frequency_analysis(tokens_words.clone())?;
    let keywords = Entry::keyword_analysis(tokens_words)?;

    Ok(Entry {
      title,
      location,
      summary: "".to_string(),
      frequency,
      vector_words: Vec::new(),
      vector_page: Vec::new(),
      vector_sentences: Vec::new(),
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

struct Crawler
{
  lexer: LexerEnglish,
}

impl Crawler
{
  fn new() -> Self
  {
    Crawler {
      lexer: LexerEnglish::new(),
    }
  }

  fn crawl(
    &self,
    path: String,
  ) -> Result<(), Error>
  {
    // open the directory and read the files
    let files = std::fs::read_dir(path).unwrap();

    for file in files {
      match file {
        Ok(file) => {
          let path = file.path();
          println!("Filename: {:?}", path);
          let extension = path.extension().unwrap().to_str().unwrap();

          match extension {
            "pdf" => self.handle_pdf(file)?,
            "txt" => self.handle_txt(file)?,
            _ => continue,
          }
        }
        Err(_) => continue,
      }
    }
    Ok(())
  }

  fn handle_pdf(
    &self,
    file_path: DirEntry,
  ) -> Result<(), Error>
  {
    let mut doc = match lopdf::Document::load(file_path.path()) {
      Ok(doc) => doc,
      Err(_) => return Err(Error::new(std::io::ErrorKind::Other, "Failed to load PDF")),
    };
    if doc.is_encrypted() {
      doc.decrypt("").unwrap();
    }
    doc.version = "1.4".to_string();

    let pages = doc.get_pages();
    for (page_num, _) in pages.iter() {
      // We try to extract as much as possible, but if we can't, we just skip the
      // page.
      let content = match doc.extract_text(&[*page_num]) {
        Ok(content) => content,
        Err(_) => continue,
      };

      println!("Page: {}", page_num);
      println!("{}", content);
    }
    Ok(())
  }

  fn handle_txt(
    &self,
    file_path: DirEntry,
  ) -> Result<(), Error>
  {
    // TODO: Implement location of the file by 10ths of a document.
    let text = std::fs::read_to_string(file_path.path())?;

    println!("Sentences: {:?}", text);

    Ok(())
  }
}

mod tests
{
  use super::*;

  #[test]
  fn test_crawler()
  {
    let crawler = Crawler::new();
    let res = crawler.crawl("../test-assets".to_string());
    assert!(res.is_ok());
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
