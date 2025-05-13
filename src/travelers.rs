use std::time::SystemTime;

/// Traveler objects move between other objects. These are pure abstractions of
/// data objects. The input page is the page read in from the disk.
#[derive(Debug, Clone)]
pub struct InputPage
{
  pub(crate) page_number: u64,
  pub(crate) date_written: SystemTime,
  pub(crate) date_modified: SystemTime,
  pub(crate) file_format: String,
  pub(crate) file_name: String,
  pub(crate) content: String,
}