pub mod cpu;
pub mod disk;
pub mod mem;
pub mod temp;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
