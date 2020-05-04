pub mod cpu;
pub mod disk;
pub mod mem;
pub mod temp;
pub mod core;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
