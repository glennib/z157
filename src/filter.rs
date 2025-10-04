use serde::ser::{Serialize, SerializeMap, SerializeStruct, Serializer};

use crate::Tree;

/// A wrapper that filters a value during serialization based on a field tree.
pub struct Filter<'t, T> {
    tree: &'t Tree<'t>,
    value: T,
    path: Vec<&'static str>,
}

impl<'t> Tree<'t> {
    /// Wrap a value for filtered serialization.
    ///
    /// The returned `Filter` implements `Serialize` and will only include
    /// fields that are present in the tree.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tree = Tree::parse("(name,address(city))")?;
    /// let filtered = tree.filter(&user);
    /// serde_json::to_string(&filtered)?
    /// ```
    pub fn filter<T>(&self, value: T) -> Filter<'_, T> {
        Filter {
            tree: self,
            value,
            path: vec![],
        }
    }
}


impl<'t, T: Serialize> Serialize for Filter<'t, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let filtering_serializer = FilteringSerializer {
            tree: self.tree,
            inner: serializer,
            path: &self.path,
        };
        self.value.serialize(filtering_serializer)
    }
}

struct FilteringSerializer<'t, S> {
    tree: &'t Tree<'t>,
    inner: S,
    path: &'t [&'static str],
}

impl<'t, S: Serializer> Serializer for FilteringSerializer<'t, S> {
    type Ok = S::Ok;
    type Error = S::Error;
    type SerializeSeq = S::SerializeSeq;
    type SerializeTuple = S::SerializeTuple;
    type SerializeTupleStruct = S::SerializeTupleStruct;
    type SerializeTupleVariant = S::SerializeTupleVariant;
    type SerializeMap = FilteringSerializeMap<'t, S::SerializeMap>;
    type SerializeStruct = FilteringSerializeStruct<'t, S::SerializeStruct>;
    type SerializeStructVariant = S::SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i64(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u64(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_none()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        // Wrap the value to maintain path context
        Filter {
            tree: self.tree,
            value,
            path: self.path.to_vec(),
        }
        .serialize(self.inner)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        // Wrap the value to maintain path context
        let filtered = Filter {
            tree: self.tree,
            value,
            path: self.path.to_vec(),
        };
        self.inner.serialize_newtype_struct(name, &filtered)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.inner.serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.inner.serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.inner.serialize_tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.inner.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.inner.serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(FilteringSerializeMap {
            tree: self.tree,
            inner: self.inner.serialize_map(len)?,
            path: self.path.to_vec(),
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(FilteringSerializeStruct {
            tree: self.tree,
            inner: self.inner.serialize_struct(name, len)?,
            path: self.path.to_vec(),
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.inner.serialize_struct_variant(name, variant_index, variant, len)
    }
}

struct FilteringSerializeStruct<'t, S> {
    tree: &'t Tree<'t>,
    inner: S,
    path: Vec<&'static str>,
}

impl<'t, S: SerializeStruct> SerializeStruct for FilteringSerializeStruct<'t, S> {
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // Build path for this field
        let mut field_path = self.path.clone();
        field_path.push(key);

        // Check if this field should be included
        let field_path_refs: Vec<&str> = field_path.iter().map(|s| *s).collect();

        let in_tree = self.tree.index(&field_path_refs).is_some();
        let has_descendants = self.has_descendant_in_tree(&field_path_refs);

        let should_include = if self.tree.negation() {
            // In negation mode, include if NOT in tree
            !in_tree
        } else {
            // In normal mode, include if in tree OR if a child is in tree
            in_tree || has_descendants
        };

        if should_include {
            // If this field is a leaf in the filter tree (present but no children),
            // serialize it without filtering (include all descendants)
            if in_tree && !has_descendants && !self.tree.negation() {
                // Serialize normally without filtering
                self.inner.serialize_field(key, value)?;
            } else {
                // Continue filtering nested structures
                let filtered = Filter {
                    tree: self.tree,
                    value,
                    path: field_path,
                };
                self.inner.serialize_field(key, &filtered)?;
            }
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

impl<'t, S> FilteringSerializeStruct<'t, S> {
    fn has_descendant_in_tree(&self, path: &[&str]) -> bool {
        // Check if any field in the tree starts with this path
        self.tree.walk().any(|field| {
            let field_path = field.path();
            field_path.len() > path.len()
                && field_path.iter().zip(path.iter()).all(|(a, b)| a == b)
        })
    }
}

struct FilteringSerializeMap<'t, S> {
    tree: &'t Tree<'t>,
    inner: S,
    path: Vec<&'static str>,
}

impl<'t, S: SerializeMap> SerializeMap for FilteringSerializeMap<'t, S> {
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.inner.serialize_key(key)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // For maps, we can't easily filter by key name at this level
        // Just serialize the value with current path
        let filtered = Filter {
            tree: self.tree,
            value,
            path: self.path.clone(),
        };
        self.inner.serialize_value(&filtered)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}
