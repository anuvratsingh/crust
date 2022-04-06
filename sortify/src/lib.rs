pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}
mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;

pub struct StdSorter;

impl<T> Sorter<T> for StdSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

pub struct StdUnstableSorter;

impl<T> Sorter<T> for StdUnstableSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort_unstable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn std_works() {
        let mut x = vec![4, 3, 1, 2];
        StdSorter.sort(&mut x);
        assert_eq!(x, &[1, 2, 3, 4]);
    }

    #[test]
    fn stdunstable_works() {
        let mut x = vec![4, 2, 3, 1];
        StdUnstableSorter.sort(&mut x);
        assert_eq!(x, &[1, 2, 3, 4]);
    }
}
