use std::collections::HashMap;

use crate::entry::Entry;
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

      let path = file_path
        .path()
        .to_str()
        .unwrap_or_default()
        .to_string()
        .parse()
        .unwrap_or_default();
      let new_entry = Entry::new(file_path.file_name().into_string().unwrap(), path, content);

      // TODO: Working here.
      // Now that the entry is created, we need to persist the entry in the
      // database.
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
