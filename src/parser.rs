// <fields>            ::= [ <negation> ] <fields_struct>
// <fields_struct>     ::= "(" <field_items> ")"
// <field_items>       ::= <field> [ "," <field_items> ]
// <field>             ::= <field_name> | <fields_substruct>
// <fields_substruct>  ::= <field_name> <fields_struct>
// <field_name>        ::= <dash_letter_digit> [ <field_name> ]
// <dash_letter_digit> ::= <dash> | <letter> | <digit>
// <dash>              ::= "-" | "_"
// <letter>            ::= "A" | ... | "Z" | "a" | ... | "z"
// <digit>             ::= "0" | ... | "9"
// <negation>          ::= "!"

use winnow::token::take_while;
use winnow::{PResult, Parser};

struct Fields<'s> {
    negation: bool,
    fields_struct: FieldsStruct<'s>,
}
struct FieldsStruct<'s>(FieldItems<'s>);
struct FieldItems<'s>(Vec<Field<'s>>);
enum Field<'s> {
    FieldName(FieldName<'s>),
    FieldSubstruct(FieldSubstruct<'s>),
}
struct FieldName<'s>(&'s str);
struct FieldSubstruct<'s> {
    field_name: FieldName<'s>,
    fields_struct: FieldsStruct<'s>,
}

impl<'s> FieldName<'s> {
    fn parse(input: &mut &'s str) -> PResult<Self> {
        let field_name =
            take_while(1.., ('-', '_', 'A'..='Z', 'a'..='z', '0'..='9')).parse_next(input)?;
        Ok(Self(field_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn field_name() {
        const VALID: &[&str] = &[
            "a",
            "A",
            "0",
            "_",
            "-",
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_",
        ];
        for &s in VALID {
            let input = &mut (s.clone());
            let field_name = FieldName::parse(input).unwrap();
            assert_eq!(s, field_name.0);
            assert!(input.is_empty());
        }
    }
}
