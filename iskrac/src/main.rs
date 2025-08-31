use iskra_lexer::Lexer;

fn main() {
    let source = include_str!("../etc/example.krm");
    for token in Lexer::new(source) {
        println!("{token:?}");
    }
}
