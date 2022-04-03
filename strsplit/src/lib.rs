#[derive(Debug)] // D can be anything that impl Delimiter
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

// str ~> [char] Stack or Heap
// &str ~> &[char] Stack or Heap
// String ~> Vec<char> Heap only
// String -> &str String knows the pointer to &str Cheap \ AsRef \
// &str -> String Hard: &str doesn't knows String's pointer \ Expensive Clone \

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

// impl a struct for life time <'T>
//                                     We can skip a lifetime
impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        // `&mut a` matches pattern `&mut T` where a == T while `ref mut a` on `T` would give `&mut T` taking a mutable reference
        // if let Some(ref mut remainder) = self.remainder {

        // here `let` is being used to pattern match `as_mut()` gives `Option<&mut T>` and `?` opens it up
        // let remainder = self.remainder.as_mut()?;
        // if let Some(next_delim) = remainder.find(self.delimiter) {
        //     let until_delimiter = &remainder[..next_delim];
        //     // &mut &'a str  != &'a str, so we deref it using `*`
        //     *remainder = &remainder[(next_delim + self.delimiter.len())..];
        //     Some(until_delimiter)
        // } else {
        //     // Take, litreally take out `Some(_)` and replace it with `None` if there exists `Some(_)` else `None`
        //     self.remainder.take()
        // }
        // // } else {
        // // None
        // // }

        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

#[allow(dead_code)] // returns &str with `s`'s lifetime
fn until_char(s: &str, c: char) -> &str {
    // Happy when s and c have different lifetimes
    // Does heap allocation
    // let delim = &format!("{}", c);
    // Here s and c have different lifetimes but StrSplit expects them to have same
    // To compensate rust take the shorter lifetime `c` and assign it to `s`
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")

    // delim goes out of scope
}

#[test]
fn it_works() {
    let haystack = "a b c d e f";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e", "f"]);
}
#[test]
fn tail() {
    let haystack = "a b c d e ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e", ""]);
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("Hello", 'l'), "He");
}
