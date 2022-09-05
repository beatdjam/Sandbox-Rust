use std::fs::File;

pub struct DiscManager {
    heap_file: File,
    next_page_id: u64
}