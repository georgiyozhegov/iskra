use iskrac_interner::Interner;
use iskrac_lexer::Lexer;

pub struct Runer<'s> {
    source: &'s str,
    interner: Interner,
}

impl<'s> Runer<'s> {
    pub fn new(source: &'s str) -> Self {
        Self { source, interner: Interner::new() }
    }

    pub fn run(&mut self) {
        let lexer = Lexer::new(self.source, &mut self.interner);
        for token in lexer {
            println!("{token:?}");
        }
        println!("{:?}", self.interner);
    }
}
