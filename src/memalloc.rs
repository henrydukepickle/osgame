use core::slice::{self, from_raw_parts};

pub static ALLOCATOR: MemAllocator = MemAllocator {
    mem: [0; MEM_SIZE],
    mapped: [false; MEM_SIZE],
};

const MAX_BLOCK_SIZE: usize = MEM_SIZE >> 4;
const MEM_SIZE: usize = 1 << 16;
static ALL_FALSE: [bool; MAX_BLOCK_SIZE] = [false; MAX_BLOCK_SIZE];
static ALL_TRUE: [bool; MAX_BLOCK_SIZE] = [true; MAX_BLOCK_SIZE];
pub struct MemAllocator {
    //horribly inefficient
    mem: [u8; MEM_SIZE],
    mapped: [bool; MEM_SIZE],
}

impl MemAllocator {
    fn ptr_at(&self, address: usize) -> *mut u8 {
        unsafe { self.mem.as_ptr().cast_mut().offset(address as isize) }
    }
    fn slice_at<'a>(&'a self, address: usize, size: usize) -> &'a mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr_at(address), size) }
    }
    fn get_address(&self, ptr: *mut u8) -> usize {
        ptr.addr() - self.mem.as_ptr().addr()
    }
    fn find_free_block(&self, block_size: usize) -> usize {
        let mut address = 0;
        while address <= MEM_SIZE - block_size {
            if self.mapped[address..address + block_size] == ALL_FALSE[0..block_size] {
                return address;
            }
            address += block_size;
        }
        panic!("No space found!")
    }
    fn malloc<'a>(&'a self, size: usize) -> &'a mut [u8] {
        let address = self.find_free_block(size);
        unsafe {
            self.mapped
                .as_ptr()
                .cast_mut()
                .offset(address as isize)
                .copy_from_nonoverlapping(ALL_TRUE.as_ptr(), size);
            self.slice_at(address, size)
        }
    }
    fn free(&self, ptr: *mut u8, size: usize) {
        let address = self.get_address(ptr);
        if address >= MEM_SIZE {
            panic!("ptr out of bounds!")
        }
        unsafe {
            self.mapped
                .as_ptr()
                .cast_mut()
                .offset(address as isize)
                .copy_from_nonoverlapping(ALL_FALSE.as_ptr(), size)
        };
    }
}

pub fn malloc(size: usize) -> &'static mut [u8] {
    ALLOCATOR.malloc(size)
}

pub fn malloc_lit(lit: &'static str) -> &'static str {
    let bytes = lit.as_bytes();
    let dest = malloc(bytes.len());
    dest.copy_from_slice(bytes);
    str::from_utf8(dest).unwrap()
}

pub fn free(mem: &'static [u8]) {
    ALLOCATOR.free(mem.as_ptr().cast_mut(), mem.len());
}
