// For a binary to used with valgrind

use nvec::NVec;

fn main() {
    let mut vec = NVec::new();
    // let mut vec =  Vec::new();
    for i in 1..=1000 {
        vec.push(i)
    }

    assert_eq!(vec.capacity(), 1024); // Allocated on the heap of size 2^n
    assert_eq!(vec.len(), 1000);
}
