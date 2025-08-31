use crate::Interner;

#[test]
fn the_same_string() {
    let mut interner = Interner::new();
    let a = interner.intern("iskra");
    let b = interner.intern("iskra");
    assert_eq!(a, b);
}

#[test]
fn two_different_strings() {
    let mut interner = Interner::new();
    let a = interner.intern("iskra");
    let b = interner.intern("i hate javascript");
    assert_ne!(a, b);
}

#[test]
fn lots_of_identical_strings() {
    let mut interner = Interner::new();
    let original = interner.intern("rust <3");
    for _ in 1..100 {
        let duplicate = interner.intern("rust <3");
        assert_eq!(original, duplicate);
    }
}

#[test]
fn resolve_gives_the_original_string_back() {
    let mut interner = Interner::new();
    let a = interner.intern("george");
    let resolved = interner.resolve(a);
    assert_eq!(resolved, "george");
}
