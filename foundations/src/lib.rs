pub mod common;
pub mod crawl;
pub mod search;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_works() {
        let result = common::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn search_works() {
        let result = search::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn crawl_works() {
        let result = crawl::add(2, 2);
        assert_eq!(result, 4);
    }

}
