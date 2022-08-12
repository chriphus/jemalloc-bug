use std::ops::Add;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    let test = String::from("Hello World!");

    println!("{}", test);
}
