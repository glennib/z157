# z157

[![Crates.io](https://img.shields.io/crates/v/z157.svg)](https://crates.io/crates/z157)
[![docs.rs (with version)](https://img.shields.io/docsrs/z157/latest)](https://docs.rs/z157/latest/z157/)

Parser for the field filter defined
in [Zalando's RESTful API and Event guideline #157](https://opensource.zalando.com/restful-api-guidelines/#157).

## When would I need this?

If your HTTP service accepts a query parameter that lets the caller specify which fields
they would like, this crate helps you parse such a string.

```
GET http://localhost/users/0001?fields=(age,address(street,city))
```

The value of the `fields` query parameter is parsed into a tree of field names.

## Example

An example program is found in the [`examples/`](./examples) directory.
You can run it like this:

```shell
cargo run --example example -- '-(name,bio(height_cm),last_seen)'
```

A more simple example is shown below:

```rust
use z157::Tree;

fn main() {
    // Select fields to include
    let tree = Tree::parse("(name,bio(height(meters,centimeters),age))").unwrap();

    assert!(!tree.negation());
    let height = tree.index(&["bio", "height"]).unwrap();
    assert!(height.children().any(|field| field.name() == "meters"));
    assert!(height.children().any(|field| field.name() == "centimeters"));

    for field in tree.walk() {
        // z157::Field::path returns a vector of ancestors from the top-level
        // field name until and including itself.
        println!("{}", field.path().join("."));
        // This would print out:
        // name
        // bio
        // bio.height
        // bio.height.meters
        // bio.height.centimeters
        // bio.age
    }

    // Select fields to exclude
    let tree = Tree::parse("!(bio)").unwrap();

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
