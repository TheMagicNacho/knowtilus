use crate::travlers::InputPage;
use anyhow::Result;
use pdf_extract;
use std::time::SystemTime;

struct DiskIO;

impl DiskIO
{
  pub fn read_pdf_file(file_path: &str) -> Result<Vec<InputPage>>
  {
    let metadata = std::fs::metadata(file_path).expect("Unable to read metadata");
    let date_written = metadata.created().unwrap_or(SystemTime::now());
    let date_modified = metadata.modified().unwrap_or(SystemTime::now());
    let file_format = String::from("pdf");

    let pages =
      pdf_extract::extract_text_by_pages(file_path).expect("Unable to extract text from PDF");

    let mut output = Vec::new();

    for (index, page) in pages.iter().enumerate() {
      let content = page.clone();
      let page_number = index as u64;
      let file_name = String::from(file_path);
      let input_page = InputPage {
        page_number,
        date_written,
        date_modified,
        file_format: file_format.clone(),
        file_name,
        content,
      };
      output.push(input_page);
    }

    Ok(output)
  }

  /// Reads a text file and returns a vector of InputPage objects.
  /// Each InputPage object contains the page number, date written, date
  /// modified, file format, and content of the page.
  pub fn read_txt_file(file_path: &str) -> Result<Vec<InputPage>>
  {
    let content = std::fs::read_to_string(file_path).expect("Unable to read file");

    let metadata = std::fs::metadata(file_path).expect("Unable to read metadata");
    let date_written = metadata.created().unwrap_or(SystemTime::now());
    let date_modified = metadata.modified().unwrap_or(SystemTime::now());
    let file_format = String::from("txt");
    let file_name = String::from(file_path);

    let output = vec![InputPage {
      page_number: 0,
      date_written,
      date_modified,
      file_name,
      file_format,
      content,
    }];

    Ok(output)
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn io_should_read_pdf_file()
  {
    // Arrange
    let excerpt = "This seemed to Alice a good opportunity for making her escape";

    let file_path = "./test-assets/alice.pdf";
    // Act
    let pages: Vec<InputPage> = DiskIO::read_pdf_file(file_path).unwrap();
    //
    // for page in pages.iter() {
    //   println!("Page number: {}", page.page_number);
    //   println!("Date written: {:?}", page.date_written);
    //   println!("Date modified: {:?}", page.date_modified);
    //   println!("File format: {}", page.file_format);
    //   println!("File name: {}", page.file_name);
    //   println!("Content: {}", page.content);
    // }
    let substring_exists = pages.iter().any(|page| page.content.contains(excerpt));

    // Assert
    assert!(substring_exists);
    assert_eq!(pages.len(), 77);
  }

  #[test]
  fn io_should_read_txt_file()
  {
    // Arrange
    let excerpt = "This seemed to Alice a good opportunity for making her escape";

    let file_path = "./test-assets/alice.txt";
    // Act
    let page: Vec<InputPage> = DiskIO::read_txt_file(file_path).unwrap();
    let page = page.get(0).unwrap();

    // Assert
    assert!(page.content.contains(excerpt));
    assert_eq!(page.page_number, 0);
    assert_eq!(page.file_format, "txt");
    assert_ne!(page.date_written, SystemTime::now());
    assert_ne!(page.date_modified, SystemTime::now());
  }
}
