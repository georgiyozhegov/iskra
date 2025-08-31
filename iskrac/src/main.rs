use iskrac_interner::Interner;
use iskrac_lexer::Lexer;

fn main() {
    let source = include_str!("../etc/example.krm");
    let mut interner = Interner::new();
    for token in Lexer::new(source, &mut interner) {
        println!("{token:?}");
    }
    println!("{interner:?}");
}
