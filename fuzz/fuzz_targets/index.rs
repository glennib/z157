#![no_main]

use libfuzzer_sys::Corpus;
use libfuzzer_sys::fuzz_target;
use z157::Tree;

fuzz_target!(|data: &[u8]| -> Corpus {
    let Ok(s) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };
    let paths: Vec<Vec<&str>> = s.lines().map(|line| line.split('.').collect()).collect();
    let tree = Tree::parse(include_str!("index-input.txt")).unwrap();
    for path in paths {
        if let Some(field) = tree.index(&path) {
            let _ = field.name();
        }
    }

    Corpus::Keep
});
