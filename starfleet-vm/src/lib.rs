//! Crate containing code implementing the virtual machine that controls ship systems
//! in starfleet

mod parse;
mod ast;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
