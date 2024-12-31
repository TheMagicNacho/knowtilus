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
}
