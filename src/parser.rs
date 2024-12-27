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

use std::fmt;

use winnow::PResult;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::combinator::delimited;
use winnow::combinator::opt;
use winnow::combinator::separated;
use winnow::token::take_while;

pub struct Fields<'s> {
    pub fields_struct: FieldsStruct<'s>,
    pub negation: bool,
}

#[derive(Debug)]
pub struct Error {
    parse_error_message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse: {}", self.parse_error_message)
    }
}

impl std::error::Error for Error {}

impl<'s> TryFrom<&'s str> for Fields<'s> {
    type Error = Error;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        Self::parse.parse(value).map_err(|parse_error| Error {
            parse_error_message: parse_error.to_string(),
        })
    }
}

pub struct FieldsStruct<'s>(pub FieldItems<'s>);
pub struct FieldItems<'s>(pub Vec<Field<'s>>);
pub enum Field<'s> {
    FieldsSubstruct(FieldsSubstruct<'s>),
    FieldName(FieldName<'s>),
}
pub struct FieldsSubstruct<'s> {
    pub field_name: FieldName<'s>,
    pub fields_struct: FieldsStruct<'s>,
}

#[derive(Debug)]
pub struct FieldName<'s>(pub &'s str);

impl<'s> FieldName<'s> {
    fn parse(input: &mut &'s str) -> PResult<Self> {
        let field_name =
            take_while(1.., ('-', '_', 'A'..='Z', 'a'..='z', '0'..='9')).parse_next(input)?;
        Ok(Self(field_name))
    }
}

impl<'s> FieldsSubstruct<'s> {
    fn parse(input: &mut &'s str) -> PResult<Self> {
        let field_name = FieldName::parse.parse_next(input)?;
        let fields_struct = FieldsStruct::parse.parse_next(input)?;
        Ok(Self {
            field_name,
            fields_struct,
        })
    }
}

impl<'s> Field<'s> {
    fn parse(input: &mut &'s str) -> PResult<Self> {
        alt((
            FieldsSubstruct::parse.map(Self::FieldsSubstruct),
            FieldName::parse.map(Self::FieldName),
        ))
        .parse_next(input)
    }
}

impl<'s> FieldItems<'s> {
    fn parse(input: &mut &'s str) -> PResult<Self> {
        Ok(Self(separated(1.., Field::parse, ',').parse_next(input)?))
    }
}
impl<'s> FieldsStruct<'s> {
    fn parse(input: &mut &'s str) -> PResult<Self> {
        Ok(Self(
            delimited('(', FieldItems::parse, ')').parse_next(input)?,
        ))
    }
}

impl<'s> Fields<'s> {
    fn parse(input: &mut &'s str) -> PResult<Self> {
        let negation = opt('-').parse_next(input)?.is_some();
        let fields_struct = FieldsStruct::parse.parse_next(input)?;
        Ok(Self {
            fields_struct,
            negation,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn field_name() {
        // clippy or rustc seem to have a false positive on code like
        // let input = &mut (s.clone()); below.
        #![allow(noop_method_call)]

        const VALID: &[&str] = &[
            "a",
            "A",
            "0",
            "_",
            "-",
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_",
        ];
        const INVALID: &[&str] = &["", "!", "abc/"];

        for &s in VALID {
            let input = &mut (s.clone());
            let field_name = FieldName::parse(input).unwrap();
            assert_eq!(s, field_name.0);
            assert!(input.is_empty());
        }

        for &s in INVALID {
            let input = &mut (s.clone());
            let field_name = FieldName::parse(input);
            assert!(
                !input.is_empty() || field_name.is_err(),
                "{s}\n{field_name:?}"
            );
        }
    }

    #[test]
    fn fields() {
        let s = "-(field_a)";
        let fields: Fields = s.try_into().unwrap();
        assert!(fields.negation);
        let field_items = fields.fields_struct.0.0;
        assert_eq!(field_items.len(), 1);
        assert!(matches!(
            field_items[0],
            Field::FieldName(FieldName("field_a"))
        ));

        let s = "(field_a(field_b,field_c(field_d)),field_d)";
        let fields: Fields = s.try_into().unwrap();
        assert!(!fields.negation);
        let field_items = fields.fields_struct.0.0;
        assert_eq!(field_items.len(), 2);
        assert!(matches!(field_items[0], Field::FieldsSubstruct(_)));
        assert!(matches!(
            field_items[1],
            Field::FieldName(FieldName("field_d"))
        ));
    }
}
