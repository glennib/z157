use std::fmt;

use crate::parser;
use crate::str_range::StrRange;

/// Contains fields parsed from a fields filtering string.
///
/// See usage examples in the [crate documentation](crate).
pub struct Tree {
    buffer: String,
    tree: ego_tree::Tree<StrRange>,
    negation: bool,
}

impl Tree {
    /// Attempt to parse `s` into `Fields` and create a tree of fields from it.
    ///
    /// Construct via `TryFrom`.
    ///
    /// # Errors
    ///
    /// If the string cannot be parsed into fields, an error is returned.
    #[allow(clippy::missing_panics_doc)] // panics should be impossible
    pub fn parse(s: impl Into<String>) -> Result<Self, Unparsable> {
        let s = s.into();
        let fields = match parser::Fields::try_from(s.as_str()) {
            Ok(fields) => fields,
            Err(error) => {
                return Err(Unparsable {
                    unparsable: s,
                    inner: error,
                });
            }
        };
        // The root node should not be exposed - does not represent a Field.
        // "" is not a valid field name, so will never appear further down in the tree.
        let mut tree = ego_tree::Tree::new(&s.as_str()[0..0]);
        let mut stack: Vec<_> = fields
            .fields_struct
            .0
            .0
            .into_iter()
            .filter_map(|field| match field {
                parser::Field::FieldsSubstruct(parser::FieldsSubstruct {
                    field_name,
                    fields_struct,
                }) => {
                    let mut parent = tree.root_mut();
                    let current = parent.append(field_name.0);
                    let current_id = current.id();
                    Some((current_id, fields_struct.0.0))
                }
                parser::Field::FieldName(field_name) => {
                    tree.root_mut().append(field_name.0);
                    None
                }
            })
            .collect();

        while let Some((v_id, v_children)) = stack.pop() {
            let mut v = tree.get_mut(v_id).expect("all node ids are valid");
            for w in v_children {
                match w {
                    parser::Field::FieldsSubstruct(parser::FieldsSubstruct {
                        field_name,
                        fields_struct,
                    }) => {
                        let id = v.append(field_name.0).id();
                        stack.push((id, fields_struct.0.0));
                    }
                    parser::Field::FieldName(field_name) => {
                        v.append(field_name.0);
                    }
                }
            }
        }

        let tree = tree.map(|field_name| {
            StrRange::new(&s, field_name).expect("all field names are slices of the buffer s")
        });
        let negation = fields.negation;
        Ok(Self {
            buffer: s,
            negation,
            tree,
        })
    }

    /// Whether these fields should represent a denylist rather than an
    /// allowlist.
    #[must_use]
    pub fn negation(&self) -> bool {
        self.negation
    }

    /// Look up a field by its path.
    ///
    /// # Example
    ///
    /// ```
    /// let tree = z157::Tree::parse("(a(b(c)))").unwrap();
    /// let field = tree.index(&["a", "b"]).unwrap();
    /// assert_eq!(field.name(), "b");
    /// ```
    #[must_use]
    pub fn index<'p, 'i>(&'p self, path: &'i [&'i str]) -> Option<Field<'p>> {
        let mut node_ref = self.tree.root();
        for &element in path {
            if let Some(match_) = node_ref
                .children()
                .find(|child| &self.buffer.as_str()[child.value().range()] == element)
            {
                node_ref = match_;
            } else {
                return None;
            }
        }
        Some(Field {
            buffer: &self.buffer,
            node_ref,
        })
    }

    /// Iterate over all fields.
    #[must_use]
    pub fn walk(&self) -> Walk<'_> {
        Walk {
            buffer: &self.buffer,
            descendants: self.tree.root().descendants(),
        }
    }

    /// Iterate over the top-level [`Field`]s.
    #[must_use]
    pub fn top(&self) -> Children<'_> {
        Children {
            buffer: &self.buffer,
            children: self.tree.root().children(),
        }
    }
}

/// One node in the tree of fields.
#[derive(Copy, Clone)]
pub struct Field<'p> {
    buffer: &'p str,
    node_ref: ego_tree::NodeRef<'p, StrRange>,
}

impl<'p> Field<'p> {
    /// Get the field name.
    #[must_use]
    pub fn name(self) -> &'p str {
        &self.buffer[self.node_ref.value().range()]
    }

    /// Return the parent of this field if possible.
    ///
    /// Top-level fields do not have parents.
    ///
    /// # Example
    ///
    /// ```
    /// let tree = z157::Tree::parse("(a(b))").unwrap();
    /// let b = tree.index(&["a", "b"]).unwrap();
    /// let a = b.parent().unwrap();
    /// assert!(a.parent().is_none());
    /// ```
    #[must_use]
    pub fn parent(self) -> Option<Field<'p>> {
        self.node_ref
            .parent()
            // Field names are at least 1 character long, so only the root note (which is not an
            // actual field) is empty
            .filter(|parent| !parent.value().is_empty())
            .map(|node_ref| Field {
                buffer: self.buffer,
                node_ref,
            })
    }

    /// Iterate over this field's children (one level).
    #[must_use]
    pub fn children(self) -> Children<'p> {
        Children {
            buffer: self.buffer,
            children: self.node_ref.children(),
        }
    }

    /// Iterate over all descendants of this field (all levels below this
    /// level).
    #[must_use]
    pub fn walk(self) -> Walk<'p> {
        Walk {
            buffer: self.buffer,
            descendants: self.node_ref.descendants(),
        }
    }

    /// Return the path for this node.
    ///
    /// # Example
    ///
    /// ```
    /// let tree = z157::Tree::parse("(a(b))").unwrap();
    /// let b = tree.index(&["a", "b"]).unwrap();
    /// assert_eq!(["a", "b"].as_slice(), b.path());
    /// ```
    #[must_use]
    pub fn path(self) -> Vec<&'p str> {
        let mut path_list = vec![&self.buffer[self.node_ref.value().range()]];
        let mut current = self;
        while let Some(parent) = current.parent() {
            path_list.push(&self.buffer[parent.node_ref.value().range()]);
            current = parent;
        }
        path_list.reverse();
        path_list
    }
}

/// Returned when parsing of a string into a [`Tree`] fails.
#[derive(Debug)]
pub struct Unparsable {
    /// The unparsable string.
    pub unparsable: String,
    inner: parser::Error,
}

impl Unparsable {
    /// Extract the unparsable string.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.unparsable
    }
}

impl fmt::Display for Unparsable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::error::Error for Unparsable {}

/// Iterator for walking descendants of a [`Field`] or the whole [`Tree`]
/// tree.
pub struct Walk<'p> {
    buffer: &'p str,
    descendants: ego_tree::iter::Descendants<'p, StrRange>,
}

impl<'p> Iterator for Walk<'p> {
    type Item = Field<'p>;

    fn next(&mut self) -> Option<Self::Item> {
        self.descendants.next().map(|node_ref| Field {
            buffer: self.buffer,
            node_ref,
        })
    }
}

/// Iterator for traversing the children of a [`Field`].
pub struct Children<'p> {
    buffer: &'p str,
    children: ego_tree::iter::Children<'p, StrRange>,
}

impl<'p> Iterator for Children<'p> {
    type Item = Field<'p>;

    fn next(&mut self) -> Option<Self::Item> {
        self.children.next().map(|node_ref| Field {
            buffer: self.buffer,
            node_ref,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parent() {
        let tree = Tree::parse("(a(b))".to_string()).unwrap();
        let b = tree.index(&["a", "b"]).unwrap();
        let a = b.parent().unwrap();
        assert!(a.parent().is_none());
    }
}