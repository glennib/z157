#![no_main]

use libfuzzer_sys::Corpus;
use libfuzzer_sys::fuzz_target;
use z157::Tree;

fuzz_target!(|data: &[u8]| -> Corpus {
    let Ok(s) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };
    let result = Tree::parse(s);
    match result {
        Ok(tree) => {
            for field in tree.walk() {
                let _ = field.name();
                let _ = field.has_children();
                let _ = field.parent();
                let _ = field.children();
                let _ = field.path();
            }
            for field in tree.top() {
                let _ = field.walk();
            }
        }
        Err(unparsable) => {
            let _ = unparsable.to_string();
        }
    }
    Corpus::Keep
});
