# z157

Parser for the field name filter described
in [Zalando's RESTful API and Event guideline #157](https://opensource.zalando.com/restful-api-guidelines/#157).

## Example

```rust
use z157::Params;

let params: Params = "(name,bio(height(meters,centimeters),age))".parse().unwrap();

assert!(!params.negation());
let height = params.index(&["bio", "height"]).unwrap();
assert!(height.children().any(|param| param.field_name() == "meters"));
assert!(height.children().any(|param| param.field_name() == "centimeters"));

for param in params.walk() {
    // z157::Param::path returns a vector of ancestors from the top-level field name until
    // and including itself.
    println!("{:?}", param.path());
}

let params: Params = "-(bio)".parse().unwrap();

assert!(params.negation());
```