pub mod z;
pub use z::Z;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
       let z = Z::new("baabaa");
       let matches = z.search("baa");
       assert_eq!(matches, [0,3]);
    }
}
