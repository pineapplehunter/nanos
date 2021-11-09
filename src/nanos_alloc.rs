use core::{
    alloc::{GlobalAlloc, Layout},
    cmp, fmt,
    mem::size_of,
    ptr,
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

    pub unsafe fn alloc_unchecked(&mut self, layout: &Layout) -> *mut u8 {
        let OffsetSize { item_ptr, size } = self.get_item_ptr_and_size(layout);
        match self.size() {
            None => {
                self.set_allocated(true);
                self.set_size(size);
                println!("alloc: use {:?}", self);
                item_ptr
            }
            Some(marker_size) => {
                if marker_size >= size + 2 {
                    print!("alloc: split {:?} to ", self);
                    self.set_size(size);
                    let next_marker = self.next().unwrap();
                    (*next_marker).set_size(marker_size - size - 1);
                    (*next_marker).set_allocated(false);
                    println!("{:?} and {:?}", self, *next_marker);
                }
                println!(
                    "alloc: reuse: marker size = {}, item size = {}",
                    marker_size, size
                );
                self.set_allocated(true);
                println!("alloc: use {:?}", self);
                item_ptr
            }
        }
    }

    pub unsafe fn realloc(&mut self, old_layout: Layout, new_layout: Layout) -> *mut u8 {
        let OffsetSize { size, item_ptr } = self.get_item_ptr_and_size(&old_layout);
        let old_size = size;
        let old_item_ptr = item_ptr;
        let OffsetSize { size, .. } = self.get_item_ptr_and_size(&new_layout);
        let new_size = size;

        self.set_allocated(false);
        let initial_marker = __heap_start.get() as *mut Marker;
        (*initial_marker).clean();

        let new_marker = (*initial_marker).walk_to_free_satisfing_size(new_size);
        let OffsetSize { item_ptr, .. } = (*new_marker).get_item_ptr_and_size(&new_layout);
        ptr::copy(
            old_item_ptr,
            item_ptr,
            cmp::min(old_size, new_size) * size_of::<Self>(),
        );
        (*new_marker).alloc_unchecked(&new_layout)
    }

    pub unsafe fn walk_alloc(&mut self, layout: &Layout) -> *mut u8 {
        let OffsetSize { size, .. } = self.get_item_ptr_and_size(layout);
        let marker = self.walk_to_free_satisfing_size(size);
        (*marker).alloc_unchecked(layout)
    }

    pub unsafe fn next(&self) -> Option<*mut Marker> {
        self.size()
            .map(|size| (self as *const Marker as *mut Marker).add(size + 1))
    }

    pub unsafe fn walk_to_free(&mut self) -> *mut Marker {
        let mut this_marker = self as *mut Marker;
        loop {
            if !(*this_marker).allocated() {
                return this_marker;
            }
            this_marker = (*this_marker).next().expect("next marker");
        }
    }

    pub unsafe fn walk_to_free_satisfing_size(&mut self, size: usize) -> *mut Marker {
        let mut this_marker = self as *mut Marker;
        loop {
            this_marker = (*this_marker).walk_to_free();
            match (*this_marker).size() {
                None => return this_marker,
                Some(marker_size) if marker_size >= size => return this_marker,
                _ => this_marker = (*this_marker).next().expect("next marker"),
            }
        }
    }

    unsafe fn get_item_ptr_and_size(&self, layout: &Layout) -> OffsetSize {
        let max_align = cmp::max(size_of::<Self>(), layout.align());
        let item_ptr_start = (self as *const Marker).add(1) as *mut u8;
        let item_ptr = item_ptr_start.add(item_ptr_start.align_offset(max_align));

        let offset = item_ptr.offset_from(item_ptr_start) as usize;
        let item_size = layout.size() + offset;
        let size = if item_size % size_of::<Self>() == 0 {
            item_size / size_of::<Self>()
        } else {
            item_size / size_of::<Self>() + 1
        };

        OffsetSize { item_ptr, size }
    }

    unsafe fn clean(&mut self) {
        let mut this_marker = self as *mut Marker;
        while let Some(next_marker) = (*this_marker).next() {
            println!("alloc: clean {:?}", *this_marker);
            if (*this_marker).allocated() || (*next_marker).allocated() {
                this_marker = next_marker;
            } else {
                if let Some(next_next_marker) = (*next_marker).next() {
                    print!("alloc: merge {:?} and {:?} to ", *this_marker, *next_marker);
                    let new_size = next_next_marker.offset_from(this_marker) - 1;
                    (*this_marker).set_size(new_size as usize);
                    println!("{:?}", *this_marker);
                } else {
                    (*this_marker).set_size(0);
                    println!("alloc: remove end {:?}", *this_marker);
                }
            }
        }
    }
}

struct OffsetSize {
    pub item_ptr: *mut u8,
    pub size: usize,
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

impl fmt::Debug for MinimumAllocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut this_marker = unsafe { __heap_start.get() as *mut Marker };
        let mut f = f.debug_list();
        unsafe { f.entry(&*this_marker) };
        while let Some(next_marker) = unsafe { (*this_marker).next() } {
            unsafe {
                f.entry(&*next_marker);
                this_marker = next_marker;
            };
        }
        f.finish()
    }
}

unsafe impl GlobalAlloc for MinimumAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        println!("alloc: New alloc {:?}", layout);
        let this_marker = __heap_start.get() as *mut Marker;
        let a = (*this_marker).walk_alloc(&layout);
        println!("{:#?}", self);
        a
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let initial_marker = __heap_start.get() as *mut Marker;
        let mut this_marker = initial_marker;
        while let Some(next_marker) = (*this_marker).next() {
            println!("dealloc: check {:?}", *this_marker);
            if next_marker.offset(1) as *mut u8 > ptr {
                break;
            }
            this_marker = next_marker;
        }
        (*this_marker).set_allocated(false);
        (*initial_marker).clean();
        println!("{:#?}", self);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());

        let initial_marker = __heap_start.get() as *mut Marker;
        let mut this_marker = initial_marker;
        while let Some(next_marker) = (*this_marker).next() {
            println!("alloc: realloc check {:?}", *this_marker);
            if next_marker.offset(1) as *mut u8 > ptr {
                break;
            }
            this_marker = next_marker;
        }
        let a = (*this_marker).realloc(layout, new_layout);
        println!("{:#?}", self);
        a
    }
}

#[global_allocator]
static ALLOCATOR: MinimumAllocator = MinimumAllocator;

#[alloc_error_handler]
fn alloc_error(_: core::alloc::Layout) -> ! {
    panic!("Alloc Error!")
}
