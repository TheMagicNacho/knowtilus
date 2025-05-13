// use std::fs::{metadata, read_to_string};
use crate::travelers::InputPage;
use anyhow::Result;
use pdf_extract;
use std::path::PathBuf;
use std::pin::Pin;
use std::time::SystemTime;
use tokio::fs::{metadata, read_dir, read_to_string};

struct DiskIO;

impl DiskIO
{
  /// Recursively reads all files in a directory and its subdirectories,
  /// processing them into `InputPage` objects.
  ///
  /// # Arguments
  ///
  /// * `dir_path` - A string slice that holds the path to the directory to be
  ///   read.
  ///
  /// # Returns
  ///
  /// A pinned future that resolves to a `Result` containing a vector of
  /// `InputPage` objects if successful, or an error if the operation fails.
  ///
  /// # Behavior
  ///
  /// - Traverses the directory and its subdirectories.
  /// - For each file, it calls `process_file` to process the file and append
  ///   its content to the output vector.
  /// - If a subdirectory is encountered, it recursively processes the
  ///   subdirectory.
  pub fn read_directory_recursively(
    dir_path: &str
  ) -> Pin<Box<dyn Future<Output = Result<Vec<InputPage>>> + '_>>
  {
    Box::pin(async move {
      let mut output = Vec::new();

      let mut entries = read_dir(dir_path).await?;

      while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
          let subdirectory_path = path.to_str().unwrap();
          let pages = DiskIO::read_directory_recursively(subdirectory_path).await?;
          output.extend(pages);
        } else {
          Self::process_file(&mut output, path).await?;
        }
      }
      Ok(output)
    })
  }

  /// Reads all files in a given directory (non-recursively) and processes them
  /// into `InputPage` objects.
  ///
  /// # Arguments
  ///
  /// * `dir_path` - A string slice that holds the path to the directory to be
  ///   read.
  ///
  /// # Returns
  ///
  /// A `Result` containing a vector of `InputPage` objects if successful, or an
  /// error if the operation fails.
  ///
  /// # Behavior
  ///
  /// - Iterates through all entries in the specified directory.
  /// - For each file, it calls `process_file` to process the file and append
  ///   its content to the output vector.
  /// - Only processes files in the given directory, does not traverse
  ///   subdirectories.
  pub async fn read_directory_files(dir_path: &str) -> Result<Vec<InputPage>>
  {
    let mut output = Vec::new();

    let mut entries = read_dir(dir_path).await?;

    while let Some(entry) = entries.next_entry().await? {
      let path = entry.path();
      Self::process_file(&mut output, path).await?;
    }
    Ok(output)
  }

  /// Processes a single file and appends its content to the provided output
  /// vector.
  ///
  /// # Arguments
  ///
  /// * `output` - A mutable reference to a vector of `InputPage` objects where
  ///   the processed file's content will be stored.
  /// * `path` - The path to the file to be processed.
  ///
  /// # Behavior
  ///
  /// - If the file is a PDF, it extracts its pages using `read_pdf_file` and
  ///   appends them to the output.
  /// - If the file is a TXT file, it reads its content using `read_txt_file`
  ///   and appends it to the output.
  async fn process_file(
    output: &mut Vec<InputPage>,
    path: PathBuf,
  ) -> Result<()>
  {
    if path.is_file() {
      let file_path = match path.to_str() {
        Some(s) => s,
        None => return Err(anyhow::anyhow!("File path is not valid UTF-8")),
      };
      if file_path.ends_with(".pdf") {
        let pages = DiskIO::read_pdf_file(file_path).await?;
        output.extend(pages);
      } else if file_path.ends_with(".txt") {
        let pages = DiskIO::read_txt_file(file_path).await?;
        output.extend(pages);
      }
    }
    Ok(())
  }

  /// Reads a PDF file and returns a vector of InputPage objects.
  /// Each InputPage object contains the page number, date written, date
  /// modified, file format, and content of the page.
  async fn read_pdf_file(file_path: &str) -> Result<Vec<InputPage>>
  {
    let metadata = metadata(file_path).await?;
    let date_written = metadata.created().unwrap_or(SystemTime::now());
    let date_modified = metadata.modified().unwrap_or(SystemTime::now());
    let file_format = String::from("pdf");

    let pages = pdf_extract::extract_text_by_pages(file_path)?;

    let mut output = Vec::new();

    for (index, page) in pages.iter().enumerate() {
      let content = page.clone();
      let page_number = index as u64 + 1;
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

  /// Reads a text file and returns a vector of `InputPage` objects.
  /// Each InputPage object contains the page number, date written, date
  /// modified, file format, and content of the page.
  /// # Arguments
  ///
  /// * `file_path` - A string slice that holds the path to the text file to be
  ///   read.
  ///
  /// # Returns
  ///
  /// A `Result` containing a vector of `InputPage` objects if successful, or an
  /// error if the operation fails.
  ///
  /// # Behavior
  ///
  /// - Reads the content of the text file.
  /// - Retrieves metadata such as creation and modification dates.
  /// - Constructs an `InputPage` object with the file's content and metadata.
  async fn read_txt_file(file_path: &str) -> Result<Vec<InputPage>>
  {
    let content = read_to_string(file_path).await?;

    let metadata = metadata(file_path).await?;
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

  #[tokio::test]
  async fn io_should_read_directory_recursively()
  {
    // Arrange
    let dir_path = "./test-assets/topics";
    let search = "Radial ProbabilityBohr radius (a0): 0.0529nmBohr radius (a0): 0.0529nmr2|Rn,2|";

    // Act
    let pages = DiskIO::read_directory_recursively(dir_path).await.unwrap();

    // Checks if the titles of multiple directories are present. And checks for file
    // names with spaces in the title.
    let art = pages.iter().any(|page| {
      page
        .file_name
        .contains("visualelementsandprinciplesofcomposition.pdf")
    });
    let electronics = pages
      .iter()
      .any(|page| page.file_name.contains("Lecture16_ppt.pdf"));
    let biology = pages.iter().any(|page| {
      page
        .file_name
        .contains("Themes and Concepts of Biology.pdf")
    });

    // Checks if the content of the file contains the search string.
    let substring_exists = pages.iter().any(|page| page.content.contains(search));

    // Assert
    assert!(substring_exists);
    assert!(art);
    assert!(electronics);
    assert!(biology);
    assert_eq!(pages.len(), 248);
  }
  #[tokio::test]
  async fn io_should_read_directory_flat()
  {
    let dir_path = "./test-assets/topics/electronics";
    // Act
    let pages = DiskIO::read_directory_files(dir_path)
      .await
      .expect("Unable to read directory");

    let title1 = pages
      .iter()
      .any(|page| page.file_name.contains("Lecture10_ppt.pdf"));
    let title2 = pages
      .iter()
      .any(|page| page.file_name.contains("Lecture16_ppt.pdf"));
    // Assert
    assert!(title1 && title2);
    assert_eq!(pages.len(), 166);
  }

  #[tokio::test]
  async fn io_should_read_pdf_file()
  {
    // Arrange
    let excerpt = "This seemed to Alice a good opportunity for making her escape";
    let real_page = 19;
    let file_path = "./test-assets/alice.pdf";
    // Act
    let pages: Vec<InputPage> = DiskIO::read_pdf_file(file_path).await.unwrap();
    let page = pages.get(real_page - 1).unwrap();
    // Assert
    let substring_exists = page.content.contains(excerpt);
    assert!(substring_exists);
    assert_eq!(pages.len(), 77);
    assert_eq!(page.page_number, 19);
    assert_eq!(page.file_format, "pdf");
    assert_eq!(page.file_name, file_path);
    assert_ne!(page.date_written, SystemTime::now());
    assert_ne!(page.date_modified, SystemTime::now());
  }

  #[tokio::test]
  async fn io_should_read_txt_file()
  {
    // Arrange
    let excerpt = "This seemed to Alice a good opportunity for making her escape";

    let file_path = "./test-assets/alice.txt";
    // Act
    let page: Vec<InputPage> = DiskIO::read_txt_file(file_path).await.unwrap();
    let page = page.get(0).unwrap();

    // Assert
    assert!(page.content.contains(excerpt));
    assert_eq!(page.page_number, 0);
    assert_eq!(page.file_format, "txt");
    assert_eq!(page.file_name, file_path);
    assert_ne!(page.date_written, SystemTime::now());
    assert_ne!(page.date_modified, SystemTime::now());
  }
}
