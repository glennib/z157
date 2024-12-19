mod params;
mod parser;

pub use params::Children;
pub use params::Error;
pub use params::Param;
pub use params::Params;
pub use params::Walk;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_index() {
        let params: Params = "(a(b,c(d)),e)".parse().unwrap();
        assert!(!params.negation());
        params.index(&["a", "b"]).unwrap();
        params.index(&["a", "c"]).unwrap();
        params.index(&["a", "c", "d"]).unwrap();
        params.index(&["e"]).unwrap();
        assert!(params.index(&["a", "d"]).is_none());
    }
}
