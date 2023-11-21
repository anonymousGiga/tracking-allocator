// You can run this example from the root of the tracking_allocator repo:
// cargo run --example measure_hashmap
use hashbrown::HashMap;
use tracking_allocator::{Stats, TrackingAllocator};

use rand::{thread_rng, Fill, Rng};

pub fn test<T>(name: &str, size: usize, f: impl FnOnce() -> T) {
    tracking_allocator::reset();

    let t = f();

    let Stats {
        alloc,
        dealloc,
        diff,
    } = tracking_allocator::stats();
    println!("{name},{size},{alloc},{dealloc},{diff}");

    drop(t);
}

#[derive(Clone, Copy, Debug)]
pub struct Data {
    pub data: [u8; 100],
}

pub fn generate_mock_datas(len: usize) -> Vec<(u64, Data)> {
    let mut rng = thread_rng();
    let mut datas = Vec::with_capacity(len);

    for _ in 0..len {
        let mut data: [u8; 100] = [0; 100];
        data.try_fill(&mut rng).expect("filling data error");
        let val = Data { data };
        let key = rng.gen();
        datas.push((key, val));
    }

    datas
}

fn main() {
    let mock_datas = generate_mock_datas(1_000_000);
    let mut sizes: Vec<usize> = vec![0, 10, 100, 1_000];
    for size in (10_000..=1_000_000).step_by(10_000) {
        sizes.push(size);
    }

    println!("name,size,alloced,dealloced,diff");
    let alloc = TrackingAllocator;
    for size in sizes {
        test("hashmap", size, || {
            let mut m = HashMap::new_in(&alloc);

            for (key, val) in &mock_datas[..size] {
                m.insert(*key, *val);
            }

            m
        });
    }
}
