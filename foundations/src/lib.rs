pub mod common;
pub mod crawl;
pub mod search;

pub mod lexer;
mod lexer_en;

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn common_works()
  {
    let words = &common::STOPWORDS_EN;
    let anything = *words.first().unwrap();
    assert_ne!(anything, "");
  }

  #[test]
  fn search_works()
  {
    let result = search::add(2, 2);
    assert_eq!(result, 4);
  }

  #[test]
  fn crawl_works()
  {
    let result = crawl::add(2, 2);
    assert_eq!(result, 4);
  }
}
