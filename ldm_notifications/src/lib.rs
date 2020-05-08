pub mod core;
pub mod opsgenie;
pub mod sender;
pub mod setup;
pub mod slack;

#[macro_use]
extern crate log;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
