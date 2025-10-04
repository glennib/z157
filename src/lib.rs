//! Parse filter fields according to <https://opensource.zalando.com/restful-api-guidelines/#157>.
//!
//! # Example
//!
//! ```
//! use z157::Tree;
//!
//! // Select fields to include
//! let tree = Tree::parse(
//!     "(name,bio(height(meters,centimeters),age))",
//! )
//! .unwrap();
//!
//! assert!(!tree.negation());
//! let height = tree.index(&["bio", "height"]).unwrap();
//! assert!(
//!     height
//!         .children()
//!         .any(|field| field.name() == "meters")
//! );
//! assert!(
//!     height
//!         .children()
//!         .any(|field| field.name() == "centimeters")
//! );
//!
//! let nested_fields: Vec<_> = tree
//!     .walk()
//!     .map(|field| field.path().join("."))
//!     .collect();
//! assert_eq!(
//!     nested_fields,
//!     [
//!         "name",
//!         "bio",
//!         "bio.height",
//!         "bio.height.meters",
//!         "bio.height.centimeters",
//!         "bio.age",
//!     ]
//! );
//!
//! // Select fields to exclude
//! let tree = Tree::parse("!(bio)").unwrap();
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

#[cfg(feature = "serde")]
mod filter;

pub use tree::Field;
pub use tree::Tree;
pub use tree::Unparsable;

#[cfg(feature = "serde")]
pub use filter::Filter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_index() {
        let tree = Tree::parse("(a(b,c(d)),e)").unwrap();
        assert!(!tree.negation());
        tree.index(&["a", "b"]).unwrap();
        tree.index(&["a", "c"]).unwrap();
        tree.index(&["a", "c", "d"]).unwrap();
        tree.index(&["e"]).unwrap();
        assert!(tree.index(&["a", "d"]).is_none());
    }

    #[test]
    fn test_benchmark_input_ok() {
        Tree::parse(include_str!("../benches/inputs/large-input.txt")).unwrap();
        Tree::parse(include_str!("../benches/inputs/small-input.txt")).unwrap();
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_filter() {
        use serde::Serialize;

        #[derive(Serialize)]
        struct Address {
            street: String,
            city: String,
            zip: String,
        }

        #[derive(Serialize)]
        struct User {
            name: String,
            age: u32,
            email: String,
            address: Address,
        }

        let user = User {
            name: "Alice".to_string(),
            age: 30,
            email: "alice@example.com".to_string(),
            address: Address {
                street: "123 Main St".to_string(),
                city: "Springfield".to_string(),
                zip: "12345".to_string(),
            },
        };

        // Test basic field filtering
        let tree = Tree::parse("(name,address(city))").unwrap();
        let filtered = tree.filter(&user);
        let json = serde_json::to_value(&filtered).unwrap();

        assert_eq!(json["name"], "Alice");
        assert_eq!(json["address"]["city"], "Springfield");
        assert!(json["age"].is_null());
        assert!(json["email"].is_null());
        assert!(json["address"]["street"].is_null());
        assert!(json["address"]["zip"].is_null());
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_filter_negation() {
        use serde::Serialize;

        #[derive(Serialize)]
        struct User {
            name: String,
            password: String,
            email: String,
        }

        let user = User {
            name: "Bob".to_string(),
            password: "secret123".to_string(),
            email: "bob@example.com".to_string(),
        };

        // Test negation - exclude password
        let tree = Tree::parse("!(password)").unwrap();
        let filtered = tree.filter(&user);
        let json = serde_json::to_value(&filtered).unwrap();

        assert_eq!(json["name"], "Bob");
        assert_eq!(json["email"], "bob@example.com");
        assert!(json["password"].is_null());
    }
}
