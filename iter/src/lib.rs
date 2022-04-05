pub trait IteratorExt: Iterator {
    fn our_flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator,
    {
        flatten(self)
    }
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
        Self: Sized,
    {
        flatten(self)
    }
}
pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

// Flattening requires allocation, current approch runs on infinite array
impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut front_iter) = self.front_iter {
                // check front_iter value, if None skip the block
                if let Some(i) = front_iter.next() {
                    return Some(i);
                }
                self.front_iter = None;
            }
            if let Some(next_inner) = self.outer.next() {
                // check if outer has next() value, set it to front iter goes up again
                self.front_iter = Some(next_inner.into_iter());
            } else {
                // return self.back_iter.as_mut()?.next();// if outer doesn't have next(), return outer_iter which is None
                return None;
            }
        }

        // loop { // Don't know what is breaking this loop
        //     if let Some(ref mut inner_iter) = self.inner {
        //         if let Some(i) = inner_iter.next() {
        //             return Some(i);
        //         }
        //         self.inner = None;
        //     }
        //     let next_inner_iter = self.outer.next()?.into_iter();
        //     self.inner = Some(next_inner_iter);
        // }

        // self.outer.next().and_then(|inner| inner.into_iter().next())
        // Easily readable form
        // let inner_item = self.outer.next()?; // every next calls the whole iter
        // let mut inner_it = inner_item.into_iter(); //
        // inner_it.next() // returns the first one and drop everything, it might contain other items
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <<O as Iterator>::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }
            if let Some(next_back_inner) = self.outer.next_back() {
                self.back_iter = Some(next_back_inner.into_iter());
            } else {
                // return self.front_iter.as_mut()?.next_back();
                return None;
            }
        }
    }

    //     loop {
    //         if let Some(ref mut inner_iter) = self.inner {
    //             if let Some(i) = inner_iter.next_back() {
    //                 return Some(i);
    //             }
    //             self.inner = None;
    //         }

    //         let next_inner_iter = self.outer.next_back()?.into_iter();
    //         self.inner = Some(next_inner_iter)
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }
    #[test]
    fn vec_e_vec() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0)
    }
    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1)
    }
    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2)
    }

    #[test]
    fn vec_o_vec() {
        assert_eq!(flatten(std::iter::once(vec![vec!["a"]])).count(), 1)
    }

    #[test]
    fn vec_t_vec() {
        assert_eq!(
            flatten(std::iter::once(vec![vec!["a"], vec!["b"]])).count(),
            2
        )
    }

    // Double ended iter
    #[test]
    fn rev_empty() {
        assert_eq!(
            flatten(std::iter::empty::<Vec<()>>())
                .rev()
                .collect::<Vec<()>>(),
            vec![]
        )
    }

    #[test]
    fn rev_two() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }

    #[test]
    fn rev_three() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b", "c"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["c", "b", "a"]
        )
    }

    #[test]
    fn rev_vec_t_vec() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"], vec!["c"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["c", "b", "a"]
        )
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3")); // this return a3 as we are only storing one inner loop
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    // Inf array
    #[test]
    fn inf() {
        let mut iter = flatten((0..).map(|i| 0..i));
        assert_eq!(iter.next(), Some(0));
    }

    #[test]
    fn deep() {
        assert_eq!(flatten(flatten(vec![vec![vec![0, 1]]])).count(), 2);
    }

    #[test]
    fn ext() {
        assert_eq!(vec![vec![0, 1]].into_iter().our_flatten().count(), 2);
    }
}
