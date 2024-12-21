//! Parse filter fields according to <https://opensource.zalando.com/restful-api-guidelines/#157>.
//!
//! # Example
//!
//! ```
//! use z157::Tree;
//!
//! let params: Tree = "(name,bio(height(meters,centimeters),age))"
//!     .to_string().try_into().unwrap();
//!
//! assert!(!params.negation());
//! let height = params.index(&["bio", "height"]).unwrap();
//! assert!(height.children().any(|param| param.field_name() == "meters"));
//! assert!(height.children().any(|param| param.field_name() == "centimeters"));
//!
//! for param in params.walk() {
//!     // z157::Param::path returns a vector of ancestors from the top-level
//!     // field name until and including itself.
//!     println!("{:?}", param.path());
//! }
//!
//! let params: Tree = "-(bio)".to_string().try_into().unwrap();
//!
//! assert!(params.negation());
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
pub use tree::Error;
pub use tree::Field;
pub use tree::Tree;
pub use tree::Walk;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_index() {
        let params: Tree = "(a(b,c(d)),e)".to_string().try_into().unwrap();
        assert!(!params.negation());
        params.index(&["a", "b"]).unwrap();
        params.index(&["a", "c"]).unwrap();
        params.index(&["a", "c", "d"]).unwrap();
        params.index(&["e"]).unwrap();
        assert!(params.index(&["a", "d"]).is_none());
    }
}
