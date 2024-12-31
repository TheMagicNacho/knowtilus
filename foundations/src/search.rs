// REMOVE BEFORE FLIGHT
pub fn add(
  left: u64,
  right: u64,
) -> u64
{
  left + right
}

struct Search;

impl Search {}

mod tests
{
  use super::*;

  #[test]
  fn test_add()
  {
    let x = add(2, 2);

    assert_eq!(x, 4);
  }
}
