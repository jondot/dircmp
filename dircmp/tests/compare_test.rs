use std::path::Path;

use dircmp::Comparison;

#[test]
fn sanity_test() {
    let cmp = Comparison::default();
    let res = cmp
        .compare("tests/fixtures/bar", "tests/fixtures/baz")
        .expect("should compare");
    assert!(res.is_empty()); // no difference
    let res = cmp
        .compare("tests/fixtures/bar", "tests/fixtures/foo")
        .expect("should compare");

    assert!(!res.is_empty()); // has difference
    assert_eq!(
        res.missing_left,
        vec![Path::new("tests/fixtures/foo/new.txt")]
    );
    assert_eq!(res.changed, vec![Path::new("tests/fixtures/bar/a.txt")]);
    assert!(res.missing_right.is_empty());
    assert!(res.different_type.is_empty());
    assert!(res.similar.is_empty());
}
