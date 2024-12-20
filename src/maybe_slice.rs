//! This module contains the [`MaybeSlice`] pointer type which is useful for
//! self-referencing containers.
//!
//! The use case for a [`MaybeSlice`] is for keeping pointers to a buffer along
//! with the buffer, without actually having a self-referential struct.
//!
//! Instead of a type like
//!
//! ```
//! struct SelfReferential<'a> {
//!     buffer: String,
//!     references_buffer: &'a str,
//! }
//! ```
//!
//! which most of the time won't work due to challenges with pinning etc., we
//! can have
//!
//! ```ignore
//! struct KindOfSelfReferential {
//!     buffer: String,
//!     references_buffer: MaybeSlice,
//! }
//! ```
//!
//! and index the `buffer` using the `MaybeSlice`.

use std::ops::Index;

/// A reference to a [`str`] based on offsets.
///
/// A [`MaybeSlice`] can be used to index a [`str`], but it will panic if the
/// contained offsets surpasses the bounds of the `str`.
///
/// The `MaybeSlice` can only be constructed by [`MaybeSlice::new`], which fails
/// if the inner subslice does not point to within the outer slice.
#[derive(Copy, Clone, Debug)]
pub struct MaybeSlice {
    start: usize,
    end: usize,
}

impl MaybeSlice {
    /// Create a [`MaybeSlice`] from two [`str`]s.
    ///
    /// The `inner` `str` must be entirely within the `outer` `str`, otherwise
    /// this function will return `None`.
    ///
    /// The ways to create an inner slice from a `str` is usually by pointer
    /// chasing within parser code (`winnow` etc.), but can also be done with
    /// range indexing, e.g., `&"Hello, world!"[1..3]`.
    ///
    /// Using an inner `str` that matches a substring of `outer`, but does not
    /// actually point to within `outer` will also fail.
    pub fn new(outer: &str, inner: &str) -> Option<Self> {
        let outer_begin = outer.as_ptr() as usize;
        let outer_end = outer_begin + outer.len();
        let inner_begin = inner.as_ptr() as usize;
        let inner_end = inner_begin + inner.len();

        if inner_begin < outer_begin || inner_end > outer_end {
            return None;
        }

        let start = inner_begin - outer_begin;
        Some(Self {
            start,
            end: start + inner.len(),
        })
    }

    /// Check if the `MaybeSlice` is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Get the length of the `MaybeSlice`.
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

impl Index<MaybeSlice> for str {
    type Output = str;

    fn index(&self, index: MaybeSlice) -> &Self::Output {
        &self[index.start..index.end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let outer = "The quick brown fox jumps over the lazy dog";
        let ms = MaybeSlice::new(outer, &outer[0..3]).unwrap();
        assert_eq!(ms.start, 0);
        assert_eq!(ms.end, 3);
        let ms = MaybeSlice::new(outer, &outer[4..12]).unwrap();
        assert_eq!(ms.start, 4);
        assert_eq!(ms.end, 12);

        let other = "brown";
        assert!(MaybeSlice::new(outer, other).is_none());
    }

    #[test]
    fn index() {
        let outer = "The quick brown fox jumps over the lazy dog";
        let ms = MaybeSlice::new(outer, &outer[0..3]).unwrap();
        let the = &outer[ms];
        assert_eq!("The", the);
    }
}
