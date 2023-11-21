// You can run this example from the root of the tracking_allocator repo:
// cargo run --example measure_vec
use rand::{thread_rng, Fill};
use tracking_allocator::{Stats, TrackingAllocator, Vec};

pub fn test(name: &str, size: usize, allocator: &TrackingAllocator) {
    let mut rng = thread_rng();
    let mut data: [u8; 100] = [0; 100];
    data.try_fill(&mut rng).expect("filling data error");
    let val = Data { data };

    tracking_allocator::reset();
    let mut v = Vec::new_in(allocator);
    for _i in 0..size {
        v.push(val)
    }

    let Stats {
        alloc,
        dealloc,
        diff,
    } = tracking_allocator::stats();
    println!("{name},{size},{alloc},{dealloc},{diff}");

    drop(v);
}

#[derive(Clone, Copy, Debug)]
pub struct Data {
    pub data: [u8; 100],
}

fn main() {
    let mut sizes: Vec<usize> = Vec::new();
    sizes.push(0);
    sizes.push(10);
    sizes.push(100);
    sizes.push(1000);

    for size in (10_000..=1_000_000).step_by(10_000) {
        sizes.push(size);
    }

    println!("name,size,alloced,dealloced,diff");
    let alloc = TrackingAllocator;

    for size in sizes {
        test("vec", size, &alloc);
    }
}
