
use sea_query::{Query, SelectStatement};
static DEFAULT_SIZE: usize = 10;
struct PaginationParams {
    page: usize,
    per_page: usize,
}
//build to query
impl PaginationParams {
    fn new(page: Option<usize>, per_page: Option<usize>) -> Self {
        Self { page: page.unwrap_or(1), per_page: per_page.unwrap_or(DEFAULT_SIZE) }
    }
    fn offset(&self) -> usize {
        (self.page - 1) * self.per_page
    }
    fn limit(&self) -> usize {
        self.per_page
    }
}
pub fn build_query(page: Option<usize>, per_page: Option<usize>) -> SelectStatement {
        let params = PaginationParams::new(page, per_page);
        Query::select()
            .limit(params.limit() as u64)
            .offset(params.offset() as u64).to_owned()
    }