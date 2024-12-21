# z157

[![Crates.io](https://img.shields.io/crates/v/z157.svg)](https://crates.io/crates/z157)
[![docs.rs (with version)](https://img.shields.io/docsrs/z157/latest)](https://docs.rs/z157/latest/z157/)

Parser for the field name filter described
in [Zalando's RESTful API and Event guideline #157](https://opensource.zalando.com/restful-api-guidelines/#157).

## When do I need this?

If your HTTP service accepts a query parameter that lets the caller specify which fields
they would like, this crate helps you parse such a string.

```
GET http://localhost/users/0001?fields=(age,address(street,city))
```

This crate helps you parse the value of the `fields` query parameter into a tree of fields.

## Example

```rust
use z157::Tree;

fn main() {
    let tree = Tree::try_from("(name,bio(height(meters,centimeters),age))"
        .to_string()).unwrap();

    assert!(!tree.negation());
    let height = tree.index(&["bio", "height"]).unwrap();
    assert!(height.children().any(|field| field.name() == "meters"));
    assert!(height.children().any(|field| field.name() == "centimeters"));

    for field in tree.walk() {
        // z157::Field::path returns a vector of ancestors from the top-level
        // field name until and including itself.
        println!("{:?}", field.path());
    }

    let tree: Tree = "-(bio)".to_string().try_into().unwrap();

    assert!(tree.negation());
}
```

## The field filter specification

```
<fields>            ::= [ <negation> ] <fields_struct>
<fields_struct>     ::= "(" <field_items> ")"
<field_items>       ::= <field> [ "," <field_items> ]
<field>             ::= <field_name> | <fields_substruct>
<fields_substruct>  ::= <field_name> <fields_struct>
<field_name>        ::= <dash_letter_digit> [ <field_name> ]
<dash_letter_digit> ::= <dash> | <letter> | <digit>
<dash>              ::= "-" | "_"
<letter>            ::= "A" | ... | "Z" | "a" | ... | "z"
<digit>             ::= "0" | ... | "9"
<negation>          ::= "!"
```
