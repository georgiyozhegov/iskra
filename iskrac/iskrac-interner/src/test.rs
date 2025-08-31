use crate::Interner;

#[test]
fn the_same_string() {
    let mut interner = Interner::new();
    let a = interner.intern("iskra".to_string());
    let b = interner.intern("iskra".to_string());
    assert_eq!(a, b);
}

#[test]
fn two_different_strings() {
    let mut interner = Interner::new();
    let a = interner.intern("iskra".to_string());
    let b = interner.intern("i hate javascript".to_string());
    assert_ne!(a, b);
}

#[test]
fn lots_of_identical_strings() {
    let mut interner = Interner::new();
    let original = interner.intern("rust <3".to_string());
    for _ in 1..100 {
        let duplicate = interner.intern("rust <3".to_string());
        assert_eq!(original, duplicate);
    }
}

#[test]
fn resolve_gives_the_original_string_back() {
    let mut interner = Interner::new();
    let a = interner.intern("george".to_string());
    let resolved = interner.resolve(a);
    assert_eq!(resolved, "george");
}
