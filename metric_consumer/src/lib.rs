pub mod consumer;
pub mod core;
pub mod file;

#[macro_use]
extern crate log;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
