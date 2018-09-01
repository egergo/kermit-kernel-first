
use spin::Mutex;

static FIRST_PAGE: Mutex<u64> = Mutex::new(0);

pub fn init_first_page(page: u64) {
  let mut first_page = FIRST_PAGE.lock();
  *first_page = page;
}

pub fn alloc(num_pages: u64) -> u64 {
  let mut first_page = FIRST_PAGE.lock();
  let result = *first_page;
  *first_page += num_pages;
  // TODO check overflow
  result
}

pub fn free(page: usize, num_pages: usize) {

}
