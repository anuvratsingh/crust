use rand::prelude::*;
use std::{cell::Cell, cmp::Ordering, rc::Rc};

use sortify::{
    BubbleSort, InsertionSort, QuickSort, SelectionSort, Sorter, StdSorter, StdUnstableSorter,
};
#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}
impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}
impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));

    for &n in &[0, 1, 10, 100, 1000, 10000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            })
        }

        // let bench = |sorter: &dyn Sorter| {
        //     let mut values = values.clone();
        //     counter.set(0);
        //     sorter.sort(&mut values);
        //     counter.get();
        // };
        for _ in 0..10 {
            values.shuffle(&mut rand);

            let took = bench(BubbleSort, &values, &counter);
            println!("{} {} {} {}", "bubble", n, took.0, took.1);
            let took = bench(InsertionSort { smart: true }, &values, &counter);
            println!("{} {} {} {}", "insertion-smart", n, took.0, took.1);
            let took = bench(InsertionSort { smart: false }, &values, &counter);
            println!("{} {} {} {}", "insertion-dumb", n, took.0, took.1);
            let took = bench(SelectionSort, &values, &counter);
            println!("{} {} {} {}", "selection", n, took.0, took.1);
            let took = bench(QuickSort, &values, &counter);
            println!("{} {} {} {}", "quick", n, took.0, took.1);

            // STD lib
            let took = bench(StdUnstableSorter, &values, &counter);
            println!("{} {} {} {}", "std_unstable", n, took.0, took.1);
            let took = bench(StdSorter, &values, &counter);
            println!("{} {} {} {}", "std", n, took.0, took.1);
        }
    }
}

fn bench<T: Ord + Clone, S: Sorter<SortEvaluator<T>>>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> (usize, f64) {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    let count = counter.get();
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1])
    }
    (count, took.as_secs_f64())
}
