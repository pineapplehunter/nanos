use core::{
    alloc::{GlobalAlloc, Layout},
    fmt,
    mem::align_of,
};

use crate::{print, println};

extern "C" {
    static __heap_start: core::cell::UnsafeCell<usize>;
}

struct MinimumAllocator;

#[repr(C)]
struct Marker(usize);

impl Marker {
    const MASK: usize = 1 << (usize::BITS - 1);

    pub fn allocated(&self) -> bool {
        self.0 & Self::MASK != 0
    }

    pub fn set_allocated(&mut self, allocated: bool) {
        if allocated {
            self.0 |= Self::MASK;
        } else {
            self.0 &= !Self::MASK;
        }
    }

    pub fn size(&self) -> Option<usize> {
        let size = self.0 & !Self::MASK;
        if size == 0 {
            return None;
        } else {
            Some(size)
        }
    }

    pub fn set_size(&mut self, size: usize) {
        self.0 = (size & !Self::MASK) | (Self::MASK & self.0);
    }

    pub unsafe fn alloc(&mut self, layout: &Layout) -> Option<*mut u8> {
        let ptr = self as *const Marker;
        let output = ptr.offset(1) as *mut u8;
        let output = output.offset(output.align_offset(layout.align()) as isize);
        match self.size() {
            None => {
                let end = output.offset(layout.size() as isize);
                let offset = end.align_offset(align_of::<Marker>());
                let next = end.add(offset) as *mut Marker;
                self.set_allocated(true);
                self.set_size(next.offset_from(ptr) as usize - 1);
                println!("alloc: use {:?}", self);
                Some(output)
            }
            Some(size) if size * align_of::<Marker>() > layout.size() => {
                self.set_allocated(true);
                println!("alloc: use {:?}", self);
                Some(output)
            }
            _ => None,
        }
    }

    pub unsafe fn next(&self) -> Option<*mut Marker> {
        self.size()
            .map(|size| (self as *const Marker as *mut Marker).add(size + 1))
    }
}

impl fmt::Debug for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Marker")
            .field("ptr", &(self as *const Marker))
            .field("allocated", &self.allocated())
            .field("size", &self.size())
            .finish()
    }
}

unsafe impl GlobalAlloc for MinimumAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        println!("alloc: New alloc {:?}", layout);
        let mut this_marker = __heap_start.get() as *mut Marker;
        loop {
            println!("alloc: check {:?}", *this_marker);
            if let Some(output) = (*this_marker).alloc(&layout) {
                break output;
            } else {
                this_marker = (*this_marker).next().unwrap();
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let initial_marker = __heap_start.get() as *mut Marker;
        let mut this_marker = initial_marker;
        while let Some(next_marker) = (*this_marker).next() {
            println!("dealloc: check {:?}", *this_marker);
            if next_marker.offset(1) as *mut u8 > ptr {
                (*this_marker).set_allocated(false);
                break;
            }
            this_marker = next_marker;
        }
        (*this_marker).set_allocated(false);
        let mut this_marker = initial_marker;
        while let Some(next_marker) = (*this_marker).next() {
            println!("dealloc: clean {:?}", *this_marker);
            if (*this_marker).allocated() || (*next_marker).allocated() {
                this_marker = next_marker;
            } else {
                if let Some(next_next_marker) = (*next_marker).next() {
                    print!(
                        "dealloc: merge {:?} and {:?} to ",
                        *this_marker, *next_marker
                    );
                    let new_size = next_next_marker.offset_from(this_marker) - 1;
                    (*this_marker).set_size(new_size as usize);
                    println!("{:?}", *this_marker);
                } else {
                    (*this_marker).set_size(0);
                }
            }
        }
    }
}

#[global_allocator]
static ALLOCATOR: MinimumAllocator = MinimumAllocator;

#[alloc_error_handler]
fn alloc_error(_: core::alloc::Layout) -> ! {
    panic!("Alloc Error!")
}
