use std::fs;

use crate::Lexer;

#[test]
fn run() {
    let update_target = std::env::var("UPDATE_TARGET").is_ok_and(|v| v == "1");

    for entry in std::fs::read_dir("test").unwrap() {
        let mut path = entry.unwrap().path();

        if path.extension().unwrap() != "source" {
            continue;
        }

        println!("> {path:?}");

        let source = std::fs::read_to_string(&path).unwrap();
        let lexer = Lexer::new(&source);
        let actual: Vec<_> = lexer.map(|t| format!("{t:?}")).collect();

        path.set_extension("target");

        if update_target {
            let target = actual.join("\n");
            fs::write(path, target).unwrap();
            continue;
        }

        let target = std::fs::read_to_string(&path).unwrap();
        let target: Vec<_> = target.lines().collect();

        assert_eq!(actual.len(), target.len());
        for (line, (a, t)) in actual.iter().zip(target).enumerate() {
            assert_eq!(a, t, "{path:?} at line {line}");
        }
    }
}
