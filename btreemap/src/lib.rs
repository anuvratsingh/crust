#[allow(unused_imports)]
use std::collections::BTreeMap;

#[macro_export]
macro_rules! abmap {

    ($($key: expr, $value: expr);*) => {{
        #[allow(unused_mut)]
        let mut bmap = std::collections::BTreeMap::new();
        $(bmap.insert($key, $value);)*
        bmap
    }};

    ($($key: expr, $value: expr;)*) => {{
        $crate::abmap!($($key, $value);*)
    }};
}

#[test]
fn empty_tree() {
    let x: BTreeMap<u32, u32> = abmap![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x = abmap!(1, "a");
    assert_eq!(x[&1], "a");
    assert_eq!(x.get_key_value(&1), Some((&1, &"a")));
    assert_eq!(x.get(&5), None);
}

#[test]
fn double() {
    let x = abmap!(1, "a"; 2, "b");
    assert_eq!(x[&1], "a");
    assert_eq!(x[&2], "b");
    assert_eq!(x.get(&5), None);
}

#[test]
fn tail() {
    let x = abmap!(1, "a"; 2,"b"; 3, "c";);
    assert_eq!(x.len(), 3);
    assert_eq!(x[&1], "a");
    assert_eq!(x[&2], "b");
    assert_eq!(x.get(&5), None);
}

#[test]
fn some_abmap() {
    let mut a = Some("a");
    let mut b = Some("b");
    let x = abmap!(1, a.take().unwrap();2, b.take().unwrap());
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[&1], "a");
    assert_eq!(x[&2], "b");
    assert_eq!(x.get(&5), None);
}

#[test]
fn nested_map() {
    let x = abmap!("a1", abmap!("a2", 1 + 1); "b1", abmap!("b2", 2+2));
    assert_eq!(x.len(), 2);
    assert_eq!(x[&"a1"], abmap!("a2", 2));
}
