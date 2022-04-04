// Declarative Macros are just substitution

#[macro_export] // pub for macro
macro_rules! avec {
    // Should be valid rust grammer
    // Each arm returns an expression, here `Vec::new()` is one so {{block}} isn't required
    // () => {
    //     Vec::new()
    // };
    // When writting expr use {{}}  blocks
    // $(),(+,?,*) Repetation with one or more, zero or more, optional, where `,` denotes the seperation
    // $(,)? Zero or One `,` in the end
    ($($element: expr),*) => {{  // Vec::new() can be removed when using * instead of +
        #[allow(unused_mut)] // In case of empty vec when we never push
        // check that count is const
        const C: usize = $crate::count![@COUNT; $($element),*];
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(C);
        // let mut vs = Vec::new();
        $(vs.push($element);)* // Repeat (x) as many time as given from above
        vs // Block return an expression
    }};

    // not allow !avec[,] empty array with `,`
    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
        }};

    // Macros couldn't ask for expr to imp `Clone` it will generate the code and error out and error would propagate to call
    ($element: expr; $count: expr) => {{
        let count = $count;
        // let mut vs = Vec::with_capacity(count); // Optimizng for Vec reallocation

        // Copying the expr
        // let x = $element;

        // extend already knows the size of iter so it can allocate only once
        //        ::std root level path
        // vs.extend(::std::iter::repeat($element).take(count)); // iterate and add element upto count
        // for _ in 0..count {
        //     vs.push(x.clone());  // push is expensive
        // }
        let mut vs = Vec::new();
        vs.resize(count, $element); // better than extend as it doesnt do bound checks
        vs
    }};
}
#[macro_export]
#[doc(hidden)]
macro_rules! count {
    // internal macro
    (@COUNT; $($element:expr), *) => {
        // [$(($element)),*].len() // we are consuming the input
        // Make an array, take it ref and call len impl for slices of unit
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
        // () is zero sized type so complier can be sure to does't allocate it
    };

    (@SUBST; $_element:expr) => {()}; // for input return unit type ()
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    // let x: Vec<u32> = avec![,]; // this is legal now but std library doesn't allow it
    assert!(x.is_empty());
}
#[test]
fn single() {
    let x: Vec<u32> = avec! {10}; // All (), {}, [] are valid delimiter with same meaning, call can choose any
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 10);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![10, 11];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 10);
    assert_eq!(x[1], 11);
}

#[test]
fn tail() {
    let x: Vec<u32> = avec![
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 32, 33,
        34, 35, 36, 37,
    ];
    assert!(!x.is_empty());
    assert_eq!(x[x.len() - 1], 37);
}

#[test]
fn clone_2() {
    let x: Vec<u32> = avec![10; 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 10);
    assert_eq!(x[1], 10);
}

#[test]
fn some_avec() {
    let mut a = Some(10);
    let x: Vec<u32> = avec!(a.take().unwrap();2);
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 10);
    assert_eq!(x[1], 10);
}

/// ```compile_fail
/// let x:Vec<u32> = vecmac::avec![42, "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;
