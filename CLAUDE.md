# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

z157 is a Rust library that parses field filter strings according to [Zalando's RESTful API guideline #157](https://opensource.zalando.com/restful-api-guidelines/#157). It allows HTTP services to parse query parameters that specify which fields a caller wants to include or exclude.

Example: `GET /users/0001?fields=(age,address(street,city))`

## Commands

### Testing and Development
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run a specific test
- `cargo clippy -- -D warnings` - Run lints (CI fails on warnings)
- `cargo +nightly fmt` - Format code (uses nightly rustfmt)
- `cargo +nightly fmt -- --check` - Check formatting without modifying files

### Benchmarks and Examples
- `cargo bench` - Run benchmarks
- `cargo run --example example -- '<filter_string>'` - Run the example with a filter string

### Fuzzing
- Fuzz targets are in `fuzz/fuzz_targets/`
- Uses cargo-fuzz (requires nightly Rust)

### Documentation
- `cargo doc --open` - Build and open documentation locally

## Architecture

### Core Components

**Parser (`src/parser.rs`)**: Uses the `winnow` parser combinator library to parse filter strings according to the grammar. Produces intermediate AST structures (`Fields`, `FieldsStruct`, `FieldItems`, `Field`, `FieldsSubstruct`, `FieldName`).

**Tree (`src/tree.rs`)**: The main public API. Contains:
- `Tree<'buffer>`: Binds a parsed structure to its source buffer using `Cow<'buffer, str>` for flexible ownership
- `DetachedTree`: Internal structure holding `ego_tree::Tree<StrRange>` - a tree of string range references
- `Field<'p>`: A node in the tree with methods for navigation (`parent()`, `children()`, `walk()`) and introspection (`name()`, `path()`, `has_children()`)

**String Ranges (`src/str_range.rs`)**: `StrRange` stores offset+length references into the source buffer instead of pointers, allowing the tree structure to be detached from its buffer and re-attached later.

### Key Design Patterns

The library uses a two-phase approach:
1. Parse the input string into a `DetachedTree` (just offsets/ranges)
2. Attach the buffer to create a `Tree` that can dereference field names

This design allows:
- Efficient parsing without copying field names
- Flexible ownership (borrowed or owned strings via `Cow`)
- Conversion to owned trees with `into_owned()` for longer lifetimes

The `ego_tree` crate provides the underlying tree data structure with efficient parent/child navigation.

## Testing Strategy

- Unit tests are inline in `src/lib.rs`, `src/tree.rs`, and `src/parser.rs`
- Benchmark inputs in `benches/inputs/` are validated in tests to ensure they parse correctly
- Fuzzing tests parse and walk trees to catch edge cases

## Field Filter Grammar

The parser implements this grammar:
```
<fields>            ::= [ <negation> ] <fields_struct>
<fields_struct>     ::= "(" <field_items> ")"
<field_items>       ::= <field> [ "," <field_items> ]
<field>             ::= <field_name> | <fields_substruct>
<fields_substruct>  ::= <field_name> <fields_struct>
<field_name>        ::= <dash_letter_digit> [ <field_name> ]
<dash_letter_digit> ::= "-" | "_" | letter | digit
<negation>          ::= "!"
```

Field names can contain letters, digits, dashes, and underscores. A leading `!` indicates negation (exclude these fields rather than include).
