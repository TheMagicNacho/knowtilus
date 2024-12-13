use std::collections::HashMap;

use std::fs::DirEntry;
use std::io::Error;
use lopdf;

pub fn add(left: u64, right: u64) -> u64 { left + right }

// Originally called the FileReport
struct Entry
{
  /// The filename of the document crawled.
  title: String,
  /// The number index, in human terms, of the page crawled.
  page: u32,
  /// A summary of the document crawled.
  /// AI Generated.
  summary: String,
  /// The number (value) of times each word (key) appears on a page.
  frequency: HashMap<String, u32>,
  /// The vectorized array of words.
  // TODO : Update this to match the vectorization library
  vectors_words: Vec<u32>,

  /// The vectorized array of the entire page.
  vectors_page: Vec<u32>,

  /// The vectorized array of the sentences.
  vectors_sentences: Vec<u32>,

  /// Keywords extracted from the document using n-gram analysis.
  keywords: Vec<String>,
}

struct Crawler;

impl Crawler
{
  fn new() -> Self
  {
    Crawler
  }

  fn crawl(&self, path: String) -> Result<(), Error>
  {
    // open the directory and read the files
    let files = std::fs::read_dir(path).unwrap();

    for file in files
    {
      match file {
        Ok(file) => {
          let path = file.path();
          println!("Filename: {:?}", path);
          let extension = path.extension().unwrap().to_str().unwrap();

          match extension {
            "pdf" => {self.handle_pdf(file)?},
            "txt" => {self.handle_txt(file)?},
            _ => {continue}
          }

        }
        Err(_) => {continue}
      }
    }
  Ok(())
  }

  fn handle_pdf(&self, file_path: DirEntry) -> Result<(), Error>{
    let mut doc = match lopdf::Document::load(file_path.path()){
      Ok(doc) => doc,
      Err(_) => return Err(Error::new(std::io::ErrorKind::Other, "Failed to load PDF"))
    };
    if doc.is_encrypted() {
      doc.decrypt("").unwrap();
    }
    doc.version = "1.4".to_string();

    let pages = doc.get_pages();
    for (page_num, _) in pages.iter() {

      // We try to extract as much as possible, but if we can't, we just skip the page.
      let content = match doc.extract_text(&[*page_num]){
        Ok(content) => content,
        Err(_) => continue
      };
      
      println!("Page: {}", page_num);
      println!("{}", content);
    }
    Ok(())   
  }

  fn handle_txt(&self, file_path: DirEntry) -> Result<(), Error>{
    println!("TODO: Handle Text");
    let text = std::fs::read_to_string(file_path.path())?;
    println!("{}", text);
    
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