use std::alloc::{GlobalAlloc, Layout, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        eprintln!("alloc  @ {:p}, {:?}", ptr, layout);
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        eprintln!("dealloc@ {:p}, {:?}", ptr, layout);
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

fn main() {
    println!("isize: {:?}", isize::MAX);
    {
        let mut v = Vec::<isize>::new();
        println!("create");
        let layout = Layout::for_value(&*v);
        eprintln!("crate---------- {:?}", layout);
        println!("push");
        for i in 1..100 {
            v.push(i);
            let layout = Layout::for_value(&*v);
            eprintln!("{:?}---------- {:?}", i, layout);
        }

        for _ in 1..50 {
            println!(
                "vector size: {:?} / {:?}",
                Layout::for_value(&v).size(),
                Layout::for_value(&*v).size()
            );
            v.pop();
        }
        println!("pop");

        println!(
            "vector size: {:?} / {:?}",
            Layout::for_value(&v).size(),
            Layout::for_value(&*v).size()
        );

        println!("drop");
    }
    println!("test");
}
