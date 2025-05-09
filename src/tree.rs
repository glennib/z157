use std::borrow::Cow;
use std::fmt;

use crate::parser;
use crate::str_range::StrRange;

/// Contains a tree of references to fields parsed from a filter string.
///
/// This struct is not so useful on its own. [Attaching](DetachedTree::attach)
/// the `DetachedTree` to a buffer via [`Tree`] will allow useful operations
/// such as indexing and walking.
///
/// See usage examples in the [crate documentation](crate).
#[derive(Debug, Clone)]
struct DetachedTree {
    /// Contains _free_ references to a string buffer. These are just offsets
    /// and lengths, not actual pointers.
    tree: ego_tree::Tree<StrRange>,
    /// Whether this tree was parsed as a denylist.
    negation: bool,
}

impl DetachedTree {
    /// Attach this freestanding [`DetachedTree`] to a string buffer or
    /// reference, which allows useful operations such as walking and
    /// indexing.
    ///
    /// # Warning
    ///
    /// It is entirely possible to attach this `DetachedTree` to a _different_
    /// string than the one used to parse the `DetachedTree`. This is
    /// immediately a mistake, and should not be done. It is _safe_ to do
    /// so, i.e., no undefined behavior will happen. However, the resulting
    /// fields will not match whatever was parsed. If a shorter string is
    /// attached, dereferencing a [`Field`] will likely cause a panic. Similar
    /// if either string is non-ASCII, since `Field` checks the substring's
    /// UTF8-compliance.
    ///
    /// A low-cost way of ensuring the same string is used for parsing and
    /// attaching has not yet been found. However, by sticking to
    /// [`parse`](DetachedTree::parse) and refraining from
    /// [`detach`](Tree::detach)ing, this situation will not occur.
    #[must_use]
    fn attach<'buffer>(self, buffer: impl Into<Cow<'buffer, str>>) -> Tree<'buffer> {
        Tree {
            buffer: buffer.into(),
            tree: self,
        }
    }

    /// See [`Tree::negation`].
    #[must_use]
    fn negation(&self) -> bool {
        self.negation
    }

    /// See [`Tree::index`].
    fn index<'tree, 'string, 'index>(
        &'tree self,
        s: &'string str,
        path: &'index [&'index str],
    ) -> Option<Field<'string>>
    where
        'tree: 'string,
    {
        let mut node_ref = self.tree.root();
        for &element in path {
            if let Some(match_) = node_ref
                .children()
                .find(|child| &s[child.value().range()] == element)
            {
                node_ref = match_;
            } else {
                return None;
            }
        }
        Some(Field {
            buffer: s,
            node_ref,
        })
    }

    /// See [`Tree::walk`].
    fn walk<'string>(&'string self, s: &'string str) -> impl Iterator<Item = Field<'string>> {
        self.tree.root().descendants().filter_map(|node_ref| {
            if node_ref.value().is_empty() {
                None
            } else {
                Some(Field {
                    buffer: s,
                    node_ref,
                })
            }
        })
    }

    /// See [`Tree::top`].
    fn top<'string>(&'string self, s: &'string str) -> impl Iterator<Item = Field<'string>> {
        self.tree.root().children().map(|node_ref| Field {
            buffer: s,
            node_ref,
        })
    }

    /// See [`Tree::leaves`].
    fn leaves<'string>(&'string self, s: &'string str) -> impl Iterator<Item = Field<'string>> {
        self.walk(s).filter(|field| !field.has_children())
    }
}

/// Contains fields parsed from a filtering string.
///
/// See usage examples in the [crate documentation](crate).
#[derive(Clone)]
pub struct Tree<'buffer> {
    buffer: Cow<'buffer, str>,
    tree: DetachedTree,
}

impl<'buffer> Tree<'buffer> {
    /// Detach the buffer from the parsed structure.
    ///
    /// Should rarely be needed. Can be re-[`attach`](DetachedTree::attach)ed.
    #[must_use]
    fn detach(self) -> (Cow<'buffer, str>, DetachedTree) {
        (self.buffer, self.tree)
    }

    /// Attempt to parse `s` into a tree of [`Field`]s.
    ///
    /// Returns a [`Tree`] which binds the buffer to a parsed structure.
    ///
    /// # Errors
    ///
    /// Returns an error if `s` does not match the expected format.
    pub fn parse(s: impl Into<Cow<'buffer, str>>) -> Result<Tree<'buffer>, Unparsable<'buffer>> {
        /// Avoids exessive code due to monomorphization.
        fn inner(cow: Cow<str>) -> Result<Tree, Unparsable> {
            let detached = Tree::parse_detached(&cow);
            match detached {
                Ok(detached) => Ok(detached.attach(cow)),
                Err(Unparsable { error, buffer }) => {
                    drop(buffer);
                    Err(Unparsable { error, buffer: cow })
                }
            }
        }
        let cow = s.into();
        inner(cow)
    }

    /// Clones the buffer if needed to produce an owned `Tree`.
    #[must_use]
    pub fn into_owned(self) -> Tree<'static> {
        let Tree { buffer, tree } = self;
        let buffer = buffer.into_owned();
        Tree {
            tree,
            buffer: Cow::Owned(buffer),
        }
    }

    /// Retrieve the parsed buffer.
    #[must_use]
    pub fn free(self) -> Cow<'buffer, str> {
        self.detach().0
    }
}

impl Tree<'_> {
    /// Attempt to parse `s` into a tree of [`Field`]s.
    ///
    /// Returns a detached [`DetachedTree`] which is not linked to the parsed
    /// string. This can cause trouble if later
    /// [`attach`](DetachedTree::attach)ing to a different string. See that
    /// method's documentation for more information. In most cases, prefer
    /// [`parse`](Self::parse).
    ///
    /// # Errors
    ///
    /// Returns an error if `s` does not match the expected format.
    #[allow(clippy::missing_panics_doc)] // panics should be impossible
    fn parse_detached(s: &str) -> Result<DetachedTree, Unparsable<'_>> {
        let fields = match parser::Fields::try_from(s) {
            Ok(fields) => fields,
            Err(error) => {
                return Err(Unparsable {
                    error,
                    buffer: Cow::Borrowed(s),
                });
            }
        };
        // The root node should not be exposed - does not represent a Field.
        // "" is not a valid field name, so will never appear further down in the tree.
        let mut tree = ego_tree::Tree::new(&s[0..0]);
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
            StrRange::new(s, field_name).expect("all field names are slices of the buffer s")
        });
        let negation = fields.negation;
        Ok(DetachedTree { tree, negation })
    }

    /// Whether these fields should represent a denylist rather than an
    /// allowlist.
    #[must_use]
    pub fn negation(&self) -> bool {
        self.tree.negation()
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
    pub fn index<'index>(&self, path: &'index [&'index str]) -> Option<Field<'_>> {
        self.tree.index(&self.buffer, path)
    }

    /// Iterate over all fields.
    pub fn walk(&self) -> impl Iterator<Item = Field<'_>> {
        self.tree.walk(&self.buffer)
    }

    /// Iterate over the top-level [`Field`]s.
    pub fn top(&self) -> impl Iterator<Item = Field<'_>> {
        self.tree.top(&self.buffer)
    }

    /// Iterate over the [`Field`]s that are leaves in the tree (i.e., fields
    /// that do not have any children).
    ///
    /// # Example
    ///
    /// ```
    /// let tree = z157::Tree::parse(
    ///     "(parent_1(child_1,parent_2(child_2)),child_3)",
    /// )
    /// .unwrap();
    /// let mut leaves: Vec<_> =
    ///     tree.leaves().map(|f| f.name()).collect();
    /// leaves.sort();
    /// assert_eq!(leaves, ["child_1", "child_2", "child_3"]);
    /// ```
    pub fn leaves(&self) -> impl Iterator<Item = Field<'_>> {
        self.tree.leaves(&self.buffer)
    }
}

/// One node in the tree of fields.
#[derive(Clone)]
pub struct Field<'p> {
    buffer: &'p str,
    node_ref: ego_tree::NodeRef<'p, StrRange>,
}

impl<'p> Field<'p> {
    /// Get the field name.
    #[must_use]
    pub fn name(&self) -> &'p str {
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
    pub fn parent(&self) -> Option<Field<'p>> {
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

    /// Iterate over this field'string children (one level).
    pub fn children(&self) -> impl Iterator<Item = Field<'p>> + use<'p> {
        self.node_ref.children().map(|node_ref| Field {
            buffer: self.buffer,
            node_ref,
        })
    }

    /// Iterate over all descendants of this field (including self).
    pub fn walk(&self) -> impl Iterator<Item = Field<'p>> + 'p + use<'p> {
        self.node_ref.descendants().map(|node_ref| Field {
            buffer: self.buffer,
            node_ref,
        })
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
    pub fn path(&self) -> Vec<&'p str> {
        let mut path_list = vec![&self.buffer[self.node_ref.value().range()]];
        let mut current = self.clone();
        while let Some(parent) = current.parent() {
            path_list.push(&self.buffer[parent.node_ref.value().range()]);
            current = parent;
        }
        path_list.reverse();
        path_list
    }

    /// Return true if this field has children.
    #[must_use]
    pub fn has_children(&self) -> bool {
        self.node_ref.has_children()
    }
}

/// Returned when parsing of a string into a [`Tree`] fails.
#[derive(Debug)]
pub struct Unparsable<'buffer> {
    error: parser::Error,
    pub buffer: Cow<'buffer, str>,
}

impl fmt::Display for Unparsable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl std::error::Error for Unparsable<'_> {}

#[derive(Debug)]
pub struct UnparsableRef {
    error: parser::Error,
}

impl fmt::Display for UnparsableRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl std::error::Error for UnparsableRef {}

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

    #[test]
    fn ego_tree_root_is_excluded_from_walk() {
        let tree = Tree::parse("(a)".to_string()).unwrap();
        let mut fields: Vec<_> = tree.walk().map(|f| f.name()).collect();
        fields.sort_unstable();
        assert_eq!(fields, ["a"]);
    }

    #[test]
    fn field_walk_works() {
        let tree = Tree::parse("(a(b(c)))".to_string()).unwrap();
        let a = tree.top().next().unwrap();
        let mut all: Vec<_> = a.walk().map(|f| f.name()).collect();
        all.sort_unstable();
        assert_eq!(all, ["a", "b", "c"]);
    }

    #[test]
    fn children_works() {
        let tree = Tree::parse("(a(b,c))".to_string()).unwrap();
        let a = tree.top().next().unwrap();
        let mut children: Vec<_> = a.children().map(|f| f.name()).collect();
        children.sort_unstable();
        assert_eq!(children, ["b", "c"]);
    }

    #[test]
    fn leaves_works() {
        let tree = Tree::parse("(a(b(c),d),e)".to_string()).unwrap();
        let mut leaves: Vec<_> = tree.leaves().map(|f| f.name()).collect();
        leaves.sort_unstable();
        assert_eq!(leaves, ["c", "d", "e"]);
    }
}
