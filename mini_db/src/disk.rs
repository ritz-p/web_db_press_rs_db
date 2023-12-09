use std::{fs::File, io::{self, Write, SeekFrom, Seek, Read}};
const PAGE_SIZE:u64 = 4096;

pub struct PageId(pub u64);
pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

impl DiskManager{
    pub fn new(heap_file: File) -> io::Result<Self>{
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self{heap_file,next_page_id})
    }

    pub fn allocate_page(&mut self) -> PageId{
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }

    pub fn write_page_data(&mut self,page_id: PageId, data: &[u8])->io::Result<()>{
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        self.heap_file.seek(SeekFrom::Start(offset))?;
        self.heap_file.write_all(data)
    }

    pub fn read_page_data(&mut self,page_id: PageId, data: &[u8])->io::Result<()>{
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        self.heap_file.seek(SeekFrom::Start(offset))?;
        self.heap_file.read_exact(data)
    }
}