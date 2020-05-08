pub mod collector;
pub mod core;
pub mod cpu;
pub mod disk;
pub mod errors;
pub mod mem;
pub mod network;
pub mod temp;

#[macro_use]
extern crate log;
extern crate simplelog;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
