use iskrac_runer::Runer;

fn main() {
    let source = include_str!("../etc/example.krm");
    let mut runer = Runer::new(source);
    runer.run();
}
