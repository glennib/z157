//! Parse filter fields according to <https://opensource.zalando.com/restful-api-guidelines/#157>.
//!
//! # Example
//!
//! ```
//! use z157::Tree;
//!
//! // Select fields to include
//! let tree = Tree::parse("(name,bio(height(meters,centimeters),age))").unwrap();
//!
//! assert!(!tree.negation());
//! let height = tree.index(&["bio", "height"]).unwrap();
//! assert!(height.children().any(|field| field.name() == "meters"));
//! assert!(height.children().any(|field| field.name() == "centimeters"));
//!
//! for field in tree.walk() {
//!     // z157::Field::path returns a vector of ancestors from the top-level
//!     // field name until and including itself.
//!     println!("{}", field.path().join("."));
//!     // This would print out:
//!     // name
//!     // bio
//!     // bio.height
//!     // bio.height.meters
//!     // bio.height.centimeters
//!     // bio.age
//! }
//!
//! // Select fields to exclude
//! let tree = Tree::parse("-(bio)").unwrap();
//!
//! assert!(tree.negation());
//! ```
//!
//! # Specification
//!
//! From the Zalando RESTful API guidelines:
//!
//! ```text
//! <fields>            ::= [ <negation> ] <fields_struct>
//! <fields_struct>     ::= "(" <field_items> ")"
//! <field_items>       ::= <field> [ "," <field_items> ]
//! <field>             ::= <field_name> | <fields_substruct>
//! <fields_substruct>  ::= <field_name> <fields_struct>
//! <field_name>        ::= <dash_letter_digit> [ <field_name> ]
//! <dash_letter_digit> ::= <dash> | <letter> | <digit>
//! <dash>              ::= "-" | "_"
//! <letter>            ::= "A" | ... | "Z" | "a" | ... | "z"
//! <digit>             ::= "0" | ... | "9"
//! <negation>          ::= "!"
//! ```

mod parser;
mod str_range;
mod tree;

pub use tree::Children;
pub use tree::Field;
pub use tree::Tree;
pub use tree::Unparsable;
pub use tree::Walk;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_index() {
        let tree = Tree::parse("(a(b,c(d)),e)").unwrap();
        assert!(!tree.negation());
        tree.index(&["a", "b"]).unwrap();
        tree.index(&["a", "c"]).unwrap();
        tree.index(&["a", "c", "d"]).unwrap();
        tree.index(&["e"]).unwrap();
        assert!(tree.index(&["a", "d"]).is_none());
    }

    #[test]
    fn benchmark_input_ok() {
        Tree::parse(include_str!("../benches/inputs/large-input.txt")).unwrap();
        Tree::parse(include_str!("../benches/inputs/small-input.txt")).unwrap();
    }
}
