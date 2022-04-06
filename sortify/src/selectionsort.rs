use crate::Sorter;

pub struct SelectionSort;
impl<T> Sorter<T> for SelectionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len() {
            let smallest_in_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| unsorted + i)
                .expect("slice is non-empty");

            // let smallest_in_rest = unsorted + smallest_in_rest;
            // let mut smallest_in_rest2 = unsorted;
            // for i in (unsorted + 1)..slice.len() {
            //     if slice[i] < slice[smallest_in_rest2] {
            //         smallest_in_rest2 = i;
            //     }
            // }

            // assert_eq!(smallest_in_rest, smallest_in_rest2);

            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

#[test]
fn it_works() {
    let mut x = vec![4, 1, 3, 2];
    SelectionSort.sort(&mut x);
    assert_eq!(x, &[1, 2, 3, 4]);
}
